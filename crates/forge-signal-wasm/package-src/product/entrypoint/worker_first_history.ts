import { freezeObject } from "../graph_support.js";
import {
  createReplayArtifactProofReport,
  createReplayParityProofReport,
  createWorkerFirstSnapshotArtifact,
  createWorkerFirstSnapshotEnvelopeArtifact,
} from "./sessions/support/worker_first_history_proofs.js";

export function createWorkerFirstHistoryFacade(session) {
  const branchSnapshotArtifacts = new Map();
  const branchSnapshotEnvelopeArtifacts = new Map();
  let runtimeProofReportPromise = null;

  return Object.freeze({
    replay_for(id) {
      return session.bridge.replayFor(id);
    },
    lineage_for(id) {
      return session.bridge.lineageFor(id);
    },
    recentHistory() {
      return session.bridge.recentHistory();
    },
    async snapshot() {
      return createWorkerFirstSnapshotEnvelopeArtifact(
        await session.bridge.exportWorkerSnapshotEnvelopeArtifact(),
      );
    },
    restore_snapshot(snapshot) {
      void snapshot;
      throwWorkerFirstHistoryUnavailable("history.restore_snapshot");
    },
    restore_exact_snapshot(snapshot) {
      void snapshot;
      throwWorkerFirstHistoryUnavailable("history.restore_exact_snapshot");
    },
    current_branch() {
      return session.bridge.currentBranch();
    },
    branches() {
      return session.bridge.branches();
    },
    create_branch(name) {
      void name;
      throwWorkerFirstHistoryUnavailable("history.create_branch");
    },
    switch_branch(branchId) {
      void branchId;
      throwWorkerFirstHistoryUnavailable("history.switch_branch");
    },
    replay_for_branch(branchId) {
      return session.bridge.replayForBranch(branchId);
    },
    branch_snapshot(branchId) {
      return requireWorkerFirstBranchSnapshotArtifact(
        session,
        branchSnapshotArtifacts,
        branchId,
      ).then((artifact) => createWorkerFirstSnapshotArtifact(artifact));
    },
    branch_snapshot_id(branchId) {
      return requireWorkerFirstBranchSnapshotArtifact(
        session,
        branchSnapshotArtifacts,
        branchId,
      ).then((artifact) => artifact.snapshot.meta.snapshot_id);
    },
    async branch_snapshot_envelope(branchId) {
      return createWorkerFirstSnapshotEnvelopeArtifact(
        await requireWorkerFirstBranchSnapshotEnvelopeArtifact(
          session,
          branchSnapshotEnvelopeArtifacts,
          branchId,
        ),
      );
    },
    restore_branch_snapshot(branchId, snapshot) {
      void branchId;
      void snapshot;
      throwWorkerFirstHistoryUnavailable("history.restore_branch_snapshot");
    },
    restore_exact_branch_snapshot(branchId, snapshot) {
      void branchId;
      void snapshot;
      throwWorkerFirstHistoryUnavailable("history.restore_exact_branch_snapshot");
    },
    restore_branch_snapshot_by_id(branchId, snapshotId) {
      void branchId;
      void snapshotId;
      throwWorkerFirstHistoryUnavailable("history.restore_branch_snapshot_by_id");
    },
    merge_branches(sourceBranchId, targetBranchId) {
      void sourceBranchId;
      void targetBranchId;
      throwWorkerFirstHistoryUnavailable("history.merge_branches");
    },
    merge_branches_with_proof(sourceBranchId, targetBranchId) {
      void sourceBranchId;
      void targetBranchId;
      throwWorkerFirstHistoryUnavailable("history.merge_branches_with_proof");
    },
    plan_merge_branches(sourceBranchId, targetBranchId) {
      void sourceBranchId;
      void targetBranchId;
      throwWorkerFirstHistoryUnavailable("history.plan_merge_branches");
    },
    plan_merge_branches_with_proof(sourceBranchId, targetBranchId) {
      void sourceBranchId;
      void targetBranchId;
      throwWorkerFirstHistoryUnavailable("history.plan_merge_branches_with_proof");
    },
    plan_merge_policy_preview(request) {
      void request;
      throwWorkerFirstHistoryUnavailable("history.plan_merge_policy_preview");
    },
    plan_merge_policy_preview_with_proof(request) {
      void request;
      throwWorkerFirstHistoryUnavailable("history.plan_merge_policy_preview_with_proof");
    },
    merge_branches_policy_preview(request) {
      void request;
      throwWorkerFirstHistoryUnavailable("history.merge_branches_policy_preview");
    },
    merge_branches_policy_preview_with_proof(request) {
      void request;
      throwWorkerFirstHistoryUnavailable("history.merge_branches_policy_preview_with_proof");
    },
    branch_state_proof(branchId) {
      return session.bridge.branchStateProof(branchId);
    },
    replay_parity_proof(expectedBranchId, replayedBranchId) {
      return Promise.all([
        runtimeProofReport(),
        session.bridge.branchStateProof(expectedBranchId),
        session.bridge.branchStateProof(replayedBranchId),
      ]).then(([runtimeProofReportValue, expected, replayed]) => createReplayParityProofReport(
        runtimeProofReportValue.proofSchemaVersion,
        expected,
        replayed,
      ));
    },
    replay_artifact_proof(expected, replayedBranchId) {
      return Promise.all([
        runtimeProofReport(),
        session.bridge.branchStateProof(replayedBranchId),
      ]).then(([runtimeProofReportValue, replayedState]) => createReplayArtifactProofReport(
        runtimeProofReportValue.proofSchemaVersion,
        expected,
        freezeObject({
          proofSchemaVersion: runtimeProofReportValue.proofSchemaVersion,
          registryBundleDigest: runtimeProofReportValue.registryBundleDigest,
          loweredStrategyBundleDigest: null,
          mergePlanDigest: null,
          mergeResultDigest: null,
          lineageDigest: null,
          branchStateDigest: replayedState.stateDigest,
        }),
      ));
    },
    free() {},
    [Symbol.dispose]() {},
  });

  function runtimeProofReport() {
    runtimeProofReportPromise ??= session.bridge.runtimeProofReport();
    return runtimeProofReportPromise;
  }
}

