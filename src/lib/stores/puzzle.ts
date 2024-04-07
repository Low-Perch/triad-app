import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { getContext, setContext } from 'svelte'

const STORE = 'puzzle'

export enum PuzzleState {
    START = 'start',
    CLUE = 'clue',
    SOLUTION = 'solution'
}

export type Puzzle = { 
    key: string,
    clue: string,
    start: string,
    solved: boolean,
    solution: string
    state: PuzzleState,
}
type Context = Writable<Puzzle>

export const INIT_PUZZLE: Puzzle = {
    key: "FIRM",
    solved: false,
    state: PuzzleState.START,
    start: 'WARE / REAF / CON',
    clue: 'WARE____ / REAF / CON', 
    solution: 'FIRMWARE / REAFFIRM / CONFIRM'
}

const puzzle = writable(INIT_PUZZLE)

export function initPuzzle() {
    setContext(STORE, puzzle)
}

export function setPuzzle(state: Puzzle) {
    puzzle.set(state)
}

export function getPuzzle() {
    return getContext<Context>(STORE)
}

export function updatePuzzleState(state: PuzzleState) {
    puzzle.update(prevState => ({...prevState, state }))
}

export function markPuzzleSolved() {
    puzzle.update(prevState => ({...prevState, solved: true }))
}
