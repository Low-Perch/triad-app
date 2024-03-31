<script lang="ts">
    import Info from '../views/Info.svelte'
    import { closeModal, getModal } from '../stores/modal'
    const modal = getModal()

    function handleKey(evt: KeyboardEvent) {
        if (!$modal.visible) return
        if (!['Esc', 'Escape'].includes(evt.key)) return

        closeModal()
    }
</script>

<svelte:body on:keydown|preventDefault={handleKey} />

<div
    id="modal"
    role="alertdialog"
    aria-modal="true"
    class="fixed w-5/6 h-5/6 mt-3 outline-white border-2 border-white overflow-y-hidden top-1/2 -translate-y-1/2 z-[100] inset-0 mx-auto"
>
    <div
        id="modal-content"
        class="absolute h-full z-50 w-full mx-auto bg-transparent border-white"
    >

        <div class="relative flex w-full h-full justify-center">
            {#if $modal?.view == "info"}
                <Info />
            {/if}
        </div>
    </div>
</div>
