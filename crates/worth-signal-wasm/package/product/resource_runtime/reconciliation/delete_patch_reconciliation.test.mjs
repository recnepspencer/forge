import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import {
  assertLineStateUnchanged,
  captureLineState,
} from "./reconciliation_proof_helpers.mjs";

test("direct-array route families admit delete patch and delivery with narrow item scope", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const tasks = runtime.signals.api({}).url("/tasks")
      .items((item) => item.id)
      .list({
        load: () => [
          { id: "t1", title: "First" },
          { id: "t2", title: "Second" },
        ],
      });
    const line = tasks.line({});

    const patchResult = line.patch(tasks.patch.delete({ itemId: "t1" }));

    assert.deepEqual(patchResult, {
      kind: "narrowed",
      scope: "item",
      itemId: "t1",
      aspect: null,
      field: null,
    });
    assert.deepEqual(line.value(), [{ id: "t2", title: "Second" }]);
    assert.equal(line.diagnostics().lastPatchKind, "delete");
    assert.equal(line.diagnostics().lastPatchScope, "item");
    assert.equal(line.diagnostics().lastPatchedItemId, "t1");

    const deliveryResult = line.deliver(
      tasks.delivery.delete({
        packetId: "pkt-delete",
        basisId: null,
        nextBasisId: "basis-1",
        itemId: "t2",
      }),
    );

    assert.deepEqual(deliveryResult, {
      kind: "applied",
      deliveryKind: "patch",
      scope: "item",
      packetId: "pkt-delete",
      basisId: null,
      nextBasisId: "basis-1",
      supersededOperation: null,
    });
    assert.deepEqual(line.value(), []);
    assert.equal(line.diagnostics().lastDeliveryKind, "patch");
    assert.equal(line.diagnostics().lastDeliveryScope, "item");
    assert.equal(line.diagnostics().lastPatchedItemId, "t2");
    assert.equal(line.history().lifecycle.at(-1)?.event, "delivered");
  } finally {
    await runtime.cleanup();
  }
});

test("delete patch denies missing collection item ids without side effects", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const tasks = runtime.signals.api({}).url("/tasks")
      .items((item) => item.id)
      .list({
        load: () => [{ id: "t1", title: "First" }],
      });
    const line = tasks.line({});
    const before = captureLineState(line);

    assert.throws(
      () => line.patch(tasks.patch.delete({ itemId: "t9" })),
      /could not find itemId "t9" for patch\(\.\.\.\)/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await runtime.cleanup();
  }
});

test("grouped and named delete patch preserves empty sibling buckets and certifies bucket-scoped cost", async () => {
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
            todo: [
              { id: "task:1", group: "todo", title: "First" },
              { id: "task:2", group: "todo", title: "Second" },
            ],
            done: [],
          },
        }),
      });
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
            backlog: [
              { id: "task:1", collection: "backlog", title: "First" },
              { id: "task:2", collection: "backlog", title: "Second" },
            ],
            active: [],
          },
        }),
      });

    groupedTasks.line({}).patch(groupedTasks.patch.delete({ itemId: "task:1" }));
    namedTasks.line({}).patch(namedTasks.patch.delete({ itemId: "task:1" }));

    assert.deepEqual(groupedTasks.line({}).value(), {
      groups: {
        todo: [{ id: "task:2", group: "todo", title: "Second" }],
        done: [],
      },
    });
    assert.deepEqual(groupedTasks.line({}).diagnostics().lastEffect.locusProof.cost, {
      lookup: "group-key-item-id",
      lookupBreadth: 1,
      traversal: "single-group",
      traversalBreadth: 1,
      reconstruction: "replaceGroups",
      reconstructionBreadth: 1,
    });
    assert.equal(
      groupedTasks.line({}).diagnostics().lastEffect.locusProof.effectLocusDigest.includes("delete"),
      true,
    );

    assert.deepEqual(namedTasks.line({}).value(), {
      collections: {
        backlog: [{ id: "task:2", collection: "backlog", title: "Second" }],
        active: [],
      },
    });
    assert.deepEqual(namedTasks.line({}).diagnostics().lastEffect.locusProof.cost, {
      lookup: "collection-key-item-id",
      lookupBreadth: 1,
      traversal: "single-named-collection",
      traversalBreadth: 1,
      reconstruction: "replaceCollections",
      reconstructionBreadth: 1,
    });
    assert.equal(
      namedTasks.line({}).diagnostics().lastEffect.locusProof.effectLocusDigest.includes("delete"),
      true,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("grouped and named delete patch deny lookup bucket mismatch without side effects", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const groupedTasks = runtime.signals.api({}).url("/grouped-tasks")
      .response(runtime.signals.resource.response.grouped()({
        itemId: (task) => task.id,
        groupId: (task) => task.group,
        groupForItem: (itemId) => itemId === "task:1" ? "done" : "todo",
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
    const namedTasks = runtime.signals.api({}).url("/named-tasks")
      .response(runtime.signals.resource.response.named()({
        itemId: (task) => task.id,
        collectionId: (task) => task.collection,
        collectionForItem: (itemId) => itemId === "task:1" ? "active" : "backlog",
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
    const groupedLine = groupedTasks.line({});
    const namedLine = namedTasks.line({});
    const groupedBefore = captureLineState(groupedLine);
    const namedBefore = captureLineState(namedLine);

    assert.throws(
      () => groupedLine.patch(groupedTasks.patch.delete({ itemId: "task:1" })),
      /grouped lookup group id "done" to match actual item group id "todo"/,
    );
    assert.throws(
      () => namedLine.patch(namedTasks.patch.delete({ itemId: "task:1" })),
      /named lookup collection id "active" to match actual item collection id "backlog"/,
    );

    assertLineStateUnchanged(groupedLine, groupedBefore);
    assertLineStateUnchanged(namedLine, namedBefore);
  } finally {
    await runtime.cleanup();
  }
});
