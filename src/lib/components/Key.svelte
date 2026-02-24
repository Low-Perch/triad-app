<script lang="ts">
    import { updateGameState } from '../stores/app'
    import { getPuzzle, markPuzzleSolved } from '../stores/puzzle.svelte'
    import { getInput, addKey, removeKey, updateInputState, InputState } from '../stores/input.svelte'

    import { validSolution } from '../utils/validation'

    let { key, width = 8, disabled = false }: { key: string, width?: number, disabled?: boolean } = $props()

    const input = getInput()
    const puzzle = getPuzzle()

    async function handleKey(e: Event) {
        e.preventDefault()
        e.stopPropagation()

        if (puzzle.solved) return

        const button = e.currentTarget as HTMLButtonElement
        const keyName = button.name.toUpperCase()

        if (keyName == "GO") {
            const solved = validSolution({ input: input.keys, key: puzzle.key })
            const inputState = solved ? InputState.CORRECT : InputState.INCORRECT
            solved && markPuzzleSolved()
            return updateInputState(inputState)
        }

        updateInputState(InputState.EDIT)
        keyName == "DEL" ? removeKey() : addKey(keyName)

        await updateGameState({ key: 'input', state: $state.snapshot(input) })
    }
</script>

<button
    name={key}
    {disabled}
    style="width: {width == 8 ? '2rem' : '3rem'}"
    onclick={handleKey}
    class="h-10 rounded-sm bg-gray-700 hover:bg-gray-500 disabled:bg-gray-400 disabled:cursor-auto"
>
    <span class="text-sm font-semibold uppercase w-full">{key}</span>
</button>
