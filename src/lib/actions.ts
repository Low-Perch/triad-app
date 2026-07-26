import * as bridge from '$lib/bridge'
import { setClues, getClues } from './stores/clues.svelte'
import { getPuzzle, setPuzzle } from './stores/puzzle.svelte'
import { setInput } from './stores/input.svelte'
import { setKeys } from './stores/keys.svelte'
import { getStats, setStats } from './stores/stats.svelte'
import { getGuesses, setGuesses } from './stores/guesses.svelte'
import { setGameMode, setDayRecord } from './stores/mode.svelte'
import { dateStringFromPuzzleNumber } from './date'
import type { GameState } from './types'

export function generateShareText(): string {
    const clues = getClues()
    const stats = getStats()
    const puzzle = getPuzzle()
    const guesses = getGuesses()

    const squares = clues.clues.map(c => {
        if (c.id === 'solve') return c.active ? '⬛' : '🟩'
        return c.active ? '🟨' : '🟩'
    }).join('')

    const solveUsed = clues.clues.find(c => c.id === 'solve')?.active
    const guessDisplay = solveUsed ? 'X' : String(guesses)

    const title = puzzle.puzzleNumber !== null
        ? `Triad #${puzzle.puzzleNumber} ${squares} ${guessDisplay}/6`
        : `Triad ${squares} ${guessDisplay}/6`
    const lines = [title]

    if (stats.currentStreak > 0) lines.push(`🔥 ${stats.currentStreak}`)

    return lines.join('\n')
}

export async function copyShareText(): Promise<boolean> {
    try {
        await bridge.copyText(generateShareText())
        return true
    } catch (e) {
        console.error('Failed to copy share text:', e)
        return false
    }
}

export function hydrateGame(game: GameState) {
    setPuzzle(game.puzzle)
    setClues(game.clues)
    setInput(game.input)
    setKeys(game.keys)
    setStats(game.stats)
    setGuesses(game.guesses)
    setGameMode(game.mode)
    const number = game.puzzle.puzzleNumber
    setDayRecord(number !== null ? game.history[dateStringFromPuzzleNumber(number)] ?? null : null)
}

export async function handleNewGame() {
    hydrateGame(await bridge.newGame())
}

export async function handleArchiveGame(date: string) {
    hydrateGame(await bridge.archiveGame(date))
}

export async function handleResumeDaily() {
    hydrateGame(await bridge.resumeDaily())
}
