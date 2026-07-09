import { normalizeWorkerRuntimeEnvelope } from "../../bridge/worker_runtime_envelope_normalization.js";
import {
  createWorkerFirstSnapshotArtifact,
  createWorkerFirstSnapshotEnvelopeArtifact,
} from "./worker_first_history_proofs.js";

export async function buildActiveImportContext(bridge, definition, snapshot) {
  const inputIds = definition.inputDescriptors.map((descriptor) => descriptor.sourceId);
  const outputIds = definition.descriptors.map((descriptor) => descriptor.publishedId);
  const outputSourceIds = definition.descriptors.map((descriptor) => descriptor.sourceId);
  const signalIds = [...new Set([...inputIds, ...outputIds, ...outputSourceIds])];
  const branchesPromise = bridge.branches();
  const [
    runtimeEnvelope,
    snapshotEnvelopeArtifact,
    runtimeEnvelopeRestoreToken,
    runtimeEnvelopePortableWire,
    diagnosticsSummaryPacket,
    diagnosticsHistoryPacket,
    signalReadbackPacket,
    versionSummaries,
    health,
    currentBranch,
    branches,
    latestFlow,
    latestObservation,
    performanceSummary,
    latestFailure,
    latestRollback,
    latestFrontierExecution,
    latestInvalidationTraceRecords,
    recentHistory,
    runtimeProofReport,
    whys,
    replays,
    lineages,
    replaysByBranch,
    branchSnapshotArtifacts,
    branchSnapshotEnvelopes,
    branchStateProofs,
  ] = await Promise.all([
    bridge.exportWorkerRuntimeEnvelope(),
    bridge.exportWorkerSnapshotEnvelopeArtifact(),
    bridge.exportWorkerRuntimeEnvelopeWire(),
    bridge.exportWorkerRuntimeEnvelopePortableWire(),
    bridge.readDiagnosticsSummary(),
    bridge.readDiagnosticsHistory(),
    bridge.readSignals({ signalIds }),
    bridge.readVersions(signalIds),
    bridge.health(),
    bridge.currentBranch(),
    branchesPromise,
    bridge.latestFlow(),
    bridge.latestObservation(),
    bridge.performanceSummary(),
    bridge.latestFailure(),
    bridge.latestRollback(),
    bridge.latestFrontierExecution(),
    bridge.latestInvalidationTraceRecords(),
    bridge.recentHistory(),
    bridge.runtimeProofReport(),
    Promise.all(signalIds.map(async (id) => [id, await bridge.why(id)])),
    Promise.all(signalIds.map(async (id) => [id, await bridge.replayFor(id)])),
    Promise.all(signalIds.map(async (id) => [id, await bridge.lineageFor(id)])),
    branchesPromise.then((workerBranches) => Promise.all(
      workerBranches.map(async (branch) => [branch.id, await bridge.replayForBranch(branch.id)]),
    )),
    branchesPromise.then((workerBranches) => Promise.all(
      workerBranches.map(async (branch) => [branch.id, await bridge.branchSnapshotArtifact(branch.id)]),
    )),
    branchesPromise.then((workerBranches) => Promise.all(
      workerBranches.map(async (branch) => [
        branch.id,
        await bridge.branchSnapshotEnvelopeArtifact(branch.id),
      ]),
    )),
    branchesPromise.then((workerBranches) => Promise.all(
      workerBranches.map(async (branch) => [branch.id, await bridge.branchStateProof(branch.id)]),
    )),
  ]);
  const normalizedRuntimeEnvelope = normalizeWorkerRuntimeEnvelope(runtimeEnvelope);
  return Object.freeze({
    definition,
    snapshot,
    inputDescriptorBySourceId: new Map(
      definition.inputDescriptors.map((descriptor) => [descriptor.sourceId, descriptor]),
    ),
    outputDescriptorBySourceId: new Map(
      definition.descriptors.map((descriptor) => [descriptor.sourceId, descriptor]),
    ),
    sourceDefinitionById: new Map(
      (normalizedRuntimeEnvelope.definitions.sources ?? []).map((source) => [source.id, source]),
    ),
    recipeDefinitionById: new Map(
      (normalizedRuntimeEnvelope.definitions.recipes ?? []).map((recipe) => [recipe.id, recipe]),
    ),
    runtimeDefinitionEnvelope: normalizedRuntimeEnvelope.definitions,
    runtimeEnvelopeArtifact: Object.freeze({
      ...normalizedRuntimeEnvelope,
      runtimeEnvelopeRestoreToken,
      runtimeEnvelopeRestoreMode: "SameRuntimeExact",
      runtimeEnvelopePortableWire,
    }),
    snapshotEnvelope: createWorkerFirstSnapshotEnvelopeArtifact(snapshotEnvelopeArtifact),
    diagnosticsSummary: diagnosticsSummaryPacket.summary,
    diagnosticsHistory: diagnosticsHistoryPacket.history,
    signalValueById: new Map(
      signalReadbackPacket.signals.map((signal) => [signal.id, signal.value]),
    ),
    publishedOutputIds: new Set(outputIds),
    versionById: new Map(versionSummaries.map((summary) => [summary.id, summary])),
    health,
    currentBranch,
    branches,
    latestFlow,
    latestObservation,
    performanceSummary,
    latestFailure,
    latestRollback,
    latestFrontierExecution,
    latestInvalidationTraceRecords,
    recentHistory,
    runtimeProofReport,
    whyById: new Map(whys),
    replayById: new Map(replays),
    lineageById: new Map(lineages),
    replayByBranchId: workerFirstBranchMap(replaysByBranch),
    branchSnapshotArtifactByBranchId: workerFirstBranchMap(
      branchSnapshotArtifacts.map(([branchId, artifact]) => [
        branchId,
        createWorkerFirstSnapshotArtifact(artifact),
      ]),
    ),
    branchSnapshotIdByBranchId: workerFirstBranchMap(
      branchSnapshotEnvelopes.map(([branchId, artifact]) => [
        branchId,
        artifact.snapshotEnvelope.snapshot.meta.snapshot_id,
      ]),
    ),
    branchSnapshotEnvelopeByBranchId: workerFirstBranchMap(
      branchSnapshotEnvelopes.map(([branchId, artifact]) => [
        branchId,
        createWorkerFirstSnapshotEnvelopeArtifact(artifact),
      ]),
    ),
    branchStateProofByBranchId: workerFirstBranchMap(branchStateProofs),
  });
}

function workerFirstBranchMap(entries) {
  return new Map(entries.map(([branchId, value]) => [BigInt(branchId), value]));
}
