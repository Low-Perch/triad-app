/**
 * Local calendar date as "YYYY-MM-DD" — matches the engine's local-shifted
 * day boundary.
 */
export function localDateString(d: Date = new Date()): string {
    const month = String(d.getMonth() + 1).padStart(2, '0')
    const day = String(d.getDate()).padStart(2, '0')
    return `${d.getFullYear()}-${month}-${day}`
}
