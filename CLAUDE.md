# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Triad is a desktop word-puzzle game built with **Tauri v2 + Svelte 5 + TypeScript**. It runs as a system tray application — the window is non-resizable (350x500), undecorated, always-on-top, and toggled via tray icon click. Players find a 4-letter key that completes multiple clue words simultaneously.

## Commands

| Task | Command |
|------|---------|
| Dev (full app) | `pnpm tauri dev` |
| Dev (frontend only) | `pnpm dev` |
| Type check | `pnpm check` |
| Build desktop app | `pnpm tauri build` |
| Build frontend only | `pnpm build` |

Package manager is **pnpm** (lockfile: `pnpm-lock.yaml`). Vite dev server runs on port 1420 (Tauri requirement).

## Architecture

### Frontend (`src/`)

**Component tree:**
```
App.svelte → Header, Clues, Input, Keys, Modal
                ↓                        ↓
           Dropdown/DropdownItem    Info, Stats (views)
```

`App.svelte` is the orchestrator: loads persisted state on mount, handles keyboard input, validates solutions, and saves game state on focus/close events.

### State Management (`src/lib/stores/`)

Stores use Svelte 5 runes in `.svelte.ts` files with module-level `$state`:
- `getX()` — returns the reactive `$state` object directly
- `setX()` — hydrates from persistence via `Object.assign()` (preserves reactive proxy reference)
- Mutations are direct property assignments (e.g., `puzzle.solved = true`)

**Important:** When passing `$state` objects to external APIs (Tauri store, JSON serialization), use `$state.snapshot()` to get a plain object. Derived state cannot be exported from `.svelte.ts` modules — use getter functions (see `getKeyLocked()` in `clues.svelte.ts`).

**Stores:**
- **puzzle.svelte.ts** — current puzzle data, `PuzzleState` enum (START | CLUE | SOLUTION), solved flag
- **input.svelte.ts** — user's guessed letters (4 slots), `InputState` enum (EDIT | CORRECT | INCORRECT), `lastPositionLocked` flag
- **keys.svelte.ts** — keyboard state, `disabledKeys` array (for 50/50 lifeline)
- **clues.svelte.ts** — lifeline usage tracking (position, letter, 50/50), max 3 per puzzle. `getKeyLocked()` wraps a `$derived` value
- **modal.svelte.ts** — visibility + current view (`"info"` | `"support"` | `"stats"`)
- **app.ts** — persistence layer via `@tauri-apps/plugin-store`, saves to `.settings.dat`. Uses lazy async `Store.load()` init pattern

### Rust Backend (`src-tauri/`)

- `lib.rs` — app builder with `TrayIconBuilder`, menu setup (Hide/Quit), window positioning via `tauri-plugin-positioner`, store via `tauri-plugin-store`
- `main.rs` — thin shim calling `triad::run()`
- `capabilities/default.json` — Tauri v2 permissions (core, store, positioner)
- Tray left-click toggles window show/hide; window positions at `TrayCenter`

### Game Flow

1. Keyboard events captured on `App.svelte` via `keydown` listener
2. Alpha keys → `addKey()`, Backspace/Delete → `removeKey()`
3. Enter → `validSolution()` check (in `src/lib/utils/validation.ts`)
4. Correct → `markPuzzleSolved()` + flip animation; Incorrect → shake animation
5. State auto-persisted on focus change and close request

### Styling

Tailwind CSS v3 with PostCSS. Dark theme base color `#242124`. Custom CSS animations defined in `Input.svelte` (`.solved` 3D flip, `.shake` for incorrect answers).
