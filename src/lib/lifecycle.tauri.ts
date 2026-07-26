import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import * as bridge from '$lib/bridge'

export type UpdateInfo = {
    version: string
    install: () => Promise<void>
}

export type LifecycleHandlers = {
    onResume?: () => void
    /** Desktop only: fired when a newer release is available. */
    onUpdateAvailable?: (info: UpdateInfo) => void
}

async function checkForUpdate(handlers: LifecycleHandlers) {
    try {
        const update = await check()
        if (!update) return
        handlers.onUpdateAvailable?.({
            version: update.version,
            install: async () => {
                await update.downloadAndInstall()
                await relaunch()
            },
        })
    } catch (e) {
        // No published release yet, offline, or endpoint unreachable — not fatal
        console.warn('Update check failed:', e)
    }
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

    checkForUpdate(handlers)

    return cleanups
}
