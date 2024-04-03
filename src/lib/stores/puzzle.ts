import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { getContext, setContext } from 'svelte'

const STORE = 'puzzle'

export enum PuzzleState {
    START = 'start',
    CLUE = 'clue',
    SOLUTION = 'solution'
}

export type Puzzle = { key: string, state: PuzzleState, start: string, clue: string, solution: string }
type Context = Writable<Puzzle>

export const INIT_PUZZLE: Puzzle = {
    key: "FIRM",
    state: PuzzleState.START,
    start: 'WARE / REAF / CON',
    clue: 'WARE____ / REAF / CON', 
    solution: 'FIRMWARE / REAFFIRM / CONFIRM'
}

const puzzle = writable(INIT_PUZZLE)

export function initPuzzle() {
    setContext(STORE, puzzle)
}

export function getPuzzle() {
    return getContext<Context>(STORE)
}

export function updatePuzzleState(state: PuzzleState) {
    console.log({ pz: state })
    puzzle.update(prevState => ({...prevState, state }))
}
