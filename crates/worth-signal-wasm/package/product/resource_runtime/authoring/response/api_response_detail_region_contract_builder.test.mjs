import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("detail region response contracts expose detail family helpers and narrow reconciliation truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const response = runtime.signals.resource.response.detailRegions()({
      graph: {
        read: (value) => value.graph,
        write: (value, graph) => ({ ...value, graph }),
        identityBoundary: "outside",
        mergeGranularity: "region-subtree",
        cost: {
          traversalBreadth: 2,
          reconstructionBreadth: 2,
        },
      },
    });
    assert.equal(response.kind, "detail");
    assert.deepEqual(response.lensProof.regionNames, ["graph"]);
    assert.equal(
      response.lensProof.capabilityRows.some(
        (row) => row.locus === "detailRegion" && row.patchScope === "region",
      ),
      true,
    );

    const workflow = runtime.signals.api({}).url("/workflows/:workflowId").response(response).detail({
      load: ({ workflowId }) => ({
        id: workflowId,
        graph: { nodes: [{ id: "n1" }] },
      }),
    });
    const regionPatch = workflow.patch.region({
      region: "graph",
      value: { nodes: [{ id: "n2" }] },
    });
    const regionDelivery = workflow.delivery.region({
      packetId: "pkt-workflow-graph",
      basisId: null,
      nextBasisId: "basis-1",
      region: "graph",
      value: { nodes: [{ id: "n3" }] },
    });
    const line = workflow.line({ workflowId: "wf-1" });

    assert.equal(regionPatch.kind, "region");
    assert.equal(regionDelivery.patch.kind, "region");
    assert.equal(line.reconciliation().narrowRegion, true);
    assert.deepEqual(line.reconciliation().regionNames, ["graph"]);

    line.patch(regionPatch);
    assert.deepEqual(line.value(), {
      id: "wf-1",
      graph: { nodes: [{ id: "n2" }] },
    });

    line.deliver(regionDelivery);
    assert.deepEqual(line.value(), {
      id: "wf-1",
      graph: { nodes: [{ id: "n3" }] },
    });
  } finally {
    await runtime.cleanup();
  }
});

test("detail region declarations reject accessor-backed declaration maps without invoking them", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    let getterReadCount = 0;
    const declarations = {};
    Object.defineProperty(declarations, "graph", {
      enumerable: true,
      get() {
        getterReadCount += 1;
        return null;
      },
    });

    assert.throws(
      () => runtime.signals.resource.response.detailRegions()(declarations),
      /rejects accessor detail region declaration "graph"/,
    );
    assert.equal(getterReadCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("detail region declarations require explicit metadata", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    assert.throws(
      () =>
        runtime.signals.resource.response.detailRegions()({
          graph: {
            read: (value) => value.graph,
            write: (value, graph) => ({ ...value, graph }),
            identityBoundary: "outside",
            mergeGranularity: "",
            cost: {
              traversalBreadth: 0,
              reconstructionBreadth: 1,
            },
          },
        }),
      /requires non-empty mergeGranularity|requires positive safe integer traversalBreadth/,
    );
  } finally {
    await runtime.cleanup();
  }
});
