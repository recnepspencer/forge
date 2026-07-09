import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

function createWorkflowRuntime(runtime) {
  const detailFields = runtime.signals.resource.detailFields({
    status: {
      read: (value) => value.status,
      write: (value, status) => ({ ...value, status }),
    },
  });
  const permissionFields = runtime.signals.resource.detailFields({
    canEdit: {
      read: (value) => value.canEdit,
      write: (value, canEdit) => ({ ...value, canEdit }),
    },
  });
  const workflowDetail = runtime.signals.api({}).url("/workflows/:workflowId").detail({
    reconcile: detailFields,
    load: ({ workflowId }) => ({ id: workflowId, status: "draft" }),
  });
  const workflowList = runtime.signals.api({}).url("/workflow-list")
    .response(runtime.signals.resource.response.collection({
      itemId: (item) => item.id,
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      summaries: runtime.signalsMod.resourceValueSummaries({
        version: {
          read: (value) => value.version,
          write: (value, version) => ({ ...value, version }),
        },
      }),
    }))
    .list({
      load: () => ({
        items: [{ id: "wf-1", status: "draft" }],
        version: 1,
      }),
    });
  const workflowSearch = runtime.signals.api({}).url("/workflow-search").paged({
    itemIdentity: (item) => item.id,
    reconcile: runtime.signalsMod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
    }),
    accumulatePage: (existing, next) => ({
      items: [...existing.items, ...next.items],
      cursor: next.cursor,
    }),
    load: () => ({
      items: [{ id: "wf-1", status: "draft" }],
      cursor: "next",
    }),
  });
  const workflowPermissions = runtime.signals.api({}).url("/workflow-permissions/:workflowId").detail({
    reconcile: permissionFields,
    load: ({ workflowId }) => ({ id: workflowId, canEdit: false }),
  });
  return { workflowDetail, workflowList, workflowSearch, workflowPermissions };
}

test("save responses admit mixed exact and fallback multi-family reconciliation only when partialAllowed is declared", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { workflowDetail, workflowList, workflowSearch, workflowPermissions } =
      createWorkflowRuntime(runtime);
    const workflowLine = workflowDetail.line({ workflowId: "wf-1" });
    const listLine = workflowList.line({});
    const searchLine = workflowSearch.line({});
    const saveWorkflow = runtime.signals.api({}).url("/workflows/:workflowId")
      .response(runtime.signals.resource.response.detail()({
        status: "status",
        version: "version",
        canEdit: "canEdit",
        warnings: "warnings",
      }))
      .update({
        atomicity: "partialAllowed",
        reconciles: [{
          family: workflowDetail,
          params: ({ workflowId }) => ({ workflowId }),
          fallback: "partialReconciliation",
          detail: { kind: "field", field: "status" },
        }, {
          family: workflowList,
          params: () => ({}),
          fallback: "partialReconciliation",
          collection: { kind: "item" },
        }, {
          family: workflowList,
          params: () => ({}),
          fallback: "partialReconciliation",
          summary: { kind: "summary", summary: "version" },
        }, {
          family: workflowSearch,
          params: () => ({}),
          fallback: "partialReconciliation",
          collection: { kind: "item" },
        }, {
          family: workflowPermissions,
          params: ({ workflowId }) => ({ workflowId }),
          fallback: "deliveryAwaited",
          detail: { kind: "field", field: "canEdit" },
        }],
        diagnostics: [{ kind: "warnings", field: "warnings" }],
        load: ({ workflowId }) => ({
          id: workflowId,
          status: "published",
          version: 2,
          canEdit: true,
          warnings: ["permissions delivery expected"],
        }),
      });

    const saveLine = saveWorkflow.line({ workflowId: "wf-1", body: {} });
    const plan = saveLine.mutationResponse();
    const latestMutationResponse = saveLine.summary().diagnostics.latest;

    assert.equal(workflowLine.value().status, "published");
    assert.equal(listLine.value().items[0].status, "published");
    assert.equal(listLine.value().version, 2);
    assert.equal(searchLine.value().items[0].status, "published");
    assert.equal(plan.partialAdmission, "admitted");
    assert.equal(plan.reconciliationAtomicity, "partialAllowed");
    assert.equal(plan.confirmation.kind, "partialCanonicalTruth");
    assert.deepEqual(plan.executionArtifacts.map((artifact) => artifact.kind), [
      "exactDetail",
      "exactCollectionItem",
      "exactSummary",
      "exactCollectionItem",
      "fallback",
    ]);
    assert.equal(plan.executionArtifacts[4].fallback, "deliveryAwaited");
    assert.equal(plan.counters.appliedTargetBreadth, 4);
    assert.equal(plan.counters.fallbackBreadth, 1);
    assert.equal(plan.counters.targetFanoutBreadth, 5);
    assert.equal(plan.counters.payloadFieldExtractionBreadth, 4);
    assert.equal(plan.counters.topologyTraversalBreadth, 3);
    assert.equal(plan.counters.reconstructionBreadth, 4);
    assert.equal(latestMutationResponse.mutationResponseTargetCount, 5);
    assert.equal(latestMutationResponse.mutationResponseExactTargetCount, 4);
    assert.equal(latestMutationResponse.mutationResponseFallbackTargetCount, 1);
    assert.equal(latestMutationResponse.mutationResponseTargetLookupBreadth, 5);
    assert.equal(latestMutationResponse.mutationResponseTargetFanoutBreadth, 5);
    assert.equal(latestMutationResponse.mutationResponsePayloadFieldExtractionBreadth, 4);
    assert.equal(latestMutationResponse.mutationResponseTopologyTraversalBreadth, 3);
    assert.equal(latestMutationResponse.mutationResponseReconstructionBreadth, 4);
    assert.equal(latestMutationResponse.mutationResponseFallbackBreadth, 1);
    assert.deepEqual(
      latestMutationResponse.mutationResponseTargetOutcomes.map((entry) => ({
        targetId: entry.targetId,
        outcomeKind: entry.outcomeKind,
        executionKind: entry.executionKind,
        fallbackKind: entry.fallbackKind,
        locus: entry.locus,
      })),
      [{
        targetId: "mutationTarget1",
        outcomeKind: "exact",
        executionKind: "exactDetail",
        fallbackKind: null,
        locus: "status",
      }, {
        targetId: "mutationTarget2",
        outcomeKind: "exact",
        executionKind: "exactCollectionItem",
        fallbackKind: null,
        locus: "wf-1",
      }, {
        targetId: "mutationTarget3",
        outcomeKind: "exact",
        executionKind: "exactSummary",
        fallbackKind: null,
        locus: "version",
      }, {
        targetId: "mutationTarget4",
        outcomeKind: "exact",
        executionKind: "exactCollectionItem",
        fallbackKind: null,
        locus: "wf-1",
      }, {
        targetId: "mutationTarget5",
        outcomeKind: "fallback",
        executionKind: "fallback",
        fallbackKind: "deliveryAwaited",
        locus: null,
      }],
    );
    assert.equal(plan.diagnostics.count, 1);
    assert.deepEqual(plan.diagnostics.entries[0].value, ["permissions delivery expected"]);
    assert.equal(workflowLine.history().lifecycle.at(-1)?.event, "delivered");
    assert.equal(listLine.history().lifecycle.at(-1)?.event, "delivered");
    assert.equal(searchLine.history().lifecycle.at(-1)?.event, "delivered");
  } finally {
    await runtime.cleanup();
  }
});
