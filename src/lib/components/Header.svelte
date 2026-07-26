<script lang="ts">
    import { getModal, updateModal, type ModalView } from '../stores/modal.svelte'
    import { closeDropdown } from '../stores/dropdown.svelte'
    import { getTheme, toggleTheme } from '../stores/theme.svelte'

    import Dropdown from './Dropdown.svelte'

    const modal = getModal()

    const TITLE = 'triad'.split('')

    function displayModal(e: MouseEvent) {
        e.preventDefault()
        e.stopPropagation()

        const button = e.currentTarget as HTMLButtonElement
        const viewName = button.name as ModalView

        const visible = modal?.view == viewName ? false : true
        const view = modal?.view == viewName ? null : viewName

        updateModal({ view, visible })
        closeDropdown()
    }

    let isDark = $derived(getTheme() === 'dark')
</script>

<header class="flex w-full items-center relative px-2 py-2 border-b border-tone-border">
    <div class="flex gap-x-1.5">
        {#each TITLE as char (char)}
            <p class="inline-flex justify-center items-center uppercase font-bold border-tone-border-strong border-2 w-6 h-6 aspect-square leading-none p-2 text-xs text-tone-text">
                {char}
            </p>
        {/each}
    </div>

    <div class="relative flex justify-end items-center w-full space-x-1">
        <Dropdown />

        <button aria-label="Puzzle archive" class="icon-btn" id="archive" name="archive" onclick={displayModal}>
            <svg class="fill-tone-text" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24"><path d="M19 4h-1V2h-2v2H8V2H6v2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2zm.002 16H5V8h14l.002 12z"/><path d="M7 10h4v4H7z"/></svg>
        </button>

        <button aria-label="How to play" class="icon-btn" id="info" name="info" onclick={displayModal}>
            <svg class="fill-tone-text" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"><path d="M12 6C9.831 6 8.066 7.765 8.066 9.934h2C10.066 8.867 10.934 8 12 8s1.934.867 1.934 1.934c0 .598-.481 1.032-1.216 1.626-.255.207-.496.404-.691.599C11.029 13.156 11 14.215 11 14.333V15h2l-.001-.633c.001-.016.033-.386.441-.793.15-.15.339-.3.535-.458.779-.631 1.958-1.584 1.958-3.182C15.934 7.765 14.169 6 12 6zM11 16H13V18H11z"/><path d="M12,2C6.486,2,2,6.486,2,12s4.486,10,10,10s10-4.486,10-10S17.514,2,12,2z M12,20c-4.411,0-8-3.589-8-8s3.589-8,8-8 s8,3.589,8,8S16.411,20,12,20z"/></svg>
        </button>

        <button aria-label="View statistics" class="icon-btn" id="stats" name="stats" onclick={displayModal}>
            <svg class="fill-tone-text" xmlns="http://www.w3.org/2000/svg" height="18" viewBox="0 0 512 512" width="18">
                <path d="m128 496h-80v-192h80z"/>
                <path d="m352 496h-80v-288h80z"/>
                <path d="m464 496h-80v-400h80z"/>
                <path d="m240 496h-80v-480h80z"/>
            </svg>
        </button>

        <button aria-label="Toggle theme" class="icon-btn" onclick={toggleTheme}>
            {#if isDark}
                <svg class="fill-tone-text" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24"><path d="M6.995 12c0 2.761 2.246 5.007 5.007 5.007s5.007-2.246 5.007-5.007-2.246-5.007-5.007-5.007S6.995 9.239 6.995 12zM11 19h2v3h-2zm0-17h2v3h-2zm-9 9h3v2H2zm17 0h3v2h-3zM5.637 19.778l-1.414-1.414 2.121-2.121 1.414 1.414zM16.242 6.344l2.122-2.122 1.414 1.414-2.122 2.122zM6.344 7.759 4.223 5.637l1.415-1.414 2.12 2.122zm13.434 10.605-1.414 1.414-2.122-2.122 1.414-1.414z"/></svg>
            {:else}
                <svg class="fill-tone-text" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24"><path d="M12 11.807A9.002 9.002 0 0 1 10.049 2a9.942 9.942 0 0 0-5.12 2.735c-3.905 3.905-3.905 10.237 0 14.142 3.906 3.906 10.237 3.905 14.143 0a9.946 9.946 0 0 0 2.735-5.119A9.003 9.003 0 0 1 12 11.807z"/></svg>
            {/if}
        </button>
    </div>
</header>

<style lang="postcss">
    .icon-btn {
        @apply w-8 h-8 flex items-center place-content-center rounded cursor-pointer;
        @apply hover:bg-tone-key-active;
        transition: background-color 0.15s;
    }
</style>
