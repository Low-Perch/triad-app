<script lang="ts">
    import { onMount } from 'svelte'
    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

    import { getGameState, updateGameState, initGameState } from './lib/stores/app'

    import { getModal } from './lib/stores/modal.svelte'
    import { getKeys, setKeys } from './lib/stores/keys.svelte'
    import { getClues, setClues } from './lib/stores/clues.svelte'
    import { getPuzzle, setPuzzle, markPuzzleSolved } from './lib/stores/puzzle.svelte'
    import { setInput, addKey, removeKey, getInput, InputState, updateInputState } from './lib/stores/input.svelte'

    import Keys from './lib/components/Keys.svelte'
    import Modal from './lib/components/Modal.svelte'
    import Clues from './lib/components/Clues.svelte'
    import Input from './lib/components/Input.svelte'
    import Header from './lib/components/Header.svelte'

    import { validSolution } from './lib/utils/validation'

    const appWindow = getCurrentWebviewWindow()

    const keys = getKeys()
    const input = getInput()
    const modal = getModal()
    const clues = getClues()
    const puzzle = getPuzzle()

    async function handleKeyboard(e: KeyboardEvent) {
        if (puzzle.solved) return

        const allowedKeys = ['Enter', 'Backspace', 'Delete']
        const alphaRegex = /^[a-zA-Z]$/

        const allowedInput = allowedKeys.includes(e.key) || alphaRegex.test(e.key)
        if (!allowedInput) return

        if (e.key == 'Enter') {
            const solved = validSolution({ input: input.keys, key: puzzle.key })
            const inputState = solved ? InputState.CORRECT : InputState.INCORRECT
            solved && markPuzzleSolved()
            return updateInputState(inputState)
        }

        if (input.state !== InputState.EDIT) updateInputState(InputState.EDIT)
        ;['Backspace', 'Delete'].includes(e.key) ? removeKey() : addKey(e.key.toUpperCase())

        await updateGameState({ key: 'input', state: $state.snapshot(input) })
    }

    async function saveGameState() {
        await initGameState({
            puzzle: $state.snapshot(puzzle),
            clues: $state.snapshot(clues),
            input: $state.snapshot(input),
            keys: $state.snapshot(keys),
        })
    }

    let puzzleText = $derived(puzzle[puzzle.state])
    let disabledKeys = $derived(keys.disabledKeys)

    let unlistenClose: (() => void) | undefined
    let unlistenFocus: (() => void) | undefined

    async function init() {
        const game = await getGameState()

        if (!game) {
            await saveGameState()
        } else {
            setPuzzle(game.puzzle)
            setClues(game.clues)
            setInput(game.input)
            setKeys(game.keys)
        }

        unlistenClose = await appWindow.onCloseRequested(async () => {
            await saveGameState()
        })

        unlistenFocus = await appWindow.onFocusChanged(async () => {
            await saveGameState()
        })
    }

    onMount(() => {
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
        <Modal />
    {/if}

    <Header />
    <Clues text={puzzleText} />
    <Input />
    <Keys {disabledKeys} />
</main>
