import assert from "node:assert/strict";

function createHistoryStub(options = {}) {
  const calls = [];
  const originBranchId = options.originBranchId ?? 7;
  const speculativeBranchId = options.speculativeBranchId ?? 8;
  const branches = new Map([
    [originBranchId, {
      id: originBranchId,
      name: "main",
      parent_branch_id: null,
      head_snapshot_id: 77,
    }],
  ]);
  let currentBranchId = originBranchId;
  return {
    calls,
    current_branch() {
      calls.push(["current_branch"]);
      return branches.get(currentBranchId);
    },
    create_branch(name) {
      calls.push(["create_branch", name]);
      const branch = {
        id: speculativeBranchId,
        name,
        parent_branch_id: currentBranchId,
        head_snapshot_id: 78,
      };
      branches.set(branch.id, branch);
      return branch;
    },
    switch_branch(branchId) {
      calls.push(["switch_branch", branchId]);
      assert.ok(branches.has(branchId), `unknown branch id: ${branchId}`);
      currentBranchId = branchId;
    },
    plan_merge_policy_preview_with_proof(request) {
      calls.push(["plan_merge_policy_preview_with_proof", request]);
      assert.equal(currentBranchId, request.source_branch_id);
      assert.notEqual(request.source_branch_id, request.target_branch_id);
      return {
        proof: { planDigest: "plan-proof-digest" },
        source_branch_id: request.source_branch_id,
        target_branch_id: request.target_branch_id,
      };
    },
    merge_branches_with_proof(sourceBranchId, targetBranchId) {
      calls.push(["merge_branches_with_proof", sourceBranchId, targetBranchId]);
      assert.equal(currentBranchId, sourceBranchId);
      assert.notEqual(sourceBranchId, targetBranchId);
      return {
        proof: {
          resultDigest: "merge-result-digest",
          lineageDigest: "merge-lineage-digest",
        },
        source_branch_id: sourceBranchId,
        target_branch_id: targetBranchId,
      };
    },
  };
}

function createSpecialistStub(runSummary = {}) {
  const calls = [];
  return {
    calls,
    evaluateDirty() {
      calls.push(["evaluateDirty"]);
      return {
        touchedNodes: 0,
        nodesEvaluated: 0,
        nodesRecomputed: 0,
        nodesSuppressed: 0,
        plansBuilt: 0,
        stagesExecuted: 0,
        totalNanos: "0",
        evaluationNanos: "0",
        commitNanos: "0",
        ...runSummary,
      };
    },
  };
}

export {
  createHistoryStub,
  createSpecialistStub,
};
