import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

async function runtime(id) {
  const loaded = await loadSignalsModule();
  const { declareLocalTruthSchema } = await loaded.importProductModule("local_truth/schema/schema_declaration.js");
  const { createLocalTruthAuthority } = await loaded.importProductModule("local_truth/authority/local_truth_authority.js");
  const schema = declareLocalTruthSchema({
    id: "merge.gear",
    aspects: ["label", "material", "rotation", "teeth"].map((field) => ({
      id: field,
      field,
      valueType: field === "teeth" || field === "rotation" ? "number" : "string",
      equivalence: { kind: "exact" },
      costClass: "constant",
    })),
  });
  return createLocalTruthAuthority({
    authorityId: id,
    schema,
    initialEntities: { gear: { teeth: 16, material: "steel", rotation: 0, label: "Drive" } },
  });
}

async function fork(truth, parent, name) {
  return (await truth.forkBranch({
    parentBranchId: parent.id,
    expectedParentBasis: parent.basis,
    name,
  })).value;
}

async function mutate(truth, branch, requestId, aspectId, value) {
  return truth.commit({
    requestId,
    branchId: branch.id,
    expectedBasis: (await truth.branch(branch.id)).value.basis,
    operations: [{ entityId: "gear", aspectId, value }],
  });
}

async function preview(truth, source, target, scope) {
  return truth.previewMerge({
    sourceBranchId: source.id,
    targetBranchId: target.id,
    expectedSourceBasis: (await truth.branch(source.id)).value.basis,
    expectedTargetBasis: (await truth.branch(target.id)).value.basis,
    scope,
  });
}

test("sibling disjoint aspects compose and preserve unselected fields", async () => {
  const truth = await runtime("sibling-disjoint");
  const main = (await truth.branch()).value;
  const source = await fork(truth, main, "source");
  const target = await fork(truth, main, "target");
  await mutate(truth, source, "source-teeth", "teeth", 24);
  await mutate(truth, target, "target-label", "label", "Final drive");
  const planned = await preview(truth, source, target);
  assert.equal(planned.posture, "success");
  const merged = await truth.resolveMerge({ requestId: "merge", reviewId: planned.value.id, selections: [] });
  assert.equal(merged.posture, "success");
  assert.deepEqual((await truth.inspect()).values[target.id].gear, {
    teeth: 24,
    material: "steel",
    rotation: 0,
    label: "Final drive",
  });
});

test("repeated partial merges use per-locus integration lineage", async () => {
  const truth = await runtime("repeated-partial");
  const main = (await truth.branch()).value;
  const source = await fork(truth, main, "source");
  const target = await fork(truth, main, "target");
  await mutate(truth, source, "teeth-1", "teeth", 20);
  await mutate(truth, source, "material-1", "material", "titanium");
  const teethOnly = await preview(truth, source, target, { entityIds: ["gear"], aspectIds: ["teeth"] });
  await truth.resolveMerge({ requestId: "partial-1", reviewId: teethOnly.value.id, selections: [] });
  const noNewTeeth = await preview(truth, source, target, { entityIds: ["gear"], aspectIds: ["teeth"] });
  assert.equal(noNewTeeth.value.classifications[0].kind, "Unchanged");
  await mutate(truth, source, "teeth-2", "teeth", 22);
  const secondTeeth = await preview(truth, source, target, { entityIds: ["gear"], aspectIds: ["teeth"] });
  assert.equal(secondTeeth.value.classifications[0].kind, "AdoptSource");
  await truth.resolveMerge({ requestId: "partial-2", reviewId: secondTeeth.value.id, selections: [] });
  const materialOnly = await preview(truth, source, target, { entityIds: ["gear"], aspectIds: ["material"] });
  assert.equal(materialOnly.value.classifications[0].kind, "AdoptSource");
});

test("parent and child direction resolve from retained structural ancestry", async () => {
  const truth = await runtime("parent-child");
  const parent = (await truth.branch()).value;
  const child = await fork(truth, parent, "child");
  await mutate(truth, child, "child-rotation", "rotation", 45);
  const childIntoParent = await preview(truth, child, parent);
  assert.equal(childIntoParent.posture, "success");
  await truth.resolveMerge({ requestId: "child-parent", reviewId: childIntoParent.value.id, selections: [] });
  await mutate(truth, parent, "parent-label", "label", "Parent edited");
  const parentIntoChild = await preview(truth, parent, child);
  assert.equal(parentIntoChild.posture, "success");
  assert.equal(parentIntoChild.value.structuralAncestorCommitId, child.forkCommitId);
});

