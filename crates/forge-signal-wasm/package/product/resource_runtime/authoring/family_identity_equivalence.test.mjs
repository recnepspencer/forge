import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createPhase1FamilyCases } from "../runtime_fixture/phase1_family_cases.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("resource families reuse one line per canonical parameter identity", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const calls = [];
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId, productId }) =>
        mod.resourceParamIdentity(
          { workspaceId, productId },
          `${workspaceId}:${productId}`,
        ),
      load: ({ workspaceId, productId }) => {
        calls.push(`${workspaceId}:${productId}`);
        return { workspaceId, productId, label: `${workspaceId}/${productId}` };
      },
    });

    const first = detail.line({ workspaceId: "demo", productId: "p1" });
    const second = detail.line({ workspaceId: "demo", productId: "p1" });
    const third = detail.line({ workspaceId: "demo", productId: "p2" });

    assert.equal(first, second);
    assert.notEqual(first, third);
    assert.deepEqual(calls, ["demo:p1", "demo:p2"]);
    assert.equal(first.descriptor(), second.descriptor());
    assert.equal(first.descriptor().family.kind, "detail");
    assert.equal(first.descriptor().canonicalParams.canonicalKey, "demo:p1");
    assert.notEqual(
      first.descriptor().runtimeLineId,
      third.descriptor().runtimeLineId,
    );
  } finally {
    await mod.cleanup();
  }
});

test("resource param identity snapshots stay stable after caller mutation", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ filters }) =>
        mod.resourceParamIdentity({ filters }, filters.join(",")),
      load: ({ filters }) => ({ filters }),
    });

    const rawParams = { filters: ["open", "ready"] };
    const line = detail.line(rawParams);
    rawParams.filters.push("mutated");

    assert.deepEqual(line.descriptor().canonicalParams.params, {
      filters: ["open", "ready"],
    });
  } finally {
    await mod.cleanup();
  }
});

test("equivalent direct and helper-built declarations preserve the same semantic descriptor story", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const familyCases = createPhase1FamilyCases(resource, mod);

    for (const familyCase of familyCases) {
      const direct = familyCase.build(familyCase.directLoad);
      const helperBuilt = familyCase.build(familyCase.helperLoad);

      const directLine = direct.line({ productId: "p1" });
      const helperLine = helperBuilt.line({ productId: "p1" });

      const directArtifact = {
        kind: directLine.descriptor().family.kind,
        canonicalKey: directLine.descriptor().canonicalParams.canonicalKey,
        value: directLine.value(),
        status: directLine.status(),
        freshness: directLine.freshness(),
      };
      const helperArtifact = {
        kind: helperLine.descriptor().family.kind,
        canonicalKey: helperLine.descriptor().canonicalParams.canonicalKey,
        value: helperLine.value(),
        status: helperLine.status(),
        freshness: helperLine.freshness(),
      };

      assert.deepEqual(helperArtifact, directArtifact);
    }
  } finally {
    await mod.cleanup();
  }
});

test("resource line descriptors stay stable across family kinds while preserving distinct family identity", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace();
    const resource = mod.createResourceNamespace(signalNamespace, {});

    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });
    const collection = resource.collection({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      itemIdentity: (item) => item.id,
      load: ({ id }) => [{ id }],
    });
    const paged = resource.paged({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      itemIdentity: (item) => item.id,
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ id }) => [{ id }],
    });

    const detailDescriptor = detail.line({ id: "a" }).descriptor();
    const collectionDescriptor = collection.line({ id: "a" }).descriptor();
    const pagedDescriptor = paged.line({ id: "a" }).descriptor();

    assert.equal(detailDescriptor.family.kind, "detail");
    assert.equal(collectionDescriptor.family.kind, "collection");
    assert.equal(pagedDescriptor.family.kind, "paged");
    assert.notEqual(
      detailDescriptor.family.familyId,
      collectionDescriptor.family.familyId,
    );
    assert.notEqual(
      collectionDescriptor.family.familyId,
      pagedDescriptor.family.familyId,
    );
    assert.equal(detailDescriptor.canonicalParams.canonicalKey, "a");
    assert.equal(collectionDescriptor.canonicalParams.canonicalKey, "a");
    assert.equal(pagedDescriptor.canonicalParams.canonicalKey, "a");
  } finally {
    await mod.cleanup();
  }
});
