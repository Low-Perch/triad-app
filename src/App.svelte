<script lang="ts">
    import { onMount } from 'svelte'
    import { appWindow } from '@tauri-apps/api/window'

    import { getGameState, updateGameState, initGameState } from './lib/stores/app'

    import { initModal, getModal } from './lib/stores/modal'
    import { initKeys, getKeys, setKeys } from './lib/stores/keys'
    import { getClues, initClues, setClues } from './lib/stores/clues'
    import { getPuzzle, setPuzzle, initPuzzle, markPuzzleSolved } from './lib/stores/puzzle'
    import { initInput, setInput, addKey, removeKey, getInput, InputState, updateInputState } from './lib/stores/input'

    import Keys from './lib/components/Keys.svelte'
    import Modal from './lib/components/Modal.svelte'
    import Clues from './lib/components/Clues.svelte'
    import Input from './lib/components/Input.svelte'
    import Header from './lib/components/Header.svelte'

    import { validSolution } from './lib/utils/validation'

    initKeys()
    initModal()
    initInput()
    initClues()
    initPuzzle()

    const keys = getKeys()
    const input = getInput()
    const modal = getModal()
    const clues = getClues()
    const puzzle = getPuzzle()

    async function handleKeyboard(e: KeyboardEvent) {
        if ($puzzle.solved) return

        const allowedKeys = ['Enter', 'Backspace', 'Delete']
        const alphaRegex = /^[a-zA-Z]$/

        const allowedInput = allowedKeys.includes(e.key) || alphaRegex.test(e.key)
        if (!allowedInput) return

        if (e.key == 'Enter') {
            const solved = validSolution({ input: $input.keys, key: $puzzle.key })
            const inputState = solved ? InputState.CORRECT : InputState.INCORRECT
            solved && markPuzzleSolved()
            return updateInputState(inputState)
        }

        if ($input.state !== InputState.EDIT) updateInputState(InputState.EDIT)
        ;['Backspace', 'Delete'].includes(e.key) ? removeKey() : addKey(e.key.toUpperCase())

        await updateGameState({ key: 'input', state: $input })
    }

    async function saveGameState() {
        await initGameState({ puzzle: $puzzle, clues: $clues, input: $input, keys: $keys, })
    }

    $: puzzleText = $puzzle[$puzzle.state]
    $: disabledKeys = $keys.disabledKeys

    onMount(async () => {
        const game = await getGameState()

        if (!game) {
            await saveGameState()
        } else {
            setPuzzle(game.puzzle)
            setClues(game.clues)
            setInput(game.input)
            setKeys(game.keys)
        }

        const unlistenMenu = await appWindow.onMenuClicked(async () => {
            await saveGameState()
        })

        const unlistenClose = await appWindow.onCloseRequested(async () => {
            await saveGameState()
        })

        const unlistenFocus = await appWindow.onFocusChanged(async () => {
            await saveGameState()
        })

        window.addEventListener('keydown', handleKeyboard)

        return () => {
            unlistenMenu()
            unlistenClose()
            unlistenFocus()

            window.removeEventListener('keydown', handleKeyboard)
        }
    })
</script>

<main class="absolute flex-col w-full h-full">
    {#if $modal.visible}
        <Modal />
    {/if}

    <Header />
    <Clues text={puzzleText} />
    <Input />
    <Keys disabledKeys={disabledKeys} />
</main>
