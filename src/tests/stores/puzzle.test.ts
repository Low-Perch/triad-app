import { describe, it, expect, beforeEach } from "vitest";
import {
  getPuzzle, setPuzzle,
  PuzzleState, INIT_PUZZLE,
} from "../../lib/stores/puzzle.svelte";

describe("puzzle store", () => {
  beforeEach(() => {
    setPuzzle({ ...INIT_PUZZLE });
  });

  it("initializes with default puzzle data", () => {
    const puzzle = getPuzzle();
    expect(puzzle.key).toBe("FIRM");
    expect(puzzle.solved).toBe(false);
    expect(puzzle.state).toBe(PuzzleState.START);
  });

  it("hydrates full puzzle state via setPuzzle", () => {
    setPuzzle({
      key: "TEST",
      solved: true,
      state: PuzzleState.SOLUTION,
      start: "A / B / C",
      clue: "A____ / B / C",
      solution: "ATEST / BTEST / CTEST",
      puzzleNumber: null,
    });
    const puzzle = getPuzzle();
    expect(puzzle.key).toBe("TEST");
    expect(puzzle.solved).toBe(true);
    expect(puzzle.state).toBe(PuzzleState.SOLUTION);
    expect(puzzle.start).toBe("A / B / C");
  });

  it("preserves reactive proxy reference after setPuzzle", () => {
    const ref = getPuzzle();
    setPuzzle({ ...INIT_PUZZLE, key: "NEWK" });
    expect(ref.key).toBe("NEWK");
  });
});
