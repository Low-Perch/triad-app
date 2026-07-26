<script lang="ts">
    import { onMount } from 'svelte'

    import * as bridge from '$lib/bridge'
    import { registerLifecycleHooks, type UpdateInfo } from '$lib/lifecycle'
    import { handleNewGame, handleArchiveGame, handleResumeDaily, hydrateGame } from './lib/actions'

    import { getModal, openModal } from './lib/stores/modal.svelte'
    import { getKeys } from './lib/stores/keys.svelte'
    import { getPuzzle, setPuzzle } from './lib/stores/puzzle.svelte'
    import { setInput, getInput } from './lib/stores/input.svelte'
    import { setStats } from './lib/stores/stats.svelte'
    import { getGuesses, setGuesses } from './lib/stores/guesses.svelte'
    import { getGameMode } from './lib/stores/mode.svelte'
    import { initTheme } from './lib/stores/theme.svelte'

    import Keys from './lib/components/Keys.svelte'
    import Modal from './lib/components/Modal.svelte'
    import Clues from './lib/components/Clues.svelte'
    import Input from './lib/components/Input.svelte'
    import Header from './lib/components/Header.svelte'
    import SideRail from './lib/components/SideRail.svelte'
    import SplashScreen from './lib/components/SplashScreen.svelte'
    import ResultScreen from './lib/components/ResultScreen.svelte'

    import { localDateString } from './lib/date'

    type AppPhase = 'loading' | 'splash' | 'solved-today' | 'revealing' | 'playing' | 'congrats' | 'failed' | 'error'

    const keys = getKeys()
    const input = getInput()
    const modal = getModal()
    const puzzle = getPuzzle()

    let phase = $state<AppPhase>('loading')
    let puzzleDate: string | null = null
    let hooksRegistered = false

    let update = $state<UpdateInfo | null>(null)
    let updateState = $state<'idle' | 'installing' | 'failed'>('idle')

    async function handleInstallUpdate() {
        if (!update || updateState === 'installing') return
        updateState = 'installing'
        try {
            await update.install() // relaunches on success
        } catch (e) {
            console.error('Failed to install update:', e)
            updateState = 'failed'
        }
    }

    async function handleKeyboard(e: KeyboardEvent) {
        if (phase !== 'playing') return
        if (puzzle.solved) return

        const allowedKeys = ['Enter', 'Backspace', 'Delete']
        const alphaRegex = /^[a-zA-Z]$/

        const allowedInput = allowedKeys.includes(e.key) || alphaRegex.test(e.key)
        if (!allowedInput) return

        if (e.key == 'Enter') {
            const result = await bridge.submitSolution()
            setInput({ ...input, state: result.inputState })
            setStats(result.stats)
            setGuesses(result.guesses)
            if (result.solved || result.exhausted) {
                setPuzzle({ ...puzzle, solved: true, state: result.puzzleState })
                if (result.exhausted) {
                    setInput({ ...input, keys: puzzle.key.split(''), state: result.inputState, disabled: true })
                }
                setTimeout(() => {
                    phase = result.solved ? 'congrats' : 'failed'
                }, 700)
            } else {
                setTimeout(async () => {
                    const clearedInput = await bridge.clearInput()
                    setInput(clearedInput)
                }, 350)
            }
            return
        }

        const updatedInput = ['Backspace', 'Delete'].includes(e.key)
            ? await bridge.removeKey()
            : await bridge.addKey(e.key.toUpperCase())

        setInput(updatedInput)
    }

    let puzzleText = $derived(puzzle[puzzle.state])
    let disabledKeys = $derived(keys.disabledKeys)
    let gameMode = $derived(getGameMode())

    let lifecycleCleanups: (() => void)[] = []

    function startReveal() {
        phase = 'revealing'
        setTimeout(() => { phase = 'playing' }, 800)
    }

    function handleSplashDone() {
        if (puzzle.solved) {
            phase = 'solved-today'
        } else if (input.keys.some(k => k !== '')) {
            phase = 'playing'
        } else {
            startReveal()
        }
    }

    async function handleNewGameFromResult() {
        await handleNewGame()
        startReveal()
    }

    async function handleBackToDaily() {
        await handleResumeDaily()
        phase = puzzle.solved ? 'solved-today' : 'playing'
    }

    async function handlePlayArchiveDate(date: string) {
        try {
            await handleArchiveGame(date)
            startReveal()
        } catch (e) {
            console.error('Failed to load archive puzzle:', e)
        }
    }

    let archiveParamChecked = false

    // Web deep link: ?date=YYYY-MM-DD opens that day's archive puzzle.
    // Stripped after applying so a reload resumes instead of restarting.
    async function applyArchiveParam() {
        if (archiveParamChecked) return
        archiveParamChecked = true

        const date = new URLSearchParams(window.location.search).get('date')
        if (!date) return
        window.history.replaceState(null, '', window.location.pathname)

        try {
            await handleArchiveGame(date)
        } catch (e) {
            console.error('Failed to load archive puzzle from URL:', e)
        }
    }

    function checkRollover() {
        if (puzzleDate && localDateString() !== puzzleDate) {
            init()
        }
    }

    async function init() {
        try {
            const game = await bridge.initGame()
            hydrateGame(game)
            puzzleDate = game.puzzleDate
        } catch (e) {
            console.error('Failed to load game state:', e)
            phase = 'error'
            return
        }

        await applyArchiveParam()

        if (!hooksRegistered) {
            lifecycleCleanups = await registerLifecycleHooks({
                onResume: checkRollover,
                onUpdateAvailable: (info) => { update = info },
            })
            hooksRegistered = true
        }

        phase = 'splash'
    }

    onMount(() => {
        initTheme()
        init()

        window.addEventListener('keydown', handleKeyboard)
        const rolloverTimer = setInterval(checkRollover, 60_000)

        return () => {
            clearInterval(rolloverTimer)
            lifecycleCleanups.forEach(fn => fn())
            window.removeEventListener('keydown', handleKeyboard)
        }
    })
