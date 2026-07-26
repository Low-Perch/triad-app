<script lang="ts">
    import Info from '../views/Info.svelte'
    import Stats from '../views/Stats.svelte'
    import Archive from '../views/Archive.svelte'

    import { closeModal, getModal } from '../stores/modal.svelte'

    let { onpostNewGame }: { onpostNewGame?: () => void } = $props()

    const modal = getModal()

    const VIEW_LABELS = { info: 'How to play', stats: 'Statistics', archive: 'Archive' } as const

    // Move focus into the dialog on open so keyboard and screen-reader
    // users land inside it
    function focusOnMount(el: HTMLElement) {
        el.focus()
    }

    function handleKey(evt: KeyboardEvent) {
        if (!modal.visible) return
        if (!['Esc', 'Escape'].includes(evt.key)) return

        evt.preventDefault()
        closeModal()
    }
</script>

<svelte:body onkeydown={handleKey} />

<div
    id="modal"
    role="presentation"
    class="modal-overlay"
    onclick={(e) => { if (e.target === e.currentTarget) closeModal() }}
>
    <div
        class="modal-content"
        role="dialog"
        aria-modal="true"
        aria-label={modal.view ? VIEW_LABELS[modal.view] : 'Dialog'}
        tabindex="-1"
        {@attach focusOnMount}
    >
        <button class="close-btn" onclick={closeModal} aria-label="Close">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
        </button>
        {#if modal?.view == "info"}
            <Info />
        {:else if modal?.view == "stats"}
            <Stats {onpostNewGame} />
        {:else if modal?.view == "archive"}
            <Archive {onpostNewGame} />
        {/if}
    </div>
</div>

<style>
    .modal-overlay {
        position: fixed;
        inset: 0;
        z-index: 100;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: var(--tone-overlay);
    }

    .close-btn {
        position: absolute;
        top: 0.5rem;
        right: 0.5rem;
        z-index: 110;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 2rem;
        height: 2rem;
        border: none;
        border-radius: 6px;
        background: transparent;
        color: var(--tone-text-sub);
        cursor: pointer;
        transition: background-color 0.15s, color 0.15s;
    }

    .close-btn:hover {
        background-color: var(--tone-key);
        color: var(--tone-text);
    }

    .modal-content {
        position: relative;
        width: 100%;
        max-width: 400px;
        max-height: 90vh;
        overflow-y: auto;
        background-color: var(--tone-surface);
        border-radius: 8px;
        padding: 1.5rem;
    }

    /* Programmatic focus target only — not a tab stop, no ring needed */
    .modal-content:focus {
        outline: none;
    }
</style>
