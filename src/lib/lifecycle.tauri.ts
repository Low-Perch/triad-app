import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import * as bridge from '$lib/bridge'

export async function registerLifecycleHooks(): Promise<(() => void)[]> {
    const appWindow = getCurrentWebviewWindow()
    const cleanups: (() => void)[] = []

    try {
        const unlistenClose = await appWindow.onCloseRequested(async () => {
            await bridge.saveGame()
        })
        cleanups.push(unlistenClose)

        const unlistenFocus = await appWindow.onFocusChanged(async () => {
            await bridge.saveGame()
        })
        cleanups.push(unlistenFocus)
    } catch (e) {
        console.error('Failed to register window event listeners:', e)
    }

    return cleanups
}
