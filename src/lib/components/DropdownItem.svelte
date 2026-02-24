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

<li role="none">
    <button
        role="menuitem"
        id={clue.id}
        onclick={passClue}
        disabled={clue.active}
        class="dropdown-item"
    >
        {clue.note}
    </button>
</li>

<style>
    .dropdown-item {
        width: 100%;
        padding: 0.375rem 0.75rem;
        cursor: pointer;
        text-align: left;
        background-color: transparent;
        color: var(--tone-text);
        transition: background-color 0.1s;
    }

    .dropdown-item:hover:not(:disabled) {
        background-color: var(--tone-key);
    }

    .dropdown-item:disabled {
        color: var(--tone-text-sub);
        cursor: auto;
        text-decoration: line-through;
    }
</style>
