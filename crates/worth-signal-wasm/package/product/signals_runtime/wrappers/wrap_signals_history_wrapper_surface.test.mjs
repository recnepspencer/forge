import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

test("wrapSignals history wrapper accepts numeric branch ids and normalizes preview requests", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawHistory = {
      current_branch() {
        return {
          id: 7,
          name: "main",
          parent_branch_id: null,
          head_snapshot_id: null,
        };
      },
      replay_for_branch(branchId) {
        calls.push(["replay_for_branch", typeof branchId, branchId]);
        return { frames: [] };
      },
      branch_snapshot(branchId) {
        calls.push(["branch_snapshot", typeof branchId, branchId]);
        return { meta: { branch_id: 7 } };
      },
      branch_snapshot_wire(branchId) {
        calls.push(["branch_snapshot_wire", typeof branchId, branchId]);
        return JSON.stringify({ meta: { branch_id: 7 } });
      },
      branch_snapshot_portable_wire(branchId) {
        calls.push([
          "branch_snapshot_portable_wire",
          typeof branchId,
          branchId,
        ]);
        return JSON.stringify({ meta: { branch_id: 7 } });
      },
      branch_snapshot_envelope(branchId) {
        calls.push(["branch_snapshot_envelope", typeof branchId, branchId]);
        return {
          snapshot: { meta: { branch_id: 7 } },
          state: { sources: [], recipes: [] },
        };
      },
      branch_snapshot_envelope_wire(branchId) {
        calls.push([
          "branch_snapshot_envelope_wire",
          typeof branchId,
          branchId,
        ]);
        return JSON.stringify({
          snapshot: { meta: { branch_id: 7 } },
          state: { sources: [], recipes: [] },
        });
      },
      branch_snapshot_envelope_portable_wire(branchId) {
        calls.push([
          "branch_snapshot_envelope_portable_wire",
          typeof branchId,
          branchId,
        ]);
        return JSON.stringify({
          snapshot: { meta: { branch_id: 7 } },
          state: { sources: [], recipes: [] },
        });
      },
      restore_snapshot(snapshot) {
        calls.push(["restore_snapshot", snapshot.snapshot.meta.branch_id]);
      },
      restore_snapshot_wire(snapshot) {
        calls.push([
          "restore_snapshot_wire",
          JSON.parse(snapshot).snapshot.meta.branch_id,
        ]);
      },
      restore_branch_snapshot(branchId, snapshot) {
        calls.push([
          "restore_branch_snapshot",
          typeof branchId,
          branchId,
          snapshot.meta.branch_id,
        ]);
      },
      restore_branch_snapshot_wire(branchId, snapshot) {
        calls.push([
          "restore_branch_snapshot_wire",
          typeof branchId,
          branchId,
          JSON.parse(snapshot).meta.branch_id,
        ]);
      },
      restore_branch_snapshot_portable_wire(branchId, snapshot) {
        calls.push([
          "restore_branch_snapshot_portable_wire",
          typeof branchId,
          branchId,
          JSON.parse(snapshot).meta.branch_id,
        ]);
      },
      branch_state_proof(branchId) {
        calls.push(["branch_state_proof", typeof branchId, branchId]);
        return { stateDigest: "digest" };
      },
      replay_parity_proof(expectedBranchId, replayedBranchId) {
        calls.push([
          "replay_parity_proof",
          typeof expectedBranchId,
          expectedBranchId,
          replayedBranchId,
        ]);
        return { parity: true, mismatch_classes: [] };
      },
      replay_artifact_proof(expected, replayedBranchId) {
        calls.push([
          "replay_artifact_proof",
          expected.branchStateDigest,
          typeof replayedBranchId,
          replayedBranchId,
        ]);
        return { parity: true, mismatch_classes: [] };
      },
      plan_merge_policy_preview(request) {
        calls.push(["plan_merge_policy_preview", request]);
        return { source_branch_id: request.source_branch_id };
      },
      plan_merge_policy_preview_with_proof(request) {
        calls.push(["plan_merge_policy_preview_with_proof", request]);
        return {
          plan: { source_branch_id: request.source_branch_id },
          proof: { planDigest: "plan" },
        };
      },
      merge_branches_policy_preview(request) {
        calls.push(["merge_branches_policy_preview", request]);
        return { target_branch: request.target_branch_id };
      },
      merge_branches_policy_preview_with_proof(request) {
        calls.push(["merge_branches_policy_preview_with_proof", request]);
        return {
          result: { target_branch: request.target_branch_id },
          proof: { resultDigest: "result" },
        };
      },
      free() {},
    };

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
          latestInvalidationPlanningEstimate() {
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
        return rawHistory;
      },
      specialist() {
        return {};
      },
      adapters() {
        return { free() {} };
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
    const history = signals.history();
    const currentBranch = history.current_branch();
    const snapshot = history.branch_snapshot(currentBranch.id);
    const envelope = history.branch_snapshot_envelope(currentBranch.id);
    history.restore_exact_snapshot(envelope);
    history.restore_exact_branch_snapshot(currentBranch.id, snapshot);
    const proof = history.branch_state_proof(currentBranch.id);
    const parity = history.replay_parity_proof(
      currentBranch.id,
      currentBranch.id,
    );
    const artifact = history.replay_artifact_proof(
      { branchStateDigest: proof.stateDigest },
      currentBranch.id,
    );
    const previewPlan = history.plan_merge_policy_preview({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });
    const previewPlanProof = history.plan_merge_policy_preview_with_proof({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });
    const previewResult = history.merge_branches_policy_preview({
      source_branch_id: currentBranch.id,
      target_branch_id: currentBranch.id,
    });
    const previewResultProof = history.merge_branches_policy_preview_with_proof(
      {
        source_branch_id: currentBranch.id,
        target_branch_id: currentBranch.id,
      },
    );

    assert.equal(previewPlan.source_branch_id, 7);
    assert.equal(previewPlanProof.proof.planDigest, "plan");
    assert.equal(previewResult.target_branch, 7);
    assert.equal(previewResultProof.proof.resultDigest, "result");
    assert.equal(parity.parity, true);
    assert.equal(artifact.parity, true);
    assert.equal(typeof snapshot.snapshotRestoreToken, "string");
    assert.equal(snapshot.snapshotRestoreMode, "SameRuntimeExact");
    assert.equal(typeof snapshot.snapshotPortableWire, "string");
    assert.equal(typeof envelope.snapshotEnvelopeRestoreToken, "string");
    assert.equal(envelope.snapshotEnvelopeRestoreMode, "SameRuntimeExact");
    assert.equal(typeof envelope.snapshotEnvelopePortableWire, "string");

    assert.deepEqual(calls, [
      ["branch_snapshot", "bigint", 7n],
      ["branch_snapshot_wire", "bigint", 7n],
      ["branch_snapshot_portable_wire", "bigint", 7n],
      ["branch_snapshot_envelope", "bigint", 7n],
      ["branch_snapshot_envelope_wire", "bigint", 7n],
      ["branch_snapshot_envelope_portable_wire", "bigint", 7n],
      ["restore_snapshot_wire", 7],
      ["restore_branch_snapshot_wire", "bigint", 7n, 7],
      ["branch_state_proof", "bigint", 7n],
      ["replay_parity_proof", "bigint", 7n, 7n],
      ["replay_artifact_proof", "digest", "bigint", 7n],
      [
        "plan_merge_policy_preview",
        { source_branch_id: 7, target_branch_id: 7 },
      ],
      [
        "plan_merge_policy_preview_with_proof",
        { source_branch_id: 7, target_branch_id: 7 },
      ],
      [
        "merge_branches_policy_preview",
        { source_branch_id: 7, target_branch_id: 7 },
      ],
      [
        "merge_branches_policy_preview_with_proof",
        { source_branch_id: 7, target_branch_id: 7 },
      ],
    ]);

    assert.throws(
      () => history.switch_branch(-1),
      /history\.switch_branch expects a non-negative safe integer branch id/,
    );
    assert.throws(
      () => history.plan_merge_policy_preview("bad"),
      /history\.plan_merge_policy_preview expects a merge preview request object/,
    );
    assert.throws(
      () =>
        history.restore_exact_snapshot({
          snapshot: { meta: { branch_id: 7 } },
          state: { sources: [], recipes: [] },
        }),
      /history\.restore_exact_snapshot expects an artifact returned by history\.snapshot\(\) or history\.branch_snapshot_envelope\(\)/,
    );
    assert.throws(
      () =>
        history.plan_merge_policy_preview({
          source_branch_id: 9007199254740992n,
          target_branch_id: currentBranch.id,
        }),
      /exceeds the safe integer range supported by merge preview requests/,
    );
  } finally {
    await cleanup();
  }
});


