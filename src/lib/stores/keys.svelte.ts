export type { Keys } from '../types'

import type { Keys } from '../types'

export const INIT_KEYS: Keys = { disabledKeys: [], keysDisabled: false }

let keys = $state<Keys>({ ...INIT_KEYS })

export function getKeys(): Keys {
    return keys
}

export function setKeys(state: Keys) {
    Object.assign(keys, state)
}
