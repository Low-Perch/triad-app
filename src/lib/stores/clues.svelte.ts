export type { Clue, Clues } from '../types'

import type { Clues } from '../types'

export const INIT_CLUES: Clues = {
    clues: [
        { id: 'position', active: false, note: 'Reveal 1 position' },
        { id: 'letter', active: false, note: 'Reveal last letter' },
        { id: '50/50', active: false, note: '50/50' }
    ],
    used: 0,
    available: true
}

let clues = $state<Clues>(structuredClone(INIT_CLUES))

export function getClues(): Clues {
    return clues
}

export function setClues(state: Clues) {
    Object.assign(clues, state)
}

const _keyLocked = $derived.by(() => {
    const letterClue = clues.clues.find(({ id }) => id == 'letter')
    if (!letterClue) return false
    return letterClue.active
})

export function getKeyLocked(): boolean {
    return _keyLocked
}
