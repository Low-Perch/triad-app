import type { GameState, Input, SubmitResult, ClueResult, History } from './types'
import init, * as wasm from 'triad-wasm'

const STORAGE_KEY = 'triad-game'
const wasmBinUrl = new URL('../../wasm-pkg/triad_wasm_bg.wasm', import.meta.url)

let initialized = false

async function ensureInit() {
    if (!initialized) {
        await init(wasmBinUrl)
        initialized = true
    }
}

function loadSaved(): string | null {
    return localStorage.getItem(STORAGE_KEY)
}

function persist() {
    try {
        const json = wasm.save_game()
        localStorage.setItem(STORAGE_KEY, json)
    } catch (e) {
        console.error('Failed to persist game state:', e)
    }
}

export async function initGame(): Promise<GameState> {
    await ensureInit()
    const saved = loadSaved()
    const state = wasm.init_game(saved ?? undefined) as GameState
    persist()
    return state
}

export async function addKey(key: string): Promise<Input> {
    const input = wasm.add_key(key) as Input
    persist()
    return input
}

export async function removeKey(): Promise<Input> {
    const input = wasm.remove_key() as Input
    persist()
    return input
}

export async function submitSolution(): Promise<SubmitResult> {
    const result = wasm.submit_solution() as SubmitResult
    persist()
    return result
}

export async function activateClue(clueId: string): Promise<ClueResult> {
    const result = wasm.activate_clue(clueId) as ClueResult
    persist()
    return result
}

export async function saveGame(): Promise<void> {
    persist()
}

export async function newGame(): Promise<GameState> {
    const state = wasm.new_game() as GameState
    persist()
    return state
}

export async function archiveGame(date: string): Promise<GameState> {
    const state = wasm.archive_game(date) as GameState
    persist()
    return state
}

export async function resumeDaily(): Promise<GameState> {
    const state = wasm.resume_daily() as GameState
    persist()
    return state
}

export async function clearInput(): Promise<Input> {
    const input = wasm.clear_input() as Input
    persist()
    return input
}

export async function getHistory(): Promise<History> {
    return wasm.get_history() as History
}

export async function copyText(text: string): Promise<void> {
    await navigator.clipboard.writeText(text)
}
