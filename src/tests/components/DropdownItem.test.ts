import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import DropdownItem from "../../lib/components/DropdownItem.svelte";

describe("DropdownItem component", () => {
  const activeClue = { id: "position", note: "Reveal 1 position", active: true };
  const inactiveClue = { id: "letter", note: "Reveal last letter", active: false };

  it("renders the clue note text", () => {
    const onMessage = vi.fn();
    render(DropdownItem, { props: { clue: inactiveClue, onMessage } });
    expect(screen.getByText("Reveal last letter")).toBeInTheDocument();
  });

  it("calls onMessage with clue id when clicked", async () => {
    const onMessage = vi.fn();
    render(DropdownItem, { props: { clue: inactiveClue, onMessage } });
    await fireEvent.click(screen.getByText("Reveal last letter"));
    expect(onMessage).toHaveBeenCalledWith({ id: "letter" });
  });

  it("disables button when clue is active", () => {
    const onMessage = vi.fn();
    render(DropdownItem, { props: { clue: activeClue, onMessage } });
    const button = screen.getByText("Reveal 1 position").closest("button");
    expect(button).toBeDisabled();
  });

  it("enables button when clue is inactive", () => {
    const onMessage = vi.fn();
    render(DropdownItem, { props: { clue: inactiveClue, onMessage } });
    const button = screen.getByText("Reveal last letter").closest("button");
    expect(button).not.toBeDisabled();
  });
});
