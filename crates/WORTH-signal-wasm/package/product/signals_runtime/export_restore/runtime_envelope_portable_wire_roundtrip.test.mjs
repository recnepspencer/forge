import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

test("main-thread adapters portable runtime-envelope wire restores callback-free graph truth", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const graph = signals.graph("portableRuntimeEnvelope", (graph) => {
    const scope = graph.scope("worker");
    const count = scope.input(1, { id: "count" });
    const doubled = scope.computedSpec("doubled", {
      reads: [count.id],
      expr: {
        kind: "sum",
        args: [{ kind: "read", id: count.id }, { kind: "read", id: count.id }],
      },
      identity: { kind: "exact" },
    });
    return graph.expose({ inputs: { count }, outputs: { doubled } });
  });

  try {
    graph.input("count").set(9);
    const envelope = signals.adapters().exportRuntimeEnvelope();

    graph.input("count").set(2);
    assert.equal(graph.output("doubled").get(), 4);

    signals.adapters().replaceRuntimeEnvelope(envelope);
    assert.equal(graph.output("doubled").get(), 18);
  } finally {
    signals.free();
    await cleanup();
  }
});
