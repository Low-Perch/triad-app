import type { DayRecord, GameMode } from '../types'

const state = $state<{ mode: GameMode; dayRecord: DayRecord | null }>({ mode: 'daily', dayRecord: null })

export function getGameMode(): GameMode {
    return state.mode
}

export function setGameMode(mode: GameMode) {
    state.mode = mode
}

/** History record for the current puzzle's date (null for random games
 * or dates never played). Lets the UI acknowledge a prior solve when
 * replaying an archive day. */
export function getDayRecord(): DayRecord | null {
    return state.dayRecord
}

export function setDayRecord(record: DayRecord | null) {
    state.dayRecord = record
}
