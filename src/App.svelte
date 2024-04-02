<script lang="ts">
    import { initModal, getModal } from './lib/stores/modal'

    import Modal from "./lib/components/Modal.svelte"
    import Header from "./lib/components/Header.svelte"
    import Clues from './lib/components/Clues.svelte';
    import Input from './lib/components/Input.svelte';
    import Keys from './lib/components/Keys.svelte';

    initModal()

    const modal = getModal()
</script>

<main class="absolute flex-col w-full h-full">
    {#if $modal.visible}
        <Modal />
    {/if}

    <Header />
    <Clues />
    <Input />
    <Keys />
</main>
