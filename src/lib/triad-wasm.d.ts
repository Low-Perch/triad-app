declare module 'triad-wasm' {
    export default function init(module_or_path?: any): Promise<any>
    export function init_game(saved_json?: string): unknown
    export function add_key(key: string): unknown
    export function remove_key(): unknown
    export function submit_solution(): unknown
    export function activate_clue(clue_id: string): unknown
    export function save_game(): string
    export function new_game(): unknown
    export function archive_game(date: string): unknown
    export function resume_daily(): unknown
    export function clear_input(): unknown
    export function get_history(): unknown
}
