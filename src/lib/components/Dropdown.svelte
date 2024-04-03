<script lang="ts">
    import { closeModal } from '../stores/modal'
    import { lockClueKey } from '../stores/input'
    import { getClues, activateClue } from '../stores/clues'
    import { updatePuzzleState, PuzzleState, getPuzzle } from '../stores/puzzle'

    import DropdownItem from './DropdownItem.svelte'

    const clues = getClues()
    const puzzle = getPuzzle()

    function toggleClues() {
        const dropdown = document.getElementById('dropdown') as HTMLButtonElement
        if (!dropdown) return

        closeModal()

        const isHidden = dropdown.classList.contains("hidden")
        isHidden ? dropdown.classList.remove("hidden") : dropdown.classList.add("hidden")
    }

    function handleClue(e: CustomEvent<{ id: string }>) {
        const clueId = e.detail.id

        toggleClues()

        activateClue(clueId)
        if (clueId == "letter") {
            const clueKey = $puzzle.key.at(-1)
            lockClueKey(clueKey)
        } else if (clueId == "position") {
            updatePuzzleState(PuzzleState.CLUE)
        } else {
            // TODO
        }
    }
</script>

<div tabIndex="0" class="group relative inline-block">
    <button 
        id="support"
        name="support"
        disabled={!$clues?.available}
        on:click|preventDefault|stopPropagation={toggleClues}
        class="w-9 h-9 flex items-center place-content-center cursor-pointer disabled:cursor-auto hover:rounded-full disabled:rounded-none hover:bg-gray-600 disabled:bg-transparent"
    >
        <svg xmlns="http://www.w3.org/2000/svg" height="20" viewBox="0 0 64 64" width="20">
            <g class="{$clues.available ? 'fill-white' : 'fill-gray-500'}">
                <path d="m32 16c2.961 0 5.699.859 8.078 2.262l11.52-11.52c-5.418-4.211-12.207-6.742-19.598-6.742-7.395 0-14.18 2.531-19.602 6.742l11.52 11.523c2.379-1.406 5.121-2.265 8.082-2.265z"/>
                <path d="m57.262 12.406-11.524 11.516c1.403 2.379 2.262 5.117 2.262 8.078s-.859 5.703-2.262 8.078l11.52 11.52c4.211-5.418 6.742-12.207 6.742-19.598s-2.531-14.18-6.738-19.594z"/>
                <path d="m40.078 45.738c-2.379 1.403-5.117 2.262-8.078 2.262s-5.699-.859-8.078-2.262l-11.52 11.52c5.418 4.211 12.207 6.742 19.598 6.742s14.18-2.531 19.598-6.742z"/>
                <path d="m16 32c0-2.961.859-5.699 2.262-8.078l-11.52-11.52c-4.211 5.418-6.742 12.207-6.742 19.598 0 7.395 2.531 14.18 6.742 19.602l11.52-11.523c-1.403-2.376-2.262-5.118-2.262-8.079z"/>
            </g>
        </svg>
    </button>

    <ul id="dropdown" class="absolute z-20 mt-2 hidden bg-gray-700 border-gray-200 border-2 text-sm -left-6 w-36 text-white list-none">
        {#each $clues.clues as clue (clue.id) }
            <DropdownItem clue={clue} on:message={handleClue} />
        {/each}
    </ul>
</div>
