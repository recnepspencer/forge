import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

test("64 simultaneous conflicts admit mixed source, target, and custom choices exactly once", async () => {
  const loaded = await loadSignalsModule();
  const { declareLocalTruthSchema } = await loaded.importProductModule(
    "local_truth/schema/schema_declaration.js",
  );
  const { createLocalTruthAuthority } = await loaded.importProductModule(
    "local_truth/authority/local_truth_authority.js",
  );
  const aspectIds = Array.from({ length: 64 }, (_, index) => `aspect${index}`);
  const schema = declareLocalTruthSchema({
    id: "conflict-scale",
    aspects: aspectIds.map((field) => ({
      id: field,
      field,
      valueType: "number",
      equivalence: { kind: "exact" },
      costClass: "constant",
    })),
  });
  const initial = Object.fromEntries(aspectIds.map((id) => [id, 0]));
  const truth = createLocalTruthAuthority({
    authorityId: "conflict-scale",
    schema,
    initialEntities: { gear: initial },
  });
  const main = required(await truth.branch());
  const source = await fork(truth, main, "source");
  const target = await fork(truth, main, "target");
  await commitAll(truth, source.id, "source-values", aspectIds, (index) => 100 + index);
  await commitAll(truth, target.id, "target-values", aspectIds, (index) => -100 - index);
  const review = await preview(truth, source.id, target.id);
  assert.equal(review.posture, "reviewRequired");
  assert.equal(review.review.conflicts.length, 64);

  const selections = [];
  const expected = {};
  for (const [index, conflict] of review.review.conflicts.entries()) {
    const aspectIndex = Number(conflict.aspectId.slice("aspect".length));
    const choice = index % 3;
    let alternative;
    if (choice < 2) {
      alternative = conflict.alternatives.find(({ choice: candidate }) => (
        candidate === (choice === 0 ? "source" : "target")
      ));
      expected[conflict.aspectId] = choice === 0 ? 100 + aspectIndex : -100 - aspectIndex;
    } else {
      const admission = required(await truth.createResolutionBranch({
        reviewId: review.review.id,
        conflictId: conflict.id,
        name: `custom-${index}`,
      }));
      const customValue = 1000 + index;
      const resolutionCommit = await truth.commit({
        requestId: `custom-value-${index}`,
        branchId: admission.branch.id,
        expectedBasis: admission.branch.basis,
        operations: [{ entityId: "gear", aspectId: conflict.aspectId, value: customValue }],
      });
      assert.equal(resolutionCommit.posture, "success");
      alternative = required(await truth.resolutionAlternative({
        reviewId: review.review.id,
        conflictId: conflict.id,
        resolutionBranchId: admission.branch.id,
      }));
      expected[conflict.aspectId] = customValue;
    }
    selections.push({
      reviewId: review.review.id,
      conflictId: conflict.id,
      alternativeId: alternative.id,
    });
  }
  const before = await truth.inspect();
  for (const invalid of invalidSelections(review.review, selections)) {
    assert.equal((await truth.resolveMerge({
      requestId: invalid.requestId,
      reviewId: review.review.id,
      selections: invalid.selections,
    })).posture, "denied");
    assert.equal((await truth.inspect()).digest, before.digest);
  }
  const merged = await truth.resolveMerge({
    requestId: "mixed-resolution",
    reviewId: review.review.id,
    selections: [...selections].reverse(),
  });
  assert.equal(merged.posture, "success");
  assert.deepEqual((await truth.inspect()).values[target.id].gear, expected);
  assert.equal(merged.value.commit.decisions.length, 64);
});

function invalidSelections(review, selections) {
  const first = selections[0];
  return [
    { requestId: "invalid-omitted", selections: selections.slice(1) },
    { requestId: "invalid-duplicate", selections: [first, ...selections.slice(0, -1)] },
    {
      requestId: "invalid-forged",
      selections: [{ ...first, alternativeId: "truth-alternative:forged" }, ...selections.slice(1)],
    },
    {
      requestId: "invalid-cross-review",
      selections: [{ ...first, reviewId: "truth-review:foreign" }, ...selections.slice(1)],
    },
    {
      requestId: "invalid-extra",
      selections: [...selections, {
        reviewId: review.id,
        conflictId: "truth-conflict:extra",
        alternativeId: "truth-alternative:extra",
      }],
    },
  ];
}

async function fork(truth, parent, name) {
  return required(await truth.forkBranch({
    parentBranchId: parent.id,
    expectedParentBasis: parent.basis,
    name,
  }));
}

async function commitAll(truth, branchId, requestId, aspectIds, value) {
  const branch = required(await truth.branch(branchId));
  const outcome = await truth.commit({
    requestId,
    branchId,
    expectedBasis: branch.basis,
    operations: aspectIds.map((aspectId, index) => ({ entityId: "gear", aspectId, value: value(index) })),
  });
  assert.equal(outcome.posture, "success");
}

async function preview(truth, sourceId, targetId) {
  return truth.previewMerge({
    sourceBranchId: sourceId,
    targetBranchId: targetId,
    expectedSourceBasis: required(await truth.branch(sourceId)).basis,
    expectedTargetBasis: required(await truth.branch(targetId)).basis,
  });
}

function required(outcome) {
  assert.equal(outcome.posture, "success", outcome.message);
  return outcome.value;
}
