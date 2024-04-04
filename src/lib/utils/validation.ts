type ValidSolution = { input: string[], key: string }

export function validSolution({ input, key }: ValidSolution): boolean {
    const userInput = input.join('').toUpperCase()
    return userInput == key
}