test("manual source, target, and custom alternatives are runtime-owned", async () => {
  for (const choice of ["source", "target", "custom"]) {
    const truth = await runtime(`conflict-${choice}`);
    const main = (await truth.branch()).value;
    const source = await fork(truth, main, "source");
    const target = await fork(truth, main, "target");
    await mutate(truth, source, `source-${choice}`, "teeth", 24);
    await mutate(truth, target, `target-${choice}`, "teeth", 12);
    const planned = await preview(truth, source, target);
    assert.equal(planned.posture, "reviewRequired");
    const conflict = planned.review.conflicts[0];
    let alternative;
    if (choice === "custom") {
      const receipt = (await truth.createResolutionBranch({
        reviewId: planned.review.id,
        conflictId: conflict.id,
        name: "custom-resolution",
      })).value;
      await mutate(truth, receipt.branch, "custom-value", "teeth", 18);
      alternative = (await truth.resolutionAlternative({
        reviewId: planned.review.id,
        conflictId: conflict.id,
        resolutionBranchId: receipt.branch.id,
      })).value;
    } else {
      alternative = conflict.alternatives.find((candidate) => candidate.choice === choice);
    }
    const merged = await truth.resolveMerge({
      requestId: `merge-${choice}`,
      reviewId: planned.review.id,
      selections: [{
        reviewId: planned.review.id,
        conflictId: conflict.id,
        alternativeId: alternative.id,
      }],
    });
    assert.equal(merged.posture, "success");
    assert.equal((await truth.inspect()).values[target.id].gear.teeth, {
      source: 24,
      target: 12,
      custom: 18,
    }[choice]);
  }
});

test("stale review and incomplete resolution publish no merge", async () => {
  const truth = await runtime("stale-review");
  const main = (await truth.branch()).value;
  const source = await fork(truth, main, "source");
  const target = await fork(truth, main, "target");
  await mutate(truth, source, "source", "teeth", 20);
  await mutate(truth, target, "target", "teeth", 10);
  const planned = await preview(truth, source, target);
  const before = await truth.inspect();
  assert.equal((await truth.resolveMerge({
    requestId: "incomplete",
    reviewId: planned.review.id,
    selections: [],
  })).code, "incompleteLocalTruthResolution");
  assert.equal((await truth.inspect()).digest, before.digest);
  await mutate(truth, source, "advance", "label", "advanced");
  assert.equal((await truth.resolveMerge({
    requestId: "stale",
    reviewId: planned.review.id,
    selections: [{
      reviewId: planned.review.id,
      conflictId: planned.review.conflicts[0].id,
      alternativeId: planned.review.conflicts[0].alternatives[0].id,
    }],
  })).code, "staleLocalTruthReview");
});

test("checkpoint compaction preserves repeated per-locus merge semantics", async () => {
  const truth = await runtime("checkpoint-retention");
  const main = (await truth.branch()).value;
  const source = await fork(truth, main, "source");
  const target = await fork(truth, main, "target");
  await mutate(truth, source, "source-teeth-before-checkpoint", "teeth", 24);
  const firstReview = await preview(truth, source, target, {
    entityIds: ["gear"],
    aspectIds: ["teeth"],
  });
  assert.equal((await truth.resolveMerge({
    requestId: "merge-before-checkpoint",
    reviewId: firstReview.value.id,
    selections: [],
  })).posture, "success");

  for (const branchId of [main.id, source.id, target.id]) {
    assert.equal((await truth.checkpoint(branchId)).posture, "success");
  }
  const compacted = await truth.history(target.id);
  assert.equal(compacted.value.checkpoint.headCommitId, (await truth.branch(target.id)).value.headCommitId);
  assert.equal(compacted.value.commits.length, 0);
  assert.equal((await truth.inspect()).counters.compactions, 1);
  assert.equal((await truth.checkpoint(target.id)).posture, "success");
  assert.equal((await truth.inspect()).counters.compactions, 1);

  await mutate(truth, source, "source-teeth-after-checkpoint", "teeth", 27);
  const nextReview = await preview(truth, source, target, {
    entityIds: ["gear"],
    aspectIds: ["teeth"],
  });
  assert.equal(nextReview.posture, "success");
  assert.equal(nextReview.value.classifications[0].kind, "AdoptSource");
  assert.equal((await truth.resolveMerge({
    requestId: "merge-after-checkpoint",
    reviewId: nextReview.value.id,
    selections: [],
  })).posture, "success");
  assert.equal((await truth.inspect()).values[target.id].gear.teeth, 27);
});

