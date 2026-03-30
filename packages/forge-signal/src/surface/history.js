import { decodeSnapshotEnvelope, encodeSignalValue } from "../internal/codec.js";
import { summarizeMergePlan, summarizeMergeResult } from "../internal/merge-reports.js";

function normalizeBranchId(branchId) {
  return typeof branchId === "bigint" ? branchId : BigInt(branchId);
}

function normalizeSnapshot(snapshot) {
  return {
    ...snapshot,
    state: {
      ...snapshot.state,
      sources: snapshot.state.sources.map((source) => ({
        ...source,
        value: encodeSignalValue(source.value)
      })),
      recipes: snapshot.state.recipes.map((recipe) => ({
        ...recipe,
        value: encodeSignalValue(recipe.value)
      }))
    }
  };
}

export class SignalHistory {
  constructor(inner) {
    this.inner = inner;
  }

  replayFor(id) {
    return this.inner.replay_for(id);
  }

  lineageFor(id) {
    return this.inner.lineage_for(id);
  }

  snapshot() {
    return decodeSnapshotEnvelope(this.inner.snapshot());
  }

  restoreSnapshot(snapshot) {
    return this.inner.restore_snapshot(normalizeSnapshot(snapshot));
  }

  currentBranch() {
    return this.inner.current_branch();
  }

  branches() {
    return this.inner.branches();
  }

  createBranch(name) {
    return this.inner.create_branch(name);
  }

  switchBranch(branchId) {
    return this.inner.switch_branch(normalizeBranchId(branchId));
  }

  replayForBranch(branchId) {
    return this.inner.replay_for_branch(normalizeBranchId(branchId));
  }

  branchSnapshot(branchId) {
    return this.inner.branch_snapshot(normalizeBranchId(branchId));
  }

  planMergeBranches(sourceBranchId, targetBranchId) {
    return summarizeMergePlan(this.planMergeBranchesDetailed(sourceBranchId, targetBranchId));
  }

  planMergeBranchesDetailed(sourceBranchId, targetBranchId) {
    return this.inner.plan_merge_branches(
      normalizeBranchId(sourceBranchId),
      normalizeBranchId(targetBranchId)
    );
  }

  mergeBranches(sourceBranchId, targetBranchId) {
    return summarizeMergeResult(this.mergeBranchesDetailed(sourceBranchId, targetBranchId));
  }

  mergeBranchesDetailed(sourceBranchId, targetBranchId) {
    return this.inner.merge_branches(
      normalizeBranchId(sourceBranchId),
      normalizeBranchId(targetBranchId)
    );
  }
}
