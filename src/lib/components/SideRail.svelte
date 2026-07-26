<script lang="ts">
    import { getStats, getSolveRate } from '../stores/stats.svelte'
    import { openModal } from '../stores/modal.svelte'
    import { localDateString } from '../date'
    import { isTauri, DESKTOP_DOWNLOAD_URL } from '../platform'

    let { onplaydate }: { onplaydate: (date: string) => void } = $props()

    const stats = getStats()
    const yesterday = localDateString(new Date(Date.now() - 86_400_000))
</script>

<aside class="side-rail hidden xl:flex" aria-label="Stats and archive">
    <section class="rail-panel">
        <h3 class="rail-title">Stats</h3>
        <div class="rail-stats">
            <div>
                <p class="rail-num">{stats.currentStreak}</p>
                <p class="rail-label">Streak</p>
            </div>
            <div>
                <p class="rail-num">{stats.played}</p>
                <p class="rail-label">Played</p>
            </div>
            <div>
                <p class="rail-num">{getSolveRate()}%</p>
                <p class="rail-label">Solve rate</p>
            </div>
        </div>
        <button class="rail-link" onclick={() => openModal('stats')}>Full stats</button>
    </section>

    <section class="rail-panel">
        <h3 class="rail-title">Archive</h3>
        <p class="rail-text">Missed a day? Every puzzle since Jan 2025 is playable — without touching your stats.</p>
        <button class="rail-btn" onclick={() => onplaydate(yesterday)}>
            Yesterday's puzzle
        </button>
        <button class="rail-link" onclick={() => openModal('archive')}>Browse archive</button>
    </section>

    {#if !isTauri}
        <a
            class="rail-link rail-download"
            href={DESKTOP_DOWNLOAD_URL}
            target="_blank"
            rel="noopener noreferrer"
        >
            Download Triad for desktop
        </a>
    {/if}
</aside>

<style>
    .side-rail {
        position: fixed;
        left: calc(50% + 320px);
        top: 50%;
        transform: translateY(-50%);
        width: 15rem;
        flex-direction: column;
        gap: 1rem;
    }

    .rail-panel {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        padding: 1rem;
        border: 1px solid var(--tone-border);
        border-radius: 8px;
        background-color: var(--tone-surface);
    }

    .rail-title {
        font-size: 0.875rem;
        font-weight: 600;
        color: var(--tone-text);
    }

    .rail-stats {
        display: flex;
        justify-content: space-between;
        text-align: center;
        padding: 0 0.25rem;
    }

    .rail-num {
        font-size: 1.25rem;
        font-weight: 700;
        color: var(--tone-text);
    }

    .rail-label {
        font-size: 0.6875rem;
        color: var(--tone-text-sub);
    }

    .rail-text {
        font-size: 0.75rem;
        line-height: 1.4;
        color: var(--tone-text-sub);
    }

    .rail-btn {
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 9999px;
        background-color: var(--tone-correct);
        color: #ffffff;
        font-size: 0.8125rem;
        font-weight: 600;
        cursor: pointer;
        transition: opacity 0.15s;
    }

    .rail-btn:hover {
        opacity: 0.9;
    }

    .rail-link {
        background: none;
        border: none;
        padding: 0;
        font-size: 0.75rem;
        color: var(--tone-text-sub);
        text-decoration: underline;
        cursor: pointer;
    }

    .rail-link:hover {
        color: var(--tone-text);
    }

    .rail-download {
        text-align: center;
    }
</style>
