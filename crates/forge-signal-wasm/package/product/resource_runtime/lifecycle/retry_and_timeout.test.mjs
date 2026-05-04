import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createDeferred } from "../runtime_fixture/deferred.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("timeout policy marks pending refresh as timed out while preserving visible value", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const deferred = createDeferred();
    let callCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.timeoutFast(),
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
    line.refresh();
    await new Promise((resolve) => setTimeout(resolve, 0));

    assert.deepEqual(line.value(), { id: "p1", version: 1 });
    assert.deepEqual(line.status(), {
      kind: "timedOut",
      operation: "refresh",
      continuity: "preservedVisibleValue",
    });
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "refreshTimedOut",
    });
    assert.equal(line.diagnostics().timeoutCount, 1);
    assert.equal(line.diagnostics().lastTimeoutOperation, "refresh");
    assert.equal(line.diagnostics().lastOutcome, "timedOut");
  } finally {
    await mod.cleanup();
  }
});

test("retry policy retries one failed pending refresh before succeeding", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const firstDeferred = createDeferred();
    const secondDeferred = createDeferred();
    let callCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.retryOnce(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        callCount += 1;
        if (callCount === 1) {
          return { id: productId, version: 1 };
        }
        return callCount === 2 ? firstDeferred.promise : secondDeferred.promise;
      },
    });

    const line = detail.line({ productId: "p1" });
    const refreshStatus = line.refresh();

    assert.equal(refreshStatus.kind, "pending");
    firstDeferred.reject(new Error("temporary failure"));
    await Promise.resolve();
    secondDeferred.resolve({ id: "p1", version: 2 });
    await secondDeferred.promise;
    await Promise.resolve();

    assert.deepEqual(line.value(), { id: "p1", version: 2 });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "refresh",
    });
    assert.equal(line.diagnostics().retryAttemptCount, 1);
    assert.equal(line.diagnostics().rejectionCount, 0);
  } finally {
    await mod.cleanup();
  }
});

test("retry policy also retries synchronous refresh failure before succeeding", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    let callCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.retryOnce(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        callCount += 1;
        if (callCount === 1) {
          return { id: productId, version: 1 };
        }
        if (callCount === 2) {
          throw new Error("temporary sync failure");
        }
        return { id: productId, version: 2 };
      },
    });

    const line = detail.line({ productId: "p1" });
    const refreshStatus = line.refresh();

    assert.deepEqual(refreshStatus, {
      kind: "fulfilled",
      operation: "refresh",
    });
    assert.deepEqual(line.value(), { id: "p1", version: 2 });
    assert.equal(line.diagnostics().retryAttemptCount, 1);
    assert.equal(line.diagnostics().rejectionCount, 0);
  } finally {
    await mod.cleanup();
  }
});
