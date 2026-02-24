<script lang="ts">
    import { onMount } from 'svelte'
    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

    import * as bridge from './lib/bridge'

    import { getModal } from './lib/stores/modal.svelte'
    import { getKeys, setKeys } from './lib/stores/keys.svelte'
    import { getClues, setClues } from './lib/stores/clues.svelte'
    import { getPuzzle, setPuzzle } from './lib/stores/puzzle.svelte'
    import { setInput, getInput } from './lib/stores/input.svelte'
    import { getStats, setStats } from './lib/stores/stats.svelte'
    import { initTheme } from './lib/stores/theme.svelte'

    import Keys from './lib/components/Keys.svelte'
    import Modal from './lib/components/Modal.svelte'
    import Clues from './lib/components/Clues.svelte'
    import Input from './lib/components/Input.svelte'
    import Header from './lib/components/Header.svelte'

    const appWindow = getCurrentWebviewWindow()

    const keys = getKeys()
    const input = getInput()
    const modal = getModal()
    const puzzle = getPuzzle()

    const stats = getStats()
    const clues = getClues()

    let loading = $state(true)
    let copied = $state(false)

    async function handleKeyboard(e: KeyboardEvent) {
        if (puzzle.solved) return

        const allowedKeys = ['Enter', 'Backspace', 'Delete']
        const alphaRegex = /^[a-zA-Z]$/

        const allowedInput = allowedKeys.includes(e.key) || alphaRegex.test(e.key)
        if (!allowedInput) return

        if (e.key == 'Enter') {
            const result = await bridge.submitSolution()
            setInput({ ...input, state: result.inputState })
            setStats(result.stats)
            if (result.solved) {
                setPuzzle({ ...puzzle, solved: true, state: result.puzzleState })
            } else {
                setTimeout(async () => {
                    const clearedInput = await bridge.clearInput()
                    setInput(clearedInput)
                }, 350)
            }
            return
        }

        const updatedInput = ['Backspace', 'Delete'].includes(e.key)
            ? await bridge.removeKey()
            : await bridge.addKey(e.key.toUpperCase())

        setInput(updatedInput)
    }

    function generateShareText(): string {
        const squares = clues.clues.map(c => c.active ? '🟨' : '🟩').join('')
        const lines = [`Triad ${squares}`]

        const latestTime = stats.solveTimes.length > 0
            ? stats.solveTimes[stats.solveTimes.length - 1]
            : null

        const parts: string[] = []
        if (latestTime !== null) parts.push(`⏱️ ${latestTime}s`)
        if (stats.currentStreak > 0) parts.push(`🔥 ${stats.currentStreak}`)
        if (parts.length > 0) lines.push(parts.join(' | '))

        return lines.join('\n')
    }

    async function handleShare() {
        const text = generateShareText()
        try {
            await navigator.clipboard.writeText(text)
            copied = true
            setTimeout(() => { copied = false }, 2000)
        } catch {
            console.error('Failed to copy to clipboard')
        }
    }

    async function handleNewGame() {
        const game = await bridge.newGame()
        setPuzzle(game.puzzle)
        setClues(game.clues)
        setInput(game.input)
        setKeys(game.keys)
        setStats(game.stats)
    }

    let puzzleText = $derived(puzzle[puzzle.state])
    let disabledKeys = $derived(keys.disabledKeys)
    let solveTimeDisplay = $derived.by(() => {
        if (stats.solveTimes.length === 0) return null
        const t = stats.solveTimes[stats.solveTimes.length - 1]
        if (t < 60) return `${t}s`
        const mins = Math.floor(t / 60)
        const secs = t % 60
        return secs > 0 ? `${mins}m ${secs}s` : `${mins}m`
    })

    let unlistenClose: (() => void) | undefined
    let unlistenFocus: (() => void) | undefined

    async function init() {
        try {
            const game = await bridge.initGame()
            setPuzzle(game.puzzle)
            setClues(game.clues)
            setInput(game.input)
            setKeys(game.keys)
            setStats(game.stats)
        } catch (error) {
            console.error('Failed to load game state, using defaults:', error)
        }

        try {
            unlistenClose = await appWindow.onCloseRequested(async () => {
                await bridge.saveGame()
            })

            unlistenFocus = await appWindow.onFocusChanged(async () => {
                await bridge.saveGame()
            })
        } catch (error) {
            console.error('Failed to register window event listeners:', error)
        }

        loading = false
    }

    onMount(() => {
        initTheme()
        init()

        window.addEventListener('keydown', handleKeyboard)

        return () => {
            unlistenClose?.()
            unlistenFocus?.()
            window.removeEventListener('keydown', handleKeyboard)
        }
    })
</script>

<main class="absolute flex-col w-full h-full">
    {#if !loading}
        {#if modal.visible}
            <Modal />
        {/if}

        <Header />
        <Clues text={puzzleText} />
        <Input />

        {#if puzzle.solved}
            <div class="post-solve">
                {#if solveTimeDisplay !== null}
                    <p class="solve-time">{solveTimeDisplay}</p>
                {/if}
                <div class="post-solve-actions">
                    <button onclick={handleNewGame} class="action-btn">
                        <span class="text-sm font-semibold">Next</span>
                    </button>
                    <button onclick={handleShare} class="action-btn">
                        <span class="text-sm font-semibold">{copied ? 'Copied!' : 'Share'}</span>
                    </button>
                </div>
            </div>
        {:else}
            <Keys {disabledKeys} />
        {/if}
    {/if}
</main>

<style>
    .post-solve {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.75rem;
    }

    .solve-time {
        font-size: 1.25rem;
        font-weight: 700;
        color: var(--tone-text-sub);
    }

    .post-solve-actions {
        display: flex;
        gap: 0.75rem;
    }

    .action-btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 0.625rem 1.5rem;
        border-radius: 9999px;
        cursor: pointer;
        background-color: var(--tone-correct);
        color: #ffffff;
        border: none;
        transition: opacity 0.15s;
    }

    .action-btn:hover {
        opacity: 0.9;
    }
</style>
