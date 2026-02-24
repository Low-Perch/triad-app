/** @type {import('tailwindcss').Config} */
export default {
    darkMode: 'class',
    content: [
        "./index.html",
        "./src/**/*.{svelte,js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                tone: {
                    bg: 'var(--tone-bg)',
                    surface: 'var(--tone-surface)',
                    text: 'var(--tone-text)',
                    'text-sub': 'var(--tone-text-sub)',
                    border: 'var(--tone-border)',
                    'border-strong': 'var(--tone-border-strong)',
                    key: 'var(--tone-key)',
                    'key-active': 'var(--tone-key-active)',
                    'key-text': 'var(--tone-key-text)',
                    correct: 'var(--tone-correct)',
                    overlay: 'var(--tone-overlay)',
                }
            }
        },
    },
    plugins: [],
}
