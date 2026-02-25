import { describe, it, expect, beforeEach } from "vitest";
import {
  getClues, setClues, getKeyLocked, getSolveClueAvailable, INIT_CLUES,
} from "../../lib/stores/clues.svelte";

describe("clues store", () => {
  beforeEach(() => {
    setClues(structuredClone(INIT_CLUES));
  });

  it("initializes with 4 inactive clues", () => {
    const clues = getClues();
    expect(clues.clues).toHaveLength(4);
    expect(clues.clues.every((c) => !c.active)).toBe(true);
    expect(clues.used).toBe(0);
    expect(clues.available).toBe(true);
  });

  it("has solve clue as the 4th clue", () => {
    const clues = getClues();
    expect(clues.clues[3].id).toBe("solve");
    expect(clues.clues[3].note).toBe("Reveal answer");
  });

  describe("getKeyLocked", () => {
    it("returns false when letter clue is not active", () => {
      expect(getKeyLocked()).toBe(false);
    });

    it("returns true when letter clue is marked active via hydration", () => {
      const cluesState = structuredClone(INIT_CLUES);
      cluesState.clues[1].active = true; // letter clue
      cluesState.used = 1;
      setClues(cluesState);
      expect(getKeyLocked()).toBe(true);
    });

    it("is not affected by other clue activations", () => {
      const cluesState = structuredClone(INIT_CLUES);
      cluesState.clues[0].active = true; // position
      cluesState.clues[2].active = true; // 50/50
      cluesState.used = 2;
      setClues(cluesState);
      expect(getKeyLocked()).toBe(false);
    });
  });

  describe("getSolveClueAvailable", () => {
    it("returns false when fewer than 3 clues used", () => {
      const cluesState = structuredClone(INIT_CLUES);
      cluesState.clues[0].active = true;
      cluesState.used = 1;
      setClues(cluesState);
      expect(getSolveClueAvailable()).toBe(false);
    });

    it("returns true when all 3 standard clues used", () => {
      const cluesState = structuredClone(INIT_CLUES);
      cluesState.clues[0].active = true;
      cluesState.clues[1].active = true;
      cluesState.clues[2].active = true;
      cluesState.used = 3;
      setClues(cluesState);
      expect(getSolveClueAvailable()).toBe(true);
    });

    it("returns false when solve clue already used", () => {
      const cluesState = structuredClone(INIT_CLUES);
      cluesState.clues[0].active = true;
      cluesState.clues[1].active = true;
      cluesState.clues[2].active = true;
      cluesState.clues[3].active = true;
      cluesState.used = 4;
      setClues(cluesState);
      expect(getSolveClueAvailable()).toBe(false);
    });
  });

  describe("setClues (hydration)", () => {
    it("restores full state", () => {
      const cluesState = structuredClone(INIT_CLUES);
      cluesState.clues[0].active = true;
      cluesState.used = 1;
      setClues(cluesState);
      const clues = getClues();
      expect(clues.clues[0].active).toBe(true);
      expect(clues.used).toBe(1);
    });

    it("preserves reactive proxy reference", () => {
      const ref = getClues();
      const cluesState = structuredClone(INIT_CLUES);
      cluesState.used = 2;
      setClues(cluesState);
      expect(ref.used).toBe(2);
    });
  });
});
