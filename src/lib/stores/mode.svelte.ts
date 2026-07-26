import type { GameMode } from '../types'

const state = $state<{ mode: GameMode }>({ mode: 'daily' })

export function getGameMode(): GameMode {
    return state.mode
}

export function setGameMode(mode: GameMode) {
    state.mode = mode
}
