import assert from "node:assert/strict";
import test from "node:test";

import { createRealLifecycleRuntime } from "../runtime_fixture/real_lifecycle_runtime.mjs";

test("line invalidation marks freshness stale and records line-scoped diagnostics", async () => {
  const runtime = await createRealLifecycleRuntime();
  try {
    const { mod, resource } = runtime;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });

    const line = detail.line({ productId: "p1" });
    const freshness = line.invalidate();

    assert.deepEqual(freshness, {
      kind: "stale",
      reason: "manualLineInvalidate",
    });
    assert.deepEqual(line.freshness(), {
      kind: "stale",
      reason: "manualLineInvalidate",
    });
    assert.equal(line.diagnostics().invalidationCount, 1);
    assert.equal(
      line.diagnostics().lastInvalidationCause,
      "manualLineInvalidate",
    );
    assert.equal(line.diagnostics().lastInvalidationScope, "line");
  } finally {
    await runtime.cleanup();
  }
});

test("family invalidation narrows to one existing member without broadening siblings", async () => {
  const runtime = await createRealLifecycleRuntime();
  try {
    const { mod, resource } = runtime;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });

    const first = detail.line({ productId: "p1" });
    const second = detail.line({ productId: "p2" });
    const invalidated = detail.invalidate({ productId: "p1" });

    assert.equal(invalidated, true);
    assert.deepEqual(first.freshness(), {
      kind: "stale",
      reason: "manualFamilyInvalidate",
    });
    assert.deepEqual(second.freshness(), { kind: "fresh" });
    assert.equal(first.diagnostics().lastInvalidationScope, "familyMember");
    assert.equal(second.diagnostics().invalidationCount, 0);
    assert.equal(detail.invalidate({ productId: "missing" }), false);
  } finally {
    await runtime.cleanup();
  }
});

test("family invalidateAll marks every materialized line and returns the breadth honestly", async () => {
  const runtime = await createRealLifecycleRuntime();
  try {
    const { mod, resource } = runtime;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({ id: productId }),
    });

    const first = detail.line({ productId: "p1" });
    const second = detail.line({ productId: "p2" });
    const breadth = detail.invalidateAll();

    assert.equal(breadth, 2);
    assert.deepEqual(first.freshness(), {
      kind: "stale",
      reason: "manualFamilyInvalidateAll",
    });
    assert.deepEqual(second.freshness(), {
      kind: "stale",
      reason: "manualFamilyInvalidateAll",
    });
    assert.equal(first.diagnostics().lastInvalidationScope, "familyAll");
    assert.equal(second.diagnostics().lastInvalidationScope, "familyAll");
  } finally {
    await runtime.cleanup();
  }
});
