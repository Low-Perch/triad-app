/**
 * Local calendar date as "YYYY-MM-DD" — matches the engine's local-shifted
 * day boundary.
 */
export function localDateString(d: Date = new Date()): string {
    const month = String(d.getMonth() + 1).padStart(2, '0')
    const day = String(d.getDate()).padStart(2, '0')
    return `${d.getFullYear()}-${month}-${day}`
}

/**
 * Date of a dated puzzle, from its number (days since the 2025-01-01
 * epoch). Mirrors the engine's `date_string_from_number`, which keys
 * `history` — `puzzleDate` can't be used for this, as it stays on the
 * live daily's date during archive games.
 */
export function dateStringFromPuzzleNumber(n: number): string {
    return localDateString(new Date(2025, 0, 1 + n))
}
