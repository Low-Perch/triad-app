<script lang="ts">
    import { getInput, InputState } from '../stores/input'

    const input = getInput()

    $: correct = $input.state == InputState.CORRECT
    $: incorrect = $input.state == InputState.INCORRECT
</script>

<div class="flex justify-center items-center my-8 h-40">
    {#each $input.keys as key, i (i)}
        <div 
            class:solved={correct}
            class:shake={incorrect}
            class:bg-green-500={correct}
            class="box flex m-4 aspect-square border-white border-2 w-12 h-12 justify-center place-items-center"
        >
            <p class="text-white uppercase text-3xl">{key}</p>
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
