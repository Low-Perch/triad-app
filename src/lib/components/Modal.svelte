<script lang="ts">
    import Info from '../views/Info.svelte'
    import Stats from '../views/Stats.svelte'

    import { closeModal, getModal } from '../stores/modal.svelte'

    let { onpostNewGame }: { onpostNewGame?: () => void } = $props()

    const modal = getModal()

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
    role="alertdialog"
    aria-modal="true"
    class="modal-overlay"
>
    <button class="close-btn" onclick={closeModal} aria-label="Close">
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
    </button>
    <div class="modal-content">
        {#if modal?.view == "info"}
            <Info />
        {:else if modal?.view == "stats"}
            <Stats {onpostNewGame} />
        {/if}
    </div>
</div>

<style>
    .modal-overlay {
        position: fixed;
        inset: 0;
        z-index: 100;
        overflow-y: auto;
        background-color: var(--tone-surface);
    }

    .close-btn {
        position: absolute;
        top: 0.75rem;
        right: 0.75rem;
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
        width: 100%;
        min-height: 100%;
        display: flex;
        justify-content: center;
    }
</style>
