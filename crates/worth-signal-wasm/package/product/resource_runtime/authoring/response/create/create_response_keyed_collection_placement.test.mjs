import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("create responses can insert entity-store items through declared append placement", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const entityTasks = runtime.signals.api({}).url("/entity-tasks")
      .response(runtime.signals.resource.response.entityStore()({
        itemId: (task) => task.id,
        entities: (value) => value.entities,
        replaceEntities: (value, nextEntities) => ({ ...value, entities: nextEntities }),
        replaceEntity: (value, itemId, nextItem) => ({
          ...value,
          entities: {
            ...value.entities,
            [itemId]: nextItem,
          },
        }),
      }))
      .list({
        load: () => ({
          entities: {
            "task:1": { id: "task:1", title: "First" },
          },
        }),
      });
    const line = entityTasks.line({});

    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: entityTasks,
          params: () => ({}),
          fallback: "placementUnavailable",
          collection: { kind: "insert", placement: "append" },
        }],
        load: ({ body }) => body,
      })
      .line({
        body: { id: "task:2", title: "Second" },
      })
      .mutationResponse();

    assert.deepEqual(line.value(), {
      entities: {
        "task:1": { id: "task:1", title: "First" },
        "task:2": { id: "task:2", title: "Second" },
      },
    });
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionInsert");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "patch");
    assert.equal(plan.executionArtifacts[0].deliveryScope, "item");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "entity-id",
      lookupBreadth: 1,
      traversal: "whole-entity-record",
      traversalBreadth: 1,
      reconstruction: "replaceEntities",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("create responses can insert map-backed items through declared prepend placement", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const mappedTasks = runtime.signals.api({}).url("/mapped-tasks")
      .response(runtime.signals.resource.response.map()({
        itemId: (task) => task.id,
        entries: (value) => new Map(value.taskMapEntries),
        replaceEntries: (value, entries) => ({ ...value, taskMapEntries: [...entries] }),
        replaceEntry: (value, itemId, nextItem) => ({
          ...value,
          taskMapEntries: replaceMapEntry(value.taskMapEntries, itemId, nextItem, true),
        }),
      }))
      .list({
        load: () => ({
          taskMapEntries: [["task:1", { id: "task:1", title: "First" }]],
        }),
      });
    const line = mappedTasks.line({});

    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: mappedTasks,
          params: () => ({}),
          fallback: "placementUnavailable",
          collection: { kind: "insert", placement: "prepend" },
        }],
        load: ({ body }) => body,
      })
      .line({
        body: { id: "task:0", title: "Zeroth" },
      })
      .mutationResponse();

    assert.deepEqual(
      line.value().taskMapEntries,
      [
        ["task:0", { id: "task:0", title: "Zeroth" }],
        ["task:1", { id: "task:1", title: "First" }],
      ],
    );
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionInsert");
    assert.equal(plan.executionArtifacts[0].itemId, "task:0");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "map-key",
      lookupBreadth: 1,
      traversal: "whole-map",
      traversalBreadth: 1,
      reconstruction: "replaceEntries",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

function replaceMapEntry(taskMapEntries, itemId, nextItem, prepend) {
  const nextEntries = prepend
    ? [[itemId, nextItem], ...taskMapEntries.filter(([key]) => key !== itemId)]
    : [...taskMapEntries.filter(([key]) => key !== itemId), [itemId, nextItem]];
  return nextEntries;
}
