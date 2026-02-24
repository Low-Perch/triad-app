<script lang="ts">
    import { closeModal } from '../stores/modal.svelte'
    import { getStats, getSolveRate, getTimeBuckets } from '../stores/stats.svelte'

    const stats = getStats()

    let solveRate = $derived(getSolveRate())
    let buckets = $derived(getTimeBuckets())
    let bestTimeDisplay = $derived(stats.bestTime !== null ? `${stats.bestTime}s` : '-')
</script>

<div class="flex flex-col w-full h-full text-white p-5 py-3 justify-center gap-3 items-center">
    <h2 class="font-semibold text-lg text-center">Stats</h2>

    <div class="flex justify-between gap-x-12 gap-y-4 flex-wrap w-full">
        <div class="flex-col">
            <p class="text-md font-semibold text-center">{stats.played}</p>
            <p class="text-md">Played</p>
        </div>

        <div class="flex-col">
            <p class="text-md font-semibold text-center">{stats.solved}</p>
            <p class="text-md">Solved</p>
        </div>

        <div class="flex-col">
            <p class="text-md font-semibold text-center">{solveRate}%</p>
            <p class="text-md">Solve Rate</p>
        </div>

        <div class="flex-col">
            <p class="text-lg font-semibold text-center">{stats.currentStreak}</p>
            <p class="text-md">Solve Streak</p>
        </div>

        <div class="flex-col">
            <p class="text-lg font-semibold text-center">{bestTimeDisplay}</p>
            <p class="text-md">Best Time</p>
        </div>
    </div>

    <div class="flex-col justify-between w-full gap-y-16">
        <p class="text-center font-semibold text-sm">Solve Times</p>

        <div class="flex justify-between w-full">
            <div class="flex-col items-center">
                <p class="text-lg font-semibold text-center">{buckets.under10}</p>
                <p class="text-md text-center">{"<10s"}</p>
            </div>

            <div class="flex-col items-center">
                <p class="text-lg font-semibold text-center">{buckets.under30}</p>
                <p class="text-md text-center">{"<30s"}</p>
            </div>

            <div class="flex-col items-center">
                <p class="text-lg font-semibold text-center">{buckets.under60}</p>
                <p class="text-md text-center">{"<60s"}</p>
            </div>

            <div class="flex-col items-center">
                <p class="text-lg font-semibold text-center">{buckets.over60}</p>
                <p class="text-md text-center">{">60s"}</p>
            </div>
        </div>
    </div>

    <div class="flex w-full justify-between h-full items-center">
        <button
            onclick={closeModal}
            class="items-center inline-flex justify-center w-32 h-10 border-2 rounded-3xl border-white">
            <span class="text-md">View Puzzle</span>
        </button>

        <button
            onclick={closeModal}
            class="items-center inline-flex bg-[#26b726] justify-center w-24 h-10 border-2 rounded-3xl border-[#26b726]">
            <span class="text-md text-white">Share</span>
        </button>
    </div>
</div>
