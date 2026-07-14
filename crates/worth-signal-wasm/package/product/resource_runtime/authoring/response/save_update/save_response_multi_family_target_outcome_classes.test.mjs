import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

function createWorkflowSurface(runtime) {
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
  const workflowAudit = runtime.signals.api({}).url("/workflow-audit/:workflowId")
    .response(runtime.signals.resource.response.array({
      itemId: (item) => item.id,
    }))
    .list({
      load: ({ workflowId }) => [
        { id: `${workflowId}:entry-1`, message: "saved draft" },
      ],
    });
  return {
    workflowDetail,
    workflowList,
    workflowAudit,
  };
}

test("save responses keep audit/history targets explicit as unsupportedTarget while exact siblings apply under partialAllowed", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const {
      workflowDetail,
      workflowList,
      workflowAudit,
    } = createWorkflowSurface(runtime);
    const workflowLine = workflowDetail.line({ workflowId: "wf-1" });
    const listLine = workflowList.line({});
    const auditLine = workflowAudit.line({ workflowId: "wf-1" });

    const saveLine = runtime.signals.api({}).url("/workflows/:workflowId")
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
            family: workflowList,
            params: () => ({}),
            fallback: "partialReconciliation",
            collection: { kind: "item" },
          },
          {
            family: workflowList,
            params: () => ({}),
            fallback: "partialReconciliation",
            summary: { kind: "summary", summary: "version" },
          },
          {
            family: workflowAudit,
            params: ({ workflowId }) => ({ workflowId }),
            fallback: "unsupportedTarget",
          },
        ],
        diagnostics: [{ kind: "warnings", field: "warnings" }],
        load: ({ workflowId }) => ({
          id: workflowId,
          status: "published",
          version: 2,
          warnings: ["audit trail remains declaration-only"],
        }),
      })
      .line({ workflowId: "wf-1", body: {} });
    const plan = saveLine.mutationResponse();
    const latest = saveLine.summary().diagnostics.latest;

    assert.equal(workflowLine.value().status, "published");
    assert.equal(listLine.value().items[0].status, "published");
    assert.equal(listLine.value().version, 2);
    assert.deepEqual(auditLine.value(), [{ id: "wf-1:entry-1", message: "saved draft" }]);
    assert.equal(plan.partialAdmission, "admitted");
    assert.deepEqual(
      plan.executionArtifacts.map((artifact) => artifact.kind),
      ["exactDetail", "exactCollectionItem", "exactSummary", "fallback"],
    );
    assert.equal(plan.executionArtifacts[3].fallback, "unsupportedTarget");
    assert.equal(latest.mutationResponseFallbackTargetCount, 1);
    assert.equal(
      latest.mutationResponseUnsupportedTargetDigest,
      "mutation-response-unsupportedTarget-targets|mutationTarget4:collection:"
      + auditLine.descriptor().family.familyId
      + ":/workflow-audit/wf-1:none:none",
    );
    assert.equal(
      latest.mutationResponsePartialReconciliationDigest,
      "mutation-response-partialReconciliation-targets|none",
    );
    assert.deepEqual(
      latest.mutationResponseTargetOutcomes.map((entry) => ({
        targetId: entry.targetId,
        familyId: entry.familyId,
        outcomeKind: entry.outcomeKind,
        fallbackKind: entry.fallbackKind,
      })),
      [
        {
          targetId: "mutationTarget1",
          familyId: workflowLine.descriptor().family.familyId,
          outcomeKind: "exact",
          fallbackKind: null,
        },
        {
          targetId: "mutationTarget2",
          familyId: listLine.descriptor().family.familyId,
          outcomeKind: "exact",
          fallbackKind: null,
        },
        {
          targetId: "mutationTarget3",
          familyId: listLine.descriptor().family.familyId,
          outcomeKind: "exact",
          fallbackKind: null,
        },
        {
          targetId: "mutationTarget4",
          familyId: auditLine.descriptor().family.familyId,
          outcomeKind: "fallback",
          fallbackKind: "unsupportedTarget",
        },
      ],
    );
    assert.equal(
      saveLine.history().verificationPackage().diagnostics.summary.latest
        .mutationResponseUnsupportedTargetDigest,
      latest.mutationResponseUnsupportedTargetDigest,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("save responses keep unsupported audit/history targets explicit under allOrNone by downgrading exact siblings to partialReconciliation", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const {
      workflowDetail,
      workflowList,
      workflowAudit,
    } = createWorkflowSurface(runtime);
    const workflowLine = workflowDetail.line({ workflowId: "wf-1" });
    const listLine = workflowList.line({});
    const auditLine = workflowAudit.line({ workflowId: "wf-1" });

    const saveLine = runtime.signals.api({}).url("/workflows/:workflowId")
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
            family: workflowList,
            params: () => ({}),
            fallback: "partialReconciliation",
            collection: { kind: "item" },
          },
          {
            family: workflowList,
            params: () => ({}),
            fallback: "partialReconciliation",
            summary: { kind: "summary", summary: "version" },
          },
          {
            family: workflowAudit,
            params: ({ workflowId }) => ({ workflowId }),
            fallback: "unsupportedTarget",
          },
        ],
        load: ({ workflowId }) => ({
          id: workflowId,
          status: "published",
          version: 2,
        }),
      })
      .line({ workflowId: "wf-1", body: {} });
    const plan = saveLine.mutationResponse();
    const latest = saveLine.summary().diagnostics.latest;

    assert.equal(workflowLine.value().status, "draft");
    assert.equal(listLine.value().items[0].status, "draft");
    assert.equal(listLine.value().version, 1);
    assert.deepEqual(auditLine.value(), [{ id: "wf-1:entry-1", message: "saved draft" }]);
    assert.equal(plan.partialAdmission, "denied");
    assert.deepEqual(
      plan.executionArtifacts.map((artifact) => artifact.fallback),
      [
        "partialReconciliation",
        "partialReconciliation",
        "partialReconciliation",
        "unsupportedTarget",
      ],
    );
    assert.equal(
      latest.mutationResponsePartialReconciliationDigest,
      "mutation-response-partialReconciliation-targets|mutationTarget1:detail:"
      + workflowLine.descriptor().family.familyId
      + ":/workflows/wf-1:none:none,mutationTarget2:collection:"
      + listLine.descriptor().family.familyId
      + ":/workflow-list:none:none,mutationTarget3:collection:"
      + listLine.descriptor().family.familyId
      + ":/workflow-list:none:none",
    );
    assert.equal(
      latest.mutationResponseUnsupportedTargetDigest,
      "mutation-response-unsupportedTarget-targets|mutationTarget4:collection:"
      + auditLine.descriptor().family.familyId
      + ":/workflow-audit/wf-1:none:none",
    );
    assert.equal(
      saveLine.history().verificationPackage().diagnostics.summary.latest
        .mutationResponsePartialReconciliationDigest,
      latest.mutationResponsePartialReconciliationDigest,
    );
    assert.equal(
      saveLine.history().verificationPackage().diagnostics.summary.latest
        .mutationResponseUnsupportedTargetDigest,
      latest.mutationResponseUnsupportedTargetDigest,
    );
  } finally {
    await runtime.cleanup();
  }
});
