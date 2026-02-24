import { describe, it, expect, beforeEach } from "vitest";
import { getKeys, setKeys, INIT_KEYS } from "../../lib/stores/keys.svelte";

describe("keys store", () => {
  beforeEach(() => {
    setKeys({ ...INIT_KEYS });
  });

  it("initializes with no disabled keys", () => {
    const keys = getKeys();
    expect(keys.disabledKeys).toEqual([]);
    expect(keys.keysDisabled).toBe(false);
  });

  describe("setKeys (hydration)", () => {
    it("restores full keys state", () => {
      setKeys({ disabledKeys: ["a", "b", "c"], keysDisabled: true });
      const keys = getKeys();
      expect(keys.disabledKeys).toEqual(["a", "b", "c"]);
      expect(keys.keysDisabled).toBe(true);
    });

    it("preserves reactive proxy reference", () => {
      const ref = getKeys();
      setKeys({ disabledKeys: ["z"], keysDisabled: true });
      expect(ref.disabledKeys).toEqual(["z"]);
    });
  });
});
