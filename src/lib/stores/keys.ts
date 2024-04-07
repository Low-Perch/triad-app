import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { getContext, setContext } from 'svelte'

const STORE = 'keys'

export type Keys = { keysDisabled: boolean, disabledKeys: string[] }
type Context = Writable<Keys>

export const INIT_KEYS: Keys = { disabledKeys: [], keysDisabled: false }

const keys = writable(INIT_KEYS)

export function initKeys() {
    setContext(STORE, keys)
}

export function getKeys() {
    return getContext<Context>(STORE)
}

export function setKeys(state: Keys) {
    keys.set(state)
}

function shuffleArray(array: string[]): string[] {
    for (let i = array.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1))
        const temp = array[i]

        array[i] = array[j]
        array[j] = temp
    }

    return array
}

export function disableKeys(puzzleKey: string) {
    keys.update(prevState => {
        const ALPHA = 'qwertyuiopasdfghjklzxcvbnm'
        const keysToDisable = ALPHA.split('').filter((char: string) => {
            return !puzzleKey.toLowerCase().includes(char)
        })

        const shuffledKeys = shuffleArray(keysToDisable)
        const disabledKeys = shuffledKeys.slice(0, 13)

        return { ...prevState, disabledKeys, keysDisabled: true }
    })
}