function requireWorkerFirstBranchSnapshotArtifact(session, branchSnapshotArtifacts, branchId) {
  const branchKey = typeof branchId === "bigint" ? branchId : BigInt(branchId);
  let artifactPromise = branchSnapshotArtifacts.get(branchKey);
  if (!artifactPromise) {
    artifactPromise = session.bridge.branchSnapshotArtifact(branchId);
    branchSnapshotArtifacts.set(branchKey, artifactPromise);
  }
  return artifactPromise;
}

function requireWorkerFirstBranchSnapshotEnvelopeArtifact(
  session,
  branchSnapshotArtifacts,
  branchId,
) {
  const branchKey = typeof branchId === "bigint" ? branchId : BigInt(branchId);
  let artifactPromise = branchSnapshotArtifacts.get(branchKey);
  if (!artifactPromise) {
    artifactPromise = session.bridge.branchSnapshotEnvelopeArtifact(branchId);
    branchSnapshotArtifacts.set(branchKey, artifactPromise);
  }
  return artifactPromise;
}

function throwWorkerFirstHistoryUnavailable(operation) {
  const error = new Error(
    `${operation} is unavailable on the current worker-first history facade; use deployment: "mainThreadCompatibility" for branch and snapshot history operations`,
  );
  error.name = "WorkerFirstHistoryUnavailable";
  error.code = "workerFirstHistoryUnavailable";
  error.compatibilityRecovery = Object.freeze({
    deployment: "mainThreadCompatibility",
    message:
      'Retry with deployment: "mainThreadCompatibility" to use full branch and snapshot history operations.',
  });
  throw error;
}
