import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("remove responses can delete connection and discriminated tuple items through exact collection deletion", async () => {
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
          edges: [
            { cursor: "cursor:0", node: { id: "task:1", title: "First" } },
            { cursor: "cursor:1", node: { id: "task:2", title: "Second" } },
          ],
        }),
      });
    const tupleTasks = runtime.signals.api({}).url("/tuple-tasks")
      .response(runtime.signals.resource.response.discriminated()({
        itemId: (task) => task.id,
        discriminator: (value) => value.kind,
        variants: {
          primary: {
            items: (value) => value.primary,
            replaceItems: (value, nextItems) => ({ ...value, primary: [...nextItems] }),
          },
          secondary: {
            items: (value) => value.secondary,
            replaceItems: (value, nextItems) => ({ ...value, secondary: [...nextItems] }),
          },
        },
      }))
      .list({
        load: () => ({
          kind: "primary",
          primary: [
            { id: "task:1", title: "First" },
            { id: "task:2", title: "Second" },
          ],
          secondary: [],
        }),
      });
    const connectedLine = connectedTasks.line({});
    const tupleLine = tupleTasks.line({});

    const connectedPlan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: connectedTasks,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "task:1" })
      .mutationResponse();
    const tuplePlan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: tupleTasks,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "task:1" })
      .mutationResponse();

    assert.deepEqual(
      connectedLine.value().edges.map((edge) => edge.node),
      [{ id: "task:2", title: "Second" }],
    );
    assert.equal(connectedPlan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(connectedLine.diagnostics().lastEffect.locusProof.cost, {
      lookup: "connection-edge-item-id",
      lookupBreadth: 1,
      traversal: "whole-connection-edges",
      traversalBreadth: 1,
      reconstruction: "replaceNodes",
      reconstructionBreadth: 1,
    });

    assert.deepEqual(tupleLine.value(), {
      kind: "primary",
      primary: [{ id: "task:2", title: "Second" }],
      secondary: [],
    });
    assert.equal(tuplePlan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(tupleLine.diagnostics().lastEffect.locusProof.cost, {
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

test("remove responses can delete entity-store and map-backed items through exact collection deletion", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const entityTasks = runtime.signals.api({}).url("/entity-tasks")
      .response(runtime.signals.resource.response.entityStore()({
        itemId: (task) => task.id,
        entities: (value) => value.entities,
        replaceEntities: (value, nextEntities) => ({ ...value, entities: nextEntities }),
        replaceEntity: (value, itemId, nextItem) => ({
          ...value,
          entities: { ...value.entities, [itemId]: nextItem },
        }),
      }))
      .list({
        load: () => ({
          entities: {
            "task:1": { id: "task:1", title: "First" },
            "task:2": { id: "task:2", title: "Second" },
          },
        }),
      });
    const mappedTasks = runtime.signals.api({}).url("/mapped-tasks")
      .response(runtime.signals.resource.response.map()({
        itemId: (task) => task.id,
        entries: (value) => new Map(value.taskMapEntries),
        replaceEntries: (value, entries) => ({ ...value, taskMapEntries: [...entries] }),
        replaceEntry: (value, itemId, nextItem) => ({
          ...value,
          taskMapEntries: replaceMapEntry(value.taskMapEntries, itemId, nextItem),
        }),
      }))
      .list({
        load: () => ({
          taskMapEntries: [
            ["task:1", { id: "task:1", title: "First" }],
            ["task:2", { id: "task:2", title: "Second" }],
          ],
        }),
      });
    const entityLine = entityTasks.line({});
    const mapLine = mappedTasks.line({});

    const entityPlan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: entityTasks,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "task:1" })
      .mutationResponse();
    const mapPlan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: mappedTasks,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "task:1" })
      .mutationResponse();

    assert.deepEqual(entityLine.value(), {
      entities: {
        "task:2": { id: "task:2", title: "Second" },
      },
    });
    assert.equal(entityPlan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(entityLine.diagnostics().lastEffect.locusProof.cost, {
      lookup: "entity-id",
      lookupBreadth: 1,
      traversal: "whole-entity-record",
      traversalBreadth: 1,
      reconstruction: "replaceEntities",
      reconstructionBreadth: 1,
    });

    assert.deepEqual(mapLine.value().taskMapEntries, [
      ["task:2", { id: "task:2", title: "Second" }],
    ]);
    assert.equal(mapPlan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(mapLine.diagnostics().lastEffect.locusProof.cost, {
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

test("remove responses can delete object-items entries through exact collection deletion", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskEnvelope = runtime.signals.api({}).url("/task-page")
      .response(runtime.signals.resource.response.objectItems()({
        field: "tasks",
        itemId: (item) => item.id,
      }))
      .list({
        load: () => ({
          tasks: [
            { id: "task:1", title: "First", status: "open" },
            { id: "task:2", title: "Second", status: "open" },
          ],
          nextCursor: null,
        }),
      });
    const envelopeLine = taskEnvelope.line({});

    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: taskEnvelope,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "task:1" })
      .mutationResponse();

    assert.deepEqual(envelopeLine.value(), {
      tasks: [{ id: "task:2", title: "Second", status: "open" }],
      nextCursor: null,
    });
    assert.equal(plan.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(envelopeLine.diagnostics().lastEffect.locusProof.cost, {
      lookup: "membership-declaration",
      lookupBreadth: 1,
      traversal: "item-scope",
      traversalBreadth: 1,
      reconstruction: "objectItems-lens",
      reconstructionBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("remove responses can patch detail child regions when the route declares a matching region lens", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskRegions = runtime.signals.resource.detailRegions({
      children: {
        read: (value) => value.children,
        write: (value, children) => ({ ...value, children }),
        identityBoundary: "inside",
        mergeGranularity: "child-list",
        cost: {
          traversalBreadth: 1,
          reconstructionBreadth: 1,
        },
      },
    });
    const taskDetail = runtime.signals.api({}).url("/tasks/:taskId").detail({
      reconcile: taskRegions,
      load: ({ taskId }) => ({
        id: taskId,
        title: "Task",
        children: [
          { id: "child:1", title: "First child" },
          { id: "child:2", title: "Second child" },
        ],
      }),
    });
    const taskLine = taskDetail.line({ taskId: "task-1" });

    const plan = runtime.signals.api({}).url("/tasks/:taskId/children/:childId")
      .response(runtime.signals.resource.response.detailRegions()(taskRegions))
      .remove({
        reconciles: [{
          family: taskDetail,
          params: ({ taskId }) => ({ taskId }),
          fallback: "refetchRequired",
          detail: { kind: "region", region: "children" },
        }],
        load: ({ taskId }) => ({
          id: taskId,
          title: "Task",
          children: [{ id: "child:2", title: "Second child" }],
        }),
      })
      .line({ taskId: "task-1", childId: "child:1" })
      .mutationResponse();

    assert.deepEqual(taskLine.value().children, [
      { id: "child:2", title: "Second child" },
    ]);
    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.executionArtifacts[0].scope, "region");
    assert.equal(taskLine.diagnostics().lastDeliveryScope, "region");
    assert.equal(taskLine.diagnostics().lastPatchedRegion, "children");
    assert.deepEqual(taskLine.diagnostics().lastEffect.patch.region.cost, {
      traversalBreadth: 1,
      reconstructionBreadth: 1,
      cloneBreadth: 1,
    });
  } finally {
    await runtime.cleanup();
  }
});

function replaceMapEntry(taskMapEntries, itemId, nextItem) {
  return taskMapEntries
    .filter(([key]) => key !== itemId)
    .concat(nextItem === null ? [] : [[itemId, nextItem]]);
}
