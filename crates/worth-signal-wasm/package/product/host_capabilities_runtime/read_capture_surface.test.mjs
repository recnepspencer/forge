import assert from "node:assert/strict";
import test from "node:test";

import { flushMicrotasks, sleep } from "./runtime_fixture/host_runtime_scheduling.mjs";
import { createMultiCapabilitySignalsCase } from "./runtime_fixture/multi_capability_signals_case.mjs";

test("host capability reads lower through wrapped signal capture", async () => {
  const { cleanup, signals, calls, state } =
    await createMultiCapabilitySignalsCase();
  try {
    const computed = signals.spec.computedCallback(
      "visibilityLabel",
      () => (signals.host.visibility.isVisible() ? "visible" : "hidden"),
    );
    const viewportComputed = signals.spec.computedCallback(
      "viewportLabel",
      () => `${signals.host.viewport.width()}x${signals.host.viewport.height()}`,
    );
    const onlineComputed = signals.spec.computedCallback(
      "onlineLabel",
      () => (signals.host.online.isOnline() ? "online" : "offline"),
    );
    const clockComputed = signals.spec.computedCallback(
      "clockLabel",
      () => signals.host.clock.now() + 1,
    );
    const persistenceComputed = signals.spec.computedCallback(
      "persistenceLabel",
      () => signals.host.persistence.value().revision,
    );

    assert.equal(computed.id, "visibilityLabel");
    assert.equal(viewportComputed.id, "viewportLabel");
    assert.equal(onlineComputed.id, "onlineLabel");
    assert.equal(clockComputed.id, "clockLabel");
    assert.equal(persistenceComputed.id, "persistenceLabel");
    assert.deepEqual(signals.host.viewport.size(), { width: 1280, height: 720 });
    assert.equal(signals.host.viewport.width(), 1280);
    assert.equal(signals.host.viewport.height(), 720);
    assert.equal(signals.host.online.state(), "online");
    assert.equal(signals.host.online.isOnline(), true);
    assert.equal(signals.host.clock.now(), 0);

    const viewportSourceId = calls.find(
      (call) =>
        call[0] === "input" &&
        String(call[1]).startsWith("__WorthSignal.host.viewport."),
    )?.[1];
    const visibilitySourceId = calls.find(
      (call) =>
        call[0] === "input" &&
        String(call[1]).startsWith("__WorthSignal.host.visibility."),
    )?.[1];
    const onlineSourceId = calls.find(
      (call) =>
        call[0] === "input" &&
        String(call[1]).startsWith("__WorthSignal.host.online."),
    )?.[1];
    const clockSourceId = calls.find(
      (call) =>
        call[0] === "input" &&
        String(call[1]).startsWith("__WorthSignal.host.clock."),
    )?.[1];
    const persistenceSourceId = calls.find(
      (call) =>
        call[0] === "input" &&
        String(call[1]).startsWith("__WorthSignal.host.persistence."),
    )?.[1];
    const computedCalls = calls.filter((call) => call[0] === "computedCallback");
    assert.deepEqual(computedCalls.map((call) => call[1]), [
      "visibilityLabel",
      "viewportLabel",
      "onlineLabel",
      "clockLabel",
      "persistenceLabel",
    ]);
    assert.deepEqual(computedCalls[0][2], {
      __WorthSignalCallbackCapture: true,
      value: "visible",
      reads: [visibilitySourceId],
      hostCapabilityReads: [
        {
          family: "visibility",
          registrationId: "visibility",
          compatibility: "LiveOnly",
        },
      ],
      runtimeReadBreadth: 0,
    });
    assert.deepEqual(computedCalls[1][2], {
      __WorthSignalCallbackCapture: true,
      value: "1280x720",
      reads: [viewportSourceId],
      hostCapabilityReads: [
        {
          family: "viewport",
          registrationId: "viewport",
          compatibility: "Reattachable",
        },
      ],
      runtimeReadBreadth: 0,
    });
    assert.deepEqual(computedCalls[2][2], {
      __WorthSignalCallbackCapture: true,
      value: "online",
      reads: [onlineSourceId],
      hostCapabilityReads: [
        {
          family: "online",
          registrationId: "online",
          compatibility: "Reattachable",
        },
      ],
      runtimeReadBreadth: 0,
    });
    assert.deepEqual(computedCalls[3][2], {
      __WorthSignalCallbackCapture: true,
      value: 1,
      reads: [clockSourceId],
      hostCapabilityReads: [
        {
          family: "clock",
          registrationId: "clock",
          compatibility: "SnapshotPortable",
        },
      ],
      runtimeReadBreadth: 0,
    });
    assert.deepEqual(computedCalls[4][2], {
      __WorthSignalCallbackCapture: true,
      value: 1,
      reads: [persistenceSourceId],
      hostCapabilityReads: [
        {
          family: "persistence",
          registrationId: "persistence",
          compatibility: "ImportDenied",
        },
      ],
      runtimeReadBreadth: 0,
    });

    state.viewport = { width: 1440, height: 900 };
    assert.equal(signals.host.viewport.width(), 1280);
    state.clockTick = 5;
    await sleep(15);
    await flushMicrotasks();
    assert.deepEqual(signals.host.viewport.size(), { width: 1280, height: 720 });
    state.persistedDraft = { mode: "draft", revision: 2 };
    const commitSummary = signals.host.persistence.commit();
    const noOpCommitSummary = signals.host.persistence.commit();
    assert.equal(typeof commitSummary?.touchedNodes, "number");
    assert.equal(noOpCommitSummary.touchedNodes, 0);

    const summary = signals.diagnostics().performanceSummary();
    const latestHostEvent = signals.diagnostics().latestHostCapabilityEvent();
    const hostReport = signals.diagnostics().hostCapabilityReport();
    assert.equal(summary.hostCapabilityPollCount > 0, true);
    assert.equal(summary.hostCapabilityInvalidationCount > 0, true);
    assert.equal(summary.hostCapabilityNoOpPollCount >= 0, true);
    assert.equal(summary.hostCapabilityManualCommitCount, 2);
    assert.equal(summary.hostCapabilityNoOpManualCommitCount, 1);
    assert.equal(summary.hostCapabilityUnavailabilityArtifactCount, 0);
    assert.equal(summary.hostCapabilityBroadFanoutDenialCount, 0);
    assert.equal(summary.hostCapabilityReadCount >= 6, true);
    assert.equal(latestHostEvent?.invalidationMode, "manually-committed");
    assert.equal(typeof hostReport.digest, "string");
    assert.equal(typeof hostReport.lineageDigest, "string");
    assert.equal(typeof hostReport.breadthDigest, "string");
    assert.equal(hostReport.totals.manualCommitCount, 2);
    assert.equal(hostReport.totals.unavailabilityArtifactCount, 0);
    assert.equal(
      hostReport.families.some((family) => family.family === "persistence"),
      true,
    );

    signals.free();
  } finally {
    await cleanup();
  }
});
