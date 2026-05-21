import { freezeObject } from "../graph_support.js";
import {
  createReplayArtifactProofReport,
  createReplayParityProofReport,
  normalizeWorkerFirstBranchId,
} from "./sessions/support/worker_first_history_proofs.js";

export function createRootHistoryFacade(rootSession) {
  return freezeObject({
    replay_for(id) {
      const context = rootSession.currentImportContext();
      if (!context.replayById.has(id)) {
        throw new TypeError(
          `worker-first root history().replay_for(${JSON.stringify(id)}) requires an id from the active imported graph`,
        );
      }
      return context.replayById.get(id);
    },
    lineage_for(id) {
      const context = rootSession.currentImportContext();
      if (!context.lineageById.has(id)) {
        throw new TypeError(
          `worker-first root history().lineage_for(${JSON.stringify(id)}) requires an id from the active imported graph`,
        );
      }
      return context.lineageById.get(id);
    },
    recentHistory() {
      return rootSession.currentImportContext().recentHistory;
    },
    snapshot() {
      return rootSession.currentImportContext().snapshotEnvelope;
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
      return rootSession.currentImportContext().currentBranch;
    },
    branches() {
      return rootSession.currentImportContext().branches;
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
      return requireWorkerFirstBranchValue(
        rootSession,
        branchId,
        "history.replay_for_branch",
        "replayByBranchId",
      );
    },
    branch_snapshot(branchId) {
      return requireWorkerFirstBranchValue(
        rootSession,
        branchId,
        "history.branch_snapshot",
        "branchSnapshotArtifactByBranchId",
      );
    },
    branch_snapshot_id(branchId) {
      return requireWorkerFirstBranchValue(
        rootSession,
        branchId,
        "history.branch_snapshot_id",
        "branchSnapshotIdByBranchId",
      );
    },
    branch_snapshot_envelope(branchId) {
      return requireWorkerFirstBranchValue(
        rootSession,
        branchId,
        "history.branch_snapshot_envelope",
        "branchSnapshotEnvelopeByBranchId",
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
      return requireWorkerFirstBranchValue(
        rootSession,
        branchId,
        "history.branch_state_proof",
        "branchStateProofByBranchId",
      );
    },
    replay_parity_proof(expectedBranchId, replayedBranchId) {
      const context = rootSession.currentImportContext();
      return createReplayParityProofReport(
        context.runtimeProofReport.proofSchemaVersion,
        requireWorkerFirstBranchValue(
          rootSession,
          expectedBranchId,
          "history.replay_parity_proof",
          "branchStateProofByBranchId",
        ),
        requireWorkerFirstBranchValue(
          rootSession,
          replayedBranchId,
          "history.replay_parity_proof",
          "branchStateProofByBranchId",
        ),
      );
    },
    replay_artifact_proof(expected, replayedBranchId) {
      const context = rootSession.currentImportContext();
      return createReplayArtifactProofReport(
        context.runtimeProofReport.proofSchemaVersion,
        expected,
        freezeObject({
          proofSchemaVersion: context.runtimeProofReport.proofSchemaVersion,
          registryBundleDigest: context.runtimeProofReport.registryBundleDigest,
          loweredStrategyBundleDigest: null,
          mergePlanDigest: null,
          mergeResultDigest: null,
          lineageDigest: null,
          branchStateDigest: requireWorkerFirstBranchValue(
            rootSession,
            replayedBranchId,
            "history.replay_artifact_proof",
            "branchStateProofByBranchId",
          ).stateDigest,
        }),
      );
    },
    free() {},
    [Symbol.dispose]() {},
  });
}

function requireWorkerFirstBranchValue(rootSession, branchId, operation, mapField) {
  const context = rootSession.currentImportContext();
  const branchMap = context[mapField];
  const normalizedBranchId = normalizeWorkerFirstBranchId(branchId, operation);
  if (!branchMap?.has(normalizedBranchId)) {
    throw new TypeError(
      `${operation}(${String(branchId)}) requires a branch from the active worker-first history context`,
    );
  }
  return branchMap.get(normalizedBranchId);
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
