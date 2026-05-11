import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

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

    line.patch(tasks.patch.itemAspect({
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

    assert.throws(
      () =>
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

function createOptionalNoteResponse(signals) {
  return signals.resource.response.objectItems()({
    field: "tasks",
    itemId: (task) => task.id,
    aspects: signals.resource.response.jsonPathAspects()({
      note: { field: "metadata", path: ["note"], presence: "optional" },
    }),
  });
}
