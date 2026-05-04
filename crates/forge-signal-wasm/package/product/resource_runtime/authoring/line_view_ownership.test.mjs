import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("resource line views stay attached to the owning line value", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
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
    await mod.cleanup();
  }
});

test("freeing a resource line disposes owned views", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
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

    assert.throws(() => inventoryView(), /fake signal handle was used after free/);
    assert.throws(() => labelView(), /fake signal handle was used after free/);
  } finally {
    await mod.cleanup();
  }
});
