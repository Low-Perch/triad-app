# Triad

A word puzzle game that lives in your system tray. Given three clue fragments, find the 4-letter key that completes all three to form full words.

**Example:** `WARE / REAF / CON` + `FIRM` = `FIRMWARE / REAFFIRM / CONFIRM`

## Screenshots

<!-- TODO: Add screenshots -->

## How to Play

1. Three word fragments are displayed as clues
2. Type a 4-letter key using the on-screen or physical keyboard
3. Press **Enter** (or **GO**) to submit your answer
4. If correct, the tiles flip green. If wrong, they shake.

### Lifelines

You get 3 lifelines per puzzle (accessible from the top-right menu):

- **Reveal 1 position** — switches the clue display to show letter positions
- **Reveal last letter** — locks the final letter of the key into place
- **50/50** — disables 13 incorrect keys from the keyboard

## Tech Stack

- [Tauri v2](https://v2.tauri.app/) — desktop runtime
- [Svelte 5](https://svelte.dev/) — UI framework (using runes)
- [TypeScript](https://www.typescriptlang.org/) — type safety
- [Tailwind CSS v3](https://tailwindcss.com/) — styling
- [Vite](https://vite.dev/) — build tool
- [Rust](https://www.rust-lang.org/) — backend (system tray, window management, persistence)

## Prerequisites

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)
- Platform-specific Tauri dependencies — see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

## Development

```bash
# Install dependencies
pnpm install

# Run the app in development mode (frontend + Tauri)
pnpm tauri dev

# Run only the frontend dev server
pnpm dev

# Type check
pnpm check
```

## Building

```bash
# Build the production desktop app
pnpm tauri build
```

Output bundles are located in `src-tauri/target/release/bundle/`.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
