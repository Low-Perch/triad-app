export { PuzzleState } from '../types'
export type { Puzzle } from '../types'

import { PuzzleState } from '../types'
import type { Puzzle } from '../types'

export const INIT_PUZZLE: Puzzle = {
    key: "FIRM",
    solved: false,
    state: PuzzleState.START,
    start: 'WARE / REAF / CON',
    clue: 'WARE____ / REAF / CON',
    solution: 'FIRMWARE / REAFFIRM / CONFIRM'
}

let puzzle = $state<Puzzle>({ ...INIT_PUZZLE })

export function getPuzzle(): Puzzle {
    return puzzle
}

export function setPuzzle(state: Puzzle) {
    Object.assign(puzzle, state)
}
