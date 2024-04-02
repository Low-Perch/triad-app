<script lang="ts">
    import { onMount } from 'svelte'

    import { initClues } from './lib/stores/clues'
    import { initModal, getModal } from './lib/stores/modal'
    import { initInput, addKey, removeKey } from './lib/stores/input'

    import Modal from './lib/components/Modal.svelte'
    import Header from './lib/components/Header.svelte'
    import Clues from './lib/components/Clues.svelte'
    import Input from './lib/components/Input.svelte'
    import Keys from './lib/components/Keys.svelte'

    initModal()
    initInput()
    initClues()

    const modal = getModal()

    function handleKeyboard(e: KeyboardEvent) {
        const allowedKeys = ['Enter', 'Backspace', 'Delete']
        const alphaRegex = /^[a-zA-Z]$/

        const allowedInput = allowedKeys.includes(e.key) || alphaRegex.test(e.key)
        if (!allowedInput) return

        if (e.key == 'Enter') {
            // TODO
        } else if (['Backspace', 'Delete'].includes(e.key)) {
            removeKey()
        } else {
            addKey(e.key.toUpperCase())
        }
    }

    onMount(() => {
        window.addEventListener('keydown', handleKeyboard)

        return () => {
            window.removeEventListener('keydown', handleKeyboard)
        }
    })
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
