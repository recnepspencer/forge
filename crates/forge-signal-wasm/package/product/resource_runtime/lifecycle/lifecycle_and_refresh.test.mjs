import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createDeferred } from "../runtime_fixture/deferred.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("resource lines refresh in place and record diagnostics", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let version = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId, version: ++version }),
    });

    const line = detail.line({ productId: "p1" });
    const refreshStatus = line.refresh();

    assert.deepEqual(line.value(), { id: "p1", version: 2 });
    assert.deepEqual(refreshStatus, {
      kind: "fulfilled",
      operation: "refresh",
    });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "refresh",
    });
    assert.deepEqual(line.freshness(), { kind: "fresh" });
    assert.deepEqual(line.diagnostics(), {
      policyProfileName: "stable",
      continuity: "preserveVisibleValue",
      freshnessPolicy: "stable",
      request: {
        auth: mod.resourceAuth.anonymous(),
        context: {
          headerNames: [],
          correlationId: null,
          branchId: null,
          basisId: null,
        },
        continuation: mod.resourceContinuation.none(),
        processingJob: mod.resourceProcessingJob.none(),
        uploadTransport: mod.resourceUploadTransport.none(),
      },
      basis: {
        currentBasisId: null,
        advanceCount: 0,
        lastAdvanceFromBasisId: null,
        lastAdvanceToBasisId: null,
      },
      processing: {
        kind: "ready",
        completionKind: "none",
        jobId: null,
        message: null,
      },
      upload: {
        kind: "ready",
        transportKind: "none",
        uploadId: null,
        descriptor: null,
        finalizeRequired: false,
        awaitingProcessing: false,
        message: null,
      },
      download: {
        count: 0,
        readyCount: 0,
        unavailableCount: 0,
        incompatibleCount: 0,
        descriptors: [],
      },
      lastOperation: "refresh",
      lastOutcome: "fulfilled",
      pendingOperation: null,
      refreshCount: 1,
      revalidateCount: 0,
      retryAttemptCount: 0,
      rejectionCount: 0,
      timeoutCount: 0,
      supersessionCount: 0,
      invalidationCount: 0,
      patchCount: 0,
      deliveryCount: 0,
      lastSupersededOperation: null,
      lastInvalidationCause: null,
      lastInvalidationScope: null,
      lastPatchKind: null,
      lastPatchScope: null,
      lastPatchedItemId: null,
      lastPatchedAspect: null,
      lastPatchedSummary: null,
      lastDeliveryKind: null,
      lastDeliveryScope: null,
      lastDeliveryPacketId: null,
      lastDeliveryBasisId: null,
      preservedVisibleValueOnLastRejection: false,
      lastTimeoutOperation: null,
      lastErrorMessage: null,
      visibleValueVersion: 2,
    });
  } finally {
    await mod.cleanup();
  }
});

test("immediately stale policy keeps freshness honest across revalidation", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let version = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.immediatelyStale(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId, version: ++version }),
    });

    const line = detail.line({ productId: "p1" });
    const revalidateStatus = line.revalidate();

    assert.deepEqual(revalidateStatus, {
      kind: "fulfilled",
      operation: "revalidate",
    });
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "policyProfile",
    });
    assert.equal(line.diagnostics().policyProfileName, "immediatelyStale");
    assert.equal(line.diagnostics().request.auth.kind, "anonymous");
    assert.equal(line.diagnostics().lastOperation, "revalidate");
    assert.equal(line.diagnostics().revalidateCount, 1);
  } finally {
    await mod.cleanup();
  }
});

test("refresh rejection preserves the visible value and records stale rejection truth", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let callCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        callCount += 1;
        if (callCount === 1) {
          return { id: productId, version: callCount };
        }
        throw new Error("refresh failed");
      },
    });

    const line = detail.line({ productId: "p1" });
    const refreshStatus = line.refresh();

    assert.deepEqual(line.value(), { id: "p1", version: 1 });
    assert.deepEqual(refreshStatus, {
      kind: "rejected",
      operation: "refresh",
      message: "refresh failed",
      continuity: "preservedVisibleValue",
    });
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "refreshRejected",
    });
    assert.equal(line.diagnostics().rejectionCount, 1);
    assert.equal(line.diagnostics().preservedVisibleValueOnLastRejection, true);
    assert.equal(line.diagnostics().lastErrorMessage, "refresh failed");
  } finally {
    await mod.cleanup();
  }
});

