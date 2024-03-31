import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { getContext, setContext } from 'svelte'

const STORE = 'modal'

export type ModalView = "info" | "support" | "stats"
export type Modal = { visible: boolean; view?: ModalView }
type Context = Writable<Modal>

export const INIT_MODAL: Modal = { visible: false, view: null }

const modal = writable(INIT_MODAL)

export function initModal() {
    setContext(STORE, modal)
}

export function getModal() {
    return getContext<Context>(STORE)
}

export function openModal(view: ModalView) {
    modal.update(() => ({ visible: true, view }))
}

export function closeModal() {
    modal.update(() => ({ visible: false, view: null }))
}

export function updateModal(state: Modal) {
    modal.update((prevState: Modal) => ({ ...prevState, ...state }))
}
