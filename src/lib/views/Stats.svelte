<script lang="ts">
    import { formatTime } from '../format'
    import { generateShareText, handleNewGame as newGame } from '../actions'
    import { closeModal } from '../stores/modal.svelte'
    import { getStats, getSolveRate, getTimeBuckets, getGuessDistribution } from '../stores/stats.svelte'
    import { getPuzzle } from '../stores/puzzle.svelte'

    const stats = getStats()
    const puzzle = getPuzzle()

    let solveRate = $derived(getSolveRate())
    let buckets = $derived(getTimeBuckets())
    let distribution = $derived(getGuessDistribution())
    let maxDistCount = $derived(Math.max(...distribution.map(d => d.count), 1))
    let bestTimeDisplay = $derived(stats.bestTime !== null ? formatTime(stats.bestTime) : '-')

    let copied = $state(false)

    async function handleShare() {
        if (!puzzle.solved) return

        const text = generateShareText()
        try {
            await navigator.clipboard.writeText(text)
            copied = true
            setTimeout(() => { copied = false }, 2000)
        } catch {
            console.error('Failed to copy to clipboard')
        }
    }

    async function handleNewGame() {
        await newGame()
        closeModal()
    }
</script>

<div class="flex flex-col w-full h-full p-5 py-3 justify-center gap-3 items-center text-tone-text">
    <h2 class="font-semibold text-lg text-center">Stats</h2>

    <div class="flex justify-between gap-x-8 gap-y-3 flex-wrap w-full">
        <div class="flex-col text-center">
            <p class="text-xl font-bold">{stats.played}</p>
            <p class="text-xs text-tone-text-sub">Played</p>
        </div>

        <div class="flex-col text-center">
            <p class="text-xl font-bold">{stats.solved}</p>
            <p class="text-xs text-tone-text-sub">Solved</p>
        </div>

        <div class="flex-col text-center">
            <p class="text-xl font-bold">{solveRate}%</p>
            <p class="text-xs text-tone-text-sub">Solve Rate</p>
        </div>

        <div class="flex-col text-center">
            <p class="text-xl font-bold">{stats.currentStreak}</p>
            <p class="text-xs text-tone-text-sub">Streak</p>
        </div>

        <div class="flex-col text-center">
            <p class="text-xl font-bold">{stats.bestStreak}</p>
            <p class="text-xs text-tone-text-sub">Best Streak</p>
        </div>

        <div class="flex-col text-center">
            <p class="text-xl font-bold">{bestTimeDisplay}</p>
            <p class="text-xs text-tone-text-sub">Best Time</p>
        </div>
    </div>

    <hr class="w-full border-tone-border" />

    <div class="flex-col w-full">
        <p class="text-center font-semibold text-sm mb-2">Guess Distribution</p>
        <div class="dist-chart">
            {#each distribution as row}
                <div class="dist-row">
                    <span class="dist-label">{row.label}</span>
                    <div class="dist-bar-bg">
                        <div
                            class="dist-bar"
                            style="width: {Math.max(row.count / maxDistCount * 100, row.count > 0 ? 8 : 0)}%"
                        >
                            {#if row.count > 0}
                                <span class="dist-count">{row.count}</span>
                            {/if}
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    </div>

    <hr class="w-full border-tone-border" />

    <div class="flex-col justify-between w-full gap-y-4">
        <p class="text-center font-semibold text-sm">Solve Times</p>

        <div class="flex justify-between w-full mt-2">
            <div class="flex-col items-center text-center">
                <p class="text-lg font-bold">{buckets.under10}</p>
                <p class="text-xs text-tone-text-sub">{"<10s"}</p>
            </div>

            <div class="flex-col items-center text-center">
                <p class="text-lg font-bold">{buckets.under30}</p>
                <p class="text-xs text-tone-text-sub">{"<30s"}</p>
            </div>

            <div class="flex-col items-center text-center">
                <p class="text-lg font-bold">{buckets.under60}</p>
                <p class="text-xs text-tone-text-sub">{"<60s"}</p>
            </div>

            <div class="flex-col items-center text-center">
                <p class="text-lg font-bold">{buckets.over60}</p>
                <p class="text-xs text-tone-text-sub">{">60s"}</p>
            </div>
        </div>
    </div>

    <div class="flex w-full justify-center h-full items-center gap-3">
        {#if puzzle.solved}
            <button onclick={handleNewGame} class="primary-btn">
                <span class="text-sm font-semibold text-white">Next</span>
            </button>
        {/if}

        <button onclick={handleShare} class="primary-btn" disabled={!puzzle.solved}>
            <span class="text-sm font-semibold text-white">{copied ? 'Copied!' : 'Share'}</span>
        </button>
    </div>
</div>

<style>
    .dist-chart {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .dist-row {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .dist-label {
        width: 0.75rem;
        font-size: 0.75rem;
        font-weight: 700;
        text-align: right;
        flex-shrink: 0;
    }

    .dist-bar-bg {
        flex: 1;
        height: 1.25rem;
        border-radius: 2px;
    }

    .dist-bar {
        height: 100%;
        background-color: var(--tone-correct);
        border-radius: 2px;
        display: flex;
        align-items: center;
        justify-content: flex-end;
        padding-right: 0.375rem;
        min-width: 0;
        transition: width 0.3s;
    }

    .dist-count {
        font-size: 0.625rem;
        font-weight: 700;
        color: #ffffff;
    }

    .primary-btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 0.625rem 1.5rem;
        border-radius: 9999px;
        cursor: pointer;
        background-color: var(--tone-correct);
        color: #ffffff;
        border: none;
        transition: opacity 0.15s;
    }

    .primary-btn:hover:not(:disabled) {
        opacity: 0.9;
    }

    .primary-btn:disabled {
        opacity: 0.4;
        cursor: not-allowed;
    }
</style>
