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

export const INIT_INPUT: Input = {
    length: 4,
    disabled: false,
    keys: ['', '', '', ''],
    state: InputState.EDIT,
    lastPositionLocked: false
}

let input = $state<Input>({ ...INIT_INPUT })

export function getInput(): Input {
    return input
}

export function setInput(state: Input) {
    Object.assign(input, state)
}

type UpdateKey = { key?: string, add: boolean }

function updateKey({ key, add = false }: UpdateKey): string[] {
    const keys = input.keys.slice()

    let emptySpaceIdx = add
        ? keys.findIndex(value => value == '')
        : (input.lastPositionLocked ? keys.slice(0, keys.length - 1) : keys)
            .findLastIndex(value => value != '')

    if (emptySpaceIdx < 0) return keys

    if (add && key) {
        if (input.lastPositionLocked) {
            if (emptySpaceIdx == keys.length - 1) return keys
        }
        keys.splice(emptySpaceIdx, 1, key)
    } else {
        keys.splice(emptySpaceIdx, 1, '')
    }

    return keys
}

export function updateInputState(state: InputState) {
    input.state = state
}

export function addKey(key: string) {
    input.keys = updateKey({ key, add: true })
}

export function removeKey() {
    input.keys = updateKey({ add: false })
}

export function lockClueKey(key: string) {
    const syncedKeys = input.keys.slice()
    syncedKeys[syncedKeys.length - 1] = key
    input.lastPositionLocked = true
    input.keys = syncedKeys
}
