export type Dropdown = { open: boolean }

export const INIT_DROPDOWN: Dropdown = { open: false }

let dropdown = $state<Dropdown>({ ...INIT_DROPDOWN })

export function getDropdown(): Dropdown {
    return dropdown
}

export function setDropdown(state: Dropdown) {
    Object.assign(dropdown, state)
}

export function toggleDropdown() {
    dropdown.open = !dropdown.open
}

export function closeDropdown() {
    dropdown.open = false
}

export function openDropdown() {
    dropdown.open = true
}
