import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createDeferred } from "../runtime_fixture/deferred.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("omitted policy and explicit stable policy lower to the same runtime truth", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});

    function createDetail(policy) {
      return resource.detail({
        params: mod.resourceParams(),
        policy,
        normalizeParams: ({ productId }) =>
          mod.resourceParamIdentity({ productId }, productId),
        load: ({ productId }) => ({ id: productId, version: 1 }),
      });
    }

    const implicitStable = createDetail(undefined);
    const explicitStable = createDetail(mod.resourcePolicyProfiles.stable());

    const implicitLine = implicitStable.line({ productId: "p1" });
    const explicitLine = explicitStable.line({ productId: "p1" });

    assert.equal(implicitLine.diagnostics().policyProfileName, "stable");
    assert.equal(explicitLine.diagnostics().policyProfileName, "stable");
    assert.equal(implicitLine.diagnostics().freshnessPolicy, "stable");
    assert.equal(explicitLine.diagnostics().freshnessPolicy, "stable");
    assert.deepEqual(implicitLine.status(), explicitLine.status());
    assert.deepEqual(implicitLine.freshness(), explicitLine.freshness());
  } finally {
    await mod.cleanup();
  }
});

test("named policy profiles lower to distinct lifecycle truth and remain family-agnostic", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const timeoutDeferred = createDeferred();
    let timeoutLoadCount = 0;

    const immediatelyStale = resource.collection({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.immediatelyStale(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      itemIdentity: (item) => item.id,
      load: ({ productId }) => [{ id: productId, version: 1 }],
    });
    const retryOnce = resource.paged({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.retryOnce(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      itemIdentity: (item) => item.id,
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ productId }) => [{ id: productId, version: 1 }],
    });
    const timeoutFast = resource.detail({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.timeoutFast(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => {
        timeoutLoadCount += 1;
        if (timeoutLoadCount === 1) {
          return { id: productId, version: 1 };
        }
        return timeoutDeferred.promise;
      },
    });

    const staleLine = immediatelyStale.line({ productId: "p1" });
    staleLine.revalidate();
    assert.equal(staleLine.diagnostics().policyProfileName, "immediatelyStale");
    assert.deepEqual(staleLine.freshness(), {
      kind: "stale",
      reason: "policyProfile",
    });

    const retryLine = retryOnce.line({ productId: "p1" });
    assert.equal(retryLine.diagnostics().policyProfileName, "retryOnce");

    const timeoutLine = timeoutFast.line({ productId: "p1" });
    timeoutLine.refresh();
    await new Promise((resolve) => setTimeout(resolve, 0));
    assert.equal(timeoutLine.diagnostics().policyProfileName, "timeoutFast");
    assert.deepEqual(timeoutLine.status(), {
      kind: "timedOut",
      operation: "refresh",
      continuity: "preservedVisibleValue",
    });
  } finally {
    await mod.cleanup();
  }
});
