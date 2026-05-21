import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

test("wrapSignals adapters wrapper marks same-runtime exact restore while preserving portable host-capability denial artifacts", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawEnvelope = {
      definitions: {
        policy: { preset: "webDevelopment" },
        sources: [],
        recipes: [],
        sourceFamilies: [],
        recipeFamilies: [],
        unavailableCallbacks: [
          {
            id: "visibleLabel",
            signalKind: "computed",
            reason: "computeCallbackUnavailableForPortableExport",
            currentReads: ["count"],
            hostCapabilityReads: [
              {
                family: "visibility",
                registrationId: "visibility",
                compatibility: "LiveOnly",
              },
            ],
            hostCapabilityTransports: [
              {
                family: "visibility",
                registrationId: "visibility",
                compatibility: "LiveOnly",
                exactRestoreOutcome: "Live",
                portableImportOutcome: "Denied",
                portableImportReason:
                  "live-only host capabilities require the exact originating runtime",
              },
            ],
          },
        ],
      },
      snapshot: {
        snapshot: { meta: { branch_id: 0 } },
        state: { sources: [], recipes: [] },
      },
    };

    const calls = [];
    const rawSignals = {
      input(id, initial) {
        return createRawReadableHandle(id, initial);
      },
      computedSpec(id, spec) {
        return createRawReadableHandle(id, spec);
      },
      computedCallback(id) {
        return createRawReadableHandle(id, id);
      },
      outputSpec(id, spec) {
        return createRawReadableHandle(id, spec);
      },
      read(target) {
        return typeof target === "string" ? target : target.id;
      },
      watch() {
        return { free() {} };
      },
      effect() {
        return { free() {} };
      },
      transaction(callback) {
        callback({ set() {}, free() {} });
        return {};
      },
      batch(callback) {
        callback({ set() {}, free() {} });
        return {};
      },
      nuke() {
        return true;
      },
      diagnostics() {
        return {
          why() {
            return null;
          },
          health() {
            return null;
          },
          summaryNow() {
            return null;
          },
          historyNow() {
            return null;
          },
          latestObservation() {
            return null;
          },
          latestFlow() {
            return null;
          },
          performanceSummary() {
            return {};
          },
          latestFailure() {
            return null;
          },
          latestRollback() {
            return null;
          },
          latestFrontierExecution() {
            return null;
          },
          latestInvalidationTraceRecords() {
            return [];
          },
          recentHistory() {
            return [];
          },
          subscribe() {
            return { free() {} };
          },
        };
      },
      history() {
        return { free() {} };
      },
      specialist() {
        return {};
      },
      adapters() {
        return {
          export_definitions() {
            calls.push(["export_definitions"]);
            return structuredClone(rawEnvelope.definitions);
          },
          export_runtime_envelope() {
            calls.push(["export_runtime_envelope"]);
            return structuredClone(rawEnvelope);
          },
          export_runtime_envelope_wire() {
            calls.push(["export_runtime_envelope_wire"]);
            return JSON.stringify(rawEnvelope);
          },
          export_runtime_envelope_portable_wire() {
            calls.push(["export_runtime_envelope_portable_wire"]);
            return JSON.stringify({ portable: true, ...rawEnvelope });
          },
          replace_runtime_envelope(envelope) {
            calls.push([
              "replace_runtime_envelope",
              envelope?.definitions?.unavailableCallbacks?.length ?? 0,
            ]);
          },
          replace_runtime_envelope_wire(envelope) {
            calls.push([
              "replace_runtime_envelope_wire",
              JSON.parse(envelope).definitions.unavailableCallbacks.length,
            ]);
          },
          replace_runtime_envelope_portable_wire(envelope) {
            calls.push([
              "replace_runtime_envelope_portable_wire",
              JSON.parse(envelope).definitions.unavailableCallbacks.length,
            ]);
          },
          runtime_proof_report() {
            calls.push(["runtime_proof_report"]);
            return { kind: "proof" };
          },
          free() {},
        };
      },
      compatibilityApp() {
        return {};
      },
      compatibilityRuntime() {
        return {};
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const adapters = signals.adapters();
    const envelope = adapters.exportRuntimeEnvelope();
    const report = adapters.hostCapabilityTransportReport(envelope);
    const proof = adapters.runtimeProofReport();

    assert.equal(envelope.runtimeEnvelopeRestoreMode, "SameRuntimeExact");
    assert.equal(typeof envelope.runtimeEnvelopeRestoreToken, "string");
    assert.equal(typeof envelope.runtimeEnvelopePortableWire, "string");
    assert.equal(proof.kind, "proof");
    assert.deepEqual(report.totals, {
      unavailableArtifactCount: 1,
      transportEntryCount: 1,
      deniedFamilyCount: 1,
      unavailableFamilyCount: 0,
      snapshotPortableFamilyCount: 0,
    });
    assert.deepEqual(report.families, [
      {
        family: "visibility",
        callbackIds: ["visibleLabel"],
        compatibilities: ["LiveOnly"],
        exactRestoreOutcomes: ["Live"],
        portableImportOutcomes: ["Denied"],
        deniedCallbackIds: ["visibleLabel"],
        unavailableCallbackIds: [],
      },
    ]);

    adapters.restoreExactRuntimeEnvelope(envelope);
    assert.throws(
      () => adapters.replaceRuntimeEnvelope(envelope),
      (error) =>
        error?.code === "computeCallbackUnavailableForRuntimeEnvelopeImport" &&
        /visibleLabel/.test(error?.message ?? ""),
    );
    assert.throws(
      () =>
        adapters.restoreExactRuntimeEnvelope({
          definitions: rawEnvelope.definitions,
        }),
      /adapters\.restoreExactRuntimeEnvelope expects an artifact returned by adapters\.exportRuntimeEnvelope\(\)/,
    );

    assert.deepEqual(calls, [
      ["export_runtime_envelope"],
      ["export_runtime_envelope_wire"],
      ["export_runtime_envelope_portable_wire"],
      ["runtime_proof_report"],
      ["replace_runtime_envelope_wire", 1],
    ]);
  } finally {
    await cleanup();
  }
});
