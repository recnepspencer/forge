import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createSignals constructs the explicit main-thread compatibility lane when requested", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({
      deployment: "mainThreadCompatibility",
    });
    const count = signals.input(1, { debugName: "count" });
    const asyncCount = await signals.inputAsync(5, { debugName: "asyncCount" });
    const asyncComputed = await signals.computedAsync({
      reads: [asyncCount.id],
      expr: { kind: "read", id: asyncCount.id },
      identity: { kind: "exact" },
    });
    const asyncOutput = await signals.outputAsync({
      reads: [asyncComputed.id],
      expr: { kind: "read", id: asyncComputed.id },
      identity: { kind: "exact" },
    });

    signals.transaction((tx) => {
      tx.set(count, 2);
    });
    await signals.transactionAsync((tx) => {
      tx.set(count, 3);
      tx.set(asyncCount, 6);
    });
    await signals.batchAsync((tx) => {
      tx.set(count, 4);
      tx.set(asyncCount, 7);
    });

    assert.equal(count(), 4);
    assert.equal(asyncCount(), 7);
    assert.equal(asyncComputed(), 7);
    assert.equal(asyncOutput(), 7);
    signals.free();
  } finally {
    await cleanup();
  }
});

test("createCallableSignals resolves through the explicit compatibility lane", async () => {
  const { createCallableSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createCallableSignals();
    const status = signals.input("draft");
    assert.equal(status(), "draft");
    signals.free();
  } finally {
    await cleanup();
  }
});
