import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "./module_loading/load_signals_module.mjs";
import { buildHostRawSignals } from "./runtime_fixture/host_raw_signals.mjs";

test("host capability stale invalidations are ignored after runtime disposal", async () => {
  const { wrapSignals, hostCapabilityPlan, visibilityCapability, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    let currentVisibility = "visible";
    let listener = null;
    const rawSignals = buildHostRawSignals({ values: new Map() }, calls);
    rawSignals.diagnostics = () => ({
      latestObservation() { return null; },
      latestFlow() { return null; },
      latestFailure() { return null; },
      latestRollback() { return null; },
      latestInvalidationPlanningEstimate() { return null; },
      latestInvalidationTraceRecords() { return []; },
      recentHistory() { return []; },
      historyNow() { return { history: {}, callbackNodes: [] }; },
      why() { return null; },
      health() { return null; },
      summaryNow() { return { profile: "Development" }; },
      performanceSummary() { return { activeHandleCount: 0 }; },
      subscribe() { return { free() {} }; },
      free() {},
    });

    const signals = wrapSignals(rawSignals, {
      hostCapabilities: hostCapabilityPlan({
        visibility: visibilityCapability({
          source: {
            current() {
              return currentVisibility;
            },
            subscribe(next) {
              listener = next;
              return () => {
                listener = null;
              };
            },
          },
        }),
      }),
    });

    const diagnostics = signals.diagnostics();
    signals.free();
    currentVisibility = "hidden";
    assert.equal(listener, null);
    const summary = diagnostics.performanceSummary();
    const hostEvents = diagnostics.recentHostCapabilityEvents();
    assert.equal(summary.hostCapabilityDisposalCount, 1);
    assert.equal(summary.hostCapabilityStaleInvalidationIgnoredCount, 0);
    assert.deepEqual(hostEvents, []);
    assert.deepEqual(calls.filter((call) => call[0] === "transaction"), []);
  } finally {
    await cleanup();
  }
});
