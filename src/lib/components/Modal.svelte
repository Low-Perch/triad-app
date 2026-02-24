<script lang="ts">
    import Info from '../views/Info.svelte'
    import Stats from '../views/Stats.svelte'

    import { closeModal, getModal } from '../stores/modal.svelte'
    const modal = getModal()

    function handleKey(evt: KeyboardEvent) {
        if (!modal.visible) return
        if (!['Esc', 'Escape'].includes(evt.key)) return

        evt.preventDefault()
        closeModal()
    }
</script>

<svelte:body onkeydown={handleKey} />

<!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
<div class="backdrop" onclick={closeModal}></div>

<div
    id="modal"
    role="alertdialog"
    aria-modal="true"
    class="modal-overlay"
>
    <div class="modal-content">
        <div class="relative flex w-full h-full justify-center">
            {#if modal?.view == "info"}
                <Info />
            {:else if modal?.view == "stats"}
                <Stats />
            {/if}
        </div>
    </div>
</div>

<style>
    .backdrop {
        position: fixed;
        inset: 0;
        z-index: 90;
        background-color: var(--tone-overlay);
    }

    .modal-overlay {
        position: fixed;
        width: 83.333%;
        height: 83.333%;
        margin-top: 0.75rem;
        overflow-y: hidden;
        top: 50%;
        transform: translateY(-50%);
        z-index: 100;
        inset-inline: 0;
        margin-inline: auto;
        border: 1px solid var(--tone-border);
        border-radius: 8px;
    }

    .modal-content {
        position: absolute;
        height: 100%;
        z-index: 50;
        width: 100%;
        margin-inline: auto;
        background-color: var(--tone-surface);
        border-radius: 8px;
    }
</style>
