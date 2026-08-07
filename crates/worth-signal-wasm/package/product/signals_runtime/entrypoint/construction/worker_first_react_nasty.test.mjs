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

async function boot() {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  const signals = await createSignals({ deployment: "workerFirst" });
  return {
    signals,
    createReactSignalsStore,
    async shutdown() {
      try {
        await signals.terminate();
      } catch {
        // already dead
      }
      await cleanupStore();
      await cleanupSignals();
      globalThis.Worker = previousWorker;
    },
  };
}

test("NASTY: diagnostics listener reenters terminate during notify", async () => {
  const world = await boot();
  try {
    const quantity = world.signals.input(1, { debugName: "nasty.reenter.term.q" });
    let terminatePromise = null;
    world.signals.diagnostics().subscribe(() => {
      if (terminatePromise === null) {
        terminatePromise = world.signals.terminate();
      }
    });
    await world.signals.transaction((tx) => tx.set(quantity, 2));
    assert.ok(terminatePromise, "notify must have started terminate");
    await terminatePromise;
    assert.throws(
      () => world.signals.diagnostics().latestObservation(),
      /cannot be used after free/u,
    );
  } finally {
    await world.shutdown();
  }
});

test("NASTY: diagnostics listener disposes React store mid-notify without killing runtime delivery", async () => {
  const world = await boot();
  try {
    const store = world.createReactSignalsStore(world.signals);
    const runtimePulses = [];
    const runtimeHandle = world.signals.diagnostics().subscribe(() => {
      runtimePulses.push(world.signals.diagnostics().latestObservation());
    });
    store.subscribeDiagnostics(() => {
      store.dispose();
    });
    const quantity = world.signals.input(1, { debugName: "nasty.dispose.mid.q" });
    await world.signals.transaction((tx) => tx.set(quantity, 3));
    await flush();
    assert.equal(runtimePulses.length > 0, true, "runtime diagnostics must still deliver");
    assert.notEqual(runtimePulses.at(-1), null);
    // Disposed store must not throw on read, but must not keep receiving pulses.
    const afterDispose = store.getDiagnosticsSnapshot();
    await world.signals.transaction((tx) => tx.set(quantity, 4));
    await flush();
    assert.equal(store.getDiagnosticsSnapshot(), afterDispose);
    runtimeHandle.free();
  } finally {
    await world.shutdown();
  }
});

test("NASTY: two React stores on one runtime must both observe mutations", async () => {
  const world = await boot();
  try {
    const storeA = world.createReactSignalsStore(world.signals);
    const storeB = world.createReactSignalsStore(world.signals);
    let a = 0;
    let b = 0;
    storeA.subscribeDiagnostics(() => {
      a += 1;
    });
    storeB.subscribeDiagnostics(() => {
      b += 1;
    });
    const quantity = world.signals.input(1, { debugName: "nasty.dual.q" });
    await world.signals.transaction((tx) => tx.set(quantity, 4));
    await flush();
    assert.equal(a > 0, true, "store A must observe diagnostics");
    assert.equal(b > 0, true, "store B must observe diagnostics");
    assert.notEqual(storeA.getDiagnosticsSnapshot().latestObservation, null);
    assert.notEqual(storeB.getDiagnosticsSnapshot().latestObservation, null);
    storeA.dispose();
    // B must keep working after A is disposed (shared runtime subscription hazard).
    const before = b;
    await world.signals.transaction((tx) => tx.set(quantity, 5));
    await flush();
    assert.equal(b > before, true, "store B must survive store A dispose");
    storeB.dispose();
  } finally {
    await world.shutdown();
  }
});

test("NASTY: Strict-Mode style subscribe/unsubscribe/resubscribe must not go permanently stale", async () => {
  const world = await boot();
  try {
    const store = world.createReactSignalsStore(world.signals);
    const quantity = world.signals.input(1, { debugName: "nasty.strict.q" });

    const unsub1 = store.subscribeDiagnostics(() => {});
    unsub1(); // Strict Mode simulated unmount
    await world.signals.transaction((tx) => tx.set(quantity, 2));
    await flush();

    // Remount: must refresh to current runtime truth, not construct-time emptiness.
    const unsub2 = store.subscribeDiagnostics(() => {});
    await flush();
    assert.notEqual(
      store.getDiagnosticsSnapshot().latestObservation,
      null,
      "resubscribe after gap mutations must resync",
    );
    unsub2();
    store.dispose();
  } finally {
    await world.shutdown();
  }
});

