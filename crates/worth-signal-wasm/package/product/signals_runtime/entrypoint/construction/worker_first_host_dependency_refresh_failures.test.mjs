import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("worker-first host dependency refresh failures do not inflate read-denial totals", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createSignals,
    hostCapabilityPlan,
    onlineCapability,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });
  let workerSignals = null;
  try {
    const onlineSource = createSubscribableSource("online");
    workerSignals = await createSignals({
      hostCapabilities: hostCapabilityPlan({
        online: onlineCapability({ source: onlineSource.source }),
      }),
    });
    const label = workerSignals.computed(() => {
      const online = workerSignals.host.online.state();
      if (online === "offline") {
        throw new Error("synthetic refresh failure");
      }
      return online;
    });

    assert.equal(label(), "online");
    onlineSource.set("offline");

    const report = await waitForRefreshFailure(workerSignals);
    assert.equal(report.totals.dependencyRefreshCount, 1);
    assert.equal(report.totals.dependencyRefreshFailureCount, 1);
    assert.equal(report.totals.readDenialCount, 0);
    assert.equal(report.totals.unavailabilityArtifactCount, 1);
    assert.equal(report.lineage.at(-1).kind, "HostDependencyRefreshFailed");
  } finally {
    workerSignals?.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

function createSubscribableSource(initialValue) {
  let currentValue = initialValue;
  const listeners = new Set();
  return {
    source: {
      current() {
        return currentValue;
      },
      subscribe(listener) {
        listeners.add(listener);
        return () => listeners.delete(listener);
      },
    },
    set(nextValue) {
      currentValue = nextValue;
      for (const listener of listeners) listener();
    },
  };
}

async function waitForRefreshFailure(workerSignals) {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    const report = workerSignals.diagnostics().hostCapabilityReport();
    if (report.totals.dependencyRefreshFailureCount === 1) return report;
    await new Promise((resolve) => setTimeout(resolve, 10));
  }
  const report = workerSignals.diagnostics().hostCapabilityReport();
  assert.equal(report.totals.dependencyRefreshFailureCount, 1);
  return report;
}
