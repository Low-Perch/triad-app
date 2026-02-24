<script lang="ts">
    import { closeModal } from '../stores/modal.svelte'
</script>

<div class="flex flex-col w-full h-full p-5 justify-center gap-3 items-center text-tone-text">
    <h2 class="font-semibold text-lg text-center">How to Play</h2>

    <p class="text-sm">
        Given 3 clue fragments, find the word fragment that completes all 3 clues to form full words.
    </p>

    <div>
        <p class="font-semibold text-sm">Example:</p>
        <p class="text-sm">TUN/GON/IN + DRA = TUNDRA/DRAGON/DRAIN</p>
    </div>

    <hr class="w-full border-tone-border" />

    <ul class="list-disc list-inside text-sm space-y-1">
        <li>Any of the 3 clue fragments may be starting or ending fragments. Their order is random.</li>
        <li>If you feel stuck, use any of the 3 lifelines available in the top right corner.</li>
    </ul>

    <div class="flex justify-center h-full items-center">
        <button onclick={closeModal} class="modal-btn">
            <span class="text-sm font-semibold">Play</span>
        </button>
    </div>
</div>

<style>
    .modal-btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 6rem;
        height: 2.5rem;
        border-radius: 9999px;
        font-weight: 600;
        cursor: pointer;
        background-color: var(--tone-correct);
        color: #ffffff;
        transition: opacity 0.15s;
    }

    .modal-btn:hover {
        opacity: 0.9;
    }
</style>
