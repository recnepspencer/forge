import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("save responses can replace a resident detail line through canonical mutation reconciliation", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const readFamily = runtime.signals.api({}).url("/users/:userId").detail({
      load: ({ userId }) => ({ id: userId, name: "First" }),
    });
    const residentLine = readFamily.line({ userId: "u1" });
    const saveUser = runtime.signals.api({}).url("/users/:userId")
      .response(runtime.signals.resource.response.detail()())
      .update({
        reconciles: [
          {
            family: readFamily,
            params: ({ userId }) => ({ userId }),
            fallback: "refetchRequired",
            detail: { kind: "replace" },
          },
        ],
        load: ({ userId, body }) => ({ id: userId, name: body.name }),
      });

    const saveLine = saveUser.line({
      userId: "u1",
      body: { name: "Updated" },
    });
    const plan = saveLine.mutationResponse();

    assert.deepEqual(residentLine.value(), { id: "u1", name: "Updated" });
    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.executionArtifacts[0].scope, "line");
    assert.equal(plan.executionArtifacts[0].outcomeKind, "applied");
    assert.equal(residentLine.diagnostics().lastDeliveryKind, "replace");
    assert.equal(residentLine.diagnostics().lastDeliveryScope, "line");
    assert.equal(residentLine.diagnostics().lastEffect.provenance, "deliveredReplace");
    assert.equal(
      plan.executionArtifacts[0].effectId,
      residentLine.diagnostics().lastEffect.effectId,
    );
    assert.equal(
      saveLine.history().verificationPackage().mutationResponse.plan.executionArtifacts[0].effectId,
      residentLine.diagnostics().lastEffect.effectId,
    );
    assert.deepEqual(
      saveLine.diagnostics().lastMutationResponsePlan,
      plan,
    );
    assert.deepEqual(
      saveLine.history().lifecycle.at(-1).mutationResponsePlan,
      plan,
    );
    assert.equal(
      saveLine.summary().diagnostics.latest.mutationResponsePlanId,
      plan.planId,
    );
    assert.equal(
      saveLine.summary().diagnostics.latest.mutationResponseTargetCount,
      1,
    );
    assert.equal(
      saveLine.summary().diagnostics.latest.mutationResponseExecutionDigest,
      plan.executionDigest,
    );
    assert.equal(
      "mutationResponseIdentityMigrationDigest"
        in saveLine.summary().diagnostics.latest,
      false,
    );
    assert.equal(
      "mutationResponseIdentityMigrationNeeded"
        in saveLine.summary().diagnostics.latest,
      false,
    );
    assert.equal(
      "identityMigrationCount" in residentLine.summary().diagnostics.latest,
      false,
    );
    assert.equal(
      "lastIdentityMigration" in residentLine.summary().diagnostics.latest,
      false,
    );
    assert.equal(
      "mutationResponsePlan" in saveLine.summary().diagnostics.latest,
      false,
    );
    assert.equal(residentLine.history().lifecycle.at(-1).event, "delivered");
  } finally {
    await runtime.cleanup();
  }
});

