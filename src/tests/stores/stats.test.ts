import { describe, it, expect, beforeEach } from "vitest";
import {
  getStats, setStats, getSolveRate, getTimeBuckets, INIT_STATS,
} from "../../lib/stores/stats.svelte";

describe("stats store", () => {
  beforeEach(() => {
    setStats({ ...INIT_STATS, solveTimes: [] });
  });

  it("initializes with default values", () => {
    const stats = getStats();
    expect(stats.played).toBe(0);
    expect(stats.solved).toBe(0);
    expect(stats.currentStreak).toBe(0);
    expect(stats.bestTime).toBeNull();
    expect(stats.solveTimes).toEqual([]);
    expect(stats.startedAt).toBeNull();
  });

  describe("getSolveRate", () => {
    it("returns 0 when no puzzles played", () => {
      expect(getSolveRate()).toBe(0);
    });

    it("computes correct percentage", () => {
      setStats({ ...INIT_STATS, played: 2, solved: 1, solveTimes: [] });
      expect(getSolveRate()).toBe(50);
    });

    it("returns 100 when all solved", () => {
      setStats({ ...INIT_STATS, played: 1, solved: 1, solveTimes: [] });
      expect(getSolveRate()).toBe(100);
    });
  });

  describe("getTimeBuckets", () => {
    it("returns all zeros with no solve times", () => {
      expect(getTimeBuckets()).toEqual({
        under10: 0, under30: 0, under60: 0, over60: 0,
      });
    });

    it("distributes times into correct buckets", () => {
      setStats({
        ...INIT_STATS,
        solveTimes: [5, 15, 45, 120],
      });

      expect(getTimeBuckets()).toEqual({
        under10: 1, under30: 1, under60: 1, over60: 1,
      });
    });
  });

  describe("setStats (hydration)", () => {
    it("restores full state", () => {
      setStats({
        played: 10,
        solved: 8,
        currentStreak: 3,
        bestTime: 5,
        solveTimes: [5, 10, 15],
        startedAt: null,
      });
      const stats = getStats();
      expect(stats.played).toBe(10);
      expect(stats.solved).toBe(8);
      expect(stats.bestTime).toBe(5);
    });

    it("preserves reactive proxy reference", () => {
      const ref = getStats();
      setStats({ ...INIT_STATS, played: 42, solveTimes: [] });
      expect(ref.played).toBe(42);
    });
  });
});
