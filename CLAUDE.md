# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Triad is a daily word-puzzle game: find the 3–4 letter key that completes three clue words (e.g. `WARE / REAF / CON` + `FIRM` → `FIRMWARE / REAFFIRM / CONFIRM`). One codebase ships two targets:

- **Desktop** — Tauri v2 app (fixed 320x500 window, config in `src-tauri/tauri.conf.json`)
- **Web/PWA** — the same Svelte frontend running the Rust engine compiled to WASM, deployed to Cloudflare Pages

Frontend is Svelte 5 (runes) + TypeScript + Tailwind CSS v3. All game logic lives in Rust.

## Commands

| Task | Command |
|------|---------|
| Desktop dev | `pnpm tauri dev` |
| Web dev (rebuilds WASM first) | `pnpm dev:web` (port 3000) |
| Frontend only, Tauri bridge | `pnpm dev` (port 1420, Tauri requirement) |
| Type check | `pnpm check` |
| Frontend tests | `pnpm test` (watch: `pnpm test:watch`, coverage: `pnpm test:coverage`) |
| Single frontend test file | `pnpm vitest run src/tests/stores/puzzle.test.ts` |
| Rust tests | `cargo test -p triad-core` |
| Single Rust test | `cargo test -p triad-core daily_puzzle_is_deterministic` |
| Rebuild WASM only | `pnpm wasm:build` (wasm-pack → `wasm-pkg/`) |
| Desktop build | `pnpm tauri build` |
| Web build | `pnpm build:web` (→ `dist-web/`) |

Package manager is **pnpm**. Dictionary curation: `python3 scripts/filter_dict.py` regenerates `crates/triad-core/src/resources/dict.json` (requires `nltk` + `wordfreq`; manual removals go in `scripts/blocklist.txt`). It removes plurals and regularly suffixed inflections (-s/-es/-ed/-ing when the base form exists) outright, and assigns each key a difficulty `tier` (1 easy / 2 medium / 3 hard — terciles of mean clue-word zipf, recomputed per run). **After any dict change**, re-run `cargo run -p triad-core --bin pin_puzzles -- <end-date>` to regenerate the future tail of `crates/triad-core/src/resources/pinned.json`, then `pnpm wasm:build` (both resources are embedded).

## Architecture

### Rust owns all game logic (Cargo workspace)

The workspace (root `Cargo.toml`) has three members:

