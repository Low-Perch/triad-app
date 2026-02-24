export { InputState } from '../types'
export type { Input } from '../types'

import { InputState } from '../types'
import type { Input } from '../types'

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
