import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";
import { createBranchHead } from "../../../runtime_fixture/real_resource_signals.mjs";

test("detail field responses close narrow field effect topology proof", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    createBranchHead(signals, "detail-field-response-closeout");
    const response = signals.resource.response.detail()({
      name: "name",
    });
    const users = createDetailApi(signals, response, {
      effects: signals.resource.effects.branchNative(),
    });
    const line = users.line({ userId: "user:1" });

    await line.patch(users.patch.field({
      field: "name",
      value: "Local",
    }));
    const localEffect = line.diagnostics().lastEffect;

    assert.deepEqual(line.value(), { id: "user:1", name: "Local" });
    assert.deepEqual(localEffect.locus, { kind: "detailField", field: "name" });
    assert.equal(localEffect.locusProof.lensSource, "resource.response.detail<T>()");
    assert.equal(localEffect.locusProof.topology, "detail");
    assert.equal(localEffect.locusProof.locus, "detailField");
    assert.equal(localEffect.locusProof.patchScope, "field");
    assert.equal(localEffect.locusProof.field, "name");
    assert.deepEqual(localEffect.locusProof.cost, {
      lookup: "detail-field",
      lookupBreadth: 1,
      traversal: "single-top-level-field",
      traversalBreadth: 1,
      reconstruction: "replaceDetailField",
      reconstructionBreadth: 1,
    });

    line.deliver(users.delivery.field({
      packetId: "pkt-detail-field",
      basisId: null,
      nextBasisId: "basis-1",
      field: "name",
      value: "Delivered",
    }));
    const deliveryEffect = line.diagnostics().lastEffect;
    assert.deepEqual(deliveryEffect.locus, { kind: "detailField", field: "name" });
    assert.equal(deliveryEffect.locusProof.locus, "detailField");
    assert.equal(deliveryEffect.locusProof.field, "name");
    assert.deepEqual(line.value(), { id: "user:1", name: "Delivered" });

    assertNarrowPatchDeniedWithoutSideEffects(
      line,
      () => line.patch(signalsMod.resourcePatch.field({
        field: "title",
        value: "Illegal",
      })),
      {
        message: /undeclared detail field "title"/,
        requestedLocus: "detailField",
        reason: "undeclaredField",
      },
    );
  } finally {
    await runtime.cleanup();
  }
});

test("detail JSON path responses close narrow JSON path effect topology proof", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    createBranchHead(signals, "detail-json-path-response-closeout");
    const response = signals.resource.response.detailJsonPaths()({
      title: { path: ["document", "title"] },
    });
    const workflow = signals.api({
      effects: signals.resource.effects.branchNative(),
    }).url("/workflow-closeout/:workflowId")
      .response(response)
      .detail({
        load: ({ workflowId }) => ({
          id: workflowId,
          document: { title: "First" },
        }),
      });
    const line = workflow.line({ workflowId: "workflow:1" });

    await line.patch(workflow.patch.jsonPath({
      path: "title",
      value: "Local",
    }));
    const localEffect = line.diagnostics().lastEffect;

    assert.deepEqual(line.value(), {
      id: "workflow:1",
      document: { title: "Local" },
    });
    assert.deepEqual(localEffect.locus, { kind: "detailJsonPath", path: "title" });
    assert.equal(localEffect.locusProof.lensSource, "resource.response.detailJsonPaths<T>()");
    assert.equal(localEffect.locusProof.topology, "detail");
    assert.equal(localEffect.locusProof.locus, "detailJsonPath");
    assert.equal(localEffect.locusProof.patchScope, "jsonPath");
    assert.equal(localEffect.locusProof.path, "title");
    assert.deepEqual(localEffect.locusProof.cost, {
      lookup: "detail-json-path",
      lookupBreadth: 1,
      traversal: "json-path-segments",
      traversalBreadth: 1,
      reconstruction: "replaceDetailJsonPath",
      reconstructionBreadth: 1,
    });

    line.deliver(workflow.delivery.jsonPath({
      packetId: "pkt-detail-json-path",
      basisId: null,
      nextBasisId: "basis-1",
      path: "title",
      value: "Delivered",
    }));
    const deliveryEffect = line.diagnostics().lastEffect;
    assert.deepEqual(deliveryEffect.locus, { kind: "detailJsonPath", path: "title" });
    assert.equal(deliveryEffect.locusProof.locus, "detailJsonPath");
    assert.equal(deliveryEffect.locusProof.path, "title");
    assert.deepEqual(line.value(), {
      id: "workflow:1",
      document: { title: "Delivered" },
    });

    assertNarrowPatchDeniedWithoutSideEffects(
      line,
      () => line.patch(signalsMod.resourcePatch.jsonPath({
        path: "subtitle",
        value: "Illegal",
      })),
      {
        message: /undeclared detail JSON path "subtitle"/,
        requestedLocus: "detailJsonPath",
        reason: "undeclaredJsonPath",
      },
    );
  } finally {
    await runtime.cleanup();
  }
});

