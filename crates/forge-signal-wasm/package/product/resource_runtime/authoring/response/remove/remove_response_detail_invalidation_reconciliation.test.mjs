import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("remove responses can replace resident detail truth with the canonical deleted item payload", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskDetail = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({
        load: ({ taskId }) => ({ id: taskId, title: "First", status: "active" }),
      });
    const detailLine = taskDetail.line({ taskId: "t1" });

    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: taskDetail,
          params: ({ taskId }) => ({ taskId }),
          fallback: "refetchRequired",
          detail: { kind: "replace" },
        }],
        load: ({ taskId }) => ({
          id: taskId,
          title: "First",
          status: "deleted",
          deletedAt: "2026-05-13T00:00:00Z",
        }),
      })
      .line({ taskId: "t1" })
      .mutationResponse();

    assert.deepEqual(detailLine.value(), {
      id: "t1",
      title: "First",
      status: "deleted",
      deletedAt: "2026-05-13T00:00:00Z",
    });
    assert.equal(plan.targets[0].reconciliation.kind, "replace");
    assert.equal(plan.targets[0].reconciliation.targetDigest, "detail:replace");
    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.executionArtifacts[0].scope, "line");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "replace");
    assert.equal(plan.executionArtifacts[0].deliveryScope, "line");
    assert.equal(detailLine.diagnostics().lastDeliveryKind, "replace");
    assert.equal(detailLine.diagnostics().lastDeliveryScope, "line");
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("remove responses can invalidate resident detail truth while patching summaries from metadata-only responses", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskDetail = runtime.signals.api({}).url("/tasks/:taskId").detail({
      load: ({ taskId }) => ({ id: taskId, title: "First", status: "active" }),
    });
    const taskList = runtime.signals.api({}).url("/task-search")
      .response(runtime.signals.resource.response.collection({
        itemId: (item) => item.id,
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: runtime.signalsMod.resourceValueSummaries({
          total: {
            read: (value) => value.total,
            write: (value, total) => ({ ...value, total }),
          },
        }),
      }))
      .list({
        load: () => ({
          items: [{ id: "t1", title: "First", status: "active" }],
          total: 1,
        }),
      });
    const detailLine = taskDetail.line({ taskId: "t1" });
    const listLine = taskList.line({});

    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.summary()())
      .remove({
        reconciles: [
          {
            family: taskDetail,
            params: ({ taskId }) => ({ taskId }),
            fallback: "refetchRequired",
            detail: { kind: "invalidate" },
          },
          {
            family: taskList,
            params: () => ({}),
            fallback: "refetchRequired",
            summary: { kind: "summary", summary: "total" },
          },
        ],
        load: () => 0,
      })
      .line({ taskId: "t1" })
      .mutationResponse();

    assert.deepEqual(detailLine.value(), { id: "t1", title: "First", status: "active" });
    assert.deepEqual(detailLine.freshness(), { kind: "stale", reason: "deliveryInvalidate" });
    assert.equal(plan.targets[0].reconciliation.kind, "invalidate");
    assert.equal(plan.targets[0].reconciliation.targetDigest, "detail:invalidate");
    assert.equal(plan.executionArtifacts[0].kind, "exactDetailInvalidation");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "invalidate");
    assert.equal(plan.executionArtifacts[0].deliveryScope, "invalidate");
    assert.equal(detailLine.diagnostics().lastDeliveryKind, "invalidate");
    assert.equal(detailLine.diagnostics().lastDeliveryScope, "invalidate");
    assert.equal(detailLine.diagnostics().lastInvalidationCause, "deliveryInvalidate");
    assert.equal(plan.executionArtifacts[1].kind, "exactSummary");
    assert.equal(listLine.value().total, 0);
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.equal(plan.confirmation.exactTargetCount, 2);
    assert.equal(plan.counters.appliedTargetBreadth, 2);
  } finally {
    await runtime.cleanup();
  }
});

test("remove responses preserve typed fallback when a detail invalidation target is not resident", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskDetail = runtime.signals.api({}).url("/tasks/:taskId").detail({
      load: ({ taskId }) => ({ id: taskId, title: "First", status: "active" }),
    });

    const plan = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.summary()())
      .remove({
        reconciles: [
          {
            family: taskDetail,
            params: ({ taskId }) => ({ taskId }),
            fallback: "refetchRequired",
            detail: { kind: "invalidate" },
          },
        ],
        load: () => 0,
      })
      .line({ taskId: "t1" })
      .mutationResponse();

    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].fallback, "refetchRequired");
    assert.equal(plan.confirmation.kind, "refetchRequired");
  } finally {
    await runtime.cleanup();
  }
});

test("remove responses preserve typed deliveryAwaited fallback when exact detail reconciliation cannot apply yet", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskDetail = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .detail({
        load: ({ taskId }) => ({ id: taskId, title: "First", status: "active" }),
      });

    const removeLine = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: taskDetail,
          params: ({ taskId }) => ({ taskId }),
          fallback: "deliveryAwaited",
          detail: { kind: "replace" },
        }],
        load: ({ taskId }) => ({ id: taskId, status: "deleted" }),
      })
      .line({ taskId: "t9" });
    const plan = removeLine.mutationResponse();

    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].fallback, "deliveryAwaited");
    assert.equal(plan.confirmation.kind, "deliveryAwaited");
    assert.match(
      removeLine.summary().diagnostics.latest.mutationResponseFallbackReasonDigest,
      /deliveryAwaited:1/,
    );
    assert.match(
      removeLine.summary().diagnostics.latest.mutationResponseDeliveryAwaitedDigest,
      /deliveryAwaited/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("detail invalidation is admitted only on remove/delete detail targets", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskDetail = runtime.signals.api({}).url("/tasks/:taskId").detail({
      load: ({ taskId }) => ({ id: taskId, title: "First" }),
    });
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [{ id: "t1", title: "First" }],
      });

    assert.throws(
      () => runtime.signals.api({}).url("/tasks/:taskId")
        .response(runtime.signals.resource.response.summary()())
        .update({
          reconciles: [{
            family: taskDetail,
            params: ({ taskId }) => ({ taskId }),
            fallback: "refetchRequired",
            detail: { kind: "invalidate" },
          }],
          load: () => 0,
        }),
      /detail invalidation is currently admitted only for remove\/delete responses/,
    );
    assert.throws(
      () => runtime.signals.api({}).url("/tasks/:taskId")
        .response(runtime.signals.resource.response.summary()())
        .remove({
          reconciles: [{
            family: taskList,
            params: () => ({}),
            fallback: "refetchRequired",
            detail: { kind: "invalidate" },
          }],
          load: () => 0,
        }),
      /detail exact reconciliation requires a detail read family/,
    );
  } finally {
    await runtime.cleanup();
  }
});
