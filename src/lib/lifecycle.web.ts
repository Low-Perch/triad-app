import * as bridge from '$lib/bridge'

export async function registerLifecycleHooks(): Promise<(() => void)[]> {
    const cleanups: (() => void)[] = []

    const handleVisibilityChange = () => {
        if (document.visibilityState === 'hidden') {
            bridge.saveGame()
        }
    }

    const handleBeforeUnload = () => {
        bridge.saveGame()
    }

    document.addEventListener('visibilitychange', handleVisibilityChange)
    window.addEventListener('beforeunload', handleBeforeUnload)

    cleanups.push(() => {
        document.removeEventListener('visibilitychange', handleVisibilityChange)
        window.removeEventListener('beforeunload', handleBeforeUnload)
    })

    return cleanups
}
