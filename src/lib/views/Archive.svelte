<script lang="ts">
    import { onMount } from 'svelte'

    import * as bridge from '$lib/bridge'
    import { handleArchiveGame } from '../actions'
    import { closeModal } from '../stores/modal.svelte'
    import { getStats } from '../stores/stats.svelte'
    import { localDateString, PUZZLE_EPOCH } from '../date'
    import type { History } from '../types'

    let { onpostNewGame }: { onpostNewGame?: () => void } = $props()

    const stats = getStats()

    const MONTHS = [
        'January', 'February', 'March', 'April', 'May', 'June',
        'July', 'August', 'September', 'October', 'November', 'December'
    ]
    const WEEKDAYS = ['S', 'M', 'T', 'W', 'T', 'F', 'S']

    // First puzzle date is the 2026-07-27 epoch (July 2026 = month index 6)
    const MIN_MONTH_INDEX = 2026 * 12 + 6

    const today = localDateString()
    const todayYear = Number(today.slice(0, 4))
    const todayMonth = Number(today.slice(5, 7)) - 1
    const MAX_MONTH_INDEX = todayYear * 12 + todayMonth

    let history = $state<History>({})
    let view = $state({ year: todayYear, month: todayMonth })
    let error = $state('')

    onMount(async () => {
        try {
            history = (await bridge.getHistory()) ?? {}
        } catch (e) {
            console.error('Failed to load puzzle history:', e)
        }
    })

    const monthOptions: { key: string; label: string }[] = []
    for (let idx = MAX_MONTH_INDEX; idx >= MIN_MONTH_INDEX; idx--) {
        const y = Math.floor(idx / 12)
        const m = idx % 12
        monthOptions.push({ key: monthKey(y, m), label: `${MONTHS[m]} ${y}` })
    }

    function monthKey(year: number, month: number): string {
        return `${year}-${String(month + 1).padStart(2, '0')}`
    }

    function dateKey(year: number, month: number, day: number): string {
        return `${monthKey(year, month)}-${String(day).padStart(2, '0')}`
    }

    type DayStatus = 'solved' | 'failed' | 'open' | 'today' | 'future'
    type Cell = {
        day: number
        date: string
        status: DayStatus
        perfect: boolean
        playable: boolean
        linkLeft: boolean
        linkRight: boolean
        edgeLeft: boolean
        edgeRight: boolean
        chainPos: number
    }

    function statusFor(date: string): DayStatus {
        // Pre-epoch days have no puzzle; records from before the series
        // relaunch are hidden with them
        if (date < PUZZLE_EPOCH || date > today) return 'future'
        const rec = history[date]
        if (rec) return rec.solved ? 'solved' : 'failed'
        return date === today ? 'today' : 'open'
    }

    const viewKey = $derived(monthKey(view.year, view.month))
    const canPrev = $derived(view.year * 12 + view.month > MIN_MONTH_INDEX)
    const canNext = $derived(view.year * 12 + view.month < MAX_MONTH_INDEX)

    const grid = $derived.by(() => {
        const { year, month } = view
        const dayCount = new Date(year, month + 1, 0).getDate()
        const offset = new Date(year, month, 1).getDay()

        const statuses: DayStatus[] = []
        for (let d = 1; d <= dayCount; d++) {
            statuses.push(statusFor(dateKey(year, month, d)))
        }

        const cells: Cell[] = []
        let run = 0
        for (let d = 1; d <= dayCount; d++) {
            const date = dateKey(year, month, d)
            const solved = statuses[d - 1] === 'solved'
            const col = (offset + d - 1) % 7
            const linkLeft = solved && d > 1 && statuses[d - 2] === 'solved'
            const linkRight = solved && d < dayCount && statuses[d] === 'solved'
            // Position within the current consecutive-solved run, staggering
            // the chain pulse so it travels along the streak
            run = linkLeft ? run + 1 : 0
            cells.push({
                day: d,
                date,
                status: statuses[d - 1],
                perfect: history[date]?.perfect ?? false,
                playable: date < today,
                linkLeft,
                linkRight,
                // Streaks wrapping a row get longer stubs poking past the
                // grid edge, hinting at the continuation
                edgeLeft: linkLeft && col === 0,
                edgeRight: linkRight && col === 6,
                chainPos: run
            })
        }
        return { offset, cells }
    })

    function shiftMonth(delta: number) {
        const idx = view.year * 12 + view.month + delta
        if (idx < MIN_MONTH_INDEX || idx > MAX_MONTH_INDEX) return
        view.year = Math.floor(idx / 12)
        view.month = idx % 12
    }

    function selectMonth(evt: Event) {
        const key = (evt.currentTarget as HTMLSelectElement).value
        view.year = Number(key.slice(0, 4))
        view.month = Number(key.slice(5, 7)) - 1
    }

    function cellLabel(cell: Cell): string {
        const date = `${MONTHS[view.month]} ${cell.day}`
        switch (cell.status) {
            case 'solved':
                return `${date} — ${cell.perfect ? 'solved perfectly' : 'solved'}`
            case 'failed':
                return `${date} — not solved`
            case 'today':
                return `${date} — today's puzzle`
            case 'future':
                return date
            default:
                return `${date} — play`
        }
    }

    async function play(cell: Cell) {
        if (!cell.playable) return
        error = ''
        try {
            await handleArchiveGame(cell.date)
        } catch (e) {
            console.error('Failed to load archive puzzle:', e)
            error = 'Could not load that puzzle — try another day.'
            return
        }
        closeModal()
        onpostNewGame?.()
    }
