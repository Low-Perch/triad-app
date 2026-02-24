export type Theme = 'light' | 'dark'

let theme = $state<Theme>('dark')

export function getTheme(): Theme {
    return theme
}

export function toggleTheme() {
    theme = theme === 'dark' ? 'light' : 'dark'
    applyTheme()
    localStorage.setItem('triad-theme', theme)
}

export function initTheme() {
    const saved = localStorage.getItem('triad-theme') as Theme | null
    if (saved) {
        theme = saved
    } else {
        theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
    }
    applyTheme()
}

function applyTheme() {
    document.documentElement.classList.toggle('dark', theme === 'dark')
}
