<script lang="ts">
    import * as bridge from '../bridge'
    import { closeModal } from '../stores/modal.svelte'
    import { getClues, setClues } from '../stores/clues.svelte'
    import { setInput } from '../stores/input.svelte'
    import { getPuzzle, setPuzzle } from '../stores/puzzle.svelte'
    import { setKeys } from '../stores/keys.svelte'
    import { getDropdown, toggleDropdown } from '../stores/dropdown.svelte'

    import DropdownItem from './DropdownItem.svelte'

    const clues = getClues()
    const dropdown = getDropdown()

    function toggleClues(e?: Event) {
        e?.preventDefault()
        e?.stopPropagation()

        closeModal()
        toggleDropdown()
    }

    async function handleClue(detail: { id: string }) {
        toggleClues()

        const result = await bridge.activateClue(detail.id)
        setClues(result.clues)
        setInput(result.input)
        setPuzzle(result.puzzle)
        setKeys(result.keys)
    }
</script>

<div class="group relative inline-block">
    <button
        id="support"
        name="support"
        disabled={!clues?.available}
        onclick={toggleClues}
        aria-label="Lifelines"
        aria-haspopup="true"
        aria-expanded={dropdown.open}
        class="w-8 h-8 flex items-center place-content-center cursor-pointer disabled:cursor-auto rounded hover:bg-tone-key-active disabled:bg-transparent transition-colors duration-150"
    >
        <svg xmlns="http://www.w3.org/2000/svg" height="18" viewBox="0 0 64 64" width="18">
            <g class="{clues.available ? 'fill-tone-text' : 'fill-tone-text-sub'}">
                <path d="m32 16c2.961 0 5.699.859 8.078 2.262l11.52-11.52c-5.418-4.211-12.207-6.742-19.598-6.742-7.395 0-14.18 2.531-19.602 6.742l11.52 11.523c2.379-1.406 5.121-2.265 8.082-2.265z"/>
                <path d="m57.262 12.406-11.524 11.516c1.403 2.379 2.262 5.117 2.262 8.078s-.859 5.703-2.262 8.078l11.52 11.52c4.211-5.418 6.742-12.207 6.742-19.598s-2.531-14.18-6.738-19.594z"/>
                <path d="m40.078 45.738c-2.379 1.403-5.117 2.262-8.078 2.262s-5.699-.859-8.078-2.262l-11.52 11.52c5.418 4.211 12.207 6.742 19.598 6.742s14.18-2.531 19.598-6.742z"/>
                <path d="m16 32c0-2.961.859-5.699 2.262-8.078l-11.52-11.52c-4.211 5.418-6.742 12.207-6.742 19.598 0 7.395 2.531 14.18 6.742 19.602l11.52-11.523c-1.403-2.376-2.262-5.118-2.262-8.079z"/>
            </g>
        </svg>
    </button>

    <ul
        role="menu"
        aria-label="Available lifelines"
        class="dropdown-menu"
        class:hidden={!dropdown.open}
    >
        {#each clues.clues as clue (clue.id) }
            <DropdownItem clue={clue} onMessage={handleClue} />
        {/each}
    </ul>
</div>

<style>
    .dropdown-menu {
        position: absolute;
        z-index: 20;
        margin-top: 0.5rem;
        left: -1.5rem;
        width: 9rem;
        list-style: none;
        font-size: 0.875rem;
        border-radius: 6px;
        border: 1px solid var(--tone-border);
        background-color: var(--tone-surface);
        color: var(--tone-text);
        overflow: hidden;
    }
</style>
