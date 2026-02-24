<script lang="ts">
    type Props = {
        clue: { id: string, note: string, active: boolean },
        onMessage: (detail: { id: string }) => void
    }

    let { clue, onMessage }: Props = $props()

    function passClue() {
        onMessage({ id: clue.id })
    }
</script>

<li>
    <button
        id={clue.id}
        onclick={passClue}
        disabled={clue.active}
        class="w-full px-2.5 py-1 cursor-pointer text-left bg-gray-700 hover:bg-gray-200 hover:text-black disabled:text-white disabled:cursor-auto disabled:bg-transparent disabled:line-through"
    >
        {clue.note}
    </button>
</li>
