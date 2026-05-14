import assert from "node:assert/strict";
import test from "node:test";

import { createDeferred } from "../../../runtime_fixture/async/deferred.mjs";
import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";
import {
  assertLineStateUnchanged,
  captureLineState,
} from "../../../reconciliation/reconciliation_proof_helpers.mjs";

test("pending remove response falls back when local target drift changes visible truth before delete confirmation settles", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const deferred = createDeferred();
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [
          { id: "t1", title: "First" },
          { id: "t2", title: "Second" },
        ],
      });
    const taskLine = taskList.line({});
    const removeTask = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "refetchRequired",
          collection: { kind: "delete" },
        }],
        load: () => deferred.promise,
      });

    const removeLine = removeTask.line({ taskId: "t1" });
    taskLine.patch(taskList.patch.item({
      itemId: "t2",
      nextItem: { id: "t2", title: "Locally changed" },
    }));
    deferred.resolve({ id: "t1" });
    await deferred.promise;
    await Promise.resolve();
    const plan = removeLine.mutationResponse();

    assert.deepEqual(taskLine.value(), [
      { id: "t1", title: "First" },
      { id: "t2", title: "Locally changed" },
    ]);
    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].staleness.reason, "visibleValueVersionChanged");
    assert.equal(plan.counters.staleTargetDenialBreadth, 1);
    assert.equal(plan.confirmation.kind, "refetchRequired");
    assert.match(
      removeLine.summary().diagnostics.latest.mutationResponseFallbackReasonDigest,
      /refetchRequired:1/,
    );
    assert.match(
      removeLine.summary().diagnostics.latest.mutationResponseRefetchRequiredDigest,
      /refetchRequired/,
    );
    assert.equal(
      removeLine.summary().diagnostics.latest.mutationResponseStaleTargetReasonDigest,
      "mutation-response-stale-target-reasons|visibleValueVersionChanged:1",
    );
    assert.match(
      removeLine.summary().diagnostics.latest.mutationResponseNoHiddenMutationDigest,
      /allDeclaredTargetsAccountedFor/,
    );
    assert.match(
      removeLine.history().verificationPackage().diagnostics.summary.latest.mutationResponseFallbackReasonDigest,
      /refetchRequired:1/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("pending remove response falls back when the resident delete target rematerializes to a new line instance", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const deferred = createDeferred();
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [
          { id: "t1", title: "First" },
          { id: "t2", title: "Second" },
        ],
      });
    const originalLine = taskList.line({});
    const originalRuntimeLineId = originalLine.descriptor().runtimeLineId;
    const removeTask = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "refetchRequired",
          collection: { kind: "delete" },
        }],
        load: () => deferred.promise,
      });

    const removeLine = removeTask.line({ taskId: "t1" });
    originalLine.free();
    const rematerializedLine = taskList.line({});

    deferred.resolve({ id: "t1" });
    await deferred.promise;
    await Promise.resolve();
    const plan = removeLine.mutationResponse();

    assert.notEqual(rematerializedLine.descriptor().runtimeLineId, originalRuntimeLineId);
    assert.deepEqual(rematerializedLine.value(), [
      { id: "t1", title: "First" },
      { id: "t2", title: "Second" },
    ]);
    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].staleness.reason, "runtimeLineIdChanged");
    assert.equal(plan.counters.staleTargetDenialBreadth, 1);
    assert.equal(plan.confirmation.kind, "refetchRequired");
    assert.equal(
      removeLine.summary().diagnostics.latest.mutationResponseStaleTargetReasonDigest,
      "mutation-response-stale-target-reasons|runtimeLineIdChanged:1",
    );
    assert.match(
      removeLine.summary().diagnostics.latest.mutationResponseTargetOutcomeDigest,
      /:fallback:fallback:none:refetchRequired:/,
    );
    assert.match(
      removeLine.summary().diagnostics.latest.mutationResponseNoHiddenMutationDigest,
      /allDeclaredTargetsAccountedFor/,
    );
    assert.match(
      removeLine.history().verificationPackage().diagnostics.summary.latest.mutationResponseTargetOutcomeDigest,
      /:fallback:fallback:none:refetchRequired:/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("duplicate delete confirmations are ignored without side effects after exact remove reconciliation applies", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [
          { id: "t1", title: "First" },
          { id: "t2", title: "Second" },
        ],
      });
    const taskLine = taskList.line({});
    const removeLine = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()())
      .remove({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "deletionUnavailable",
          collection: { kind: "delete" },
        }],
        load: ({ taskId }) => ({ id: taskId }),
      })
      .line({ taskId: "t1" });
    const plan = removeLine.mutationResponse();
    const deleteArtifact = plan.executionArtifacts[0];
    const beforeDuplicate = captureLineState(taskLine);

    const duplicate = taskLine.deliver(taskList.delivery.delete({
      packetId: deleteArtifact.packetId,
      basisId: taskLine.diagnostics().basis.currentBasisId,
      nextBasisId: taskLine.diagnostics().basis.currentBasisId,
      itemId: "t1",
    }));

    assert.deepEqual(taskLine.value(), [{ id: "t2", title: "Second" }]);
    assert.deepEqual(duplicate, {
      kind: "duplicateIgnored",
      packetId: deleteArtifact.packetId,
      deliveryKind: "patch",
    });
    assertLineStateUnchanged(taskLine, beforeDuplicate);
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
    assert.match(
      plan.executionDigest,
      /exactCollectionDelete/,
    );
    assert.match(
      plan.fallbackDigest,
      /:none:/,
    );
    assert.match(
      plan.lifecycleProof.rollbackDigest,
      /notApplicable|available/,
    );
    assert.equal(
      removeLine.summary().diagnostics.latest.mutationResponseFallbackTargetCount,
      0,
    );
    assert.equal(
      removeLine.summary().diagnostics.latest.mutationResponseStaleTargetReasonDigest,
      "mutation-response-stale-target-reasons|none",
    );
    assert.match(
      removeLine.summary().diagnostics.latest.mutationResponseNoHiddenMutationDigest,
      /allDeclaredTargetsAccountedFor/,
    );
  } finally {
    await runtime.cleanup();
  }
});
