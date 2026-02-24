import { describe, it, expect, beforeEach } from "vitest";
import { getModal, openModal, closeModal, updateModal, setModal, INIT_MODAL } from "../../lib/stores/modal.svelte";

describe("modal store", () => {
  beforeEach(() => {
    setModal({ ...INIT_MODAL });
  });

  it("initializes with modal hidden and no view", () => {
    const modal = getModal();
    expect(modal.visible).toBe(false);
    expect(modal.view).toBeNull();
  });

  it("opens modal with a specific view", () => {
    openModal("info");
    const modal = getModal();
    expect(modal.visible).toBe(true);
    expect(modal.view).toBe("info");
  });

  it("closes modal and clears view", () => {
    openModal("stats");
    closeModal();
    const modal = getModal();
    expect(modal.visible).toBe(false);
    expect(modal.view).toBeNull();
  });

  it("updates partial modal state", () => {
    updateModal({ visible: true });
    const modal = getModal();
    expect(modal.visible).toBe(true);
    expect(modal.view).toBeNull();
  });

  it("updates view without changing visibility", () => {
    openModal("info");
    updateModal({ view: "stats" });
    const modal = getModal();
    expect(modal.visible).toBe(true);
    expect(modal.view).toBe("stats");
  });

  it("sets full modal state via setModal", () => {
    setModal({ visible: true, view: "support" });
    const modal = getModal();
    expect(modal.visible).toBe(true);
    expect(modal.view).toBe("support");
  });
});
