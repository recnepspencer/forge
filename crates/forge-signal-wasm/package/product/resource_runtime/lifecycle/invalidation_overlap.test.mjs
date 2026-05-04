import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createDeferred } from "../runtime_fixture/deferred.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("duplicate invalidation and pending invalidation overlap stay breadth-honest", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const deferred = createDeferred();
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
        return deferred.promise;
      },
    });

    const line = detail.line({ productId: "p1" });
    line.invalidate();
    line.invalidate();
    assert.equal(line.diagnostics().invalidationCount, 2);
    assert.equal(line.diagnostics().lastInvalidationScope, "line");
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "manualLineInvalidate",
    });

    line.refresh();
    line.invalidate();

    assert.equal(line.diagnostics().pendingOperation, "refresh");
    assert.equal(line.diagnostics().invalidationCount, 3);
    assert.equal(line.diagnostics().lastInvalidationCause, "manualLineInvalidate");
    assert.equal(line.history().lifecycle.at(-1)?.event, "invalidated");

    deferred.resolve({ id: "p1", version: 2 });
    await deferred.promise;
    await Promise.resolve();

    assert.deepEqual(line.value(), { id: "p1", version: 2 });
    assert.deepEqual(line.freshness(), { kind: "fresh" });
  } finally {
    await mod.cleanup();
  }
});

test("family invalidation during pending reload does not silently broaden siblings", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const firstDeferred = createDeferred();
    let callCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        callCount += 1;
        if (callCount <= 2) {
          return { id: productId, version: 1 };
        }
        return firstDeferred.promise;
      },
    });

    const first = detail.line({ productId: "p1" });
    const second = detail.line({ productId: "p2" });
    first.refresh();
    const invalidated = detail.invalidate({ productId: "p1" });

    assert.equal(invalidated, true);
    assert.equal(first.diagnostics().pendingOperation, "refresh");
    assert.equal(first.diagnostics().lastInvalidationScope, "familyMember");
    assert.deepEqual(second.freshness(), { kind: "fresh" });
    assert.equal(second.diagnostics().invalidationCount, 0);

    firstDeferred.resolve({ id: "p1", version: 2 });
    await firstDeferred.promise;
    await Promise.resolve();

    assert.deepEqual(first.value(), { id: "p1", version: 2 });
    assert.deepEqual(second.value(), { id: "p2", version: 1 });
  } finally {
    await mod.cleanup();
  }
});

test("timeout and later invalidation keep stale completion from rewriting the line", async () => {
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

    const line = detail.line({ productId: "p-timeout" });
    line.refresh();
    await new Promise((resolve) => setTimeout(resolve, 0));

    assert.deepEqual(line.status(), {
      kind: "timedOut",
      operation: "refresh",
      continuity: "preservedVisibleValue",
    });
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "refreshTimedOut",
    });

    line.invalidate();
    assert.deepEqual(line.status(), {
      kind: "timedOut",
      operation: "refresh",
      continuity: "preservedVisibleValue",
    });
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "manualLineInvalidate",
    });
    assert.equal(line.diagnostics().lastInvalidationCause, "manualLineInvalidate");

    deferred.resolve({ id: "p-timeout", version: 2 });
    await deferred.promise;
    await Promise.resolve();

    assert.deepEqual(line.value(), { id: "p-timeout", version: 1 });
    assert.deepEqual(line.status(), {
      kind: "timedOut",
      operation: "refresh",
      continuity: "preservedVisibleValue",
    });
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "manualLineInvalidate",
    });
  } finally {
    await mod.cleanup();
  }
});
