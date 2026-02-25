<script lang="ts">
    import { generateShareText } from '../actions'
    import { getPuzzle } from '../stores/puzzle.svelte'
    import { getClues } from '../stores/clues.svelte'
    import { getGuesses } from '../stores/guesses.svelte'
    import { getStats } from '../stores/stats.svelte'

    let {
        mode,
        onnewgame,
        onviewstats,
        ondismiss,
    }: {
        mode: 'solved-today' | 'congrats' | 'failed'
        onnewgame: () => void
        onviewstats: () => void
        ondismiss?: () => void
    } = $props()

    const TITLE = 'triad'.split('')
    const puzzle = getPuzzle()
    const clues = getClues()
    const stats = getStats()

    let squares = $derived(
        clues.clues.map(c => {
            if (c.id === 'solve') return c.active ? 'dark' : 'correct'
            return c.active ? 'used' : 'correct'
        })
    )

    let solveUsed = $derived(clues.clues.find(c => c.id === 'solve')?.active)
    let guessDisplay = $derived(solveUsed ? 'X' : String(getGuesses()))

    let copied = $state(false)

    async function handleShare() {
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

<div class="result-screen">
    <div class="section" style="animation-delay: 0ms">
        <div class="logo-tiles">
            {#each TITLE as char, i (i)}
                <p class="logo-tile">{char}</p>
            {/each}
        </div>
    </div>

    <div class="section" style="animation-delay: 150ms">
        {#if mode === 'congrats'}
            <div class="celebration">
                <svg class="checkmark" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="20 6 9 17 4 12"></polyline>
                </svg>
                <p class="status-text">Solved!</p>
            </div>
        {:else if mode === 'failed'}
            <div class="answer-reveal">
                <p class="status-label">The answer was</p>
                <p class="answer-text">{puzzle.key}</p>
            </div>
        {:else}
            {#if puzzle.puzzleNumber !== null}
                <p class="puzzle-number">Puzzle #{puzzle.puzzleNumber}</p>
            {/if}
        {/if}
    </div>

    <div class="section" style="animation-delay: 300ms">
        <div class="squares-row">
            {#each squares as sq, i (i)}
                <div class="square {sq}"></div>
            {/each}
        </div>
        <p class="guess-text">{guessDisplay}/6</p>
    </div>

    <div class="section" style="animation-delay: 450ms">
        {#if mode === 'failed'}
            <p class="streak-label">Streak reset</p>
        {:else if stats.currentStreak > 0}
            <p class="streak-label">Streak: {stats.currentStreak}</p>
        {/if}
    </div>

    <div class="section actions" style="animation-delay: 550ms">
        <button class="action-btn" onclick={onviewstats}>
            <span class="text-sm font-semibold">Stats</span>
        </button>
        <button class="action-btn" onclick={handleShare}>
            <span class="text-sm font-semibold">{copied ? 'Copied!' : 'Share'}</span>
        </button>
        <button class="action-btn" onclick={onnewgame}>
            <span class="text-sm font-semibold">Next</span>
        </button>
    </div>

    {#if mode === 'solved-today' && ondismiss}
        <button class="dismiss-link section" style="animation-delay: 650ms" onclick={ondismiss}>
            View Puzzle
        </button>
    {/if}
</div>

<style>
    .result-screen {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        gap: 1.25rem;
        padding: 2rem 1rem;
    }

    .section {
        opacity: 0;
        animation: fadeSlideUp 0.4s ease-out forwards;
    }

    .logo-tiles {
        display: flex;
        gap: 0.375rem;
    }

    .logo-tile {
        width: 2rem;
        height: 2rem;
        display: flex;
        align-items: center;
        justify-content: center;
        text-transform: uppercase;
        font-weight: 700;
        font-size: 0.75rem;
        color: var(--tone-text);
        border: 2px solid var(--tone-border-strong);
    }

    .celebration {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5rem;
    }

    .checkmark {
        width: 2.5rem;
        height: 2.5rem;
        color: var(--tone-correct);
        animation: popIn 0.3s ease-out;
    }

    .status-text {
        font-size: 1.25rem;
        font-weight: 700;
        color: var(--tone-text);
    }

    .answer-reveal {
        text-align: center;
    }

    .status-label {
        font-size: 0.875rem;
        color: var(--tone-text-sub);
    }

    .answer-text {
        font-size: 1.5rem;
        font-weight: 700;
        color: var(--tone-text);
        letter-spacing: 0.15em;
        text-transform: uppercase;
        margin-top: 0.25rem;
    }

    .puzzle-number {
        font-size: 1.125rem;
        font-weight: 600;
        color: var(--tone-text);
    }

    .squares-row {
        display: flex;
        gap: 0.375rem;
        justify-content: center;
        margin-bottom: 0.375rem;
    }

    .square {
        width: 1.5rem;
        height: 1.5rem;
        border-radius: 2px;
    }

    .square.correct { background-color: var(--tone-correct); }
    .square.used { background-color: var(--tone-border-strong); }
    .square.dark { background-color: var(--tone-border); }

    .guess-text {
        font-size: 1rem;
        font-weight: 600;
        color: var(--tone-text);
        text-align: center;
    }

    .streak-label {
        font-size: 0.875rem;
        color: var(--tone-text-sub);
        text-align: center;
    }

    .actions {
        display: flex;
        gap: 0.75rem;
    }

    .action-btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 0.625rem 1.25rem;
        border-radius: 9999px;
        cursor: pointer;
        background-color: var(--tone-correct);
        color: #ffffff;
        border: none;
        transition: opacity 0.15s;
    }

    .action-btn:hover {
        opacity: 0.9;
    }

    .dismiss-link {
        background: none;
        border: none;
        color: var(--tone-text-sub);
        font-size: 0.75rem;
        cursor: pointer;
        text-decoration: underline;
    }

    .dismiss-link:hover {
        color: var(--tone-text);
    }

    @keyframes fadeSlideUp {
        0% { opacity: 0; transform: translateY(8px); }
        100% { opacity: 1; transform: translateY(0); }
    }

    @keyframes popIn {
        0% { transform: scale(0); }
        60% { transform: scale(1.15); }
        100% { transform: scale(1); }
    }
</style>
