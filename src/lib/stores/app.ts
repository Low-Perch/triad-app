import { Store } from '@tauri-apps/plugin-store'

let appStore: Store | null = null

async function getStore(): Promise<Store> {
    if (!appStore) {
        appStore = await Store.load('.settings.dat', { autoSave: true, defaults: {} })
    }
    return appStore
}

// TODO: GameState is just key value pairs of the other store values
type GameState = {
    [key:string]: any
}

export const initGameState = async (gameState: GameState) => {
    const store = await getStore()
    await store.set('game', gameState)
    await store.save()
}

export const getGameState = async (): Promise<GameState | null> => {
    const store = await getStore()
    const gameState = await store.get<GameState>('game')
    return gameState ?? null
}

export const updateGameState = async ({ key, state }: { key: string, state: any }) => {
    const gameState = await getGameState()
    if (!gameState) return

    const store = await getStore()
    const updatedState = {...gameState, [key]: state }
    await store.set('game', updatedState)
    await store.save()
}
