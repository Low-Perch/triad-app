<script lang="ts">
    import * as bridge from '$lib/bridge'
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

        const updatedInput = keyName == "DEL"
            ? await bridge.removeKey()
            : await bridge.addKey(keyName)

        setInput(updatedInput)
    }
</script>

<button
    name={key}
    {disabled}
    onclick={handleKey}
    class="game-key"
    class:wide={width == 12}
>
    <span class="text-xs font-bold uppercase w-full">{key}</span>
</button>

<style>
    .game-key {
        height: 3rem;
        width: 2rem;
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: var(--tone-key);
        color: var(--tone-key-text);
        cursor: pointer;
        user-select: none;
        transition: background-color 0.1s;
    }

    .game-key:hover:not(:disabled) {
        background-color: var(--tone-key-active);
    }

    .game-key:disabled {
        opacity: 0.4;
        cursor: auto;
    }

    .game-key.wide {
        width: 3.25rem;
        font-size: 0.7rem;
    }

    @media (min-width: 768px) {
        .game-key {
            height: 3.5rem;
            width: 2.75rem;
        }

        .game-key span {
            font-size: 0.875rem;
        }

        .game-key.wide {
            width: 4.25rem;
        }
    }
</style>
