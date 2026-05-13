import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import {
  assertLineStateUnchanged,
  captureLineState,
  normalizeForProof,
} from "./reconciliation_proof_helpers.mjs";

test("paged lines do not overclaim summary patch admission for line-scoped summaries", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    const feed = resource.paged({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: mod.resourceValueSummaries({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
        }),
      }),
      accumulatePage: (existing, next) => ({
        items: [...existing.items, ...next.items],
        cursor: next.cursor,
        total: next.total,
      }),
      load: ({ workspaceId }) => ({
        items: [{ id: `${workspaceId}:1`, title: "First" }],
        cursor: "next",
        total: 1,
      }),
    });

    const line = feed.line({ workspaceId: "demo" });
    const before = captureLineState(line);

    assert.deepEqual(line.reconciliation(), {
      broadReplace: true,
      narrowItem: true,
      narrowField: false,
      narrowRegion: false,
      narrowJsonPath: false,
      narrowSummary: false,
      fieldNames: [],
      regionNames: [],
      jsonPathNames: [],
      aspectNames: [],
      summaryNames: [],
    });
    assert.throws(
      () =>
        line.patch(
          mod.resourcePatch.summary({
            summary: "total",
            value: 2,
          }),
        ),
      /resourceValueSummaries\.pageWindow/,
    );
    assertLineStateUnchanged(line, before);
  } finally {
    await runtime.cleanup();
  }
});

test("paged lines admit summary patch when page-window summaries are declared explicitly", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    const feed = resource.paged({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: mod.resourceValueSummaries.pageWindow({
          visibleCount: {
            read: (value) => value.visibleCount,
            write: (value, visibleCount) => ({ ...value, visibleCount }),
          },
        }),
      }),
      accumulatePage: (existing, next) => ({
        items: [...existing.items, ...next.items],
        cursor: next.cursor,
        visibleCount: next.visibleCount,
      }),
      load: ({ workspaceId }) => ({
        items: [{ id: `${workspaceId}:1`, title: "First" }],
        cursor: "next",
        visibleCount: 1,
      }),
    });

    const line = feed.line({ workspaceId: "demo" });

    assert.deepEqual(line.reconciliation(), {
      broadReplace: true,
      narrowItem: true,
      narrowField: false,
      narrowRegion: false,
      narrowJsonPath: false,
      narrowSummary: true,
      fieldNames: [],
      regionNames: [],
      jsonPathNames: [],
      aspectNames: [],
      summaryNames: ["visibleCount"],
    });
    const result = line.patch(
      mod.resourcePatch.summary({
        summary: "visibleCount",
        value: 2,
      }),
    );

    assert.deepEqual(result, {
      kind: "narrowed",
      scope: "summary",
      itemId: null,
      aspect: null,
      field: null,
      summary: "visibleCount",
    });
    assert.deepEqual(normalizeForProof(line.value()), {
      items: [{ id: "demo:1", title: "First" }],
      cursor: "next",
      visibleCount: 2,
    });
    assert.equal(line.diagnostics().lastPatchScope, "summary");
    assert.equal(line.diagnostics().lastPatchedSummary, "visibleCount");
    assert.equal(line.history().lifecycle.at(-1)?.lastPatchedSummary, "visibleCount");
  } finally {
    await runtime.cleanup();
  }
});
