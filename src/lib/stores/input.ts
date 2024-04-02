import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { getContext, setContext } from 'svelte'

const STORE = 'input'

export type Input = { keys: Array<string>, length: number, disabled: false }
type Context = Writable<Input>

export const INIT_INPUT: Input = { keys: ['', '', '', ''], length: 4, disabled: false }

const input = writable(INIT_INPUT)

export function initInput() {
    setContext(STORE, input)
}

export function getInput() {
    return getContext<Context>(STORE)
}

type UpdateKey = { key?: string, keys: string[], add: boolean }
function updateKey({ key, keys, add = false }: UpdateKey): string[] {
    let emptySpaceIdx = add 
        ? keys.findIndex(value => value == '')
        : keys.findLastIndex(value => value != '')
    
    if (emptySpaceIdx < 0) return keys

    if (add && key) return [...keys.slice(0, emptySpaceIdx), key, ...keys.slice(emptySpaceIdx + 1)]

    return [...keys.slice(0, emptySpaceIdx), '', ...keys.slice(emptySpaceIdx + 1)]
}

export function addKey(key: string) {
    input.update(prevState => {
        const syncedKeys = updateKey({ key, keys: prevState.keys, add: true })
        console.log({ adding: syncedKeys })
        return { ...prevState, keys: syncedKeys }
    })
}

export function removeKey() {
    input.update(prevState => {
        const syncedKeys = updateKey({ keys: prevState.keys, add: false })
        console.log({ removing: syncedKeys })
        return { ...prevState, keys: syncedKeys }
    })
}
