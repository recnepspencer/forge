import { decodeSnapshotEnvelope, normalizeSnapshotEnvelope } from "../internal/codec.ts";
import { summarizeMergePlan, summarizeMergeResult } from "../internal/merge-reports.ts";

function normalizeBranchId(branchId: number | bigint) {
  return typeof branchId === "bigint" ? branchId : BigInt(branchId);
}

export class SignalHistory {
  inner: any;

  constructor(inner: any) {
    this.inner = inner;
  }

  snapshotOpaque() {
    return this.inner.snapshot();
  }

  restoreSnapshotOpaque(snapshot: unknown) {
    return this.inner.restore_snapshot(snapshot);
  }

  replayFor(id: string) {
    return this.inner.replay_for(id);
  }

  lineageFor(id: string) {
    return this.inner.lineage_for(id);
  }

  snapshot() {
    return decodeSnapshotEnvelope(this.inner.snapshot());
  }

  restoreSnapshot(snapshot: any) {
    return this.inner.restore_snapshot(normalizeSnapshotEnvelope(snapshot));
  }

  currentBranch() {
    return this.inner.current_branch();
  }

  branches() {
    return this.inner.branches();
  }

  createBranch(name: string) {
    return this.inner.create_branch(name);
  }

  switchBranch(branchId: number | bigint) {
    return this.inner.switch_branch(normalizeBranchId(branchId));
  }

  replayForBranch(branchId: number | bigint) {
    return this.inner.replay_for_branch(normalizeBranchId(branchId));
  }

  branchSnapshot(branchId: number | bigint) {
    return this.inner.branch_snapshot(normalizeBranchId(branchId));
  }

  branchSnapshotId(branchId: number | bigint) {
    return this.inner.branch_snapshot_id(normalizeBranchId(branchId));
  }

  branchSnapshotEnvelope(branchId: number | bigint) {
    return decodeSnapshotEnvelope(this.inner.branch_snapshot_envelope(normalizeBranchId(branchId)));
  }

  branchSnapshotEnvelopeOpaque(branchId: number | bigint) {
    return this.inner.branch_snapshot_envelope(normalizeBranchId(branchId));
  }

  restoreBranchSnapshotOpaque(branchId: number | bigint, snapshot: unknown) {
    return this.inner.restore_branch_snapshot(normalizeBranchId(branchId), snapshot);
  }

  restoreBranchSnapshotById(branchId: number | bigint, snapshotId: number | bigint) {
    return this.inner.restore_branch_snapshot_by_id(
      normalizeBranchId(branchId),
      normalizeBranchId(snapshotId),
    );
  }

  planMergeBranches(sourceBranchId: number | bigint, targetBranchId: number | bigint) {
    return summarizeMergePlan(this.planMergeBranchesDetailed(sourceBranchId, targetBranchId));
  }

  planMergeBranchesDetailed(sourceBranchId: number | bigint, targetBranchId: number | bigint) {
    return this.inner.plan_merge_branches(
      normalizeBranchId(sourceBranchId),
      normalizeBranchId(targetBranchId),
    );
  }

  planMergeBranchesDetailedWithProof(sourceBranchId: number | bigint, targetBranchId: number | bigint) {
    return this.inner.plan_merge_branches_with_proof(
      normalizeBranchId(sourceBranchId),
      normalizeBranchId(targetBranchId),
    );
  }

  mergeBranches(sourceBranchId: number | bigint, targetBranchId: number | bigint) {
    return summarizeMergeResult(this.mergeBranchesDetailed(sourceBranchId, targetBranchId));
  }

  mergeBranchesDetailed(sourceBranchId: number | bigint, targetBranchId: number | bigint) {
    return this.inner.merge_branches(
      normalizeBranchId(sourceBranchId),
      normalizeBranchId(targetBranchId),
    );
  }

  mergeBranchesDetailedWithProof(sourceBranchId: number | bigint, targetBranchId: number | bigint) {
    return this.inner.merge_branches_with_proof(
      normalizeBranchId(sourceBranchId),
      normalizeBranchId(targetBranchId),
    );
  }

  branchStateProof(branchId: number | bigint) {
    return this.inner.branch_state_proof(normalizeBranchId(branchId));
  }

  replayParityProof(expectedBranchId: number | bigint, replayedBranchId: number | bigint) {
    return this.inner.replay_parity_proof(
      normalizeBranchId(expectedBranchId),
      normalizeBranchId(replayedBranchId),
    );
  }

  replayArtifactProof(expected: unknown, replayedBranchId: number | bigint) {
    return this.inner.replay_artifact_proof(
      expected,
      normalizeBranchId(replayedBranchId),
    );
  }
}
