import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "./module_loading/load_signals_module.mjs";
import { buildHostRawSignals } from "./runtime_fixture/host_raw_signals.mjs";
import { flushMicrotasks } from "./runtime_fixture/host_runtime_scheduling.mjs";

test("host capability invalidation batches push churn and exposes counters honestly", async () => {
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

    signals.spec.computedCallback(
      "visibilityLabel",
      () => (signals.host.visibility.isVisible() ? "visible" : "hidden"),
    );
    currentVisibility = "hidden";
    listener();
    currentVisibility = "hidden";
    listener();
    currentVisibility = "hidden";
    listener();
    await flushMicrotasks();

    const summary = signals.diagnostics().performanceSummary();
    const latestHostEvent = signals.diagnostics().latestHostCapabilityEvent();
    const recentHostEvents = signals.diagnostics().recentHostCapabilityEvents();
    const hostReport = signals.diagnostics().hostCapabilityReport();
    assert.equal(summary.hostCapabilityRegistrationCount, 1);
    assert.equal(summary.hostCapabilityReadCount, 1);
    assert.equal(summary.hostCapabilityInvalidationCount, 3);
    assert.equal(summary.hostCapabilityInvalidationBatchFlushCount, 1);
    assert.equal(summary.hostCapabilityReevaluationCount, 1);
    assert.equal(summary.hostCapabilityNoOpInvalidationSuppressedCount, 0);
    assert.equal(summary.hostCapabilityInvalidationTouchedNodeCount, 1);
    assert.equal(typeof hostReport.lineageDigest, "string");
    assert.equal(typeof hostReport.breadthDigest, "string");
    assert.equal(hostReport.breadth.maxTouchedNodes, 1);
    assert.equal(hostReport.breadth.maxReevaluatedNodes, 1);
    assert.deepEqual(latestHostEvent, {
      sequence: 1,
      kind: "InvalidationFlushed",
      family: "visibility",
      registrationId: "visibility",
      compatibility: "LiveOnly",
      invalidationMode: "push-driven",
      queuedInvalidationCount: 3,
      previousState: "visible",
      nextState: "hidden",
      touchedNodes: 1,
      reevaluatedNodes: 1,
    });
    assert.deepEqual(recentHostEvents, [latestHostEvent]);
    assert.deepEqual(calls.filter((call) => call[0] === "transaction"), [
      ["transaction", [["set", calls[0][1], "hidden"]]],
    ]);

    signals.free();
  } finally {
    await cleanup();
  }
});
