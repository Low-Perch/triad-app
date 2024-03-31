<script lang="ts">
    import { closeModal, getModal } from '../stores/modal'
    const modal = getModal()

    function handleKey(evt: KeyboardEvent) {
        if (!$modal.visible) return
        if (!['Esc', 'Escape'].includes(evt.key)) return

        closeModal()
    }

    $: visible = $modal.visible
</script>

<svelte:body on:keydown|preventDefault={handleKey} />

<div
    id="modal"
    role="alertdialog"
    aria-modal="true"
    class="fixed w-5/6 h-5/6 mt-6 outline-white border-2 border-white {visible
        ? 'block overflow-y-hidden'
        : 'hidden overflow-y-hidden'} top-1/2 -translate-y-1/2 z-[100] inset-0 flex w-full h-full mx-auto items-center content-center place-content-center"
>
    <div
        id="modal-content"
        class="absolute z-50 w-full mx-auto bg-transparent border-white bg-white"
    >

        <div class="relative flex w-full h-full justify-center">
            <h1>Modal</h1>
        </div>
    </div>
</div>
