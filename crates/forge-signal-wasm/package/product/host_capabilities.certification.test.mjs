import assert from "node:assert/strict";
import test from "node:test";

import {
  createReactiveRawSignals,
  digestValue,
  flushMicrotasks,
  loadSignalsModule,
  loadStoreModule,
  sleep,
} from "./host_capability_certification_helpers.mjs";

test("host capability certification keeps ambient reads non-reactive and bounds invalidation to the affected frontier", async () => {
  const {
    hostCapabilityPlan,
    viewportCapability,
    visibilityCapability,
    wrapSignals,
    cleanup,
  } = await loadSignalsModule();
  try {
    const runtime = createReactiveRawSignals();
    let ambientBreakpoint = "wide";
    let visibilityState = "visible";
    let viewportState = { width: 1280, height: 720 };
    let visibilityListener = null;
    let viewportListener = null;

    const signals = wrapSignals(runtime.rawSignals, {
      hostCapabilities: hostCapabilityPlan({
        visibility: visibilityCapability({
          source: {
            current() {
              return visibilityState;
            },
            subscribe(next) {
              visibilityListener = next;
              return () => {
                visibilityListener = null;
              };
            },
          },
          compatibility: "LiveOnly",
        }),
        viewport: viewportCapability({
          source: {
            current() {
              return viewportState;
            },
            subscribe(next) {
              viewportListener = next;
              return () => {
                viewportListener = null;
              };
            },
          },
        }),
      }),
    });

    const count = signals.input(1, { id: "count" });
    const ambientMixed = signals.computed(
      () => `${ambientBreakpoint}:${signals.host.visibility.state()}`,
      { id: "ambientMixed" },
    );
    const visibilityOnly = signals.computed(
      () => (signals.host.visibility.isVisible() ? "onscreen" : "hidden"),
      { id: "visibilityOnly" },
    );
    const viewportOnly = signals.computed(
      () => `${signals.host.viewport.width()}x${signals.host.viewport.height()}`,
      { id: "viewportOnly" },
    );
    const signalOnly = signals.computed(() => count() * 2, { id: "signalOnly" });

    assert.equal(signals.read(ambientMixed), "wide:visible");
    assert.equal(signals.read(visibilityOnly), "onscreen");
    assert.equal(signals.read(viewportOnly), "1280x720");
    assert.equal(signals.read(signalOnly), 2);

    ambientBreakpoint = "narrow";
    await flushMicrotasks();

    assert.equal(
      signals.read(ambientMixed),
      "wide:visible",
      "ambient-only closure changes must not invalidate callback-authored host-capability nodes",
    );
    assert.equal(
      signals.diagnostics().performanceSummary().hostCapabilityInvalidationCount,
      0,
      "ambient closure churn must not charge host invalidation counters",
    );

    visibilityState = "hidden";
    visibilityListener();
    await flushMicrotasks();

    const flow = signals.diagnostics().latestFlow();
    const report = signals.diagnostics().hostCapabilityReport();
    const visibilitySourceId = runtime.calls.find(
      (call) => call[0] === "input" && String(call[1]).startsWith("__forgeSignal.host.visibility."),
    )?.[1];

    assert.equal(
      signals.read(ambientMixed),
      "narrow:hidden",
      "ambient closure values may affect the next declared-capability-driven recomputation without becoming reactive themselves",
    );
    assert.equal(signals.read(visibilityOnly), "hidden");
    assert.equal(signals.read(viewportOnly), "1280x720");
    assert.equal(signals.read(signalOnly), 2);
    assert.equal(report.breadth.maxTouchedNodes, 2);
    assert.equal(report.breadth.maxReevaluatedNodes, 2);
    assert.equal(report.families.find((family) => family.family === "visibility")?.maxTouchedNodes, 2);
    assert.equal(typeof report.lineageDigest, "string");
    assert.equal(typeof report.breadthDigest, "string");
    assert.deepEqual(
      flow.callbackNodes.find((node) => node.id === "ambientMixed")?.hostCapabilityReads,
      [{ family: "visibility", registrationId: "visibility", compatibility: "LiveOnly" }],
    );
    assert.deepEqual(
      flow.callbackNodes.find((node) => node.id === "ambientMixed")?.currentReads,
      [visibilitySourceId],
      "ambient closure state must not appear as a dependency edge",
    );
    signals.free();
  } finally {
    await cleanup();
  }
});

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

    const label = signals.computed(
      () => (signals.host.visibility.isVisible() ? "visible" : "hidden"),
      { id: "label" },
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

test("host capability certification preserves transport honesty, mixed-family attribution, and long-session report integrity", async () => {
  const {
    clockCapability,
    hostCapabilityPlan,
    onlineCapability,
    persistenceCapability,
    viewportCapability,
    visibilityCapability,
    wrapSignals,
    cleanup,
  } = await loadSignalsModule();
  try {
    let visibilityState = "visible";
    let viewportState = { width: 1280, height: 720 };
    let onlineState = "online";
    let clockTick = 0;
    let persistedDraft = { mode: "draft", revision: 1 };
    let visibilityListener = null;
    let viewportListener = null;
    let onlineListener = null;
    const adapterCalls = [];

    const transportEnvelope = {
      definitions: {
        unavailableCallbacks: [
          {
            id: "visibleLabel",
            hostCapabilityReads: [{ family: "visibility", registrationId: "visibility", compatibility: "LiveOnly" }],
            hostCapabilityTransports: [{ family: "visibility", registrationId: "visibility", compatibility: "LiveOnly", exactRestoreOutcome: "Live", portableImportOutcome: "Denied", portableImportReason: "visibility cannot cross runtimes" }],
          },
          {
            id: "viewportLabel",
            hostCapabilityReads: [{ family: "viewport", registrationId: "viewport", compatibility: "Reattachable" }],
            hostCapabilityTransports: [{ family: "viewport", registrationId: "viewport", compatibility: "Reattachable", exactRestoreOutcome: "Reattached", portableImportOutcome: "Unavailable", portableImportReason: "viewport requires reattachment" }],
          },
          {
            id: "onlineLabel",
            hostCapabilityReads: [{ family: "online", registrationId: "online", compatibility: "Reattachable" }],
            hostCapabilityTransports: [{ family: "online", registrationId: "online", compatibility: "Reattachable", exactRestoreOutcome: "Reattached", portableImportOutcome: "Unavailable", portableImportReason: "online requires reattachment" }],
          },
          {
            id: "clockLabel",
            hostCapabilityReads: [{ family: "clock", registrationId: "clock", compatibility: "SnapshotPortable" }],
            hostCapabilityTransports: [{ family: "clock", registrationId: "clock", compatibility: "SnapshotPortable", exactRestoreOutcome: "Unavailable", portableImportOutcome: "Unavailable", portableImportReason: "clock snapshots stay portable but not live" }],
          },
          {
            id: "persistenceLabel",
            hostCapabilityReads: [{ family: "persistence", registrationId: "persistence", compatibility: "ImportDenied" }],
            hostCapabilityTransports: [{ family: "persistence", registrationId: "persistence", compatibility: "ImportDenied", exactRestoreOutcome: "Live", portableImportOutcome: "Denied", portableImportReason: "persistence is local-only" }],
          },
        ],
      },
      snapshot: {
        snapshot: {
          meta: {
            branch_id: 7,
            runtime_policy: { tier: "Development" },
            replay_head: null,
            artifact_retention: { explanation_retention: "Rich" },
          },
        },
      },
    };

    const adaptersFactory = () => {
      return {
        export_runtime_envelope() {
          return structuredClone(transportEnvelope);
        },
        export_runtime_envelope_wire() {
          return "restore-token";
        },
        export_runtime_envelope_portable_wire() {
          return JSON.stringify({ restore: "portable-wire" });
        },
        replace_runtime_envelope_wire(token) {
          adapterCalls.push(["replace_runtime_envelope_wire", token]);
        },
        replace_runtime_envelope_portable_wire(payload) {
          adapterCalls.push(["replace_runtime_envelope_portable_wire", payload]);
        },
        replace_runtime_envelope(payload) {
          adapterCalls.push(["replace_runtime_envelope", payload]);
        },
        runtime_proof_report() {
          return { proofSchemaVersion: 1 };
        },
        free() {},
      };
    };

    const runtime = createReactiveRawSignals({
      adaptersFactory,
    });

    const signals = wrapSignals(runtime.rawSignals, {
      hostCapabilities: hostCapabilityPlan({
        visibility: visibilityCapability({
          source: {
            current() {
              return visibilityState;
            },
            subscribe(next) {
              visibilityListener = next;
              return () => {
                visibilityListener = null;
              };
            },
          },
          compatibility: "LiveOnly",
        }),
        viewport: viewportCapability({
          source: {
            current() {
              return viewportState;
            },
            subscribe(next) {
              viewportListener = next;
              return () => {
                viewportListener = null;
              };
            },
          },
        }),
        online: onlineCapability({
          source: {
            current() {
              return onlineState;
            },
            subscribe(next) {
              onlineListener = next;
              return () => {
                onlineListener = null;
              };
            },
          },
        }),
        clock: clockCapability({
          source: {
            current() {
              return clockTick;
            },
          },
          pollMs: 5,
        }),
        persistence: persistenceCapability({
          source: {
            current() {
              return persistedDraft;
            },
          },
        }),
      }),
    });

    const visibleLabel = signals.computed(() => signals.host.visibility.state(), { id: "visibleLabel" });
    const viewportLabel = signals.computed(() => `${signals.host.viewport.width()}x${signals.host.viewport.height()}`, { id: "viewportLabel" });
    const onlineLabel = signals.computed(() => (signals.host.online.isOnline() ? "online" : "offline"), { id: "onlineLabel" });
    const clockLabel = signals.computed(() => signals.host.clock.now(), { id: "clockLabel" });
    const persistenceLabel = signals.computed(() => signals.host.persistence.value().revision, { id: "persistenceLabel" });

    assert.equal(signals.read(visibleLabel), "visible");
    assert.equal(signals.read(viewportLabel), "1280x720");
    assert.equal(signals.read(onlineLabel), "online");
    assert.equal(signals.read(clockLabel), 0);
    assert.equal(signals.read(persistenceLabel), 1);

    for (let index = 0; index < 12; index += 1) {
      visibilityState = index % 2 === 0 ? "hidden" : "visible";
      visibilityListener();
      viewportState = { width: 1280 + index, height: 720 + index };
      viewportListener();
      onlineState = index % 2 === 0 ? "offline" : "online";
      onlineListener();
      clockTick = index + 1;
      persistedDraft = { mode: "draft", revision: index + 2 };
      signals.host.persistence.commit();
      await sleep(6);
      await flushMicrotasks();
    }

    const diagnostics = signals.diagnostics();
    const exporterReport = diagnostics.hostCapabilityReport();
    const exporterReportAgain = diagnostics.hostCapabilityReport();
    const recentEvents = diagnostics.recentHostCapabilityEvents();
    const adapters = signals.adapters();
    const runtimeEnvelope = adapters.exportRuntimeEnvelope();
    const secondRuntimeEnvelope = adapters.exportRuntimeEnvelope();
    const transportReport = adapters.hostCapabilityTransportReport(runtimeEnvelope);
    const implicitTransportReport = adapters.hostCapabilityTransportReport();
    const restoredExact = wrapSignals(
      createReactiveRawSignals({
        adaptersFactory,
      }).rawSignals,
    );
    restoredExact.adapters().restoreExactRuntimeEnvelope(runtimeEnvelope);

    const portableImport = wrapSignals(createReactiveRawSignals().rawSignals);
    let portableImportError = null;
    try {
      portableImport.adapters().replaceRuntimeEnvelope(runtimeEnvelope);
    } catch (error) {
      portableImportError = error;
    }
    const importerReport = portableImport.diagnostics().hostCapabilityReport();

    const matrixSummary = {
      exporterDigest: exporterReport.digest,
      exporterLineageDigest: exporterReport.lineageDigest,
      exporterBreadthDigest: exporterReport.breadthDigest,
      transportDigest: transportReport.digest,
      importerDigest: importerReport.digest,
      eventCount: recentEvents.length,
      deniedFamilies: transportReport.totals.deniedFamilyCount,
      unavailableFamilies: transportReport.totals.unavailableFamilyCount,
    };

    assert.equal(exporterReport.digest, exporterReportAgain.digest);
    assert.equal(exporterReport.lineageDigest, exporterReportAgain.lineageDigest);
    assert.equal(exporterReport.breadthDigest, exporterReportAgain.breadthDigest);
    assert.equal(recentEvents.length, 32, "long-session diagnostics should retain only the bounded recent event window");
    assert.equal(exporterReport.totals.retainedEventCount, 32);
    assert.equal(exporterReport.families.length >= 5, true);
    assert.equal(exporterReport.breadth.maxTouchedNodes >= 1, true);
    assert.equal(exporterReport.breadth.maxReevaluatedNodes >= 1, true);
    assert.equal(transportReport.digest, implicitTransportReport.digest);
    assert.equal(transportReport.totals.unavailableArtifactCount, 5);
    assert.equal(transportReport.totals.deniedFamilyCount, 2);
    assert.equal(transportReport.totals.unavailableFamilyCount, 3);
    assert.equal(runtimeEnvelope.runtimeEnvelopeRestoreToken, "restore-token");
    assert.equal(secondRuntimeEnvelope.runtimeEnvelopeRestoreToken, "restore-token");
    assert.equal(adapterCalls.some((call) => call[0] === "replace_runtime_envelope_wire" && call[1] === "restore-token"), true);
    assert.equal(portableImportError?.code, "computeCallbackUnavailableForRuntimeEnvelopeImport");
    assert.equal(
      importerReport.totals.compatibilityDenialCount,
      2,
      "import-side diagnostics must stay scoped to families that truly deny portable import",
    );
    assert.equal(importerReport.families.every((family) => family.family === "visibility" || family.family === "persistence"), true);
    assert.match(digestValue(matrixSummary), /^f1a-/);
    restoredExact.free();
    portableImport.free();
    signals.free();
  } finally {
    await cleanup();
  }
});
