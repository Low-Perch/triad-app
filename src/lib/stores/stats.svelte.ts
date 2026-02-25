export type { Stats } from '../types'

import type { Stats } from '../types'

export const INIT_STATS: Stats = {
    played: 0,
    solved: 0,
    currentStreak: 0,
    bestStreak: 0,
    bestTime: null,
    solveTimes: [],
    startedAt: null,
    guessDistribution: [0, 0, 0, 0, 0, 0],
    solveClueCount: 0,
}

let stats = $state<Stats>({ ...INIT_STATS })

export function getStats(): Stats {
    return stats
}

export function setStats(state: Stats) {
    Object.assign(stats, state)
}

export function getSolveRate(): number {
    if (stats.played === 0) return 0
    return Math.round((stats.solved / stats.played) * 100)
}

export function getGuessDistribution(): { label: string; count: number }[] {
    const dist = stats.guessDistribution?.length === 6
        ? stats.guessDistribution
        : [0, 0, 0, 0, 0, 0]
    return [
        { label: '1', count: dist[0] },
        { label: '2', count: dist[1] },
        { label: '3', count: dist[2] },
        { label: '4', count: dist[3] },
        { label: '5', count: dist[4] },
        { label: '6', count: dist[5] },
    ]
}

export function getTimeBuckets(): { under10: number; under30: number; under60: number; over60: number } {
    const times = stats.solveTimes
    return {
        under10: times.filter(t => t < 10).length,
        under30: times.filter(t => t >= 10 && t < 30).length,
        under60: times.filter(t => t >= 30 && t < 60).length,
        over60: times.filter(t => t >= 60).length,
    }
}
