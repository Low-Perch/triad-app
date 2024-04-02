import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { getContext, setContext } from 'svelte'

const STORE = 'clues'

export type Clue = { id: string; note: string; active: boolean }
export type Clues = { clues: Array<Clue>, used: number, available: boolean }
type Context = Writable<Clues>

export const INIT_CLUES: Clues = { 
    clues: [
        { id: 'position', active: false, note: 'Reveal 1 position' },
        { id: 'letter', active: false, note: 'Reveal last letter' },
        { id: '50/50', active: false, note: '50/50' }
    ],
    used: 0,
    available: true
}

const clues = writable(INIT_CLUES)

export function initClues() {
    setContext(STORE, clues)
}

export function getClues() {
    return getContext<Context>(STORE)
}

export function activateClue(clueId: string) {
    clues.update(prevState => {
        const currentClues = prevState.clues
        const clueIndex = currentClues.findIndex(({ id }) => id == clueId)
        if (clueIndex < 0 || currentClues[clueIndex].active || !prevState.available) return prevState

        const newState = currentClues.slice()
        newState[clueIndex].active = true

        const cluesUsed = prevState.used + 1
        return { used: cluesUsed, clues: newState, available: cluesUsed != 3 }
    })
}
