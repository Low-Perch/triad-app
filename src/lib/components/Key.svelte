<script lang="ts">
    import { getPuzzle, markPuzzleSolved } from '../stores/puzzle'
    import { getInput, addKey, removeKey, updateInputState, InputState } from '../stores/input'

    import { validSolution } from '../utils/validation'

    export let key: string;
    export let width: number = 8
    export let disabled: boolean = false

    const input = getInput()
    const puzzle = getPuzzle()

    function handleKey(e: Event)  {
        if ($puzzle.solved) return

        const button = e.currentTarget as HTMLButtonElement
        const key = button.name.toUpperCase()

        if (key == "GO") {
            const solved = validSolution({ input: $input.keys, key: $puzzle.key })
            const inputState = solved ? InputState.CORRECT : InputState.INCORRECT
            solved && markPuzzleSolved()
            return updateInputState(inputState)
        } 

        updateInputState(InputState.EDIT)
        key == "DEL" ? removeKey() : addKey(key)
    }
</script>

<button 
    name={key}
    disabled={disabled}
    style="width: {width == 8 ? '2rem' : '3rem'}"
    on:click|preventDefault|stopPropagation={handleKey}
    class="h-10 rounded-sm bg-gray-700 hover:bg-gray-500 disabled:bg-gray-400 disabled:cursor-auto"
>
    <span class="text-sm font-semibold uppercase w-full">{key}</span>
</button>
