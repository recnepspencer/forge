import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import {
  assertLineStateUnchanged,
  captureLineState,
} from "./reconciliation_proof_helpers.mjs";

test("connection route families admit insert patch and delivery with whole-edge reconstruction proof", async () => {
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

    line.patch(connectedTasks.patch.insert({
      itemId: "task:2",
      placement: "append",
      nextItem: { id: "task:2", title: "Second" },
    }));
    line.deliver(connectedTasks.delivery.insert({
      packetId: "pkt-connection-insert",
      basisId: null,
      nextBasisId: "basis-1",
      itemId: "task:0",
      placement: "prepend",
      nextItem: { id: "task:0", title: "Zeroth" },
    }));

    assert.deepEqual(
      line.value().edges.map((edge) => edge.node),
      [
        { id: "task:0", title: "Zeroth" },
        { id: "task:1", title: "First" },
        { id: "task:2", title: "Second" },
      ],
    );
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "connection-edge-item-id",
      lookupBreadth: 1,
      traversal: "whole-connection-edges",
      traversalBreadth: 1,
      reconstruction: "replaceNodes",
      reconstructionBreadth: 1,
    });
    assert.equal(line.diagnostics().lastPatchedItemId, "task:0");
  } finally {
    await runtime.cleanup();
  }
});

test("discriminated tuple route families admit insert patch through active variant reconstruction", async () => {
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

    line.patch(tupleTasks.patch.insert({
      itemId: "task:2",
      placement: "append",
      nextItem: { id: "task:2", title: "Second" },
    }));

    assert.deepEqual(line.value(), {
      kind: "primary",
      primary: [
        { id: "task:1", title: "First" },
        { id: "task:2", title: "Second" },
      ],
      secondary: [],
    });
    assert.deepEqual(line.diagnostics().lastEffect.locusProof.cost, {
      lookup: "tuple-discriminator-item-id",
      lookupBreadth: 1,
      traversal: "active-variant-items",
      traversalBreadth: 1,
      reconstruction: "replaceVariantItems",
      reconstructionBreadth: 1,
    });
    assert.equal(line.diagnostics().lastPatchKind, "insert");
    assert.equal(line.diagnostics().lastPatchedItemId, "task:2");
  } finally {
    await runtime.cleanup();
  }
});
