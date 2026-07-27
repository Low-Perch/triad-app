import { describe, it, expect } from "vitest";
import { dateStringFromPuzzleNumber } from "../lib/date";

describe("dateStringFromPuzzleNumber", () => {
  // Mirrors the engine's date_string_from_number (2026-07-27 epoch)
  it("maps puzzle numbers to their calendar dates", () => {
    expect(dateStringFromPuzzleNumber(0)).toBe("2026-07-27");
    expect(dateStringFromPuzzleNumber(1)).toBe("2026-07-28");
    expect(dateStringFromPuzzleNumber(158)).toBe("2027-01-01");
    expect(dateStringFromPuzzleNumber(365)).toBe("2027-07-27");
  });
});
