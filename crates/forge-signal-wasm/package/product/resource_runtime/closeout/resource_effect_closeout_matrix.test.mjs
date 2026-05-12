import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("resource effect closeout matrices certify profile capability rows", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { effects } = runtime.resource;
    const branchNative = effects.closeoutMatrix(effects.branchNative());
    const pessimistic = effects.closeoutMatrix(effects.pessimistic());

    assert.equal(branchNative.profileName, "branchNative");
    assert.equal(pessimistic.profileName, "pessimistic");
    assert.deepEqual(branchNative.proofLanes, [
      "runtime",
      "typeSurface",
      "diagnosticsHistory",
      "branchMerge",
      "performance",
    ]);
    assert.ok(Object.isFrozen(branchNative));
    assert.ok(Object.isFrozen(branchNative.rows));
    assert.ok(branchNative.rows.every((row) => Object.isFrozen(row)));
    assert.deepEqual(projectCapabilityRows(branchNative), {
      localPatch: "admitted",
      deliveryPatch: "admitted",
      optimisticWrite: "admitted",
      confirmation: "serverCanonical",
      failureRollback: "branchRestoreOrInverse",
      branchRestore: "branchRestoreOrInverse",
      mergeRebase: "nativeMergePlan",
      broadReplacement: "admitted",
      diagnosticsHistory: "admitted",
    });
    assert.deepEqual(projectCapabilityRows(pessimistic), {
      localPatch: "admitted",
      deliveryPatch: "admitted",
      optimisticWrite: "unsupported",
      confirmation: "serverCanonical",
      failureRollback: "unavailable",
      branchRestore: "unsupported",
      mergeRebase: "unavailable",
      broadReplacement: "admitted",
      diagnosticsHistory: "admitted",
    });
    assert.deepEqual(
      branchNative.rows.map((row) => projectProofColumns(row)),
      branchNative.rows.map(() => ({
        runtimeProof: true,
        typeSurfaceProof: true,
        diagnosticsHistoryProof: true,
        branchMergeProof: true,
        performanceProof: true,
      })),
    );
    assert.deepEqual(projectEvidenceFor(branchNative, "mergeRebase"), {
      runtimeTests: [
        "resource_branch_merge_rebase_closeout.test.mjs",
        "resource_branch_mapping_unavailable.test.mjs",
      ],
      typeSurface: [
        "resource_api_effect_profiles_usage.ts",
        "resource_api_effect_profiles_denials.ts",
      ],
      diagnosticsHistory: ["resource_branch_merge_rebase_closeout.test.mjs"],
      branchMerge: [
        "resource_branch_capability_summary.test.mjs",
        "resource_branch_effect_merge_execution.test.mjs",
      ],
      performance: ["resource_branch_host_region_isolation.test.mjs"],
    });
    assert.equal(
      branchNative.rows.every((row) =>
        Object.values(row.evidence).every((references) => references.length > 0)
      ),
      true,
    );
    assert.ok(Object.isFrozen(projectEvidenceFor(branchNative, "localPatch")));
    assert.ok(
      Object.isFrozen(projectEvidenceFor(branchNative, "localPatch").runtimeTests),
    );
    assert.throws(
      () =>
        effects.closeoutMatrix({
          name: "fake",
          optimism: "branchSpeculative",
          confirmation: "serverCanonical",
          rollback: "branchRestoreOrInverse",
          rebase: "nativeMergePlan",
          preimage: "compactInverse",
        }),
      /requires a profile created with resource\.effects\.\*\(\)/,
    );
  } finally {
    await runtime.cleanup();
  }
});

function projectCapabilityRows(matrix) {
  return Object.fromEntries(
    matrix.rows.map((row) => [row.effectFamily, row.capability]),
  );
}

function projectProofColumns(row) {
  return {
    runtimeProof: row.runtimeProof,
    typeSurfaceProof: row.typeSurfaceProof,
    diagnosticsHistoryProof: row.diagnosticsHistoryProof,
    branchMergeProof: row.branchMergeProof,
    performanceProof: row.performanceProof,
  };
}

function projectEvidenceFor(matrix, effectFamily) {
  return matrix.rows.find((row) => row.effectFamily === effectFamily).evidence;
}
