import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/svelte";
import Clues from "../../lib/components/Clues.svelte";

describe("Clues component", () => {
  it("renders the provided text", () => {
    render(Clues, { props: { text: "WARE / REAF / CON" } });
    expect(screen.getByText("WARE / REAF / CON")).toBeInTheDocument();
  });

  it("applies uppercase styling", () => {
    const { container } = render(Clues, { props: { text: "test text" } });
    const wrapper = container.querySelector("div");
    expect(wrapper?.className).toContain("uppercase");
  });
});
