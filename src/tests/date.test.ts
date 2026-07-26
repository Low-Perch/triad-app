import { describe, it, expect } from "vitest";
import { dateStringFromPuzzleNumber } from "../lib/date";

describe("dateStringFromPuzzleNumber", () => {
  // Mirrors the engine's date_string_from_number (2025-01-01 epoch)
  it("maps puzzle numbers to their calendar dates", () => {
    expect(dateStringFromPuzzleNumber(0)).toBe("2025-01-01");
    expect(dateStringFromPuzzleNumber(1)).toBe("2025-01-02");
    expect(dateStringFromPuzzleNumber(365)).toBe("2026-01-01");
    expect(dateStringFromPuzzleNumber(496)).toBe("2026-05-12");
  });
});
