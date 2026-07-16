import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../../runtime_fixture/real_resource_signals.mjs";

test("optional JSON path aspects read absent terminal object properties as null", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const response = createOptionalNoteResponse(signals);
    const aspect = response.aspects.definitions.note;

    assert.equal(aspect.read({ id: "t1", metadata: {} }), null);
    assert.equal(aspect.read({ id: "t1", metadata: { note: null } }), null);
    assert.equal(aspect.read({ id: "t1", metadata: { note: "present" } }), "present");
    assert.equal(aspect.jsonPathProof.policy.presence, "optional");
    assert.equal(aspect.jsonPathProof.policy.absence, "readAsNull");
  } finally {
    await runtime.cleanup();
  }
});

test("optional JSON path aspects materialize absent terminal object properties", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "json-optional-terminal");
    const tasks = createOptionalNoteApi(signals, { metadata: {} });
    const line = tasks.line({});

    await line.patch(tasks.patch.itemAspect({
      itemId: "t1",
      aspect: "note",
      value: "written",
    }));
    const effect = line.diagnostics().lastEffect;

    assert.deepEqual(line.value().tasks[0].metadata, { note: "written" });
    assert.equal(effect.locus.kind, "jsonItemAspect");
    assert.equal(effect.patch.jsonPath.policy.presence, "optional");
    assert.equal(effect.patch.jsonPath.policy.absence, "readAsNull");
    assert.equal(effect.counters.jsonPathTraversalBreadth, 2);
    assert.equal(effect.counters.jsonPathReconstructionBreadth, 2);
  } finally {
    await runtime.cleanup();
  }
});

test("optional JSON path aspects still deny missing intermediate containers", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const tasks = createOptionalNoteApi(signals, {});
    const line = tasks.line({});
    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;

    await assert.rejects(
      line.patch(tasks.patch.itemAspect({
          itemId: "t1",
          aspect: "note",
          value: "denied",
        })),
      /requires object JSON containers before segment "note"/,
    );
    assert.deepEqual(line.value(), beforeValue);
    assert.equal(line.diagnostics().lastEffect, beforeEffect);
  } finally {
    await runtime.cleanup();
  }
});

test("optional JSON path aspects do not create absent array indexes", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const response = signals.resource.response.objectItems()({
      field: "tasks",
      itemId: (task) => task.id,
      aspects: signals.resource.response.jsonPathAspects()({
        secondTag: { field: "metadata", path: ["tags", 1], presence: "optional" },
      }),
    });
    const tasks = signals.api({
      effects: signals.resource.effects.pessimistic(),
    }).url("/json-optional-array-index")
      .response(response)
      .list({
        load: () => ({
          tasks: [{ id: "t1", metadata: { tags: ["only"] } }],
        }),
      });
    const line = tasks.line({});
    const beforeValue = line.value();
    const beforeEffect = line.diagnostics().lastEffect;

    assert.throws(
      () =>
        line.patch(tasks.patch.itemAspect({
          itemId: "t1",
          aspect: "secondTag",
          value: "denied",
        })),
      /requires existing JSON path array index "1"/,
    );
    assert.deepEqual(line.value(), beforeValue);
    assert.equal(line.diagnostics().lastEffect, beforeEffect);
  } finally {
    await runtime.cleanup();
  }
});

test("optional JSON path declarations reject unknown presence policies", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;

    assert.throws(
      () =>
        signals.resource.response.jsonPathAspects()({
          note: { field: "metadata", path: ["note"], presence: "create" },
        }),
      /unsupported path presence policy "create"/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("optional absent JSON path writes reject through branch retirement", async () => {
  const runtime = await createRealRequestRuntime({
    restore_branch_snapshot_by_id: undefined,
  });
  try {
    const { mod, resource, signals } = runtime;
    createBranchHead(signals, "json-optional-absent-inverse");
    const tasks = createOptionalNoteCollection(mod, resource, {});
    const line = tasks.line({});

    await line.patch(mod.resourcePatch.itemAspect({
      itemId: "t1",
      aspect: "note",
      value: "written",
    }));
    const effect = line.diagnostics().lastEffect;

    assert.equal(effect.optimistic.kind, "applied");
    assert.equal(
      effect.optimistic.rollback.kind,
      "effectBranchRetirementAvailable",
    );
    assert.equal(effect.patch.jsonPath.policy.absence, "readAsNull");
    await line.effects().reject(effect.effectId);
    assert.deepEqual(line.value().items[0].metadata, {});
  } finally {
    await runtime.cleanup();
  }
});

test("optional present null JSON path writes reject exactly", async () => {
  const runtime = await createRealRequestRuntime({
    restore_branch_snapshot_by_id: undefined,
  });
  try {
    const { mod, resource, signals } = runtime;
    createBranchHead(signals, "json-optional-null-inverse");
    const tasks = createOptionalNoteCollection(mod, resource, { note: null });
    const line = tasks.line({});

    await line.patch(mod.resourcePatch.itemAspect({
      itemId: "t1",
      aspect: "note",
      value: "written",
    }));
    const effect = line.diagnostics().lastEffect;
    const rollback = await line.effects().reject(effect.effectId);

    assert.equal(
      effect.optimistic.rollback.kind,
      "effectBranchRetirementAvailable",
    );
    assert.equal(rollback.kind, "rejectedAndRetired");
    assert.deepEqual(line.value().items[0].metadata, { note: null });
  } finally {
    await runtime.cleanup();
  }
});

function createOptionalNoteApi(signals, taskFields) {
  const response = createOptionalNoteResponse(signals);
  return signals.api({
    effects: signals.resource.effects.branchNative(),
  }).url("/json-optional-note")
    .response(response)
    .list({
      load: () => ({
        tasks: [{
          id: "t1",
          ...taskFields,
        }],
      }),
    });
}

function createOptionalNoteCollection(mod, resource, metadata) {
  return resource.collection({
    params: mod.resourceParams(),
    normalizeParams: () => mod.resourceParamIdentity({}, "json-optional-note"),
    requestContext: mod.resourceRequestContext({ basisId: "basis-1" }),
    effects: mod.resourceEffects.branchNative(),
    itemIdentity: (task) => task.id,
    reconcile: mod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: mod.resourceResponse.jsonPathAspects()({
        note: { field: "metadata", path: ["note"], presence: "optional" },
      }),
    }),
    load: () => ({
      items: [{
        id: "t1",
        metadata,
      }],
    }),
  });
}

function createOptionalNoteResponse(signals) {
  return signals.resource.response.objectItems()({
    field: "tasks",
    itemId: (task) => task.id,
    aspects: signals.resource.response.jsonPathAspects()({
      note: { field: "metadata", path: ["note"], presence: "optional" },
    }),
  });
}