</script>

<div class="calendar flex flex-col w-full p-5 py-4 gap-3 text-tone-text">
    <div class="text-center">
        <h2 class="font-semibold text-lg">Archive</h2>
        <p class="text-sm text-tone-text-sub">
            Replay a past puzzle — stats and streak stay untouched.
        </p>
        {#if stats.currentStreak > 0}
            <p class="streak-line">
                🔥 <strong>{stats.currentStreak}</strong>-day streak
                <span class="streak-best">· best {stats.bestStreak}</span>
            </p>
        {:else if stats.bestStreak > 0}
            <p class="streak-line streak-muted">Longest streak: {stats.bestStreak} days</p>
        {/if}
    </div>

    <div class="month-nav">
        <button
            class="nav-btn"
            onclick={() => shiftMonth(-1)}
            disabled={!canPrev}
            aria-label="Previous month"
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 18 9 12 15 6"></polyline></svg>
        </button>

        <div class="month-select-wrap">
            <select
                class="month-select"
                aria-label="Month"
                value={viewKey}
                onchange={selectMonth}
            >
                {#each monthOptions as opt (opt.key)}
                    <option value={opt.key}>{opt.label}</option>
                {/each}
            </select>
            <svg class="select-chevron" xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>
        </div>

        <button
            class="nav-btn"
            onclick={() => shiftMonth(1)}
            disabled={!canNext}
            aria-label="Next month"
        >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"></polyline></svg>
        </button>
    </div>

    <div class="weekday-row" aria-hidden="true">
        {#each WEEKDAYS as wd, i (i)}
            <span class="weekday">{wd}</span>
        {/each}
    </div>

    <div class="day-grid">
        {#each { length: grid.offset } as _, i (i)}
            <span></span>
        {/each}
        {#each grid.cells as cell (cell.date)}
            <div class="cell">
                <button
                    class="tile {cell.status}"
                    class:perfect={cell.perfect}
                    class:in-chain={cell.linkLeft || cell.linkRight}
                    class:link-left={cell.linkLeft}
                    class:link-right={cell.linkRight}
                    class:edge-left={cell.edgeLeft}
                    class:edge-right={cell.edgeRight}
                    style:--streak-delay="{cell.chainPos * 140}ms"
                    disabled={!cell.playable}
                    onclick={() => play(cell)}
                    aria-label={cellLabel(cell)}
                    title={cellLabel(cell)}
                >
                    {#if cell.status === 'failed'}
                        <svg class="tile-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" aria-hidden="true"><line x1="7" y1="7" x2="17" y2="17"></line><line x1="17" y1="7" x2="7" y2="17"></line></svg>
                    {:else}
                        <span class="tile-glyph" aria-hidden="true">T</span>
                        {#if cell.perfect}
                            <svg class="tile-spark" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M12 0l2.4 9.6L24 12l-9.6 2.4L12 24l-2.4-9.6L0 12l9.6-2.4z"></path></svg>
                        {/if}
                    {/if}
                </button>
                <span class="day-num {cell.status}" class:perfect={cell.perfect}>{cell.day}</span>
            </div>
        {/each}
    </div>

    <div class="legend">
        <span class="legend-item"><span class="swatch swatch-perfect"></span>Perfect</span>
        <span class="legend-item"><span class="swatch swatch-solved"></span>Solved</span>
        <span class="legend-item"><span class="swatch swatch-failed"></span>Missed</span>
    </div>

    {#if error}
        <p class="error-text text-xs text-center" role="alert">{error}</p>
    {/if}
</div>

<style>
    .calendar {
        --cal-solved: var(--tone-correct);
        --cal-solved-edge: #55904f;
        --cal-gold: #f0c114;
        --cal-gold-hi: #ffdf6b;
        --cal-gold-edge: #cf9d00;
        --cal-gold-num: #a87b06;
        --cal-gold-ink: #6d5200;
        --cal-open-bg: #e4e7ea;
        --cal-open-edge: #d0d4d8;
        --cal-failed-edge: #a8adb2;
        --cal-failed-ink: #1a1a1b;
    }

    :global(.dark) .calendar {
        --cal-solved-edge: #3f7039;
        --cal-gold: #f5ca28;
        --cal-gold-hi: #ffe27a;
        --cal-gold-edge: #c69f10;
        --cal-gold-num: #f5ca28;
        --cal-gold-ink: #5f4700;
        --cal-open-bg: #2c2d2f;
        --cal-open-edge: #3a3a3c;
        --cal-failed-edge: #5e6062;
        --cal-failed-ink: #ffffff;
    }

    .streak-line {
        margin-top: 0.375rem;
        font-size: 0.8125rem;
        color: var(--tone-text);
    }

    .streak-best,
    .streak-muted {
        color: var(--tone-text-sub);
        font-weight: 400;
    }

    .month-nav {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.5rem;
    }

    .nav-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 2rem;
        height: 2rem;
        flex-shrink: 0;
        border: 1px solid var(--tone-border);
        border-radius: 9999px;
        background: transparent;
        color: var(--tone-text);
        cursor: pointer;
        transition: border-color 0.15s, opacity 0.15s;
    }

    .nav-btn:hover:not(:disabled) {
        border-color: var(--tone-border-strong);
    }

    .nav-btn:disabled {
        opacity: 0.3;
        cursor: default;
    }

    .month-select-wrap {
        position: relative;
        flex: 1;
        max-width: 11rem;
    }

    .month-select {
        width: 100%;
        appearance: none;
        -webkit-appearance: none;
        padding: 0.375rem 1.75rem 0.375rem 0.875rem;
        border: 1px solid var(--tone-border);
        border-radius: 8px;
        background: transparent;
        color: var(--tone-text);
        font-size: 0.875rem;
        font-weight: 600;
        text-align: center;
        cursor: pointer;
    }

    .month-select:hover {
        border-color: var(--tone-border-strong);
    }

    .select-chevron {
        position: absolute;
        right: 0.5rem;
        top: 50%;
        transform: translateY(-50%);
        pointer-events: none;
        color: var(--tone-text-sub);
    }

    .weekday-row,
    .day-grid {
        display: grid;
        grid-template-columns: repeat(7, 1fr);
        column-gap: 4px;
    }

    .weekday {
        text-align: center;
        font-size: 0.6875rem;
        font-weight: 600;
        color: var(--tone-text-sub);
        padding-bottom: 0.125rem;
        border-bottom: 1px solid var(--tone-border);
    }

    .day-grid {
        row-gap: 6px;
    }

    .cell {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 2px;
    }

    .tile {
        position: relative;
        width: 100%;
        max-width: 2.5rem;
        aspect-ratio: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0;
        border: none;
        border-radius: 9px;
        background: transparent;
        cursor: pointer;
        transition: border-color 0.15s, filter 0.15s;
    }

    .tile-icon {
        width: 58%;
        height: 58%;
    }

    .tile.solved {
        background: var(--cal-solved);
        border: 2px solid var(--cal-solved-edge);
        color: #ffffff;
    }

    .tile.solved .tile-glyph {
        color: #ffffff;
        opacity: 1;
    }

    /* Perfect solves break from the green: a bright gold tile with a
       darker gold edge and the corner sparkle. Dark amber ink — white
       washes out against the gold. */
    .tile.solved.perfect {
        background: linear-gradient(135deg, var(--cal-gold-hi) 30%, var(--cal-gold) 70%);
        border-color: var(--cal-gold-edge);
    }

    .tile.solved.perfect .tile-glyph {
        color: var(--cal-gold-ink);
    }

    .tile-spark {
        position: absolute;
        top: 6%;
        right: 6%;
        width: 28%;
        height: 28%;
        color: var(--cal-gold-ink);
    }

    .tile.failed {
        background: var(--tone-key);
        border: 2px solid var(--cal-failed-edge);
        color: var(--cal-failed-ink);
    }

    .tile.solved:hover:not(:disabled),
    .tile.failed:hover:not(:disabled) {
        filter: brightness(1.08);
    }

    .tile-glyph {
        font-size: 1.375rem;
        font-weight: 800;
        line-height: 1;
        color: var(--tone-text);
        opacity: 0.9;
    }

    .tile.open {
        background: var(--cal-open-bg);
        border: 2px solid var(--cal-open-edge);
    }

    .tile.open:hover {
        border-color: var(--tone-border-strong);
    }

    .tile.today {
        background: var(--cal-open-bg);
        border: 2px solid var(--tone-correct);
        cursor: default;
    }

    .tile.today .tile-glyph {
        color: var(--tone-correct);
        opacity: 0.9;
    }

    .tile.future {
        background: var(--cal-open-bg);
        border: 2px solid var(--cal-open-edge);
        opacity: 0.4;
        cursor: default;
    }

    /* Streak connectors between consecutive daily solves; stubs poke past
       row edges when the streak wraps to the next line, as a continuation
       hint. Bars sit at tile mid-height and match the tile color, so the
       overlap under the rounded corners is invisible. */
    .tile.link-left::before,
    .tile.link-right::after {
        content: '';
        position: absolute;
        top: 50%;
        transform: translateY(-50%);
        height: 8px;
        width: 9px;
        background: var(--cal-solved);
    }

    /* Offsets are from the padding box: 2px own border + 4px gap + 2px
       neighbor border = an 8px bridge between tile interiors */
    .tile.link-left::before {
        left: -8px;
        width: 8px;
    }

    .tile.link-right::after {
        right: -8px;
        width: 8px;
    }

    .tile.edge-left::before {
        left: -16px;
        width: 16px;
    }

    .tile.edge-right::after {
        right: -16px;
        width: 16px;
    }

    /* Streak chains breathe: a gentle scale pulse travels tile to tile
       (staggered by chain position), with the connectors glowing in sync */
    .tile.in-chain {
        animation: chain-pulse 2.8s cubic-bezier(0.37, 0, 0.63, 1) infinite;
        animation-delay: var(--streak-delay, 0ms);
    }

    .tile.in-chain::before,
    .tile.in-chain::after {
        animation: link-pulse 2.8s cubic-bezier(0.37, 0, 0.63, 1) infinite;
        animation-delay: var(--streak-delay, 0ms);
    }

    @keyframes chain-pulse {
        0%,
        100% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.06);
        }
    }

    @keyframes link-pulse {
        0%,
        100% {
            opacity: 0.6;
        }
        50% {
            opacity: 1;
        }
    }

    @media (prefers-reduced-motion: reduce) {
        .tile.in-chain,
        .tile.in-chain::before,
        .tile.in-chain::after {
            animation: none;
        }
    }

    .day-num {
        font-size: 0.6875rem;
        font-weight: 600;
        color: var(--tone-text-sub);
        line-height: 1;
    }

    .day-num.solved {
        color: var(--cal-solved);
    }

    .day-num.solved.perfect {
        color: var(--cal-gold-num);
    }

    .day-num.today {
        color: var(--tone-correct);
    }

    .legend {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 0.25rem 0.75rem;
        font-size: 0.6875rem;
        color: var(--tone-text-sub);
    }

    .legend-item {
        display: inline-flex;
        align-items: center;
        gap: 0.3125rem;
    }

    .swatch {
        width: 0.625rem;
        height: 0.625rem;
        border-radius: 3px;
    }

    .swatch-perfect {
        background: linear-gradient(135deg, var(--cal-gold-hi) 30%, var(--cal-gold) 70%);
        border: 1px solid var(--cal-gold-edge);
    }

    .swatch-solved {
        background: var(--cal-solved);
        border: 1px solid var(--cal-solved-edge);
    }

    .swatch-failed {
        background: var(--tone-key);
        border: 1px solid var(--cal-failed-edge);
    }

    .error-text {
        color: #d43d3d;
    }
</style>
