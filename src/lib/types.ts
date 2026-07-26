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
    guessDistribution: number[]
    solveClueCount: number
}

export type GameMode = 'daily' | 'random' | 'archive'

/** Outcome of one dated puzzle. `daily` is true when it was completed as
 * that day's live daily (streak-eligible), false when solved via archive.
 * `perfect` means solved on the first guess with no lifelines. */
export type DayRecord = {
    solved: boolean
    guesses: number
    daily: boolean
    perfect: boolean
}

/** Per-date results keyed by "YYYY-MM-DD". */
export type History = Record<string, DayRecord>

export type DailySnapshot = {
    puzzle: Puzzle
    input: Input
    clues: Clues
    keys: Keys
    guesses: number
}

export type GameState = {
    puzzle: Puzzle
    input: Input
    clues: Clues
    keys: Keys
    stats: Stats
    puzzleDate: string | null
    guesses: number
    mode: GameMode
    dailySnapshot: DailySnapshot | null
    history: History
}

export type SubmitResult = {
    solved: boolean
    exhausted: boolean
    guesses: number
    inputState: InputState
    puzzleState: PuzzleState
    stats: Stats
}

export type ClueResult = {
    clues: Clues
    input: Input
    puzzle: Puzzle
    keys: Keys
    stats: Stats | null
}
