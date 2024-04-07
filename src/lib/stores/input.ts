import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { getContext, setContext } from 'svelte'

const STORE = 'input'

export enum InputState {
    EDIT = 'edit',
    CORRECT = 'correct',
    INCORRECT = 'incorrect',
}

export type Input = { 
    length: number,
    state: InputState,
    disabled: boolean,
    keys: Array<string>,
    lastPositionLocked: boolean
}

type Context = Writable<Input>

export const INIT_INPUT: Input = { 
    length: 4,
    disabled: false,
    keys: ['', '', '', ''],
    state: InputState.EDIT,
    lastPositionLocked: false
}

const input = writable(INIT_INPUT)

export function initInput() {
    setContext(STORE, input)
}

export function setInput(state: Input) {
    input.set(state)
}

export function getInput() {
    return getContext<Context>(STORE)
}

type UpdateKey = { key?: string, prevState: Input, add: boolean }

function updateKey({ key, prevState, add = false }: UpdateKey): string[] {
    const keys = prevState.keys.slice()

    let emptySpaceIdx = add 
        ? keys.findIndex(value => value == '')
        : (prevState.lastPositionLocked ? keys.slice(0, keys.length - 1) : keys)
            .findLastIndex(value => value != '')
    
    if (emptySpaceIdx < 0) return keys

    if (add && key) {
        if (prevState.lastPositionLocked) {
            if (emptySpaceIdx == keys.length - 1) return keys
        } 

        keys.splice(emptySpaceIdx, 1, key)
    } else {
        keys.splice(emptySpaceIdx, 1, '')
    }

    return keys
}

export function updateInputState(state: InputState) {
    input.update(prevState => ({ ...prevState, state }))
}

export function addKey(key: string) {
    input.update(prevState => {
        const syncedKeys = updateKey({ key, prevState, add: true })
        return { ...prevState, keys: syncedKeys }
    })
}

export function removeKey() {
    input.update(prevState => {
        const syncedKeys = updateKey({ prevState, add: false })
        return { ...prevState, keys: syncedKeys }
    })
}

export function lockClueKey(key: string) {
    input.update(prevState => {
        const syncedKeys = prevState.keys.slice()
        syncedKeys[syncedKeys.length - 1] = key
        return { ...prevState, lastPositionLocked: true, keys: syncedKeys }
    })
}
