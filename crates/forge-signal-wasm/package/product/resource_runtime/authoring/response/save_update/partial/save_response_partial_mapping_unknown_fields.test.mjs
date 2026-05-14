import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../../runtime_fixture/real_request_runtime.mjs";

test("save partial mapping names unknown response-field posture without mutating undeclared truth", async () => {
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
    const workflowLine = workflowDetail.line({ workflowId: "wf-1" });
    const saveWorkflow = runtime.signals.api({}).url("/workflows/:workflowId")
      .response(runtime.signals.resource.response.detail()({
        status: "status",
        version: "version",
        warnings: "warnings",
      }))
      .update({
        atomicity: "partialAllowed",
        reconciles: [{
          family: workflowDetail,
          params: ({ workflowId }) => ({ workflowId }),
          fallback: "partialReconciliation",
          detail: { kind: "field", field: "status" },
        }],
        load: ({ workflowId }) => ({
          id: workflowId,
          status: "published",
          version: 2,
          warnings: ["ignored but named"],
        }),
      });

    const saveLine = saveWorkflow.line({ workflowId: "wf-1", body: {} });
    const plan = saveLine.mutationResponse();
    const latestMutationResponse = saveLine.summary().diagnostics.latest;

    assert.equal(workflowLine.value().status, "published");
    assert.equal(plan.response.declaredFieldDigest, "mutation-response-declared-fields|status");
    assert.deepEqual(plan.response.unknownFieldPosture, {
      kind: "present",
      fields: ["id", "version", "warnings"],
      digest: "mutation-response-unknown-fields|id,version,warnings",
    });
    assert.equal(latestMutationResponse.mutationResponseTargetCount, 1);
    assert.equal(latestMutationResponse.mutationResponseExactTargetCount, 1);
    assert.equal(latestMutationResponse.mutationResponseFallbackTargetCount, 0);
    assert.equal(latestMutationResponse.mutationResponseTargetLookupBreadth, 1);
    assert.equal(latestMutationResponse.mutationResponseTargetFanoutBreadth, 1);
    assert.equal(latestMutationResponse.mutationResponsePayloadFieldExtractionBreadth, 1);
    assert.equal(latestMutationResponse.mutationResponseTopologyTraversalBreadth, 1);
    assert.equal(latestMutationResponse.mutationResponseReconstructionBreadth, 1);
    assert.equal(latestMutationResponse.mutationResponseFallbackBreadth, 0);
    assert.deepEqual(latestMutationResponse.mutationResponseTargetOutcomes, [{
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
    }]);
    assert.deepEqual(
      saveLine.history().lifecycle.at(-1)?.mutationResponseTargetOutcomes,
      latestMutationResponse.mutationResponseTargetOutcomes,
    );
    assert.equal(plan.partialAdmission, "notNeeded");
    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.counters.payloadFieldExtractionBreadth, 1);
    assert.equal(plan.counters.topologyTraversalBreadth, 1);
    assert.equal(plan.counters.reconstructionBreadth, 1);
    assert.equal(workflowLine.history().lifecycle.at(-1)?.event, "delivered");
  } finally {
    await runtime.cleanup();
  }
});
