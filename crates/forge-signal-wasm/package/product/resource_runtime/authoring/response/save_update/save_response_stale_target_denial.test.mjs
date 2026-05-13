import assert from "node:assert/strict";
import test from "node:test";

import { createDeferred } from "../../../runtime_fixture/async/deferred.mjs";
import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("pending save response falls back when local target drift changes visible truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const deferred = createDeferred();
    const readFamily = runtime.signals.api({}).url("/profiles/:profileId")
      .response(runtime.signals.resource.response.detail()({ name: "name" }))
      .detail({
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
        load: () => deferred.promise,
      });

    const saveLine = saveProfile.line({
      profileId: "p1",
      body: { name: "Server" },
    });
    residentLine.patch(readFamily.patch.field({
      field: "name",
      value: "Local",
    }));
    deferred.resolve({ name: "Server" });
    await deferred.promise;
    await Promise.resolve();
    const plan = saveLine.mutationResponse();

    assert.equal(residentLine.value().name, "Local");
    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].staleness.reason, "visibleValueVersionChanged");
    assert.equal(plan.executionArtifacts[0].staleness.submittedVisibleValueVersion, 1);
    assert.equal(plan.executionArtifacts[0].staleness.currentVisibleValueVersion, 2);
    assert.equal(plan.submittedTargets[0].visibleValueVersion, 1);
    assert.equal(plan.counters.staleTargetDenialBreadth, 1);
    assert.equal(plan.confirmation.kind, "refetchRequired");
  } finally {
    await runtime.cleanup();
  }
});

test("pending save response falls back when delivery advances target basis", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const deferred = createDeferred();
    const readFamily = runtime.signals.api({}).url("/users/:userId")
      .response(runtime.signals.resource.response.detail()({ name: "name" }))
      .detail({
        load: ({ userId }) => ({ id: userId, name: "First" }),
      });
    const residentLine = readFamily.line({ userId: "u1" });
    const saveUser = runtime.signals.api({}).url("/users/:userId")
      .response(runtime.signals.resource.response.detail()({ name: "name" }))
      .update({
        reconciles: [
          {
            family: readFamily,
            params: ({ userId }) => ({ userId }),
            fallback: "refetchRequired",
            detail: { kind: "field", field: "name" },
          },
        ],
        load: () => deferred.promise,
      });

    const saveLine = saveUser.line({
      userId: "u1",
      body: { name: "Server" },
    });
    residentLine.deliver(readFamily.delivery.field({
      packetId: "server-drift",
      basisId: null,
      nextBasisId: "basis-delivered",
      field: "name",
      value: "Delivered",
    }));
    deferred.resolve({ name: "Server" });
    await deferred.promise;
    await Promise.resolve();
    const plan = saveLine.mutationResponse();

    assert.equal(residentLine.value().name, "Delivered");
    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].staleness.reason, "basisChanged");
    assert.equal(plan.executionArtifacts[0].staleness.submittedBasisId, null);
    assert.equal(plan.executionArtifacts[0].staleness.currentBasisId, "basis-delivered");
    assert.equal(plan.counters.targetBasisSnapshotBreadth, 1);
    assert.equal(plan.counters.staleTargetDenialBreadth, 1);
    assert.equal(plan.confirmation.kind, "refetchRequired");
  } finally {
    await runtime.cleanup();
  }
});

test("pending save response falls back when the resident target rematerializes to a new line instance", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const deferred = createDeferred();
    const readFamily = runtime.signals.api({}).url("/profiles/:profileId")
      .response(runtime.signals.resource.response.detail()({ name: "name" }))
      .detail({
        load: ({ profileId }) => ({ id: profileId, name: "First" }),
      });
    const originalLine = readFamily.line({ profileId: "p1" });
    const originalRuntimeLineId = originalLine.descriptor().runtimeLineId;
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
        load: () => deferred.promise,
      });

    const saveLine = saveProfile.line({
      profileId: "p1",
      body: { name: "Server" },
    });
    originalLine.free();
    const rematerializedLine = readFamily.line({ profileId: "p1" });
    const rematerializedRuntimeLineId = rematerializedLine.descriptor().runtimeLineId;

    deferred.resolve({ name: "Server" });
    await deferred.promise;
    await Promise.resolve();
    const plan = saveLine.mutationResponse();

    assert.notEqual(rematerializedRuntimeLineId, originalRuntimeLineId);
    assert.equal(rematerializedLine.value().name, "First");
    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].fallback, "refetchRequired");
    assert.equal(plan.executionArtifacts[0].staleness.reason, "runtimeLineIdChanged");
    assert.equal(plan.executionArtifacts[0].submittedTarget.runtimeLineId, originalRuntimeLineId);
    assert.equal(plan.executionArtifacts[0].runtimeLineId, rematerializedRuntimeLineId);
    assert.equal(plan.counters.staleTargetDenialBreadth, 1);
    assert.equal(plan.confirmation.kind, "refetchRequired");
  } finally {
    await runtime.cleanup();
  }
});
