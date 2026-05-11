import assert from "node:assert/strict";
import test from "node:test";

import {
  createBranchHead,
  createRealResourceSignals,
} from "../runtime_fixture/real_resource_signals.mjs";

test("resource branch namespace exposes product merge-plan summaries with proof digests", async () => {
  const runtime = await createRealResourceSignals();
  try {
    const branch = createBranchHead(runtime.signals, "feature/resource-branch");
    const summary = runtime.signals.resource.branch.planMerge({
      source_branch_id: branch.id,
      target_branch_id: 0,
    });

    assert.equal(summary.kind, "planned");
    assert.equal(summary.sourceBranchId, branch.id);
    assert.equal(summary.targetBranchId, 0);
    assert.equal(typeof summary.selectedSemantics.strategy, "string");
    assert.equal(typeof summary.selectedSemantics.conflictPolicy, "string");
    assert.equal(Number.isInteger(summary.breadth.nodePlanCount), true);
    assert.equal(typeof summary.proof.planDigest, "string");
    assert.equal(typeof summary.proof.semanticsDigest, "string");
    assert.equal(typeof summary.proof.selectedConflictPolicyDigest, "string");
  } finally {
    await runtime.cleanup();
  }
});

test("resource branch merge-plan summaries deny unsupported or malformed branch requests", async () => {
  const runtime = await createRealResourceSignals();
  try {
    const branch = runtime.signals.history().current_branch();
    const denied = runtime.signals.resource.branch.planMerge({
      source_branch_id: -1,
      target_branch_id: branch.id,
    });

    assert.deepEqual(denied, {
      kind: "denied",
      reason: "mergePlanUnavailable",
      detail:
        "history.plan_merge_policy_preview_with_proof.source_branch_id expects a non-negative safe integer branch id",
    });
  } finally {
    await runtime.cleanup();
  }
});
