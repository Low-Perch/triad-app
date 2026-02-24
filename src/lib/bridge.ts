import { invoke } from '@tauri-apps/api/core'
import type { GameState, Input, SubmitResult, ClueResult } from './types'

export const initGame = () => invoke<GameState>('init_game')
export const addKey = (key: string) => invoke<Input>('add_key', { key })
export const removeKey = () => invoke<Input>('remove_key')
export const submitSolution = () => invoke<SubmitResult>('submit_solution')
export const activateClue = (clueId: string) => invoke<ClueResult>('activate_clue', { clueId })
export const saveGame = () => invoke<void>('save_game')
