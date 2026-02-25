let guesses = $state(0)

export function getGuesses(): number {
    return guesses
}

export function setGuesses(value: number) {
    guesses = value
}
