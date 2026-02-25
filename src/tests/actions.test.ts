import { describe, it, expect, beforeEach } from "vitest";
import { generateShareText } from "../lib/actions";
import { setPuzzle, INIT_PUZZLE } from "../lib/stores/puzzle.svelte";
import { setClues, INIT_CLUES } from "../lib/stores/clues.svelte";
import { setStats, INIT_STATS } from "../lib/stores/stats.svelte";
import { setGuesses } from "../lib/stores/guesses.svelte";

describe("generateShareText", () => {
  beforeEach(() => {
    setPuzzle({ ...INIT_PUZZLE });
    setClues(structuredClone(INIT_CLUES));
    setStats({ ...INIT_STATS });
    setGuesses(0);
  });

  it("returns title with all green squares and guess count", () => {
    setGuesses(2);
    const text = generateShareText();
    expect(text).toBe("Triad 🟩🟩🟩🟩 2/6");
  });

  it("shows yellow squares for active (used) clues", () => {
    const cluesState = structuredClone(INIT_CLUES);
    cluesState.clues[0].active = true;
    cluesState.clues[2].active = true;
    cluesState.used = 2;
    setClues(cluesState);
    setGuesses(3);

    const text = generateShareText();
    expect(text).toBe("Triad 🟨🟩🟨🟩 3/6");
  });

  it("shows X/6 when solve clue is used", () => {
    const cluesState = structuredClone(INIT_CLUES);
    cluesState.clues[0].active = true;
    cluesState.clues[1].active = true;
    cluesState.clues[2].active = true;
    cluesState.clues[3].active = true;
    cluesState.used = 4;
    setClues(cluesState);
    setGuesses(4);

    const text = generateShareText();
    expect(text).toBe("Triad 🟨🟨🟨⬛ X/6");
  });

  it("includes puzzle number for daily puzzles", () => {
    setPuzzle({ ...INIT_PUZZLE, puzzleNumber: 42 });
    setGuesses(1);
    const text = generateShareText();
    expect(text).toBe("Triad #42 🟩🟩🟩🟩 1/6");
  });

  it("omits puzzle number for free-play puzzles", () => {
    setPuzzle({ ...INIT_PUZZLE, puzzleNumber: null });
    const text = generateShareText();
    expect(text).toMatch(/^Triad 🟩/);
    expect(text).not.toContain("#");
  });

  it("includes streak when active", () => {
    setStats({ ...INIT_STATS, currentStreak: 5 });
    const text = generateShareText();
    expect(text).toContain("🔥 5");
  });

  it("omits streak line when no streak", () => {
    const text = generateShareText();
    expect(text.split("\n")).toHaveLength(1);
  });

  it("combines daily puzzle number with clue squares and streak", () => {
    setPuzzle({ ...INIT_PUZZLE, puzzleNumber: 7 });
    const cluesState = structuredClone(INIT_CLUES);
    cluesState.clues[1].active = true;
    cluesState.used = 1;
    setClues(cluesState);
    setStats({ ...INIT_STATS, currentStreak: 2 });
    setGuesses(3);

    const text = generateShareText();
    const lines = text.split("\n");
    expect(lines[0]).toBe("Triad #7 🟩🟨🟩🟩 3/6");
    expect(lines[1]).toBe("🔥 2");
  });
});
