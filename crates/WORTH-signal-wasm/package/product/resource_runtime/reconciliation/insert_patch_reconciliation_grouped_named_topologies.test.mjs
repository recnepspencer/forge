import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("grouped route families admit insert patch while preserving empty sibling groups", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const groupedTasks = runtime.signals.api({}).url("/grouped-tasks")
      .response(runtime.signals.resource.response.grouped()({
        itemId: (task) => task.id,
        groupId: (task) => task.group,
        groupForItem: () => "todo",
        groups: (value) => value.groups,
        replaceGroups: (value, groups) => ({ ...value, groups }),
        replaceGroupItem: (value, groupId, itemId, nextItem) => ({
          ...value,
          groups: Object.fromEntries(
            Object.entries(value.groups).map(([key, items]) => [
              key,
              key === groupId
                ? items.map((item) => item.id === itemId ? nextItem : item)
                : items,
            ]),
          ),
        }),
      }))
      .list({
        load: () => ({
          groups: {
            todo: [{ id: "task:1", group: "todo", title: "First" }],
            done: [],
          },
        }),
      });
    const line = groupedTasks.line({});

    line.patch(groupedTasks.patch.insert({
      itemId: "task:2",
      placement: "append",
      nextItem: { id: "task:2", group: "todo", title: "Second" },
    }));

    assert.deepEqual(line.value(), {
      groups: {
        todo: [
          { id: "task:1", group: "todo", title: "First" },
          { id: "task:2", group: "todo", title: "Second" },
        ],
        done: [],
      },
    });
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "group-key-item-id",
      lookupBreadth: 1,
      traversal: "single-group",
      traversalBreadth: 1,
      reconstruction: "replaceGroups",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("named route families admit insert patch while preserving empty sibling collections", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const namedTasks = runtime.signals.api({}).url("/named-tasks")
      .response(runtime.signals.resource.response.named()({
        itemId: (task) => task.id,
        collectionId: (task) => task.collection,
        collectionForItem: () => "backlog",
        collections: (value) => value.collections,
        replaceCollections: (value, collections) => ({ ...value, collections }),
        replaceCollectionItem: (value, collectionId, itemId, nextItem) => ({
          ...value,
          collections: Object.fromEntries(
            Object.entries(value.collections).map(([key, items]) => [
              key,
              key === collectionId
                ? items.map((item) => item.id === itemId ? nextItem : item)
                : items,
            ]),
          ),
        }),
      }))
      .list({
        load: () => ({
          collections: {
            backlog: [{ id: "task:1", collection: "backlog", title: "First" }],
            active: [],
          },
        }),
      });
    const line = namedTasks.line({});

    line.patch(namedTasks.patch.insert({
      itemId: "task:0",
      placement: "prepend",
      nextItem: { id: "task:0", collection: "backlog", title: "Zeroth" },
    }));

    assert.deepEqual(line.value(), {
      collections: {
        backlog: [
          { id: "task:0", collection: "backlog", title: "Zeroth" },
          { id: "task:1", collection: "backlog", title: "First" },
        ],
        active: [],
      },
    });
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "collection-key-item-id",
      lookupBreadth: 1,
      traversal: "single-named-collection",
      traversalBreadth: 1,
      reconstruction: "replaceCollections",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("named route families deny insert when lookup collection disagrees with nextItem collection", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const namedTasks = runtime.signals.api({}).url("/named-tasks")
      .response(runtime.signals.resource.response.named()({
        itemId: (task) => task.id,
        collectionId: (task) => task.collection,
        collectionForItem: (itemId) => itemId === "task:0" ? "active" : "backlog",
        collections: (value) => value.collections,
        replaceCollections: (value, collections) => ({ ...value, collections }),
        replaceCollectionItem: (value, collectionId, itemId, nextItem) => ({
          ...value,
          collections: Object.fromEntries(
            Object.entries(value.collections).map(([key, items]) => [
              key,
              key === collectionId
                ? items.map((item) => item.id === itemId ? nextItem : item)
                : items,
            ]),
          ),
        }),
      }))
      .list({
        load: () => ({
          collections: {
            backlog: [{ id: "task:1", collection: "backlog", title: "First" }],
            active: [],
          },
        }),
      });
    const line = namedTasks.line({});

    assert.throws(
      () => line.patch(namedTasks.patch.insert({
        itemId: "task:0",
        placement: "prepend",
        nextItem: { id: "task:0", collection: "backlog", title: "Zeroth" },
      })),
      /nextItem collection id "backlog" to match named lookup collection id "active"/,
    );

    assert.deepEqual(line.value(), {
      collections: {
        backlog: [{ id: "task:1", collection: "backlog", title: "First" }],
        active: [],
      },
    });
  } finally {
    await runtime.cleanup();
  }
});
