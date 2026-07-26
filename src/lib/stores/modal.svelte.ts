export type ModalView = "info" | "stats" | "archive"
export type Modal = { visible: boolean; view: ModalView | null }

export const INIT_MODAL: Modal = { visible: false, view: null }

let modal = $state<Modal>({ ...INIT_MODAL })

export function getModal(): Modal {
    return modal
}

export function openModal(view: ModalView) {
    modal.visible = true
    modal.view = view
}

export function closeModal() {
    modal.visible = false
    modal.view = null
}

export function updateModal(state: Partial<Modal>) {
    if (state.visible !== undefined) modal.visible = state.visible
    if (state.view !== undefined) modal.view = state.view
}

export function setModal(state: Modal) {
    Object.assign(modal, state)
}
