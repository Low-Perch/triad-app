export enum PuzzleState {
    START = 'start',
    CLUE = 'clue',
    SOLUTION = 'solution'
}

export type Puzzle = {
    key: string
    clue: string
    start: string
    solved: boolean
    solution: string
    state: PuzzleState
    puzzleNumber: number | null
}

export enum InputState {
    EDIT = 'edit',
    CORRECT = 'correct',
    INCORRECT = 'incorrect',
}

export type Input = {
    length: number
    state: InputState
    disabled: boolean
    keys: Array<string>
    lastPositionLocked: boolean
}

export type Clue = { id: string; note: string; active: boolean }
export type Clues = { clues: Array<Clue>; used: number; available: boolean }

export type Keys = { disabledKeys: string[]; keysDisabled: boolean }

export type Stats = {
    played: number
    solved: number
    currentStreak: number
    bestStreak: number
    bestTime: number | null
    solveTimes: number[]
    startedAt: number | null
}

export type GameState = {
    puzzle: Puzzle
    input: Input
    clues: Clues
    keys: Keys
    stats: Stats
    puzzleDate: string | null
}

export type SubmitResult = {
    solved: boolean
    inputState: InputState
    puzzleState: PuzzleState
    stats: Stats
}

export type ClueResult = {
    clues: Clues
    input: Input
    puzzle: Puzzle
    keys: Keys
}
