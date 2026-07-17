import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../signals_runtime/module_loading/load_signals_module.mjs";

test("Core Signals guides execute through the real compatibility runtime", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });

  try {
    const quantity = signals.input(2, { debugName: "quantity" });
    const unitPrice = signals.input(18, { debugName: "unitPrice" });
    const total = signals.computed(() => quantity() * unitPrice(), { debugName: "total" });

    await signals.transaction((tx) => {
      tx.set(quantity, 4);
      tx.set(unitPrice, 20);
    });
    assert.equal(total(), 80);

    const options = signals.input([
      { id: "ground", label: "Ground" },
      { id: "air", label: "Air" },
    ]);
    const selected = signals.linked({
      source: () => options(),
      computation: (nextOptions, previous) =>
        nextOptions.find((option) => option.id === previous?.value?.id) ?? nextOptions[0] ?? null,
    });
    selected.set({ id: "air", label: "Air" });
    options.set([{ id: "ground", label: "Ground" }]);
    selected.relink();
    assert.equal(selected()?.id, "ground");

    const pricing = signals.graph("documentationPricing", (graph) => {
      const state = graph.scope("state");
      const graphQuantity = state.input(2);
      const graphUnitPrice = state.input(18);
      const graphTotal = state.computed(() => graphQuantity() * graphUnitPrice());
      return graph.expose({
        inputs: { quantity: graphQuantity, unitPrice: graphUnitPrice },
        outputs: { total: graphTotal },
      });
    });
    await pricing.writeInput("quantity", 4);
    assert.equal(pricing.read().total, 72);
    assert.equal((await signals.diagnostics().why(total.id)).id, total.id);
  } finally {
    signals.free();
    await cleanup();
  }
});

test("the first-signal tutorial executes on default worker-first deployment", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals();

  try {
    const quantity = signals.input(2, { debugName: "quantity" });
    const unitPrice = signals.input(18, { debugName: "unitPrice" });
    const total = signals.computed(() => quantity() * unitPrice(), { debugName: "total" });

    const summary = await signals.transaction((tx) => {
      tx.set(quantity, 4);
      tx.set(unitPrice, 20);
    });

    assert.equal(summary.touchedNodes > 0, true);
    assert.equal(total(), 80);
  } finally {
    signals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
