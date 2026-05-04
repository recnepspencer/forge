import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createDeferred } from "../runtime_fixture/deferred.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("promise-backed initial load reuses one pending line and settles in place", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const deferred = createDeferred();
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: () => deferred.promise,
    });

    const first = detail.line({ productId: "p1" });
    const second = detail.line({ productId: "p1" });

    assert.equal(first, second);
    assert.equal(first.value(), null);
    assert.deepEqual(first.status(), {
      kind: "pending",
      operation: "initialLoad",
      continuity: "noVisibleValueYet",
    });
    assert.deepEqual(first.freshness(), {
      kind: "stale",
      reason: "initialLoadPending",
    });

    deferred.resolve({ id: "p1", version: 1 });
    await deferred.promise;
    await Promise.resolve();

    assert.deepEqual(first.value(), { id: "p1", version: 1 });
    assert.deepEqual(first.status(), {
      kind: "fulfilled",
      operation: "initialLoad",
    });
  } finally {
    await mod.cleanup();
  }
});

test("initial async timeout remains honest about missing visible value", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const deferred = createDeferred();
    const detail = resource.detail({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.timeoutFast(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: () => deferred.promise,
    });

    const line = detail.line({ productId: "p1" });
    await new Promise((resolve) => setTimeout(resolve, 0));

    assert.equal(line.value(), null);
    assert.deepEqual(line.status(), {
      kind: "timedOut",
      operation: "initialLoad",
      continuity: "noVisibleValueYet",
    });
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "initialLoadTimedOut",
    });
  } finally {
    await mod.cleanup();
  }
});

test("refresh can supersede a pending initial load", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const initialDeferred = createDeferred();
    let callCount = 0;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        callCount += 1;
        if (callCount === 1) {
          return initialDeferred.promise;
        }
        return Promise.resolve({ id: productId, version: 2 });
      },
    });

    const line = detail.line({ productId: "p1" });
    const refreshStatus = line.refresh();

    assert.deepEqual(refreshStatus, {
      kind: "pending",
      operation: "refresh",
      continuity: "noVisibleValueYet",
    });
    assert.equal(line.diagnostics().lastSupersededOperation, "initialLoad");

    initialDeferred.resolve({ id: "p1", version: 1 });
    await initialDeferred.promise;
    await Promise.resolve();

    assert.deepEqual(line.value(), { id: "p1", version: 2 });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "refresh",
    });
  } finally {
    await mod.cleanup();
  }
});
