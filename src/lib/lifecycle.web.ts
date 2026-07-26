import * as bridge from '$lib/bridge'

export type UpdateInfo = {
    version: string
    install: () => Promise<void>
}

export type LifecycleHandlers = {
    onResume?: () => void
    /** Desktop only: never fired on web — the service worker auto-updates. */
    onUpdateAvailable?: (info: UpdateInfo) => void
}

export async function registerLifecycleHooks(handlers: LifecycleHandlers = {}): Promise<(() => void)[]> {
    const cleanups: (() => void)[] = []

    const handleVisibilityChange = () => {
        if (document.visibilityState === 'hidden') {
            bridge.saveGame()
        } else {
            handlers.onResume?.()
        }
    }

    // bfcache restores skip mount, so treat them as a resume too
    const handlePageShow = (e: PageTransitionEvent) => {
        if (e.persisted) handlers.onResume?.()
    }

    const handleBeforeUnload = () => {
        bridge.saveGame()
    }

    document.addEventListener('visibilitychange', handleVisibilityChange)
    window.addEventListener('pageshow', handlePageShow)
    window.addEventListener('beforeunload', handleBeforeUnload)

    cleanups.push(() => {
        document.removeEventListener('visibilitychange', handleVisibilityChange)
        window.removeEventListener('pageshow', handlePageShow)
        window.removeEventListener('beforeunload', handleBeforeUnload)
    })

    return cleanups
}
