<script lang="ts">
    import { onMount } from 'svelte'

    import { initClues } from './lib/stores/clues'
    import { initKeys, getKeys } from './lib/stores/keys'
    import { initModal, getModal } from './lib/stores/modal'
    import { getPuzzle, initPuzzle, markPuzzleSolved } from './lib/stores/puzzle'
    import { initInput, addKey, removeKey, getInput, InputState, updateInputState } from './lib/stores/input'

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
    const puzzle = getPuzzle()

    function handleKeyboard(e: KeyboardEvent) {
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
    }

    $: puzzleText = $puzzle[$puzzle.state]
    $: disabledKeys = $keys.disabledKeys

    onMount(() => {
        window.addEventListener('keydown', handleKeyboard)

        return () => {
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
