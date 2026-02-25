import { describe, it, expect } from "vitest";
import { formatTime } from "../lib/format";

describe("formatTime", () => {
  it("formats seconds under 60 as Xs", () => {
    expect(formatTime(0)).toBe("0s");
    expect(formatTime(1)).toBe("1s");
    expect(formatTime(30)).toBe("30s");
    expect(formatTime(59)).toBe("59s");
  });

  it("formats exactly 60 seconds as 1m", () => {
    expect(formatTime(60)).toBe("1m");
  });

  it("formats minutes with remaining seconds", () => {
    expect(formatTime(61)).toBe("1m 1s");
    expect(formatTime(90)).toBe("1m 30s");
    expect(formatTime(125)).toBe("2m 5s");
  });

  it("formats exact minutes without seconds", () => {
    expect(formatTime(120)).toBe("2m");
    expect(formatTime(300)).toBe("5m");
  });
});
