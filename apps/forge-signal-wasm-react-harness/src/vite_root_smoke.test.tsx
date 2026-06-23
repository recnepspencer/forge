import { createSignals } from "@aust-group/forge-signal-wasm";
import { describe, expect, it } from "vitest";

describe("Vite package root smoke", () => {
  it("constructs the compatibility runtime from the package root", async () => {
    const signals = await createSignals({
      deployment: "mainThreadCompatibility",
    });
    const count = signals.input(1);

    expect(count()).toBe(1);
  });
});
