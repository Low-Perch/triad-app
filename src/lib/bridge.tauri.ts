import { invoke } from '@tauri-apps/api/core'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import type { GameState, Input, SubmitResult, ClueResult, History } from './types'

export const initGame = () => invoke<GameState>('init_game')
export const addKey = (key: string) => invoke<Input>('add_key', { key })
export const removeKey = () => invoke<Input>('remove_key')
export const submitSolution = () => invoke<SubmitResult>('submit_solution')
export const activateClue = (clueId: string) => invoke<ClueResult>('activate_clue', { clueId })
export const saveGame = () => invoke<void>('save_game')
export const newGame = () => invoke<GameState>('new_game')
export const archiveGame = (date: string) => invoke<GameState>('archive_game', { date })
export const resumeDaily = () => invoke<GameState>('resume_daily')
export const clearInput = () => invoke<Input>('clear_input')
export const getHistory = () => invoke<History>('get_history')
export const copyText = (text: string) => writeText(text)
