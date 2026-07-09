import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "./module_loading/load_signals_module.mjs";
import { createReactiveRawSignals } from "./runtime_fixture/reactive_raw_signals.mjs";
import { digestValue } from "./runtime_fixture/digest_value.mjs";
import { flushMicrotasks, sleep } from "./runtime_fixture/scheduling.mjs";

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

    const visibleLabel = signals.spec.computedCallback(
      "visibleLabel",
      () => signals.host.visibility.state(),
    );
    const viewportLabel = signals.spec.computedCallback(
      "viewportLabel",
      () => `${signals.host.viewport.width()}x${signals.host.viewport.height()}`,
    );
    const onlineLabel = signals.spec.computedCallback(
      "onlineLabel",
      () => (signals.host.online.isOnline() ? "online" : "offline"),
    );
    const clockLabel = signals.spec.computedCallback(
      "clockLabel",
      () => signals.host.clock.now(),
    );
    const persistenceLabel = signals.spec.computedCallback(
      "persistenceLabel",
      () => signals.host.persistence.value().revision,
    );

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