test("save responses can patch a resident detail field through canonical mutation reconciliation", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detailFields = runtime.signals.resource.detailFields({
      name: {
        read: (value) => value.name,
        write: (value, name) => ({ ...value, name }),
      },
    });
    const readFamily = runtime.signals.api({}).url("/profiles/:profileId").detail({
      reconcile: detailFields,
      load: ({ profileId }) => ({ id: profileId, name: "First" }),
    });
    const residentLine = readFamily.line({ profileId: "p1" });
    const saveProfile = runtime.signals.api({}).url("/profiles/:profileId")
      .response(runtime.signals.resource.response.detail()({ name: "name" }))
      .update({
        reconciles: [
          {
            family: readFamily,
            params: ({ profileId }) => ({ profileId }),
            fallback: "refetchRequired",
            detail: { kind: "field", field: "name" },
          },
        ],
        load: ({ profileId, body }) => ({ id: profileId, name: body.name }),
      });

    const saveLine = saveProfile.line({
      profileId: "p1",
      body: { name: "Renamed" },
    });
    const plan = saveLine.mutationResponse();

    assert.equal(residentLine.value().name, "Renamed");
    assert.equal(plan.targets[0].reconciliation.kind, "field");
    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.executionArtifacts[0].scope, "field");
    assert.equal(plan.executionArtifacts[0].field, "name");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "patch");
    assert.equal(plan.executionArtifacts[0].deliveryScope, "field");
    assert.equal(residentLine.diagnostics().lastDeliveryKind, "patch");
    assert.equal(residentLine.diagnostics().lastDeliveryScope, "field");
    assert.equal(residentLine.diagnostics().lastPatchedField, "name");
    assert.equal(residentLine.diagnostics().lastEffect.provenance, "deliveredPatch");
    assert.equal(residentLine.diagnostics().lastEffect.patch.fieldProof.fieldName, "name");
    assert.equal(
      residentLine.diagnostics().lastEffect.counters.detailFieldTraversalBreadth,
      1,
    );
    assert.equal(
      residentLine.diagnostics().lastEffect.counters.detailFieldReconstructionBreadth,
      1,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("save responses can patch resident detail JSON paths through canonical mutation reconciliation", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const jsonPathRead = runtime.signals.resource.detailJsonPaths({
      title: { path: ["document", "title"] },
    });
    const jsonPathFamily = runtime.signals.api({}).url("/workflow-title/:workflowId").detail({
      reconcile: jsonPathRead,
      load: ({ workflowId }) => ({
        id: workflowId,
        document: { title: "First" },
      }),
    });
    const jsonPathLine = jsonPathFamily.line({ workflowId: "wf-1" });
    const saveTitle = runtime.signals.api({}).url("/workflow-title/:workflowId")
      .response(runtime.signals.resource.response.detailJsonPaths()({
        title: { path: ["document", "title"] },
      }))
      .update({
        reconciles: [
          {
            family: jsonPathFamily,
            params: ({ workflowId }) => ({ workflowId }),
            fallback: "refetchRequired",
            detail: { kind: "jsonPath", path: "title" },
          },
        ],
        load: ({ workflowId, body }) => ({
          id: workflowId,
          document: { title: body.title },
        }),
      });

    saveTitle.line({
      workflowId: "wf-1",
      body: { title: "Renamed" },
    });
    assert.equal(jsonPathLine.value().document.title, "Renamed");
    assert.equal(jsonPathLine.diagnostics().lastDeliveryScope, "jsonPath");
    assert.equal(jsonPathLine.diagnostics().lastPatchedPath, "title");
    assert.equal(jsonPathLine.diagnostics().lastEffect.patch.jsonPath.pathName, "title");
    assert.deepEqual(jsonPathLine.diagnostics().lastEffect.patch.jsonPath.path, [
      "document",
      "title",
    ]);
    assert.equal(
      jsonPathLine.diagnostics().lastEffect.counters.jsonPathTraversalBreadth,
      3,
    );
    assert.equal(
      jsonPathLine.diagnostics().lastEffect.counters.jsonPathReconstructionBreadth,
      3,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("save responses can patch resident detail regions through canonical mutation reconciliation", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const regionRead = runtime.signals.resource.detailRegions({
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
    const regionFamily = runtime.signals.api({}).url("/workflow-graph/:workflowId").detail({
      reconcile: regionRead,
      load: ({ workflowId }) => ({
        id: workflowId,
        graph: { nodes: [{ id: "n1" }] },
      }),
    });
    const regionLine = regionFamily.line({ workflowId: "wf-1" });
    const saveGraph = runtime.signals.api({}).url("/workflow-graph/:workflowId")
      .response(runtime.signals.resource.response.detailRegions()({
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
      }))
      .update({
        reconciles: [
          {
            family: regionFamily,
            params: ({ workflowId }) => ({ workflowId }),
            fallback: "refetchRequired",
            detail: { kind: "region", region: "graph" },
          },
        ],
        load: ({ workflowId, body }) => ({
          id: workflowId,
          graph: { nodes: body.nodes },
        }),
      });

    const saveGraphLine = saveGraph.line({
      workflowId: "wf-1",
      body: { nodes: [{ id: "n2" }] },
    });
    const regionPlan = saveGraphLine.mutationResponse();

    assert.deepEqual(regionLine.value().graph, { nodes: [{ id: "n2" }] });
    assert.equal(regionPlan.executionArtifacts[0].scope, "region");
    assert.equal(regionPlan.executionArtifacts[0].deliveryScope, "region");
    assert.equal(regionLine.diagnostics().lastDeliveryScope, "region");
    assert.equal(regionLine.diagnostics().lastPatchedRegion, "graph");
    assert.equal(regionLine.diagnostics().lastEffect.patch.region.regionName, "graph");
    assert.deepEqual(regionLine.diagnostics().lastEffect.patch.region.cost, {
      traversalBreadth: 2,
      reconstructionBreadth: 2,
      cloneBreadth: 2,
    });
    assert.equal(
      regionLine.diagnostics().lastEffect.counters.detailRegionTraversalBreadth,
      2,
    );
    assert.equal(
      regionLine.diagnostics().lastEffect.counters.detailRegionReconstructionBreadth,
      2,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("exact detail mutation reconciliation falls back for nonresident targets and denies mismatched declarations", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detailFields = runtime.signals.resource.detailFields({
      name: {
        read: (value) => value.name,
        write: (value, name) => ({ ...value, name }),
      },
    });
    const readFamily = runtime.signals.api({}).url("/profiles/:profileId").detail({
      reconcile: detailFields,
      load: ({ profileId }) => ({ id: profileId, name: "First" }),
    });
    const saveProfile = runtime.signals.api({}).url("/profiles/:profileId")
      .response(runtime.signals.resource.response.detail()({ name: "name" }))
      .update({
        reconciles: [
          {
            family: readFamily,
            params: ({ profileId }) => ({ profileId }),
            fallback: "refetchRequired",
            detail: { kind: "field", field: "name" },
          },
        ],
        load: ({ profileId, body }) => ({ id: profileId, name: body.name }),
      });

    const plan = saveProfile.line({
      profileId: "p2",
      body: { name: "Detached" },
    }).mutationResponse();
    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].fallback, "refetchRequired");

    assert.throws(
      () =>
        runtime.signals.api({}).url("/profiles/:profileId")
          .response(runtime.signals.resource.response.detail()({ name: "name" }))
          .update({
            reconciles: [
              {
                family: readFamily,
                params: ({ profileId }) => ({ profileId }),
                fallback: "refetchRequired",
                detail: { kind: "field", field: "title" },
              },
            ],
            load: ({ profileId, body }) => ({ id: profileId, name: body.name }),
          }),
      /detail\.field "title" is not declared on the target detail family/,
    );
  } finally {
    await runtime.cleanup();
  }
});