test("NASTY: overlapping concurrent transactions must not leave React diagnostics permanently empty", async () => {
  const world = await boot();
  try {
    const store = world.createReactSignalsStore(world.signals);
    store.subscribeDiagnostics(() => {});
    const quantity = world.signals.input(0, { debugName: "nasty.race.q" });
    await Promise.all([
      world.signals.transaction((tx) => tx.set(quantity, 1)),
      world.signals.transaction((tx) => tx.set(quantity, 2)),
      world.signals.transaction((tx) => tx.set(quantity, 3)),
    ]);
    await flush();
    assert.notEqual(store.getDiagnosticsSnapshot().latestObservation, null);
    assert.equal([1, 2, 3].includes(quantity()), true);
    store.dispose();
  } finally {
    await world.shutdown();
  }
});

test("NASTY: cross-runtime React store must not alias foreign handles by colliding id", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup: cleanupSignals } = await loadSignalsModule({
    rawSurface: "real",
  });
  const { createReactSignalsStore, cleanup: cleanupStore } = await loadStoreModule();
  let a = null;
  let b = null;
  try {
    a = await createSignals({ deployment: "workerFirst" });
    b = await createSignals({ deployment: "workerFirst" });
    const foreign = a.input(7, { debugName: "nasty.foreign.q" });
    const local = b.input(3, { debugName: "nasty.local.q" });
    // Generated standalone ids collide across runtimes; id-only read is the hazard.
    assert.equal(foreign.id, local.id);
    const storeB = createReactSignalsStore(b);
    assert.equal(storeB.getSignalSnapshot(local), 3);
    assert.throws(
      () => storeB.getSignalSnapshot(foreign),
      /another worker-first runtime/u,
    );
    // Must not silently return the local alias (3) or the foreign value (7).
    storeB.dispose();
  } finally {
    if (a) await a.terminate();
    if (b) await b.terminate();
    await cleanupStore();
    await cleanupSignals();
    globalThis.Worker = previousWorker;
  }
});

test("NASTY: subscribe during notify must not corrupt the listener set", async () => {
  const world = await boot();
  try {
    let late = 0;
    let handleLate = null;
    const handleEarly = world.signals.diagnostics().subscribe(() => {
      if (handleLate === null) {
        handleLate = world.signals.diagnostics().subscribe(() => {
          late += 1;
        });
      }
    });
    const quantity = world.signals.input(1, { debugName: "nasty.sub.during.q" });
    await world.signals.transaction((tx) => tx.set(quantity, 2));
    await flush();
    await world.signals.transaction((tx) => tx.set(quantity, 3));
    await flush();
    assert.equal(late > 0, true, "listener added during notify must receive a later pulse");
    handleEarly.free();
    handleLate.free();
  } finally {
    await world.shutdown();
  }
});

test("NASTY: free subscription handle during its own notify callback", async () => {
  const world = await boot();
  try {
    let handle = null;
    let calls = 0;
    handle = world.signals.diagnostics().subscribe(() => {
      calls += 1;
      handle.free();
    });
    const quantity = world.signals.input(1, { debugName: "nasty.self.free.q" });
    await world.signals.transaction((tx) => tx.set(quantity, 2));
    await flush();
    await world.signals.transaction((tx) => tx.set(quantity, 3));
    await flush();
    assert.equal(calls, 1, "self-freed listener must not be invoked again");
  } finally {
    await world.shutdown();
  }
});

test("NASTY: authored why() is thenable and resolves to the signal id", async () => {
  const world = await boot();
  try {
    const quantity = world.signals.input(1, { debugName: "nasty.why.thenable.q" });
    const total = world.signals.computed(() => quantity() * 2, {
      debugName: "nasty.why.thenable.total",
    });
    await world.signals.transaction((tx) => tx.set(quantity, 2));
    const maybe = world.signals.diagnostics().why(total.id);
    assert.equal(typeof maybe?.then, "function", "authored why must be awaitable");
    const explanation = await maybe;
    assert.equal(explanation.id, total.id);
  } finally {
    await world.shutdown();
  }
});

test("NASTY: rapid store attach/dispose churn under mutation pressure", async () => {
  const world = await boot();
  try {
    const quantity = world.signals.input(0, { debugName: "nasty.churn.q" });
    for (let i = 0; i < 8; i += 1) {
      const store = world.createReactSignalsStore(world.signals);
      const unsub = store.subscribeDiagnostics(() => {});
      await world.signals.transaction((tx) => tx.set(quantity, i + 1));
      unsub();
      store.dispose();
    }
    assert.equal(quantity(), 8);
    // Runtime must still accept a fresh store afterward.
    const finalStore = world.createReactSignalsStore(world.signals);
    finalStore.subscribeDiagnostics(() => {});
    await flush();
    assert.notEqual(finalStore.getDiagnosticsSnapshot().latestObservation, null);
    finalStore.dispose();
  } finally {
    await world.shutdown();
  }
});
