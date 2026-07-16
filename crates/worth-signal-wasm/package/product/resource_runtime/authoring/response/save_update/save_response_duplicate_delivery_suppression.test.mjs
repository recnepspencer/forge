import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("re-executing an identical write reports duplicate suppression instead of consumed canonical truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const taskList = runtime.signals.api({}).url("/tasks")
      .response(runtime.signals.resource.response.array({
        itemId: (item) => item.id,
      }))
      .list({
        load: () => [{ id: "t1", title: "First", sync: "synced" }],
      });
    const taskLine = taskList.line({});
    await taskLine.awaitSettlement();

    const saveTask = runtime.signals.api({}).url("/tasks/:taskId")
      .response(runtime.signals.resource.response.detail()({
        title: "title",
        sync: "sync",
      }))
      .update({
        reconciles: [{
          family: taskList,
          params: () => ({}),
          fallback: "partialReconciliation",
          collection: { kind: "item" },
        }],
        load: ({ body }) => ({ id: "t1", title: body.title, sync: "synced" }),
      });

    const body = { id: "t1", title: "Renamed", sync: "syncing" };

    const first = await saveTask
      .execute({ taskId: "t1", body }, { freeOnSettle: true })
      .settled();
    assert.equal(first.resultKind, "fulfilled");
    assert.equal(first.mutationResponse.confirmation.kind, "consumedCanonicalTruth");
    assert.deepEqual(taskLine.value(), [
      { id: "t1", title: "Renamed", sync: "synced" },
    ]);
    const deliveredEffect = taskLine.diagnostics().lastEffect;
    assert.equal(deliveredEffect.provenance, "deliveredPatch");

    const second = await saveTask
      .execute({ taskId: "t1", body }, { freeOnSettle: true })
      .settled();
    assert.equal(second.resultKind, "partial");

    const artifact = second.mutationResponse.executionArtifacts[0];
    assert.equal(artifact.kind, "fallback");
    assert.equal(artifact.fallback, "partialReconciliation");
    assert.equal(artifact.partial.kind, "duplicateDeliverySuppressed");
    assert.match(artifact.partial.digest, /duplicate-delivery/);
    assert.match(artifact.detail, /already consumed by an earlier mutation response/);
    assert.equal(second.mutationResponse.confirmation.kind, "partialCanonicalTruth");
    assert.equal(second.mutationResponse.confirmation.exactTargetCount, 0);
    assert.deepEqual(second.mutationResponse.confirmation.fallbackKinds, [
      "partialReconciliation",
    ]);
    assert.equal(second.mutationResponse.counters.fallbackBreadth, 1);

    // the target line was not silently re-patched and records no new delivery
    assert.equal(taskLine.diagnostics().lastEffect.effectId, deliveredEffect.effectId);
    assert.deepEqual(taskLine.value(), [
      { id: "t1", title: "Renamed", sync: "synced" },
    ]);
  } finally {
    await runtime.cleanup();
  }
});
