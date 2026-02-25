<script lang="ts">
    import { onMount } from 'svelte'
    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

    import * as bridge from './lib/bridge'
    import { handleNewGame } from './lib/actions'

    import { getModal, openModal } from './lib/stores/modal.svelte'
    import { getKeys, setKeys } from './lib/stores/keys.svelte'
    import { setClues } from './lib/stores/clues.svelte'
    import { getPuzzle, setPuzzle } from './lib/stores/puzzle.svelte'
    import { setInput, getInput } from './lib/stores/input.svelte'
    import { setStats } from './lib/stores/stats.svelte'
    import { getGuesses, setGuesses } from './lib/stores/guesses.svelte'
    import { initTheme } from './lib/stores/theme.svelte'

    import Keys from './lib/components/Keys.svelte'
    import Modal from './lib/components/Modal.svelte'
    import Clues from './lib/components/Clues.svelte'
    import Input from './lib/components/Input.svelte'
    import Header from './lib/components/Header.svelte'
    import SplashScreen from './lib/components/SplashScreen.svelte'
    import ResultScreen from './lib/components/ResultScreen.svelte'

    type AppPhase = 'loading' | 'splash' | 'solved-today' | 'revealing' | 'playing' | 'congrats' | 'failed' | 'error'

    const appWindow = getCurrentWebviewWindow()

    const keys = getKeys()
    const input = getInput()
    const modal = getModal()
    const puzzle = getPuzzle()

    let phase = $state<AppPhase>('loading')

    async function handleKeyboard(e: KeyboardEvent) {
        if (phase !== 'playing') return
        if (puzzle.solved) return

        const allowedKeys = ['Enter', 'Backspace', 'Delete']
        const alphaRegex = /^[a-zA-Z]$/

        const allowedInput = allowedKeys.includes(e.key) || alphaRegex.test(e.key)
        if (!allowedInput) return

        if (e.key == 'Enter') {
            const result = await bridge.submitSolution()
            setInput({ ...input, state: result.inputState })
            setStats(result.stats)
            setGuesses(result.guesses)
            if (result.solved || result.exhausted) {
                setPuzzle({ ...puzzle, solved: true, state: result.puzzleState })
                if (result.exhausted) {
                    setInput({ ...input, keys: puzzle.key.split(''), state: result.inputState, disabled: true })
                }
                setTimeout(() => {
                    phase = result.solved ? 'congrats' : 'failed'
                }, 700)
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

    let puzzleText = $derived(puzzle[puzzle.state])
    let disabledKeys = $derived(keys.disabledKeys)

    let unlistenClose: (() => void) | undefined
    let unlistenFocus: (() => void) | undefined

    function startReveal() {
        phase = 'revealing'
        setTimeout(() => { phase = 'playing' }, 800)
    }

    function handleSplashDone() {
        if (puzzle.solved) {
            phase = 'solved-today'
        } else if (input.keys.some(k => k !== '')) {
            phase = 'playing'
        } else {
            startReveal()
        }
    }

    async function handleNewGameFromResult() {
        await handleNewGame()
        startReveal()
    }

    async function init() {
        try {
            const game = await bridge.initGame()
            setPuzzle(game.puzzle)
            setClues(game.clues)
            setInput(game.input)
            setKeys(game.keys)
            setStats(game.stats)
            setGuesses(game.guesses)
        } catch (e) {
            console.error('Failed to load game state:', e)
            phase = 'error'
            return
        }

        try {
            unlistenClose = await appWindow.onCloseRequested(async () => {
                await bridge.saveGame()
            })

            unlistenFocus = await appWindow.onFocusChanged(async () => {
                await bridge.saveGame()
            })
        } catch (e) {
            console.error('Failed to register window event listeners:', e)
        }

        phase = 'splash'
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
    {#if modal.visible}
        <Modal onpostNewGame={startReveal} />
    {/if}

    {#if phase === 'error'}
        <div class="error-screen">
            <p class="text-tone-text font-semibold">Failed to load game</p>
            <button onclick={init} class="action-btn">
                <span class="text-sm font-semibold">Retry</span>
            </button>
        </div>
    {:else if phase === 'splash'}
        <SplashScreen ondone={handleSplashDone} />
    {:else if phase === 'solved-today' || phase === 'congrats' || phase === 'failed'}
        <ResultScreen
            mode={phase}
            onviewstats={() => openModal('stats')}
            onnewgame={handleNewGameFromResult}
            ondismiss={phase === 'solved-today' ? () => { phase = 'playing' } : undefined}
        />
    {:else if phase === 'revealing' || phase === 'playing'}
        <Header />
        <div class="relative">
            {#if puzzle.puzzleNumber !== null}
                <p class="absolute -top-9 right-3 text-xs text-tone-text-sub">#{puzzle.puzzleNumber}</p>
            {/if}
            <Clues text={puzzleText} revealing={phase === 'revealing'} />
        </div>
        <Input revealing={phase === 'revealing'} />
        <p class="guess-counter">{getGuesses()}/6</p>
        {#if !puzzle.solved}
            <Keys {disabledKeys} />
        {/if}
    {/if}
</main>

<style>
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

    .guess-counter {
        text-align: center;
        font-size: 0.75rem;
        font-weight: 600;
        color: var(--tone-text-sub);
        margin-bottom: 0.25rem;
    }

    .error-screen {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        gap: 1rem;
    }
</style>
