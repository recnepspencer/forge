import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadStoreModule } from "../../../host_capabilities_certification/module_loading/load_store_module.mjs";
import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

async function flush() {
  await Promise.resolve();
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

test("ATTACK: React diagnostics stay stale if mutations happen before first subscribeDiagnostics", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const store = createReactSignalsStore(signals);
    const quantity = signals.input(1, { debugName: "adv.stale.quantity" });
    await signals.transaction((tx) => {
      tx.set(quantity, 9);
    });
    await flush();

    assert.equal(store.getDiagnosticsSnapshot().latestObservation, null);

    const unsubscribe = store.subscribeDiagnostics(() => {});
    await flush();

    assert.notEqual(
      store.getDiagnosticsSnapshot().latestObservation,
      null,
      "subscribeDiagnostics after prior mutations must refresh from the live runtime",
    );
    unsubscribe();
    store.dispose();
  } finally {
    if (signals) await signals.terminate();
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});

test("ATTACK: one throwing diagnostics listener must not silence the others", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    let secondCalls = 0;
    const handleA = signals.diagnostics().subscribe(() => {
      throw new Error("listener-a-boom");
    });
    const handleB = signals.diagnostics().subscribe(() => {
      secondCalls += 1;
    });

    const quantity = signals.input(0, { debugName: "adv.throw.quantity" });
    await signals.transaction((tx) => {
      tx.set(quantity, 1);
    });
    await flush();

    assert.equal(secondCalls > 0, true, "sibling listener must still run");
    handleA.free();
    handleB.free();
  } finally {
    if (signals) await signals.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("ATTACK: authored why dies when importGraph supersedes the runtime", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  let compatibility = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const quantity = signals.input(2, { debugName: "adv.supersede.quantity" });
    const total = signals.computed(() => quantity() * 2, {
      debugName: "adv.supersede.total",
    });
    await signals.transaction((tx) => tx.set(quantity, 3));
    assert.equal((await signals.diagnostics().why(total.id)).id, total.id);

    compatibility = await createSignals({ deployment: "mainThreadCompatibility" });
    const count = compatibility.input(4);
    const graph = compatibility.graph("advSupersede", {
      inputs: { count },
      outputs: {
        doubled: compatibility.computedSpec("adv:supersede:doubled", {
          reads: [count.id],
          expr: {
            kind: "sum",
            args: [
              { kind: "read", id: count.id },
              { kind: "read", id: count.id },
            ],
          },
          identity: { kind: "exact" },
        }),
      },
    });
    const imported = signals.importGraph(graph.exportDefinition(), graph.exportSnapshot());
    await imported.ready();

    assert.throws(
      () => signals.diagnostics().why(total.id),
      /active imported graph|known authored signal id/u,
    );
  } finally {
    if (signals) await signals.terminate();
    if (compatibility) compatibility.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("ATTACK: disposed React store must not observe later runtime diagnostics", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const store = createReactSignalsStore(signals);
    let calls = 0;
    store.subscribeDiagnostics(() => {
      calls += 1;
    });
    await flush();
    store.dispose();
    const before = calls;

    const quantity = signals.input(1, { debugName: "adv.dispose.quantity" });
    await signals.transaction((tx) => tx.set(quantity, 2));
    await flush();
    assert.equal(calls, before);
  } finally {
    if (signals) await signals.terminate();
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});

test("ATTACK: double-free diagnostics subscription handle is idempotent", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const handle = signals.diagnostics().subscribe(() => {});
    handle.free();
    assert.doesNotThrow(() => handle.free());
    assert.doesNotThrow(() => handle[Symbol.dispose]());
  } finally {
    if (signals) await signals.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("ATTACK: post-terminate diagnostics surface must fail closed", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const quantity = signals.input(1, { debugName: "adv.term.quantity" });
    const terminated = signals;
    await terminated.terminate();
    signals = null;
    assert.throws(
      () => terminated.diagnostics().subscribe(() => {}),
      /cannot be used after free/u,
    );
    assert.throws(
      () => terminated.diagnostics().latestObservation(),
      /cannot be used after free/u,
    );
    assert.throws(
      () => terminated.diagnostics().performanceSummary(),
      /cannot be used after free/u,
    );
    assert.throws(
      () => terminated.diagnostics().why(quantity.id),
      /cannot be used after free/u,
    );
  } finally {
    if (signals) await signals.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("ATTACK: createReactSignalsStore after terminate must fail closed", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const terminated = signals;
    await terminated.terminate();
    signals = null;
    assert.throws(
      () => createReactSignalsStore(terminated),
      /cannot be used after free/u,
    );
  } finally {
    if (signals) await signals.terminate();
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});

test("ATTACK: throwing React diagnostics subscriber must not silence siblings", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let signals = null;
  try {
    signals = await createSignals({ deployment: "workerFirst" });
    const store = createReactSignalsStore(signals);
    let second = 0;
    store.subscribeDiagnostics(() => {
      throw new Error("react-diag-boom");
    });
    store.subscribeDiagnostics(() => {
      second += 1;
    });
    const quantity = signals.input(1, { debugName: "adv.react.throw.quantity" });
    await signals.transaction((tx) => tx.set(quantity, 4));
    await flush();
    assert.equal(second > 0, true);
    store.dispose();
  } finally {
    if (signals) await signals.terminate();
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});
