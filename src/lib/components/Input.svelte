<script lang="ts">
    import { getKeyLocked } from '../stores/clues.svelte'
    import { getPuzzle } from '../stores/puzzle.svelte'
    import { getInput, InputState } from '../stores/input.svelte'

    let { revealing = false }: { revealing?: boolean } = $props()

    const input = getInput()
    const puzzle = getPuzzle()

    let correct = $derived(input.state == InputState.CORRECT)
    let incorrect = $derived(input.state == InputState.INCORRECT)
    let showLock = $derived(getKeyLocked() && !puzzle.solved)
</script>

<div class="flex justify-center items-center my-6 h-24">
    {#each input.keys as key, i (i)}
        <div
            class:solved={correct}
            class:shake={incorrect}
            class:filled={key !== ''}
            class:tile-reveal={revealing}
            class="tile"
            style={revealing ? `--tile-index: ${i}` : ''}
        >
            <p class="text-tone-text uppercase text-3xl font-bold">{key}</p>

            {#if showLock && i == input.keys.length - 1}
                <svg viewBox="0 0 24 24" class="absolute bg-transparent -bottom-4 w-5 h-5 fill-tone-text-sub">
                    <path d="m18 8h-1v-2c0-2.76-2.24-5-5-5s-5 2.24-5 5v2h-1c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2v-10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9h-6.2v-2c0-1.71 1.39-3.1 3.1-3.1s3.1 1.39 3.1 3.1z"/>
                </svg>
            {/if}
        </div>
    {/each}
</div>

<style>
    .tile {
        @apply relative flex m-3 aspect-square w-14 h-14 justify-center place-items-center;
        border: 2px solid var(--tone-border);
        transition: scale .1s, border-color .2s, background-color .2s, transform .6s;
        transition-delay: 0s, .1s, .1s, 0s;
    }

    .tile.filled {
        border-color: var(--tone-border-strong);
    }

    .solved {
        background-color: var(--tone-correct);
        border-color: var(--tone-correct);
        transform: rotateY(2turn);
    }

    .solved p {
        color: #ffffff;
    }

    .shake {
        animation: shakeAnimation .3s ease-in-out;
    }

    @keyframes shakeAnimation {
        0% { transform: translateX(0) }
        25% { transform: translateX(-7px) }
        50% { transform: translateX(7px) }
        75% { transform: translateX(-7px) }
        to { transform: translateX(0) }
    }

    .tile-reveal {
        opacity: 0;
        animation: tileSlideUp 0.3s ease-out forwards;
        animation-delay: calc(var(--tile-index, 0) * 100ms);
    }

    @keyframes tileSlideUp {
        0% { opacity: 0; transform: translateY(20px); }
        100% { opacity: 1; transform: translateY(0); }
    }
</style>
