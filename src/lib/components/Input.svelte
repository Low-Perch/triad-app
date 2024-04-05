<script lang="ts">
    import { keyLocked } from '../stores/clues'
    import { getPuzzle } from '../stores/puzzle'
    import { getInput, InputState } from '../stores/input'

    const input = getInput()
    const puzzle = getPuzzle()

    $: correct = $input.state == InputState.CORRECT
    $: incorrect = $input.state == InputState.INCORRECT
    $: showLock = $keyLocked && !$puzzle.solved
</script>

<div class="flex justify-center items-center my-8 h-40">
    {#each $input.keys as key, i (i)}
        <div 
            class:solved={correct}
            class:shake={incorrect}
            class:bg-green-500={correct}
            class="relative box flex m-4 aspect-square border-white border-2 w-12 h-12 justify-center place-items-center"
        >
            <p class="text-white uppercase text-3xl">{key}</p>

            {#if showLock && i == $input.keys.length - 1}
                <svg viewBox="0 0 24 24" class="absolute bg-transparent -bottom-4 w-6 h-6 fill-white">
                    <path d="m18 8h-1v-2c0-2.76-2.24-5-5-5s-5 2.24-5 5v2h-1c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2v-10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9h-6.2v-2c0-1.71 1.39-3.1 3.1-3.1s3.1 1.39 3.1 3.1z"/>
                </svg>
            {/if}
        </div>
    {/each}
</div>

<style>
    .box {
        transition: scale .1s, color .2s, background-color .2s, transform .6s;
        transition-delay: 0s, .1s, .1s, 0s;
    }

    .solved { 
        @apply bg-[#38c346] border-[#38c346];
        transform: rotateY(2turn);
    }

    .shake {
        animation: shakeAnimation .3s ease-in-out;
    }

    @keyframes shakeAnimation {
        0% {
            transform: translateX(0)
        }

        25% {
            transform: translateX(-7px)
        }

        50% {
            transform: translateX(7px)
        }

        75% {
            transform: translateX(-7px)
        }

        to {
            transform: translateX(0)
        }
    }
</style>
