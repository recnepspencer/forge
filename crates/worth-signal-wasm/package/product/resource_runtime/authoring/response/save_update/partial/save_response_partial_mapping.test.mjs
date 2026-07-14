import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../../runtime_fixture/real_request_runtime.mjs";

test("save responses can partially reconcile declared field and summary targets when atomicity allows it", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detailFields = runtime.signals.resource.detailFields({
      status: {
        read: (value) => value.status,
        write: (value, status) => ({ ...value, status }),
      },
    });
    const workflowDetail = runtime.signals.api({}).url("/workflows/:workflowId").detail({
      reconcile: detailFields,
      load: ({ workflowId }) => ({ id: workflowId, status: "draft" }),
    });
    const workflowSummaries = runtime.signals.api({}).url("/workflow-search")
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
    const workflowLine = workflowDetail.line({ workflowId: "wf-1" });
    const summariesLine = workflowSummaries.line({});
    const saveWorkflow = runtime.signals.api({}).url("/workflows/:workflowId")
      .response(runtime.signals.resource.response.detail()({
        status: "status",
        version: "version",
        warnings: "warnings",
      }))
      .update({
        atomicity: "partialAllowed",
        reconciles: [
          {
            family: workflowDetail,
            params: ({ workflowId }) => ({ workflowId }),
            fallback: "partialReconciliation",
            detail: { kind: "field", field: "status" },
          },
          {
            family: workflowSummaries,
            params: () => ({}),
            fallback: "partialReconciliation",
            summary: { kind: "summary", summary: "version" },
          },
        ],
        diagnostics: [{ kind: "warnings", field: "warnings" }],
        load: ({ workflowId }) => ({
          id: workflowId,
          status: "published",
          warnings: ["normalized"],
        }),
      });

    const saveLine = saveWorkflow.line({ workflowId: "wf-1", body: {} });
    const plan = saveLine.mutationResponse();
    const latestMutationResponse = saveLine.summary().diagnostics.latest;

    assert.equal(workflowLine.value().status, "published");
    assert.equal(summariesLine.value().version, 1);
    assert.equal(plan.reconciliationAtomicity, "partialAllowed");
    assert.equal(plan.partialAdmission, "admitted");
    assert.equal(plan.confirmation.kind, "partialCanonicalTruth");
    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.executionArtifacts[0].field, "status");
    assert.equal(plan.executionArtifacts[1].kind, "fallback");
    assert.equal(plan.executionArtifacts[1].fallback, "partialReconciliation");
    assert.deepEqual(plan.executionArtifacts[1].partial, {
      kind: "missingResponseField",
      field: "version",
      digest: "mutation-response-partial|missing-field:version",
    });
    assert.equal(plan.diagnostics.count, 1);
    assert.deepEqual(plan.diagnostics.entries[0].value, ["normalized"]);
    assert.equal(plan.counters.partialPolicyBreadth, 1);
    assert.equal(plan.counters.payloadFieldExtractionBreadth, 3);
    assert.equal(plan.counters.topologyTraversalBreadth, 1);
    assert.equal(plan.counters.reconstructionBreadth, 1);
    assert.match(plan.fallbackDigest, /missing-field:version/);
    assert.equal(latestMutationResponse.mutationResponseTargetCount, 2);
    assert.equal(latestMutationResponse.mutationResponseExactTargetCount, 1);
    assert.equal(latestMutationResponse.mutationResponseFallbackTargetCount, 1);
    assert.equal(latestMutationResponse.mutationResponseTargetLookupBreadth, 2);
    assert.equal(latestMutationResponse.mutationResponseTargetFanoutBreadth, 2);
    assert.equal(
      latestMutationResponse.mutationResponsePayloadFieldExtractionBreadth,
      3,
    );
    assert.equal(
      latestMutationResponse.mutationResponseTopologyTraversalBreadth,
      1,
    );
    assert.equal(
      latestMutationResponse.mutationResponseReconstructionBreadth,
      1,
    );
    assert.equal(latestMutationResponse.mutationResponseFallbackBreadth, 1);
    assert.equal(
      latestMutationResponse.mutationResponseTargetOutcomeDigest,
      "mutation-response-target-outcomes|mutationTarget1:detail:"
      + workflowLine.descriptor().family.familyId
      + ":/workflows/wf-1:resident:exact:exactDetail:field:none:none:none:none:status,"
      + "mutationTarget2:collection:"
      + workflowSummaries.line({}).descriptor().family.familyId
      + ":/workflow-search:resident:fallback:fallback:none:partialReconciliation:missingResponseField:version:none:none",
    );
    assert.deepEqual(latestMutationResponse.mutationResponseTargetOutcomes, [
      {
        targetId: "mutationTarget1",
        familyKind: "detail",
        familyId: workflowLine.descriptor().family.familyId,
        canonicalKey: "/workflows/wf-1",
        residency: "resident",
        outcomeKind: "exact",
        executionKind: "exactDetail",
        scope: "field",
        fallbackKind: null,
        partialKind: null,
        partialField: null,
        staleReason: null,
        locus: "status",
        targetDigest: plan.targets[0].targetDigest,
      },
      {
        targetId: "mutationTarget2",
        familyKind: "collection",
        familyId: workflowSummaries.line({}).descriptor().family.familyId,
        canonicalKey: "/workflow-search",
        residency: "resident",
        outcomeKind: "fallback",
        executionKind: "fallback",
        scope: null,
        fallbackKind: "partialReconciliation",
        partialKind: "missingResponseField",
        partialField: "version",
        staleReason: null,
        locus: null,
        targetDigest: plan.targets[1].targetDigest,
      },
    ]);
    assert.equal(workflowLine.diagnostics().lastDeliveryScope, "field");
    assert.equal(workflowLine.history().lifecycle.at(-1)?.event, "delivered");
    assert.equal(
      saveLine.history().verificationPackage().diagnostics.summary.latest
        .mutationResponsePayloadFieldExtractionBreadth,
      3,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("save responses deny sibling exact reconciliation under all-or-none partial mapping", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detailFields = runtime.signals.resource.detailFields({
      status: {
        read: (value) => value.status,
        write: (value, status) => ({ ...value, status }),
      },
    });
    const workflowDetail = runtime.signals.api({}).url("/workflows/:workflowId").detail({
      reconcile: detailFields,
      load: ({ workflowId }) => ({ id: workflowId, status: "draft" }),
    });
    const workflowSummaries = runtime.signals.api({}).url("/workflow-search")
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
    const workflowLine = workflowDetail.line({ workflowId: "wf-1" });
    const summariesLine = workflowSummaries.line({});
    const saveWorkflow = runtime.signals.api({}).url("/workflows/:workflowId")
      .response(runtime.signals.resource.response.detail()({
        status: "status",
        version: "version",
      }))
      .update({
        reconciles: [
          {
            family: workflowDetail,
            params: ({ workflowId }) => ({ workflowId }),
            fallback: "partialReconciliation",
            detail: { kind: "field", field: "status" },
          },
          {
            family: workflowSummaries,
            params: () => ({}),
            fallback: "partialReconciliation",
            summary: { kind: "summary", summary: "version" },
          },
        ],
        load: ({ workflowId }) => ({
          id: workflowId,
          status: "published",
        }),
      });

    const plan = saveWorkflow.line({ workflowId: "wf-1", body: {} }).mutationResponse();

    assert.equal(workflowLine.value().status, "draft");
    assert.equal(summariesLine.value().version, 1);
    assert.equal(plan.reconciliationAtomicity, "allOrNone");
    assert.equal(plan.partialAdmission, "denied");
    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].fallback, "partialReconciliation");
    assert.equal(plan.executionArtifacts[0].partial, null);
    assert.equal(plan.executionArtifacts[1].kind, "fallback");
    assert.equal(plan.executionArtifacts[1].partial?.field, "version");
    assert.equal(plan.counters.payloadFieldExtractionBreadth, 2);
    assert.equal(plan.counters.topologyTraversalBreadth, 0);
    assert.equal(plan.counters.reconstructionBreadth, 0);
    assert.equal(plan.counters.fallbackBreadth, 2);
    assert.equal(plan.counters.appliedTargetBreadth ?? 0, 0);
    assert.equal(
      workflowLine.history().lifecycle.at(-1)?.event,
      "materialized",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("save partial mapping treats explicit null as present and denies malformed atomicity", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detailFields = runtime.signals.resource.detailFields({
      archivedAt: {
        read: (value) => value.archivedAt,
        write: (value, archivedAt) => ({ ...value, archivedAt }),
      },
    });
    const workflowDetail = runtime.signals.api({}).url("/workflows/:workflowId").detail({
      reconcile: detailFields,
      load: ({ workflowId }) => ({ id: workflowId, archivedAt: "2026-01-01" }),
    });
    const workflowLine = workflowDetail.line({ workflowId: "wf-1" });
    const saveWorkflow = runtime.signals.api({}).url("/workflows/:workflowId")
      .response(runtime.signals.resource.response.detail()({
        archivedAt: "archivedAt",
      }))
      .update({
        atomicity: "partialAllowed",
        reconciles: [
          {
            family: workflowDetail,
            params: ({ workflowId }) => ({ workflowId }),
            fallback: "partialReconciliation",
            detail: { kind: "field", field: "archivedAt" },
          },
        ],
        load: ({ workflowId }) => ({
          id: workflowId,
          archivedAt: null,
        }),
      });

    const plan = saveWorkflow.line({ workflowId: "wf-1", body: {} }).mutationResponse();

    assert.equal(workflowLine.value().archivedAt, null);
    assert.equal(plan.partialAdmission, "notNeeded");
    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.counters.payloadFieldExtractionBreadth, 1);
    assert.equal(plan.counters.topologyTraversalBreadth, 1);
    assert.equal(plan.counters.reconstructionBreadth, 1);

    assert.throws(
      () =>
        runtime.signals.api({}).url("/workflows/:workflowId")
          .response(runtime.signals.resource.response.detail()({ archivedAt: "archivedAt" }))
          .update({
            atomicity: "laterMaybe",
            reconciles: [
              {
                family: workflowDetail,
                params: ({ workflowId }) => ({ workflowId }),
                fallback: "partialReconciliation",
                detail: { kind: "field", field: "archivedAt" },
              },
            ],
            load: ({ workflowId }) => ({ id: workflowId, archivedAt: null }),
          }),
      /atomicity must be one of allOrNone, partialAllowed/,
    );
  } finally {
    await runtime.cleanup();
  }
});
