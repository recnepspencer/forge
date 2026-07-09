import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";

test("resource line views stay attached to the owning line value", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({
        id: productId,
        inventory: { inStock: productId === "p1" },
      }),
    });

    const line = detail.line({ productId: "p1" });
    const lineSignal = line.signal();
    const inventoryView = line.view((product) => product.inventory.inStock);
    const identityView = line.view((product) => product.id);

    assert.deepEqual(lineSignal(), {
      id: "p1",
      inventory: { inStock: true },
    });
    assert.equal(inventoryView(), true);
    assert.equal(identityView(), "p1");
    assert.notEqual(inventoryView, identityView);
    assert.equal(line.descriptor().family.kind, "detail");
    assert.equal(line.descriptor().canonicalParams.canonicalKey, "p1");
  } finally {
    await runtime.cleanup();
  }
});

test("freeing a resource line disposes owned views", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ productId }) =>
        mod.resourceParamIdentity({ productId }, productId),
      load: ({ productId }) => ({
        id: productId,
        inventory: { inStock: productId === "p1" },
      }),
    });

    const line = detail.line({ productId: "p1" });
    const inventoryView = line.view((product) => product.inventory.inStock);
    const labelView = line.view((product) => product.id);

    line.free();

    assert.throws(
      () => inventoryView(),
      /resource line view cannot be used after line\.free/,
    );
    assert.throws(
      () => labelView(),
      /resource line view cannot be used after line\.free/,
    );
  } finally {
    await runtime.cleanup();
  }
});
