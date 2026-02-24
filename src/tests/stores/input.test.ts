import { describe, it, expect, beforeEach } from "vitest";
import {
  getInput, setInput,
  InputState, INIT_INPUT,
} from "../../lib/stores/input.svelte";

describe("input store", () => {
  beforeEach(() => {
    setInput({ ...INIT_INPUT, keys: ["", "", "", ""] });
  });

  describe("setInput (hydration)", () => {
    it("restores full input state", () => {
      setInput({
        length: 4,
        disabled: false,
        keys: ["A", "B", "C", "D"],
        state: InputState.CORRECT,
        lastPositionLocked: true,
      });
      const input = getInput();
      expect(input.keys).toEqual(["A", "B", "C", "D"]);
      expect(input.state).toBe(InputState.CORRECT);
      expect(input.lastPositionLocked).toBe(true);
    });

    it("preserves reactive proxy reference", () => {
      const ref = getInput();
      setInput({ ...INIT_INPUT, keys: ["X", "", "", ""] });
      expect(ref.keys).toEqual(["X", "", "", ""]);
    });
  });
});
