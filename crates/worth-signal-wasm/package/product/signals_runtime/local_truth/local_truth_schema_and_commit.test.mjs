import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

async function modules() {
  const loaded = await loadSignalsModule();
  const schema = await loaded.importProductModule("local_truth/schema/schema_declaration.js");
  const authority = await loaded.importProductModule("local_truth/authority/local_truth_authority.js");
  return { ...schema, ...authority };
}

function gearDeclaration(aspects = [
  ["innerRadius", "innerRadius"],
  ["outerRadius", "outerRadius"],
  ["teeth", "teeth"],
  ["thickness", "thickness"],
]) {
  return {
    id: "certification.gear",
    aspects: aspects.map(([id, field]) => ({
      id,
      field,
      valueType: "number",
      equivalence: { kind: "exact" },
      costClass: "constant",
    })),
  };
}

test("schema identity and aspect order are canonical while materialization is local", async () => {
  const { declareLocalTruthSchema, materializeAspect } = await modules();
  const forward = declareLocalTruthSchema(gearDeclaration());
  const reversedDeclaration = gearDeclaration();
  reversedDeclaration.aspects.reverse();
  const reverse = declareLocalTruthSchema(reversedDeclaration);
  assert.equal(forward.identity, reverse.identity);
  assert.deepEqual(forward.aspects.map(({ id }) => id), [
    "innerRadius",
    "outerRadius",
    "teeth",
    "thickness",
  ]);
  const label = Object.freeze({ text: "Drive gear" });
  const before = Object.freeze({ innerRadius: 1, outerRadius: 2, teeth: 12, thickness: 0.5, label });
  const after = materializeAspect(forward, before, "teeth", 14);
  assert.deepEqual(after, { ...before, teeth: 14 });
  assert.deepEqual(after.label, label);
  assert.equal(Object.isFrozen(after), true);
});

test("invalid schema declarations fail before authority construction", async () => {
  const { declareLocalTruthSchema } = await modules();
  assert.throws(() => declareLocalTruthSchema({
    id: "duplicate",
    aspects: [
      { id: "a", field: "value", equivalence: { kind: "exact" } },
      { id: "a", field: "other", equivalence: { kind: "exact" } },
    ],
  }), /duplicate aspect id/);
  assert.throws(() => declareLocalTruthSchema({
    id: "missing-comparator",
    aspects: [{ id: "a", field: "value" }],
  }), /explicit equivalence posture/);
  assert.throws(() => declareLocalTruthSchema({
    id: "duplicate-field",
    aspects: [
      { id: "a", field: "value", equivalence: { kind: "exact" } },
      { id: "b", field: "value", equivalence: { kind: "exact" } },
    ],
  }), /duplicate aspect field/);
});

test("exact equivalence is canonical value equality and unsupported data cannot enter truth", async () => {
  const { declareLocalTruthSchema, aspectsEquivalent, createLocalTruthAuthority } = await modules();
  const schema = declareLocalTruthSchema({
    id: "canonical-values",
    aspects: [{
      id: "metadata",
      field: "metadata",
      valueType: "any",
      equivalence: { kind: "exact" },
      costClass: "linearInValue",
    }],
  });
  assert.equal(aspectsEquivalent(schema, "metadata", { a: 1, b: [2] }, { b: [2], a: 1 }), true);
  assert.equal(aspectsEquivalent(schema, "metadata", -0, 0), false);
  assert.throws(() => createLocalTruthAuthority({
    authorityId: "unsupported-values",
    schema,
    initialEntities: { gear: { metadata: { unsafe: () => 1 } } },
  }), /unsupported canonical value type function/);
});

