import { describe, it, expect, beforeEach } from "vitest";
import {
  getDropdown, setDropdown, toggleDropdown, closeDropdown,
  openDropdown, INIT_DROPDOWN,
} from "../../lib/stores/dropdown.svelte";

describe("dropdown store", () => {
  beforeEach(() => {
    setDropdown({ ...INIT_DROPDOWN });
  });

  it("initializes as closed", () => {
    expect(getDropdown().open).toBe(false);
  });

  describe("toggleDropdown", () => {
    it("opens when closed", () => {
      toggleDropdown();
      expect(getDropdown().open).toBe(true);
    });

    it("closes when open", () => {
      openDropdown();
      toggleDropdown();
      expect(getDropdown().open).toBe(false);
    });
  });

  describe("openDropdown", () => {
    it("sets open to true", () => {
      openDropdown();
      expect(getDropdown().open).toBe(true);
    });
  });

  describe("closeDropdown", () => {
    it("sets open to false", () => {
      openDropdown();
      closeDropdown();
      expect(getDropdown().open).toBe(false);
    });

    it("is idempotent when already closed", () => {
      closeDropdown();
      expect(getDropdown().open).toBe(false);
    });
  });

  describe("setDropdown (hydration)", () => {
    it("restores full state", () => {
      setDropdown({ open: true });
      expect(getDropdown().open).toBe(true);
    });

    it("preserves reactive proxy reference", () => {
      const ref = getDropdown();
      setDropdown({ open: true });
      expect(ref.open).toBe(true);
    });
  });
});
