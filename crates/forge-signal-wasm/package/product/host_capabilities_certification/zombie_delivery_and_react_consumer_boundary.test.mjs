import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "./module_loading/load_signals_module.mjs";
import { loadStoreModule } from "./module_loading/load_store_module.mjs";
import { createReactiveRawSignals } from "./runtime_fixture/reactive_raw_signals.mjs";
import { flushMicrotasks } from "./runtime_fixture/scheduling.mjs";

test("host capability certification rejects zombie delivery and keeps React as a pure consumer under mount churn", async () => {
  const {
    hostCapabilityPlan,
    visibilityCapability,
    wrapSignals,
    cleanup,
  } = await loadSignalsModule();
  const {
    createReactSignalsStore,
    cleanup: cleanupStore,
  } = await loadStoreModule();
  try {
    const runtime = createReactiveRawSignals();
    let visibilityState = "visible";
    let visibilityListener = null;

    const signals = wrapSignals(runtime.rawSignals, {
      hostCapabilities: hostCapabilityPlan({
        visibility: visibilityCapability({
          source: {
            current() {
              return visibilityState;
            },
            subscribe(next) {
              visibilityListener = next;
              return () => {};
            },
          },
          compatibility: "LiveOnly",
        }),
      }),
    });

    const label = signals.spec.computedCallback(
      "label",
      () => (signals.host.visibility.isVisible() ? "visible" : "hidden"),
    );
    const store = createReactSignalsStore(signals);
    const diagnosticsSnapshots = [];
    const unsubscribeDiagnostics = store.subscribeDiagnostics(() => {
      diagnosticsSnapshots.push(store.getDiagnosticsSnapshot());
    });

    for (let cycle = 0; cycle < 3; cycle += 1) {
      const unsubscribeSignal = store.subscribeSignal(label, () => {});
      assert.equal(store.getSignalSnapshot(label), signals.read(label));
      visibilityState = cycle % 2 === 0 ? "hidden" : "visible";
      visibilityListener();
      await flushMicrotasks();
      assert.equal(
        store.getSignalSnapshot(label),
        signals.read(label),
        "React snapshots must remain downstream of runtime host-capability truth during mount churn",
      );
      unsubscribeSignal();
    }

    store.dispose();
    unsubscribeDiagnostics();
    signals.free();

    visibilityState = "visible";
    visibilityListener();
    await flushMicrotasks();

    const summary = signals.diagnostics().performanceSummary();
    const report = signals.diagnostics().hostCapabilityReport();

    assert.equal(summary.hostCapabilityDisposalCount, 1);
    assert.equal(
      summary.hostCapabilityStaleInvalidationIgnoredCount,
      1,
      "post-disposal host invalidations must be classified as stale rather than mutating live runtime truth",
    );
    assert.equal(report.families.find((family) => family.family === "visibility")?.latestKind, "InvalidationIgnoredStale");
    assert.equal(diagnosticsSnapshots.length >= 3, true);
  } finally {
    await cleanupStore();
    await cleanup();
  }
});
