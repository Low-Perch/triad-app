<script lang="ts">
    import { createEventDispatcher } from 'svelte'

    export let clue: { id: string, note: string, active: boolean }

    const dispatch = createEventDispatcher()

    function passClue() {
        dispatch('message', { id: clue.id })
    }
</script>

<li>
    <button 
        id={clue.id}
        on:click={passClue}
        disabled={clue.active}
        class="w-full px-2.5 py-1 cursor-pointer text-left bg-gray-700 hover:bg-gray-200 hover:text-black disabled:text-white disabled:cursor-auto disabled:bg-transparent disabled:line-through"
    >
        {clue.note}
    </button>
</li>