test("detail region responses close narrow region effect topology proof", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    createBranchHead(signals, "detail-region-response-closeout");
    const response = signals.resource.response.detailRegions()({
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
    const workflow = signals.api({
      effects: signals.resource.effects.branchNative(),
    }).url("/workflow-regions/:workflowId")
      .response(response)
      .detail({
        load: ({ workflowId }) => ({
          id: workflowId,
          graph: { nodes: [{ id: "n1" }] },
        }),
      });
    const line = workflow.line({ workflowId: "workflow:1" });

    await line.patch(workflow.patch.region({
      region: "graph",
      value: { nodes: [{ id: "n2" }] },
    }));
    const localEffect = line.diagnostics().lastEffect;

    assert.deepEqual(line.value(), {
      id: "workflow:1",
      graph: { nodes: [{ id: "n2" }] },
    });
    assert.deepEqual(localEffect.locus, { kind: "detailRegion", region: "graph" });
    assert.equal(localEffect.locusProof.lensSource, "resource.response.detailRegions<T>()");
    assert.equal(localEffect.locusProof.topology, "detail");
    assert.equal(localEffect.locusProof.locus, "detailRegion");
    assert.equal(localEffect.locusProof.patchScope, "region");
    assert.equal(localEffect.locusProof.region, "graph");
    assert.deepEqual(localEffect.patch.region, {
      version: "resource-detail-region-proof-v1",
      regionName: "graph",
      identityBoundary: "outside",
      mergeGranularity: "region-subtree",
      cost: {
        traversalBreadth: 2,
        reconstructionBreadth: 2,
        cloneBreadth: 2,
      },
      proofDigest: localEffect.patch.region.proofDigest,
    });
    assert.equal(localEffect.patch.regionName, "graph");
    assert.equal(localEffect.counters.detailRegionTraversalBreadth, 2);
    assert.equal(localEffect.counters.detailRegionReconstructionBreadth, 2);
    assert.deepEqual(localEffect.locusProof.cost, {
      lookup: "detail-region",
      lookupBreadth: 1,
      traversal: "declared-region",
      traversalBreadth: 1,
      reconstruction: "replaceDetailRegion",
      reconstructionBreadth: 1,
    });

    line.deliver(workflow.delivery.region({
      packetId: "pkt-detail-region",
      basisId: null,
      nextBasisId: "basis-1",
      region: "graph",
      value: { nodes: [{ id: "n3" }] },
    }));
    const deliveryEffect = line.diagnostics().lastEffect;
    assert.deepEqual(deliveryEffect.locus, { kind: "detailRegion", region: "graph" });
    assert.equal(deliveryEffect.locusProof.locus, "detailRegion");
    assert.equal(deliveryEffect.locusProof.region, "graph");
    assert.equal(deliveryEffect.patch.regionName, "graph");
    assert.equal(deliveryEffect.patch.region.regionName, "graph");
    assert.deepEqual(line.value(), {
      id: "workflow:1",
      graph: { nodes: [{ id: "n3" }] },
    });

    assertNarrowPatchDeniedWithoutSideEffects(
      line,
      () => line.patch(signalsMod.resourcePatch.region({
        region: "metadata",
        value: { ok: false },
      })),
      {
        message: /undeclared detail region "metadata"/,
        requestedLocus: "detailRegion",
        reason: "undeclaredRegion",
      },
    );
  } finally {
    await runtime.cleanup();
  }
});

function createDetailApi(signals, response, apiOptions = {}) {
  return signals.api({
    effects: signals.resource.effects.pessimistic(),
    ...apiOptions,
  }).url("/detail-closeout/:userId")
    .response(response)
    .detail({
      load: ({ userId }) => ({ id: userId, name: "First" }),
    });
}

function assertNarrowPatchDeniedWithoutSideEffects(
  line,
  patchAction,
  expectedDenial,
) {
  const beforeValue = line.value();
  const beforeEffect = line.diagnostics().lastEffect;

  assert.throws(patchAction, (error) => {
    assert.match(error.message, expectedDenial.message);
    assert.equal(error.denialProof.requestedLocus, expectedDenial.requestedLocus);
    assert.equal(error.denialProof.reason, expectedDenial.reason);
    return true;
  });
  assert.deepEqual(line.value(), beforeValue);
  assert.deepEqual(line.diagnostics().lastEffect, beforeEffect);
}