- **`crates/triad-core`** — the engine, shared by both targets:
  - `engine.rs` — public API: `init_game`, `add_key`, `remove_key`, `submit_solution`, `activate_clue`, `new_game`, `archive_game`, `resume_daily`, `clear_input`, `get_history`
  - `game.rs` — state transitions: guess counting (`MAX_GUESSES = 6`), lifeline effects, stats/streak updates, daily stash/restore. Finished daily/archive games are recorded per-date in `GameState::history` (`DayRecord`: solved/guesses/`daily` flag; solved entries never downgrade) — this feeds the Archive calendar
  - `generator.rs` — puzzle generation from the embedded `resources/dict.json`. Daily puzzles are deterministic: ChaCha8 RNG seeded by puzzle number (days since the 2026-07-27 epoch — the series relaunch; #1, the first full ramped daily, is 2026-07-28), with word lists sorted before selection for cross-platform determinism. `now_unix_secs()` panics on WASM — timestamps must be passed in from JS. All timestamps fed to the engine are pre-shifted by the local UTC offset so day boundaries fall at local midnight (wasm shifts via `getTimezoneOffset`, desktop via `chrono::Local`)
  - **Difficulty & fairness**: dailies follow a weekday ramp (`tier_for_puzzle_number`: Mon/Tue easy, Wed–Fri medium, Sat/Sun hard) over the curation-assigned `DictEntry::tier`; random games draw from all tiers. Every generated selection (daily and random) must pass `is_fair_selection`: the key is the *only* same-length chunk completing all three fragments to dictionary words (no alternate valid answer), at most one fragment is ≤2 letters, and no clue word is contained in another (sugar/sugary). The frontend mirrors the epoch in `src/lib/date.ts` (`PUZZLE_EPOCH`, `dateStringFromPuzzleNumber`) — keep them in sync with `EPOCH_SECS`
  - Daily/archive puzzles are **pinned** in `resources/pinned.json` (`generate_daily_puzzle` serves pins first, seeded generation beyond the pinned range), so `dict.json` edits can't rewrite published puzzles — they affect only random games and the unpinned future. The pin file is append-only for published dates: `src/bin/pin_puzzles.rs` preserves entries through tomorrow and regenerates the rest; hand-edit an entry only to retract a bad published puzzle. Keep the pinned horizon 12+ months out — a monthly workflow (`.github/workflows/extend-pins.yml`) opens a PR extending it to 18 months, and a `triad-core` guard test (also run by the deploy workflow) fails if it drops under 6
  - `models.rs` — serde types, all `rename_all = "camelCase"`; these must stay in sync with the TS mirrors in `src/lib/types.ts`
- **`crates/triad-wasm`** — thin wasm-bindgen wrapper; holds `GameState` in a `thread_local`, returns JS values via serde
- **`src-tauri`** — thin Tauri shell: `commands.rs` (one `#[tauri::command]` per engine function, `GameState` in a managed `Mutex`, persists after every mutation), `persistence.rs` (tauri-plugin-store → `.settings.dat`)

Frontend stores are render caches, not sources of truth — every mutation goes through the bridge to Rust, and the returned state slices re-hydrate the stores. Never add game/validation logic in TypeScript.

### Build-time platform switch

The frontend never imports Tauri or WASM directly; it imports `$lib/bridge` and `$lib/lifecycle`, which each Vite config aliases to a platform implementation:

- `vite.config.ts` (desktop) → `bridge.tauri.ts` (invokes commands; Rust persists), `lifecycle.tauri.ts` (save on window close/focus change)
- `vite.config.web.ts` (web) → `bridge.web.ts` (calls WASM, persists to `localStorage` after every call), `lifecycle.web.ts` (save on `visibilitychange`/`beforeunload`)

Both bridges expose an identical async API. `tsconfig.json` `paths` maps the aliases to the `.tauri` variants for typechecking.

**Adding an engine operation requires touching all parallel surfaces:** `engine.rs` (+ `game.rs`), `triad-wasm/src/lib.rs`, `src-tauri/src/commands.rs` + the `generate_handler!` list in `src-tauri/src/lib.rs`, both bridge files, and `types.ts`.

### Frontend (`src/`)

`App.svelte` is the orchestrator: it drives an `AppPhase` state machine (`loading → splash → revealing/playing → congrats | failed | solved-today`, plus `error` with retry), captures global `keydown`, calls the bridge, and hydrates stores from results. `ResultScreen` handles end states; share-text generation and new-game flow live in `lib/actions.ts`.

### Store pattern (`src/lib/stores/`)

`.svelte.ts` files with module-level `$state`:

- `getX()` returns the reactive proxy directly; `setX()` hydrates via `Object.assign()` (preserves proxy identity)
- Derived state cannot be exported from `.svelte.ts` — wrap `$derived` in getter functions (see `getKeyLocked()` / `getSolveClueAvailable()` in `clues.svelte.ts`)
- Use `$state.snapshot()` when passing state objects to external APIs/serialization

Stores: `puzzle`, `input`, `keys`, `clues`, `guesses`, `stats`, `modal`, `dropdown`, `theme`, `mode`.

### Game rules (encoded in the engine)

- 6 guesses max; guess distribution tracked in stats; day rollover resets the streak if the day's daily went unfinished (checked against the live game or the `daily_snapshot` stash)
- 4 lifelines: reveal position, reveal last letter (locks the last input slot), 50/50 (disables 13 keys), and "solve" — which only unlocks after the other 3 are used and counts as a loss
- The daily puzzle is keyed by `puzzle_date`; `new_game` generates a random (non-daily) puzzle preserving stats
- Three `GameMode`s: `daily`, `random`, `archive`. `archive_game(date)` plays a past daily (stat-neutral — no played/solved/streak/distribution changes); starting an archive/random game stashes the daily into `daily_snapshot`, and `resume_daily` restores it. The web target accepts a `?date=YYYY-MM-DD` deep link
- The frontend re-runs `init_game` when the local date changes (on resume/focus and a 60s timer in `App.svelte`), so the daily rolls over without a relaunch

### Testing

- Frontend: Vitest + Testing Library (jsdom) in `src/tests/`; `src/tests/setup.ts` mocks the Tauri APIs (tests run against the Tauri bridge variant)
- Rust: unit tests live inside `triad-core` (`game.rs`, `generator.rs`)

### Styling / theming

Tailwind v3 with class-based dark mode. Colors are CSS variables (`--tone-*` in `src/styles.css`) exposed as the `tone-*` Tailwind palette; `theme.svelte.ts` toggles `.dark` on `<html>` and persists the choice to `localStorage`. Tile flip/shake animations are in `Input.svelte`.

### Deployment

Pushing to `main` runs `.github/workflows/deploy-web.yml`: builds the WASM package and web bundle, then deploys `dist-web/` to Cloudflare Pages. PWA behavior (autoUpdate service worker, manifest) is configured in `vite.config.web.ts`.

Desktop releases: pushing a `v*` tag runs `.github/workflows/release-desktop.yml` (tests → macOS universal/Windows/Linux installers → draft GitHub Release; keep the tag and `tauri.conf.json` version in step). The web build also ships a standalone `/download` page (`download.html`, second Vite input) that lists the latest release assets via the GitHub API — it's excluded from the SPA service-worker fallback via `navigateFallbackDenylist`.

Desktop auto-update: `tauri-plugin-updater` checks `releases/latest/download/latest.json` on launch (`lifecycle.tauri.ts` → update banner in `App.svelte`). Update artifacts are signed in CI with the `TAURI_SIGNING_PRIVATE_KEY` repo secret (empty password); the private key lives ONLY in `~/.tauri/triad-app.key` and that secret — losing it breaks auto-update for existing installs (rotating the pubkey requires users to reinstall once). The pubkey is embedded in `tauri.conf.json`.