test("promise-backed refresh enters pending and preserves the visible value until settlement", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let callCount = 0;
    const deferred = createDeferred();
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        callCount += 1;
        if (callCount === 1) {
          return { id: productId, version: 1 };
        }
        return deferred.promise;
      },
    });

    const line = detail.line({ productId: "p1" });
    const refreshStatus = line.refresh();

    assert.deepEqual(refreshStatus, {
      kind: "pending",
      operation: "refresh",
      continuity: "preservedVisibleValue",
    });
    assert.deepEqual(line.value(), { id: "p1", version: 1 });
    assert.deepEqual(line.status(), refreshStatus);
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "refreshPending",
    });
    assert.equal(line.diagnostics().lastOutcome, "pending");
    assert.equal(line.diagnostics().pendingOperation, "refresh");

    deferred.resolve({ id: "p1", version: 2 });
    await deferred.promise;
    await Promise.resolve();

    assert.deepEqual(line.value(), { id: "p1", version: 2 });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "refresh",
    });
    assert.equal(line.diagnostics().pendingOperation, null);
    assert.equal(line.diagnostics().refreshCount, 1);
    assert.equal(line.diagnostics().visibleValueVersion, 2);
  } finally {
    await mod.cleanup();
  }
});

test("superseded pending refresh completions do not overwrite newer reload truth", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const firstDeferred = createDeferred();
    const secondDeferred = createDeferred();
    let reloadCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        if (reloadCount === 0) {
          reloadCount += 1;
          return { id: productId, version: 1 };
        }
        reloadCount += 1;
        return reloadCount === 2 ? firstDeferred.promise : secondDeferred.promise;
      },
    });

    const line = detail.line({ productId: "p1" });
    line.refresh();
    const secondRefreshStatus = line.refresh();

    assert.deepEqual(secondRefreshStatus, {
      kind: "pending",
      operation: "refresh",
      continuity: "preservedVisibleValue",
    });
    assert.equal(line.diagnostics().supersessionCount, 1);
    assert.equal(line.diagnostics().lastSupersededOperation, "refresh");

    firstDeferred.resolve({ id: "p1", version: 2 });
    await firstDeferred.promise;
    await Promise.resolve();
    assert.deepEqual(line.value(), { id: "p1", version: 1 });
    assert.equal(line.status().kind, "pending");

    secondDeferred.resolve({ id: "p1", version: 3 });
    await secondDeferred.promise;
    await Promise.resolve();

    assert.deepEqual(line.value(), { id: "p1", version: 3 });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "refresh",
    });
  } finally {
    await mod.cleanup();
  }
});


test("resource lines can be explicitly freed and rematerialized", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });

    const first = detail.line({ productId: "p1" });
    const firstRuntimeLineId = first.descriptor().runtimeLineId;
    first.free();

    const second = detail.line({ productId: "p1" });

    assert.notEqual(first, second);
    assert.notEqual(second.descriptor().runtimeLineId, firstRuntimeLineId);
  } finally {
    await mod.cleanup();
  }
});

test("freed resource lines reject further operations", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });

    const line = detail.line({ productId: "p1" });
    line.free();

    assert.throws(() => line.value(), /cannot be used after line\.free/);
    assert.throws(() => line.status(), /cannot be used after line\.free/);
    assert.throws(() => line.freshness(), /cannot be used after line\.free/);
    assert.throws(() => line.request(), /cannot be used after line\.free/);
    assert.throws(() => line.download(), /cannot be used after line\.free/);
    assert.throws(() => line.diagnostics(), /cannot be used after line\.free/);
    assert.throws(() => line.invalidate(), /cannot be used after line\.free/);
    assert.throws(() => line.refresh(), /cannot be used after line\.free/);
    assert.throws(() => line.revalidate(), /cannot be used after line\.free/);
    assert.throws(
      () => line.view((value) => value.id),
      /cannot be used after line\.free/,
    );
  } finally {
    await mod.cleanup();
  }
});

test("line free disposes line-scoped views with the owning lifecycle", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId, label: productId }),
    });

    const line = detail.line({ productId: "p1" });
    const view = line.view((value) => value.label);
    line.free();

    assert.throws(() => view(), /fake signal handle was used after free/);
  } finally {
    await mod.cleanup();
  }
});
