import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("entity-store route families admit insert patch and delivery with keyed replacement semantics", async () => {
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

    line.patch(entityTasks.patch.insert({
      itemId: "task:2",
      placement: "append",
      nextItem: { id: "task:2", title: "Second" },
    }));
    line.deliver(entityTasks.delivery.insert({
      packetId: "pkt-entity-insert",
      basisId: null,
      nextBasisId: "basis-1",
      itemId: "task:0",
      placement: "prepend",
      nextItem: { id: "task:0", title: "Zeroth" },
    }));

    assert.deepEqual(line.value(), {
      entities: {
        "task:1": { id: "task:1", title: "First" },
        "task:2": { id: "task:2", title: "Second" },
        "task:0": { id: "task:0", title: "Zeroth" },
      },
    });
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "entity-id",
      lookupBreadth: 1,
      traversal: "whole-entity-record",
      traversalBreadth: 1,
      reconstruction: "replaceEntities",
      reconstructionBreadth: 1,
    });
    assert.equal(line.diagnostics().lastPatchedItemId, "task:0");
    assert.equal(line.history().lifecycle.at(-1)?.event, "delivered");
  } finally {
    await runtime.cleanup();
  }
});

test("map-backed route families admit insert patch through keyed entry replacement semantics", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const mappedTasks = runtime.signals.api({}).url("/mapped-tasks")
      .response(runtime.signals.resource.response.map()({
        itemId: (task) => task.id,
        entries: (value) => new Map(value.taskMapEntries),
        replaceEntries: (value, entries) => ({ ...value, taskMapEntries: [...entries] }),
        replaceEntry: (value, itemId, nextItem) => ({
          ...value,
          taskMapEntries: replaceMapEntry(value.taskMapEntries, itemId, nextItem, false),
        }),
      }))
      .list({
        load: () => ({
          taskMapEntries: [["task:1", { id: "task:1", title: "First" }]],
        }),
      });
    const line = mappedTasks.line({});

    line.patch(mappedTasks.patch.insert({
      itemId: "task:2",
      placement: "append",
      nextItem: { id: "task:2", title: "Second" },
    }));

    assert.deepEqual(
      line.value().taskMapEntries,
      [
        ["task:1", { id: "task:1", title: "First" }],
        ["task:2", { id: "task:2", title: "Second" }],
      ],
    );
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "map-key",
      lookupBreadth: 1,
      traversal: "whole-map",
      traversalBreadth: 1,
      reconstruction: "replaceEntries",
      reconstructionBreadth: 1,
    });
    assert.equal(line.diagnostics().lastPatchKind, "insert");
    assert.equal(line.diagnostics().lastPatchedItemId, "task:2");
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
