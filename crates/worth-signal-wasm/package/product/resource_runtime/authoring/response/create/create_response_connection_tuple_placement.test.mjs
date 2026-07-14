import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("create responses can insert connection items through declared append placement", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const connectedTasks = runtime.signals.api({}).url("/connected-tasks")
      .response(runtime.signals.resource.response.connection()({
        itemId: (task) => task.id,
        edges: (value) => value.edges,
        node: (edge) => edge.node,
        edgeIndexForItem: (value, itemId) => {
          const edgeIndex = value.edges.findIndex((edge) => edge.node.id === itemId);
          return edgeIndex === -1 ? null : edgeIndex;
        },
        replaceNodes: (value, nextNodes) => ({
          ...value,
          edges: nextNodes.map((node, index) => ({
            cursor: `cursor:${index}`,
            node,
          })),
        }),
        replaceNode: (value, itemId, nextNode) => ({
          ...value,
          edges: value.edges.map((edge) => ({
            ...edge,
            node: edge.node.id === itemId ? nextNode : edge.node,
          })),
        }),
      }))
      .list({
        load: () => ({
          edges: [{
            cursor: "cursor:0",
            node: { id: "task:1", title: "First" },
          }],
        }),
      });
    const line = connectedTasks.line({});

    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: connectedTasks,
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

    assert.deepEqual(
      line.value().edges.map((edge) => edge.node),
      [
        { id: "task:1", title: "First" },
        { id: "task:2", title: "Second" },
      ],
    );
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionInsert");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "patch");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "connection-edge-item-id",
      lookupBreadth: 1,
      traversal: "whole-connection-edges",
      traversalBreadth: 1,
      reconstruction: "replaceNodes",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("create responses can insert discriminated tuple items through declared prepend placement", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const tupleTasks = runtime.signals.api({}).url("/tuple-tasks")
      .response(runtime.signals.resource.response.discriminated()({
        itemId: (task) => task.id,
        discriminator: (value) => value.kind,
        variants: {
          primary: {
            items: (value) => value.primary,
            replaceItems: (value, nextItems) => ({
              ...value,
              primary: [...nextItems],
            }),
          },
          secondary: {
            items: (value) => value.secondary,
            replaceItems: (value, nextItems) => ({
              ...value,
              secondary: [...nextItems],
            }),
          },
        },
      }))
      .list({
        load: () => ({
          kind: "primary",
          primary: [{ id: "task:1", title: "First" }],
          secondary: [],
        }),
      });
    const line = tupleTasks.line({});

    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: tupleTasks,
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

    assert.deepEqual(line.value(), {
      kind: "primary",
      primary: [
        { id: "task:0", title: "Zeroth" },
        { id: "task:1", title: "First" },
      ],
      secondary: [],
    });
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionInsert");
    assert.equal(plan.executionArtifacts[0].itemId, "task:0");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "tuple-discriminator-item-id",
      lookupBreadth: 1,
      traversal: "active-variant-items",
      traversalBreadth: 1,
      reconstruction: "replaceVariantItems",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("create responses can insert sparse-page items through declared append placement", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const sparseTasks = runtime.signals.api({}).url("/sparse-tasks")
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
            "page-1": [{ id: "task:1", page: "page-1", title: "First" }],
            "page-2": [{ id: "task:9", page: "page-2", title: "Sibling" }],
          },
        }),
      });
    const line = sparseTasks.line({});

    const plan = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [{
          family: sparseTasks,
          params: () => ({}),
          fallback: "placementUnavailable",
          collection: { kind: "insert", placement: "append" },
        }],
        load: ({ body }) => body,
      })
      .line({
        body: { id: "task:2", page: "page-1", title: "Second" },
      })
      .mutationResponse();

    assert.deepEqual(line.value(), {
      pages: {
        "page-1": [
          { id: "task:1", page: "page-1", title: "First" },
          { id: "task:2", page: "page-1", title: "Second" },
        ],
        "page-2": [{ id: "task:9", page: "page-2", title: "Sibling" }],
      },
    });
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionInsert");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "patch");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "sparse-page-item-id",
      lookupBreadth: 1,
      traversal: "loaded-page-chunk",
      traversalBreadth: 1,
      reconstruction: "replacePages",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});
