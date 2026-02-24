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
    solution: string,
    state: PuzzleState,
}

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

export function updatePuzzleState(state: PuzzleState) {
    puzzle.state = state
}

export function markPuzzleSolved() {
    puzzle.solved = true
}
