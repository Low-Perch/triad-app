import * as bridge from './bridge'
import { formatTime } from './format'
import { setClues, getClues } from './stores/clues.svelte'
import { getPuzzle, setPuzzle } from './stores/puzzle.svelte'
import { setInput } from './stores/input.svelte'
import { setKeys } from './stores/keys.svelte'
import { getStats, setStats } from './stores/stats.svelte'

export function generateShareText(): string {
    const clues = getClues()
    const stats = getStats()
    const puzzle = getPuzzle()

    const squares = clues.clues.map(c => c.active ? '🟨' : '🟩').join('')
    const title = puzzle.puzzleNumber !== null
        ? `Triad #${puzzle.puzzleNumber} ${squares}`
        : `Triad ${squares}`
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
}
