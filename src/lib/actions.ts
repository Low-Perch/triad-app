import * as bridge from './bridge'
import { formatTime } from './format'
import { setClues, getClues } from './stores/clues.svelte'
import { getPuzzle, setPuzzle } from './stores/puzzle.svelte'
import { setInput } from './stores/input.svelte'
import { setKeys } from './stores/keys.svelte'
import { getStats, setStats } from './stores/stats.svelte'
import { getGuesses, setGuesses } from './stores/guesses.svelte'

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

    const latestTime = stats.solveTimes.length > 0
        ? stats.solveTimes[stats.solveTimes.length - 1]
        : null

    const parts: string[] = []
    if (latestTime !== null) parts.push(`⏱️ ${formatTime(latestTime)}`)
    if (stats.currentStreak > 0) parts.push(`🔥 ${stats.currentStreak}`)
    if (parts.length > 0) lines.push(parts.join(' | '))

    return lines.join('\n')
}

export async function handleNewGame() {
    const game = await bridge.newGame()
    setPuzzle(game.puzzle)
    setClues(game.clues)
    setInput(game.input)
    setKeys(game.keys)
    setStats(game.stats)
    setGuesses(game.guesses)
}
