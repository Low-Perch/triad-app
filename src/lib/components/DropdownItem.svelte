<script lang="ts">
    import { getSolveClueAvailable } from '../stores/clues.svelte'

    type Props = {
        clue: { id: string, note: string, active: boolean },
        onMessage: (detail: { id: string }) => void
    }

    let { clue, onMessage }: Props = $props()

    let isDisabled = $derived(
        clue.active || (clue.id === 'solve' && !getSolveClueAvailable())
    )

    function passClue() {
        onMessage({ id: clue.id })
    }
</script>

<li role="none" class:solve-separator={clue.id === 'solve'}>
    <button
        role="menuitem"
        id={clue.id}
        onclick={passClue}
        disabled={isDisabled}
        class="dropdown-item"
    >
        {clue.note}
    </button>
</li>

<style>
    .solve-separator {
        border-top: 1px solid var(--tone-border);
    }

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
