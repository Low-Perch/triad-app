<script lang="ts">
    import { initModal, getModal } from './lib/stores/modal'

    import Modal from "./lib/components/Modal.svelte"
    import Header from "./lib/components/Header.svelte"

    initModal()

    const modal = getModal()
</script>

<main class="container w-full h-full">
    <Header />
    {#if $modal.visible}
        <Modal />
    {/if}
</main>
