import { freezeObject } from "../graph_support.js";
import { buildImportedGraphSnapshotArtifact } from "../imported_graph_surface_support.js";
import { normalizeWorkerRuntimeEnvelope } from "./bridge/worker_runtime_envelope_normalization.js";

export async function exportWorkerFirstPublishedGraphSnapshot(session) {
  const [runtimeEnvelope, runtimeEnvelopeRestoreToken, runtimeEnvelopePortableWire, snapshotEnvelope] =
    await Promise.all([
      session.bridge.exportWorkerRuntimeEnvelope(),
      session.bridge.exportWorkerRuntimeEnvelopeWire(),
      session.bridge.exportWorkerRuntimeEnvelopePortableWire(),
      session.bridge.exportWorkerSnapshotEnvelope(),
    ]);
  return buildImportedGraphSnapshotArtifact({
    definition: session.exportDefinition(),
    runtimeEnvelope: freezeObject({
      ...normalizeWorkerRuntimeEnvelope(runtimeEnvelope),
      runtimeEnvelopeRestoreToken,
      runtimeEnvelopeRestoreMode: "SameRuntimeExact",
      runtimeEnvelopePortableWire,
    }),
    snapshotEnvelope,
    restoreMode: "SameRuntimeExact",
    contractHistory: session.definition.contractHistory,
    importPosture: session.definition.importPosture,
  });
}

export function createWorkerFirstPublishedGraphSpecialistFacade(session) {
  return freezeObject({
    evaluateDirty() {
      return session.bridge.evaluateDirty();
    },
    evaluate_dirty() {
      return session.bridge.evaluateDirty();
    },
    graphSummary() {
      return session.diagnosticsSummary();
    },
    graph_summary() {
      return session.diagnosticsSummary();
    },
    readVersions(ids) {
      return session.bridge.readVersions(ids);
    },
    read_versions(ids) {
      return session.bridge.readVersions(ids);
    },
    free() {},
    [Symbol.dispose]() {},
  });
}

export function throwWorkerFirstPublishedGraphUnavailable(graphId, operation) {
  const error = new Error(
    `worker-first published graph ${graphId} ${operation}() is unavailable because no compatibility-sidecar runtime exists; use deployment: "mainThreadCompatibility" instead`,
  );
  error.name = "WorkerFirstPublishedGraphUnavailable";
  error.code = "workerFirstPublishedGraphUnavailable";
  error.compatibilityRecovery = freezeObject({
    deployment: "mainThreadCompatibility",
    message:
      'Retry with deployment: "mainThreadCompatibility" to use compatibility app/runtime doors.',
  });
  throw error;
}
