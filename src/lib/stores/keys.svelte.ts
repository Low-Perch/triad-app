export type Keys = { keysDisabled: boolean, disabledKeys: string[] }

export const INIT_KEYS: Keys = { disabledKeys: [], keysDisabled: false }

let keys = $state<Keys>({ ...INIT_KEYS })

export function getKeys(): Keys {
    return keys
}

export function setKeys(state: Keys) {
    Object.assign(keys, state)
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
    const ALPHA = 'qwertyuiopasdfghjklzxcvbnm'
    const keysToDisable = ALPHA.split('').filter((char: string) => {
        return !puzzleKey.toLowerCase().includes(char)
    })
    const shuffledKeys = shuffleArray(keysToDisable)
    keys.disabledKeys = shuffledKeys.slice(0, 13)
    keys.keysDisabled = true
}
