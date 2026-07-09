import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createSignals does not silently fall back to compatibility when worker-first construction is unavailable", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = undefined;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    await assert.rejects(
      () => createSignals(),
      /Dedicated worker construction is unavailable/,
    );

    const compatibilitySignals = await createSignals({
      deployment: "mainThreadCompatibility",
    });
    const count = compatibilitySignals.input(1, { debugName: "count" });
    count.set(2);
    assert.equal(count(), 2);
    compatibilitySignals.free();
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