test("merge staging failures at every reconstruction boundary publish nothing", async () => {
  const loaded = await loadSignalsModule();
  const { declareLocalTruthSchema } = await loaded.importProductModule("local_truth/schema/schema_declaration.js");
  const { createLocalTruthAuthority } = await loaded.importProductModule("local_truth/authority/local_truth_authority.js");
  for (const failurePoint of [
    "mergeReconstruction",
    "mergeReconstruction:0",
    "mergeReconstruction:1",
    "mergeDigesting",
    "mergePublication",
  ]) {
    let activeFailure = null;
    const schema = declareLocalTruthSchema({
      id: `merge-failure.${failurePoint}`,
      aspects: ["label", "teeth"].map((field) => ({
        id: field,
        field,
        valueType: field === "teeth" ? "number" : "string",
        equivalence: { kind: "exact" },
        costClass: "constant",
      })),
    });
    const truth = createLocalTruthAuthority({
      authorityId: `merge-failure-${failurePoint}`,
      schema,
      initialEntities: { gear: { label: "Drive", teeth: 16 } },
    }, {
      faultInjector(point) {
        if (point === activeFailure) throw new Error(`injected ${point}`);
      },
    });
    const main = (await truth.branch()).value;
    const source = await fork(truth, main, "source");
    const target = await fork(truth, main, "target");
    const current = (await truth.branch(source.id)).value;
    assert.equal((await truth.commit({
      requestId: `source-${failurePoint}`,
      branchId: source.id,
      expectedBasis: current.basis,
      operations: [
        { entityId: "gear", aspectId: "label", value: "Final" },
        { entityId: "gear", aspectId: "teeth", value: 24 },
      ],
    })).posture, "success");
    const review = await preview(truth, source, target);
    const before = await truth.inspect();
    activeFailure = failurePoint;
    assert.equal((await truth.resolveMerge({
      requestId: `merge-${failurePoint}`,
      reviewId: review.value.id,
      selections: [],
    })).posture, "failed");
    assert.equal((await truth.inspect()).digest, before.digest);
    assert.equal((await truth.inspect()).counters.merges, 0);
  }
});

test("merge request replay is advisory and identity reuse is denied", async () => {
  const truth = await runtime("merge-request-replay");
  const main = (await truth.branch()).value;
  const source = await fork(truth, main, "source");
  const target = await fork(truth, main, "target");
  await mutate(truth, source, "source-change", "teeth", 24);
  const review = await preview(truth, source, target);
  const request = { requestId: "merge-once", reviewId: review.value.id, selections: [] };
  assert.equal((await truth.resolveMerge(request)).posture, "success");
  const after = await truth.inspect();
  assert.equal((await truth.resolveMerge(request)).posture, "advisory");
  assert.equal((await truth.inspect()).digest, after.digest);
  assert.equal((await truth.inspect()).counters.merges, 1);
  assert.equal((await truth.resolveMerge({ ...request, reviewId: "truth-review:foreign" })).code, "requestIdentityReuse");
});

test("committing a review retires every admitted resolution branch and derivation", async () => {
  const loaded = await loadSignalsModule();
  const { declareLocalTruthSchema } = await loaded.importProductModule("local_truth/schema/schema_declaration.js");
  const { createLocalTruthAuthority } = await loaded.importProductModule("local_truth/authority/local_truth_authority.js");
  const destroyed = [];
  const schema = declareLocalTruthSchema({
    id: "resolution-retirement",
    aspects: [{ id: "teeth", field: "teeth", valueType: "number", equivalence: { kind: "exact" } }],
  });
  const truth = createLocalTruthAuthority({
    authorityId: "resolution-retirement",
    schema,
    initialEntities: { gear: { teeth: 16 } },
  }, {
    projection: {
      counters: () => ({}),
      async destroy(branchId) {
        destroyed.push(branchId);
      },
    },
  });
  const main = (await truth.branch()).value;
  const source = await fork(truth, main, "source");
  const target = await fork(truth, main, "target");
  await mutate(truth, source, "source-conflict", "teeth", 24);
  await mutate(truth, target, "target-conflict", "teeth", 12);
  const review = await preview(truth, source, target);
  const conflict = review.review.conflicts[0];
  const resolutionBranches = [];
  for (const value of [18, 20]) {
    const admission = (await truth.createResolutionBranch({
      reviewId: review.review.id,
      conflictId: conflict.id,
      name: `candidate-${value}`,
    })).value;
    resolutionBranches.push(admission.branch.id);
    await mutate(truth, admission.branch, `candidate-${value}`, "teeth", value);
    assert.equal((await truth.resolutionAlternative({
      reviewId: review.review.id,
      conflictId: conflict.id,
      resolutionBranchId: admission.branch.id,
    })).posture, "success");
  }
  const sourceAlternative = conflict.alternatives.find(({ choice }) => choice === "source");
  const merged = await truth.resolveMerge({
    requestId: "retire-review-island",
    reviewId: review.review.id,
    selections: [{
      reviewId: review.review.id,
      conflictId: conflict.id,
      alternativeId: sourceAlternative.id,
    }],
  });
  assert.equal(merged.posture, "success");
  assert.deepEqual(merged.value.merge.retiredResolutionBranchIds, [...resolutionBranches].sort());
  assert.deepEqual(destroyed.sort(), [...resolutionBranches].sort());
  for (const branchId of resolutionBranches) {
    assert.equal((await truth.branch(branchId)).posture, "denied");
  }
});
