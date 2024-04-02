<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    export let clue: { id: string, note: string, active: boolean }

    const dispatch = createEventDispatcher()

    function passClue() {
        dispatch('message', {
            clueType: clue.id
        })
    }
</script>

<li class="hover:text-black disabled:cursor-auto w-full py-1 px-2.5 cursor-pointer bg-gray-700 hover:bg-gray-200 disabled:bg-gray-400">
    <button 
        id={clue.id}
        on:click={passClue}
        disabled={clue.active}
    >
        {clue.note}
    </button>
</li>
