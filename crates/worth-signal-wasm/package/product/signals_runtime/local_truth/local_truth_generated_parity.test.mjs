import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

const aspectIds = Object.freeze(["label", "material", "rotation", "teeth"]);
const initialGear = Object.freeze({ label: 0, material: 0, rotation: 0, teeth: 0 });

test("generated mixed histories converge across deployment modes and candidate order", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  try {
    for (const seed of [1, 2, 3]) {
      const compatibility = await runHistory(seed, "mainThreadCompatibility", false);
      const worker = await runHistory(seed, "workerFirst", false);
      assert.deepEqual(worker, compatibility);
      if (seed === 2) {
        assert.deepEqual(
          await runHistory(seed, "mainThreadCompatibility", true),
          compatibility,
        );
      }
    }
  } finally {
    globalThis.Worker = previousWorker;
  }
});

async function runHistory(seed, deployment, reverseCandidates) {
  const loaded = await loadSignalsModule({ rawSurface: "real" });
  const { declareLocalTruthSchema } = await loaded.importProductModule(
    "local_truth/schema/schema_declaration.js",
  );
  const signals = await loaded.createSignals({ deployment });
  const schema = declareLocalTruthSchema({
    id: "generated-parity.gear",
    aspects: aspectIds.map((field) => ({
      id: field,
      field,
      valueType: "number",
      equivalence: field === "rotation"
        ? { kind: "numberEpsilon", epsilon: 0.01 }
        : { kind: "exact" },
      costClass: "constant",
    })),
  });
  const gear = signals.input(initialGear, { debugName: "gear", producesAspects: [0, 1, 2, 3] });
  const truth = signals.localTruth({
    authorityId: `generated-parity-${seed}`,
    schema,
    initialEntities: { gear: initialGear },
    bindings: [{
      entityId: "gear",
      input: gear,
      aspectMap: { label: 0, material: 1, rotation: 2, teeth: 3 },
    }],
  });
  await truth.ready?.();
  const main = required(await truth.branch());
  const source = required(await truth.forkBranch({
    parentBranchId: main.id,
    expectedParentBasis: main.basis,
    name: "source",
  }));
  const target = required(await truth.forkBranch({
    parentBranchId: main.id,
    expectedParentBasis: main.basis,
    name: "target",
  }));
  const sourceOps = [
    operation("label", 10 + seed),
    operation("material", 20 + seed),
    operation("rotation", 30 + seed + 0.004),
  ];
  const targetOps = [
    operation("label", 100 + seed),
    operation("rotation", 30 + seed),
    operation("teeth", 40 + seed),
  ];
  if (reverseCandidates) {
    sourceOps.reverse();
    targetOps.reverse();
  }
  const commitSource = () => commit(truth, source.id, `source-${seed}`, sourceOps);
  const commitTarget = () => commit(truth, target.id, `target-${seed}`, targetOps);
  if (seed % 2 === 0) {
    await commitTarget();
    await commitSource();
  } else {
    await commitSource();
    await commitTarget();
  }
  let review = await preview(truth, source.id, target.id);
  assert.equal(review.posture, "reviewRequired");
  if (seed === 3) {
    await commit(truth, source.id, "advance-open-review", [operation("material", 200 + seed)]);
    assert.equal((await truth.resolveMerge({
      requestId: "stale-open-review",
      reviewId: review.review.id,
      selections: [],
    })).code, "staleLocalTruthReview");
    review = await preview(truth, source.id, target.id);
  }
  const conflict = review.review.conflicts[0];
  const selection = await chooseAlternative(truth, review.review, conflict, seed % 3);
  const merged = await truth.resolveMerge({
    requestId: `merge-${seed}`,
    reviewId: review.review.id,
    selections: [{
      reviewId: review.review.id,
      conflictId: conflict.id,
      alternativeId: selection.id,
    }],
  });
  assert.equal(merged.posture, "success");
  for (const branch of (await truth.inspect()).branches.filter(({ retired }) => !retired)) {
    assert.equal((await truth.checkpoint(branch.id)).posture, "success");
  }
  const inspection = await truth.inspect();
  const history = required(await truth.history(target.id));
  const destroyed = await truth.destroyDerivation(target.id);
  const rebuilt = await truth.rebuildDerivation(target.id);
  const result = {
    truthDigest: inspection.digest,
    targetValue: inspection.values[target.id].gear,
    decisions: inspection.decisionLog.map(({ aspectId, classification, selection: choice }) => ({
      aspectId,
      classification,
      choice,
    })),
    counters: inspection.counters,
    historyCommitCount: history.commits.length,
    checkpointDigest: history.checkpoint.digest,
    destroyed: destroyed.posture,
    rebuilt: rebuilt.posture,
  };
  await truth.terminate();
  await signals.terminate();
  return result;
}

async function chooseAlternative(truth, review, conflict, choice) {
  if (choice < 2) return conflict.alternatives[choice];
  const admission = required(await truth.createResolutionBranch({
    reviewId: review.id,
    conflictId: conflict.id,
    name: "generated-custom",
  }));
  await commit(truth, admission.branch.id, "generated-custom-value", [
    operation(conflict.aspectId, 500),
  ]);
  return required(await truth.resolutionAlternative({
    reviewId: review.id,
    conflictId: conflict.id,
    resolutionBranchId: admission.branch.id,
  }));
}

async function commit(truth, branchId, requestId, operations) {
  const branch = required(await truth.branch(branchId));
  const outcome = await truth.commit({ requestId, branchId, expectedBasis: branch.basis, operations });
  assert.equal(outcome.posture, "success");
  return outcome.value;
}

async function preview(truth, sourceBranchId, targetBranchId) {
  return truth.previewMerge({
    sourceBranchId,
    targetBranchId,
    expectedSourceBasis: required(await truth.branch(sourceBranchId)).basis,
    expectedTargetBasis: required(await truth.branch(targetBranchId)).basis,
  });
}

function operation(aspectId, value) {
  return { entityId: "gear", aspectId, value };
}

function required(outcome) {
  assert.ok(outcome.posture === "success" || outcome.posture === "advisory", outcome.message);
  return outcome.value;
}
