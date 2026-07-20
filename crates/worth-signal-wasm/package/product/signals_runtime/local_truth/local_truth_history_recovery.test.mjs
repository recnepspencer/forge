import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

async function modules() {
  const loaded = await loadSignalsModule();
  const paths = [
    "local_truth/schema/schema_declaration.js",
    "local_truth/authority/authority_state.js",
    "local_truth/commit/mutation_pipeline.js",
    "local_truth/history/branch_history.js",
    "local_truth/history/index_recovery.js",
    "local_truth/merge/merge_basis.js",
    "local_truth/merge/merge_review.js",
    "local_truth/merge/merge_execution.js",
  ];
  return Object.assign({}, ...await Promise.all(paths.map((path) => loaded.importProductModule(path))));
}

async function historyFixture() {
  const api = await modules();
  const schema = api.declareLocalTruthSchema({
    id: "recovery.gear",
    aspects: ["label", "teeth"].map((field) => ({
      id: field,
      field,
      valueType: field === "teeth" ? "number" : "string",
      equivalence: { kind: "exact" },
      costClass: "constant",
    })),
  });
  let state = api.createInitialAuthorityState({
    authorityId: "recovery-authority",
    schema,
    initialEntities: { gear: { label: "Drive", teeth: 16 } },
  });
  const main = state.branches.get("branch:main");
  const sourceFork = api.forkLocalTruthBranch(state, {
    parentBranchId: main.id,
    expectedParentBasis: main.basis,
    name: "source",
  });
  state = sourceFork.state;
  const source = sourceFork.outcome.value;
  const targetFork = api.forkLocalTruthBranch(state, {
    parentBranchId: main.id,
    expectedParentBasis: main.basis,
    name: "target",
  });
  state = targetFork.state;
  const target = targetFork.outcome.value;
  state = mutate(api, state, schema, source.id, "source-before-checkpoint", [
    { entityId: "gear", aspectId: "teeth", value: 20 },
  ]);
  const firstReview = api.previewLocalTruthMerge(state, schema, mergeRequest(state, source.id, target.id));
  state = firstReview.state;
  const firstMerge = api.resolveAndCommitLocalTruthMerge(state, schema, {
    requestId: "first-merge",
    reviewId: firstReview.outcome.value.id,
    selections: [],
  });
  assert.equal(firstMerge.outcome.posture, "success");
  state = firstMerge.state;
  for (const branchId of [main.id, source.id, target.id]) {
    const checkpoint = api.createLocalTruthCheckpoint(state, branchId);
    assert.equal(checkpoint.outcome.posture, "success");
    state = checkpoint.state;
  }
  state = mutate(api, state, schema, source.id, "source-after-checkpoint", [
    { entityId: "gear", aspectId: "teeth", value: 24 },
    { entityId: "gear", aspectId: "label", value: "Final drive" },
  ]);
  return { api, schema, state, sourceId: source.id, targetId: target.id };
}

test("derived lineage and locus indexes rebuild exactly from checkpoints and bounded commits", async () => {
  const fixture = await historyFixture();
  const request = mergeRequest(fixture.state, fixture.sourceId, fixture.targetId);
  const expected = fixture.api.resolveLocalTruthMergeBasis(fixture.state, fixture.schema, request);
  assert.equal(expected.posture, "success");
  const withoutIndexes = {
    ...fixture.state,
    lineageByBranch: new Map(),
    locusHeadByBranch: new Map(),
  };
  const recovered = fixture.api.rebuildLocalTruthDerivedIndexes(withoutIndexes, fixture.schema);
  assert.equal(recovered.posture, "success");
  const actual = fixture.api.resolveLocalTruthMergeBasis(recovered.value, fixture.schema, request);
  assert.equal(actual.posture, "success");
  assert.equal(actual.value.identityDigest, expected.value.identityDigest);
  assert.deepEqual(actual.value.deltas, expected.value.deltas);
});

test("recovery denies corrupt checkpoints, segments, ancestry, and foreign branch heads", async () => {
  const fixture = await historyFixture();
  const source = fixture.state.branches.get(fixture.sourceId);
  const cases = [
    corruptCheckpoint(fixture.state, fixture.sourceId),
    corruptHeadCommit(fixture.state, source.headCommitId),
    corruptParent(fixture.state, fixture.sourceId),
    foreignBranchBasis(fixture.state, fixture.sourceId),
    missingRequiredCheckpoint(fixture.state, fixture.targetId),
  ];
  for (const state of cases) {
    const outcome = fixture.api.rebuildLocalTruthDerivedIndexes({
      ...state,
      lineageByBranch: new Map(),
      locusHeadByBranch: new Map(),
    }, fixture.schema);
    assert.equal(outcome.posture, "unavailable");
    assert.equal(outcome.code, "localTruthIndexRecoveryUnavailable");
  }
});

function mutate(api, state, schema, branchId, requestId, operations) {
  const result = api.admitLocalTruthMutation(state, schema, {
    branchId,
    expectedBasis: state.branches.get(branchId).basis,
    requestId,
    operations,
  });
  assert.equal(result.outcome.posture, "success");
  return result.state;
}

function mergeRequest(state, sourceBranchId, targetBranchId) {
  return {
    sourceBranchId,
    targetBranchId,
    expectedSourceBasis: state.branches.get(sourceBranchId).basis,
    expectedTargetBasis: state.branches.get(targetBranchId).basis,
  };
}

function corruptCheckpoint(state, branchId) {
  const checkpoints = new Map(state.checkpoints);
  const checkpoint = checkpoints.get(branchId);
  checkpoints.set(branchId, { ...checkpoint, compactedSegmentDigest: "corrupt" });
  return { ...state, checkpoints };
}

function corruptHeadCommit(state, commitId) {
  const commits = new Map(state.commits);
  const commit = commits.get(commitId);
  commits.set(commitId, { ...commit, operations: [...commit.operations, { entityId: "gear" }] });
  return { ...state, commits };
}

function corruptParent(state, branchId) {
  const branches = new Map(state.branches);
  branches.set(branchId, { ...branches.get(branchId), parentBranchId: "branch:missing" });
  return { ...state, branches };
}

function foreignBranchBasis(state, branchId) {
  const branches = new Map(state.branches);
  const branch = branches.get(branchId);
  branches.set(branchId, { ...branch, basis: { ...branch.basis, authorityId: "foreign" } });
  return { ...state, branches };
}

function missingRequiredCheckpoint(state, branchId) {
  const checkpoints = new Map(state.checkpoints);
  checkpoints.delete(branchId);
  return { ...state, checkpoints };
}
