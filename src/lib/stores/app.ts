import { Store } from 'tauri-plugin-store-api'

export const appStore = new Store('.settings.dat')

// TODO: GameState is just key value pairs of the other store values
type GameState = {
    [key:string]: any
}

export const initGameState = async (gameState: GameState) => {
    await appStore.set('game', gameState)
}

export const getGameState = async (): Promise<GameState | null> => {
    const gameState = await appStore.get('game') as GameState
    return gameState
}

export const updateGameState = async ({ key, state }: { key: string, state: any }) => {
    const gameState = await getGameState()
    if (!gameState) return

    const updatedState = {...gameState, [key]: state }
    await appStore.set('game', updatedState)
}
