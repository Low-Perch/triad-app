import { describe, it, expect, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import Input from "../../lib/components/Input.svelte";
import { setInput, INIT_INPUT, InputState } from "../../lib/stores/input.svelte";
import { setPuzzle, INIT_PUZZLE } from "../../lib/stores/puzzle.svelte";
import { setClues, INIT_CLUES } from "../../lib/stores/clues.svelte";

describe("Input component", () => {
  beforeEach(() => {
    setInput({ ...INIT_INPUT, keys: ["", "", "", ""] });
    setPuzzle({ ...INIT_PUZZLE });
    setClues(structuredClone(INIT_CLUES));
  });

  it("renders 4 input boxes", () => {
    const { container } = render(Input);
    const boxes = container.querySelectorAll(".box");
    expect(boxes).toHaveLength(4);
  });

  it("displays entered keys in boxes", () => {
    setInput({ ...INIT_INPUT, keys: ["F", "I", "", ""] });
    const { container } = render(Input);
    const texts = container.querySelectorAll("p");
    expect(texts[0].textContent).toBe("F");
    expect(texts[1].textContent).toBe("I");
    expect(texts[2].textContent).toBe("");
    expect(texts[3].textContent).toBe("");
  });

  it("applies solved class when state is CORRECT", () => {
    setInput({ ...INIT_INPUT, keys: ["F", "I", "R", "M"], state: InputState.CORRECT });
    const { container } = render(Input);
    const boxes = container.querySelectorAll(".box");
    boxes.forEach((box) => {
      expect(box.className).toContain("solved");
    });
  });

  it("applies shake class when state is INCORRECT", () => {
    setInput({ ...INIT_INPUT, keys: ["T", "E", "S", "T"], state: InputState.INCORRECT });
    const { container } = render(Input);
    const boxes = container.querySelectorAll(".box");
    boxes.forEach((box) => {
      expect(box.className).toContain("shake");
    });
  });
});
