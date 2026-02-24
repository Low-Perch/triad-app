<script lang="ts">
    import { onMount } from 'svelte'
    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

    import * as bridge from './lib/bridge'

    import { getModal, openModal } from './lib/stores/modal.svelte'
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

    let loading = $state(true)

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
                openModal('stats')
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
        <Keys {disabledKeys} />
    {/if}
</main>
