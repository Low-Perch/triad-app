<script lang="ts">
    import { closeModal } from '../stores/modal.svelte'
    import { getStats, getSolveRate, getTimeBuckets } from '../stores/stats.svelte'
    import { getClues } from '../stores/clues.svelte'
    import { getPuzzle } from '../stores/puzzle.svelte'

    const stats = getStats()
    const clues = getClues()
    const puzzle = getPuzzle()

    let solveRate = $derived(getSolveRate())
    let buckets = $derived(getTimeBuckets())
    let bestTimeDisplay = $derived(stats.bestTime !== null ? `${stats.bestTime}s` : '-')

    let copied = $state(false)

    function generateShareText(): string {
        const squares = clues.clues.map(c => c.active ? '🟨' : '🟩').join('')
        const lines = [`Triad ${squares}`]

        const latestTime = stats.solveTimes.length > 0
            ? stats.solveTimes[stats.solveTimes.length - 1]
            : null

        const parts: string[] = []
        if (latestTime !== null) parts.push(`⏱️ ${latestTime}s`)
        if (stats.currentStreak > 0) parts.push(`🔥 ${stats.currentStreak}`)
        if (parts.length > 0) lines.push(parts.join(' | '))

        return lines.join('\n')
    }

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
            <p class="text-xl font-bold">{bestTimeDisplay}</p>
            <p class="text-xs text-tone-text-sub">Best Time</p>
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

    <div class="flex w-full justify-between h-full items-center gap-3">
        <button onclick={closeModal} class="outline-btn">
            <span class="text-sm font-semibold">View Puzzle</span>
        </button>

        <button onclick={handleShare} class="primary-btn" disabled={!puzzle.solved}>
            <span class="text-sm font-semibold text-white">{copied ? 'Copied!' : 'Share'}</span>
        </button>
    </div>
</div>

<style>
    .outline-btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 8rem;
        height: 2.5rem;
        border-radius: 9999px;
        cursor: pointer;
        border: 1px solid var(--tone-border-strong);
        color: var(--tone-text);
        background: transparent;
        transition: background-color 0.15s;
    }

    .outline-btn:hover {
        background-color: var(--tone-key);
    }

    .primary-btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 6rem;
        height: 2.5rem;
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