</script>

<main class="app-container">
    {#if update}
        <div class="update-banner" role="status">
            {#if updateState === 'failed'}
                <span>Update failed — try again?</span>
            {:else}
                <span>v{update.version} is available</span>
            {/if}
            <button class="update-btn" onclick={handleInstallUpdate} disabled={updateState === 'installing'}>
                {updateState === 'installing' ? 'Installing…' : 'Restart to update'}
            </button>
            <button class="update-dismiss" onclick={() => { update = null }} aria-label="Dismiss update notice">✕</button>
        </div>
    {/if}

    {#if modal.visible}
        <Modal onpostNewGame={startReveal} />
    {/if}

    {#if phase === 'error'}
        <div class="error-screen">
            <p class="text-tone-text font-semibold">Failed to load game</p>
            <button onclick={init} class="action-btn">
                <span class="text-sm font-semibold">Retry</span>
            </button>
        </div>
    {:else if phase === 'splash'}
        <SplashScreen ondone={handleSplashDone} />
    {:else if phase === 'solved-today' || phase === 'congrats' || phase === 'failed'}
        <ResultScreen
            mode={phase}
            onviewstats={() => openModal('stats')}
            onnewgame={handleNewGameFromResult}
            ondismiss={() => { phase = 'playing' }}
        />
    {:else if phase === 'revealing' || phase === 'playing'}
        <Header />
        <div class="game-area">
            {#if puzzle.puzzleNumber !== null || gameMode !== 'daily'}
                <div class="flex justify-between items-center w-full px-3 pt-1">
                    {#if gameMode !== 'daily'}
                        <button class="today-link" onclick={handleBackToDaily}>‹ Today's puzzle</button>
                    {:else}
                        <span></span>
                    {/if}
                    {#if puzzle.puzzleNumber !== null}
                        <p class="text-xs md:text-sm text-tone-text-sub">{gameMode === 'archive' ? 'Archive #' : '#'}{puzzle.puzzleNumber}</p>
                    {/if}
                </div>
            {/if}
            <Clues text={puzzleText} revealing={phase === 'revealing'} />
            <Input revealing={phase === 'revealing'} />
            <p class="guess-counter">{getGuesses()}/6</p>
            {#if !puzzle.solved}
                <Keys {disabledKeys} />
            {/if}
        </div>
        <SideRail onplaydate={handlePlayArchiveDate} />
    {/if}
</main>

<style>
    .app-container {
        position: absolute;
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        max-width: 600px;
        margin: 0 auto;
        left: 0;
        right: 0;
    }

    .action-btn {
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

    .action-btn:hover {
        opacity: 0.9;
    }

    .update-banner {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        padding: 0.375rem 0.5rem;
        font-size: 0.75rem;
        color: var(--tone-text);
        background-color: var(--tone-surface);
        border-bottom: 1px solid var(--tone-border);
    }

    .update-btn {
        border: none;
        border-radius: 9999px;
        padding: 0.25rem 0.75rem;
        font-size: 0.6875rem;
        font-weight: 600;
        background-color: var(--tone-correct);
        color: #ffffff;
        cursor: pointer;
    }

    .update-btn:disabled {
        opacity: 0.6;
        cursor: wait;
    }

    .update-dismiss {
        border: none;
        background: none;
        color: var(--tone-text-sub);
        cursor: pointer;
        font-size: 0.75rem;
        padding: 0.25rem;
    }

    .game-area {
        display: flex;
        flex-direction: column;
        align-items: center;
        flex: 1;
        min-height: 0;
        width: 100%;
    }

    /* Wide screens: the whole stack reads as one centered group; the
       keyboard joins the flow instead of pinning to the bottom */
    @media (min-width: 768px) {
        .game-area {
            justify-content: center;
            padding-bottom: 3rem;
        }
    }

    .today-link {
        background: none;
        border: none;
        padding: 0;
        font-size: 0.75rem;
        color: var(--tone-text-sub);
        cursor: pointer;
        text-decoration: underline;
    }

    .today-link:hover {
        color: var(--tone-text);
    }

    .guess-counter {
        text-align: center;
        font-size: 0.75rem;
        font-weight: 600;
        color: var(--tone-text-sub);
        margin-bottom: 0.25rem;
    }

    .error-screen {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        gap: 1rem;
    }
</style>
