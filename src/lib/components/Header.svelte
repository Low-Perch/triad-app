<script lang="ts">
    import { getModal, updateModal, type ModalView } from '../stores/modal'
    import Dropdown from './Dropdown.svelte'

    const modal = getModal()

    function displayModal(e: MouseEvent) {
        const button = e.currentTarget as HTMLButtonElement
        const viewName = button.name as ModalView

        const visible = $modal?.view == viewName ? false : true
        const view = $modal?.view == viewName ? null : viewName

        updateModal({ view, visible })

        const dropdown = getDropdown()
        dropdown?.classList?.add("hidden")
    }

    function getDropdown(): HTMLButtonElement | null {
        return document.getElementById('dropdown') as HTMLButtonElement
    }
</script>

<header class="flex w-full items-center relative p-4 py-2">
    <h1 class="inline-flex text-white text-3xl tracking-widest font-sans font-light">
        Triad
    </h1>

    <div class="relative flex justify-end items-center w-full space-x-2">
        <Dropdown />

        <button class="w-9 h-9 flex items-center place-content-center hover:rounded-full hover:bg-gray-600" id="info" name="info" on:click|preventDefault|stopPropagation={displayModal}>
            <svg class="fill-white" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 6C9.831 6 8.066 7.765 8.066 9.934h2C10.066 8.867 10.934 8 12 8s1.934.867 1.934 1.934c0 .598-.481 1.032-1.216 1.626-.255.207-.496.404-.691.599C11.029 13.156 11 14.215 11 14.333V15h2l-.001-.633c.001-.016.033-.386.441-.793.15-.15.339-.3.535-.458.779-.631 1.958-1.584 1.958-3.182C15.934 7.765 14.169 6 12 6zM11 16H13V18H11z"/><path d="M12,2C6.486,2,2,6.486,2,12s4.486,10,10,10s10-4.486,10-10S17.514,2,12,2z M12,20c-4.411,0-8-3.589-8-8s3.589-8,8-8 s8,3.589,8,8S16.411,20,12,20z"/></svg>
        </button>

        <button class="w-9 h-9 flex items-center place-content-center hover:rounded-full hover:bg-gray-600" id="stats" name="stats" on:click|preventDefault|stopPropagation={displayModal}>
            <svg class="fill-white" xmlns="http://www.w3.org/2000/svg" height="20" viewBox="0 0 512 512" width="20">
                <path d="m128 496h-80v-192h80z"/>
                <path d="m352 496h-80v-288h80z"/>
                <path d="m464 496h-80v-400h80z"/>
                <path d="m240 496h-80v-480h80z"/>
            </svg>
        </button>
    </div>
</header>
