<script lang="ts">
    import * as bridge from '../bridge'
    import { getPuzzle, setPuzzle } from '../stores/puzzle.svelte'
    import { getInput, setInput } from '../stores/input.svelte'
    import { setStats } from '../stores/stats.svelte'

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
            const result = await bridge.submitSolution()
            setInput({ ...input, state: result.inputState })
            if (result.solved) {
                setPuzzle({ ...puzzle, solved: true })
            }
            setStats(result.stats)
            return
        }

        const updatedInput = keyName == "DEL"
            ? await bridge.removeKey()
            : await bridge.addKey(keyName)

        setInput(updatedInput)
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
