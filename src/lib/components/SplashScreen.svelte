<script lang="ts">
    let { ondone }: { ondone: () => void } = $props()

    const TITLE = 'triad'.split('')

    function handleAnimationEnd(e: AnimationEvent) {
        if (e.target === e.currentTarget) {
            ondone()
        }
    }
</script>

<div class="splash" onanimationend={handleAnimationEnd}>
    <div class="splash-tiles">
        {#each TITLE as char, i (i)}
            <p class="splash-tile" style="animation-delay: {i * 100}ms">
                {char}
            </p>
        {/each}
    </div>
</div>

<style>
    .splash {
        position: fixed;
        inset: 0;
        z-index: 200;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: var(--tone-bg);
        animation: splashFadeOut 0.3s ease-in forwards;
        animation-delay: 1100ms;
    }

    .splash-tiles {
        display: flex;
        gap: 0.5rem;
    }

    .splash-tile {
        width: 2.5rem;
        height: 2.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        text-transform: uppercase;
        font-weight: 700;
        font-size: 0.875rem;
        color: var(--tone-text);
        border: 2px solid var(--tone-border-strong);
        opacity: 0;
        transform: scale(0);
        animation: tilePopIn 0.25s ease-out forwards;
    }

    @keyframes tilePopIn {
        0% { transform: scale(0); opacity: 0; }
        60% { transform: scale(1.1); }
        100% { transform: scale(1); opacity: 1; }
    }

    @keyframes splashFadeOut {
        0% { opacity: 1; }
        100% { opacity: 0; }
    }
</style>
