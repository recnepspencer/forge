import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import {
  assertLineStateUnchanged,
  captureLineState,
} from "./reconciliation_proof_helpers.mjs";

test("direct-array route families admit insert patch and delivery with narrow item scope", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signalsMod } = runtime;
    const tasks = runtime.signals.api({}).url("/tasks")
      .items((item) => item.id)
      .list({
        load: () => [{ id: "t1", title: "First" }],
      });
    const line = tasks.line({});

    const patchResult = line.patch(tasks.patch.insert({
      itemId: "t2",
      placement: "append",
      nextItem: { id: "t2", title: "Second" },
    }));

    assert.deepEqual(patchResult, {
      kind: "narrowed",
      scope: "item",
      itemId: "t2",
      aspect: null,
      field: null,
    });
    assert.deepEqual(line.value(), [
      { id: "t1", title: "First" },
      { id: "t2", title: "Second" },
    ]);
    assert.equal(line.diagnostics().lastPatchKind, "insert");
    assert.equal(line.diagnostics().lastPatchScope, "item");
    assert.equal(line.diagnostics().lastPatchedItemId, "t2");

    const deliveryResult = line.deliver(
      tasks.delivery.insert({
        packetId: "pkt-insert",
        basisId: null,
        nextBasisId: "basis-1",
        itemId: "t0",
        placement: "prepend",
        nextItem: { id: "t0", title: "Zeroth" },
      }),
    );

    assert.deepEqual(deliveryResult, {
      kind: "applied",
      deliveryKind: "patch",
      scope: "item",
      packetId: "pkt-insert",
      basisId: null,
      nextBasisId: "basis-1",
      supersededOperation: null,
    });
    assert.deepEqual(line.value(), [
      { id: "t0", title: "Zeroth" },
      { id: "t1", title: "First" },
      { id: "t2", title: "Second" },
    ]);
    assert.equal(line.diagnostics().lastDeliveryKind, "patch");
    assert.equal(line.diagnostics().lastDeliveryScope, "item");
    assert.equal(line.diagnostics().lastPatchedItemId, "t0");
    assert.equal(line.history().lifecycle.at(-1)?.event, "delivered");
  } finally {
    await runtime.cleanup();
  }
});

test("insert patch denies duplicate collection item ids without side effects", async () => {
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
      () =>
        line.patch(tasks.patch.insert({
          itemId: "t1",
          placement: "append",
          nextItem: { id: "t1", title: "Duplicate" },
        })),
      /resourcePatch\.insert\(\.\.\.\) for duplicate itemId "t1"/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await runtime.cleanup();
  }
});

test("sparse-page route families admit exact insert patch and deny page-mismatched inserts without side effects", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const tasks = runtime.signals.api({}).url("/sparse-tasks")
      .response(runtime.signals.resource.response.sparse()({
        itemId: (task) => task.id,
        pageId: (task) => task.page,
        pageForItem: () => "page-1",
        pages: (value) => value.pages,
        replacePages: (value, nextPages) => ({ ...value, pages: nextPages }),
        replacePageItem: (value, pageId, itemId, nextItem) => ({
          ...value,
          pages: Object.fromEntries(
            Object.entries(value.pages).map(([key, items]) => [
              key,
              key === pageId
                ? items.map((item) => item.id === itemId ? nextItem : item)
                : items,
            ]),
          ),
        }),
      }))
      .list({
        load: () => ({
          pages: {
            "page-1": [{ id: "t1", page: "page-1", title: "First" }],
            "page-2": [{ id: "t9", page: "page-2", title: "Sibling" }],
          },
        }),
      });
    const line = tasks.line({});

    const patchResult = line.patch(tasks.patch.insert({
      itemId: "t2",
      placement: "append",
      nextItem: { id: "t2", page: "page-1", title: "Second" },
    }));

    assert.deepEqual(patchResult, {
      kind: "narrowed",
      scope: "item",
      itemId: "t2",
      aspect: null,
      field: null,
    });
    assert.deepEqual(line.value(), {
      pages: {
        "page-1": [
          { id: "t1", page: "page-1", title: "First" },
          { id: "t2", page: "page-1", title: "Second" },
        ],
        "page-2": [{ id: "t9", page: "page-2", title: "Sibling" }],
      },
    });
    assert.equal(line.diagnostics().lastPatchKind, "insert");
    assert.equal(line.diagnostics().lastPatchScope, "item");
    assert.equal(line.diagnostics().lastPatchedItemId, "t2");

    const before = captureLineState(line);
    assert.throws(
      () =>
        line.patch(tasks.patch.insert({
          itemId: "t3",
          placement: "append",
          nextItem: { id: "t3", page: "page-2", title: "Wrong Page" },
        })),
      /nextItem page id "page-2" to match sparse page lookup page id "page-1" for itemId "t3"/,
    );
    assertLineStateUnchanged(line, before);
  } finally {
    await runtime.cleanup();
  }
});

test("insert patch denies identity-mismatched next items without side effects", async () => {
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
      () =>
        line.patch(tasks.patch.insert({
          itemId: "t2",
          placement: "append",
          nextItem: { id: "t3", title: "Wrong Identity" },
        })),
      /resourcePatch\.insert\(\.\.\.\) to preserve item identity "t2"/,
    );

    assertLineStateUnchanged(line, before);
  } finally {
    await runtime.cleanup();
  }
});
