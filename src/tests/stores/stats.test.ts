import { describe, it, expect, beforeEach } from "vitest";
import {
  getStats, setStats, getSolveRate, INIT_STATS,
} from "../../lib/stores/stats.svelte";

describe("stats store", () => {
  beforeEach(() => {
    setStats({ ...INIT_STATS });
  });

  it("initializes with default values", () => {
    const stats = getStats();
    expect(stats.played).toBe(0);
    expect(stats.solved).toBe(0);
    expect(stats.currentStreak).toBe(0);
    expect(stats.bestStreak).toBe(0);
  });

  describe("getSolveRate", () => {
    it("returns 0 when no puzzles played", () => {
      expect(getSolveRate()).toBe(0);
    });

    it("computes correct percentage", () => {
      setStats({ ...INIT_STATS, played: 2, solved: 1 });
      expect(getSolveRate()).toBe(50);
    });

    it("returns 100 when all solved", () => {
      setStats({ ...INIT_STATS, played: 1, solved: 1 });
      expect(getSolveRate()).toBe(100);
    });
  });

  describe("setStats (hydration)", () => {
    it("restores full state", () => {
      setStats({
        played: 10,
        solved: 8,
        currentStreak: 3,
        bestStreak: 5,
        guessDistribution: [1, 2, 3, 1, 1, 0],
        solveClueCount: 0,
      });
      const stats = getStats();
      expect(stats.played).toBe(10);
      expect(stats.solved).toBe(8);
      expect(stats.bestStreak).toBe(5);
    });

    it("preserves reactive proxy reference", () => {
      const ref = getStats();
      setStats({ ...INIT_STATS, played: 42 });
      expect(ref.played).toBe(42);
    });
  });
});
