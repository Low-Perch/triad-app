import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import * as bridge from '$lib/bridge'

export type LifecycleHandlers = {
    onResume?: () => void
}

export async function registerLifecycleHooks(handlers: LifecycleHandlers = {}): Promise<(() => void)[]> {
    const appWindow = getCurrentWebviewWindow()
    const cleanups: (() => void)[] = []

    try {
        const unlistenClose = await appWindow.onCloseRequested(async () => {
            await bridge.saveGame()
        })
        cleanups.push(unlistenClose)

        const unlistenFocus = await appWindow.onFocusChanged(async ({ payload: focused }) => {
            if (focused) {
                handlers.onResume?.()
            } else {
                await bridge.saveGame()
            }
        })
        cleanups.push(unlistenFocus)
    } catch (e) {
        console.error('Failed to register window event listeners:', e)
    }

    return cleanups
}