test("mutation failures at every stage publish all or nothing", async () => {
  const { declareLocalTruthSchema, createLocalTruthAuthority } = await modules();
  const schema = declareLocalTruthSchema(gearDeclaration());
  for (const failurePoint of ["validation", "planning", "reconstruction", "digesting", "publication"]) {
    const authority = createLocalTruthAuthority(
      {
        authorityId: `atomic-${failurePoint}`,
        schema,
        initialEntities: { gear: { innerRadius: 1, outerRadius: 2, teeth: 12, thickness: 0.5 } },
      },
      {
        onInitialize: null,
        onBranchFork: null,
        onCommitted: null,
        faultInjector(point) {
          if (point === failurePoint) throw new Error(`injected ${point}`);
        },
      },
    );
    const before = await authority.inspect();
    const branch = (await authority.branch()).value;
    const outcome = await authority.commit({
      requestId: `request-${failurePoint}`,
      branchId: branch.id,
      expectedBasis: branch.basis,
      operations: [{ entityId: "gear", aspectId: "teeth", value: 20 }],
    });
    const after = await authority.inspect();
    assert.equal(outcome.posture, "failed");
    assert.equal(after.digest, before.digest);
    assert.equal(after.counters.commits, 1);
  }
});

test("stale, foreign, and structurally forged bases cannot advance truth", async () => {
  const { declareLocalTruthSchema, createLocalTruthAuthority } = await modules();
  const schema = declareLocalTruthSchema(gearDeclaration());
  const make = (id) => createLocalTruthAuthority({
    authorityId: id,
    schema,
    initialEntities: { gear: { innerRadius: 1, outerRadius: 2, teeth: 12, thickness: 0.5 } },
  });
  const first = make("first");
  const second = make("second");
  const stale = (await first.branch()).value;
  const foreign = (await second.branch()).value;
  assert.equal((await first.commit({
    requestId: "advance",
    branchId: stale.id,
    expectedBasis: stale.basis,
    operations: [{ entityId: "gear", aspectId: "teeth", value: 13 }],
  })).posture, "success");
  assert.equal((await first.commit({
    requestId: "stale",
    branchId: stale.id,
    expectedBasis: stale.basis,
    operations: [{ entityId: "gear", aspectId: "teeth", value: 14 }],
  })).code, "staleLocalTruthBasis");
  assert.equal((await first.commit({
    requestId: "foreign",
    branchId: stale.id,
    expectedBasis: foreign.basis,
    operations: [{ entityId: "gear", aspectId: "teeth", value: 14 }],
  })).code, "foreignLocalTruthBasis");
  assert.equal((await first.commit({
    requestId: "forged",
    branchId: stale.id,
    expectedBasis: { ...(await first.branch()).value.basis, identityDigest: "forged" },
    operations: [{ entityId: "gear", aspectId: "teeth", value: 14 }],
  })).code, "forgedLocalTruthBasis");
  const current = (await first.branch()).value;
  assert.equal((await first.commit({
    requestId: "structurally-exact-forgery",
    branchId: current.id,
    expectedBasis: { ...current.basis },
    operations: [{ entityId: "gear", aspectId: "teeth", value: 14 }],
  })).code, "forgedLocalTruthBasis");
});

test("request replay is canonical, advisory, and never republishes derivation", async () => {
  const { declareLocalTruthSchema, createLocalTruthAuthority } = await modules();
  const schema = declareLocalTruthSchema(gearDeclaration());
  let projections = 0;
  const authority = createLocalTruthAuthority({
    authorityId: "idempotent-mutations",
    schema,
    initialEntities: { gear: { innerRadius: 1, outerRadius: 2, teeth: 12, thickness: 0.5 } },
  }, {
    onCommitted: async () => {
      projections += 1;
      return { posture: "Current" };
    },
  });
  const branch = (await authority.branch()).value;
  const request = {
    requestId: "canonical-replay",
    branchId: branch.id,
    expectedBasis: branch.basis,
    operations: [
      { entityId: "gear", aspectId: "teeth", value: 20 },
      { entityId: "gear", aspectId: "thickness", value: 0.75 },
    ],
  };
  assert.equal((await authority.commit(request)).posture, "success");
  const afterCommit = await authority.inspect();
  assert.equal(projections, 1);
  const replay = await authority.commit({ ...request, operations: [...request.operations].reverse() });
  assert.equal(replay.posture, "advisory");
  assert.equal(replay.value.id, afterCommit.branches.find(({ id }) => id === branch.id).headCommitId);
  assert.equal((await authority.inspect()).digest, afterCommit.digest);
  assert.equal(projections, 1);
  assert.equal((await authority.commit({
    ...request,
    operations: [{ entityId: "gear", aspectId: "teeth", value: 21 }],
  })).code, "requestIdentityReuse");
});
