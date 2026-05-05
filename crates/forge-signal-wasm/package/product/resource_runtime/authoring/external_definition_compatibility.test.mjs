import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";
import { projectAuthoringConvergenceDigest } from "../closeout/resource_verification_package_helpers.mjs";

function createNativeCollectionDeclaration(mod, restoreState) {
  return {
    params: mod.resourceParams(),
    requestContext: mod.resourceRequestContext({ basisId: "basis-1" }),
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    itemIdentity: (item) => item.id,
    reconcile: mod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: mod.resourceItemAspects({
        title: {
          read: (item) => item.title,
          write: (item, title) => ({ ...item, title: String(title) }),
        },
      }),
    }),
    load: (_params, request) => ({
      items: [{
        id: "demo:1",
        title: restoreState.active
          ? "Restored Snapshot"
          : `Load:${request.context.basisId}`,
      }],
    }),
  };
}

function createExternalCollectionDefinition(mod, restoreState) {
  return {
    version: "forge-resource-external-v1",
    family: "collection",
    definitionId: "external-demo-collection",
    requestContract: "native-v1",
    reconciliationContract: "collection-v1",
    declaration: createNativeCollectionDeclaration(mod, restoreState),
  };
}

function projectExternalConvergenceDigest(line) {
  return projectAuthoringConvergenceDigest(line.history().verificationPackage());
}

function runMixedHistory(line, mod) {
  line.patch(
    mod.resourcePatch.itemAspect({
      itemId: "demo:1",
      aspect: "title",
      value: "Locally Patched",
    }),
  );
  line.refresh();
  line.deliver(
    mod.resourceDelivery.replace({
      packetId: "pkt-basis-2",
      basisId: "basis-1",
      nextBasisId: "basis-2",
      nextValue: {
        items: [{ id: "demo:1", title: "Delivered Basis 2" }],
      },
    }),
  );
  line.deliver(
    mod.resourceDelivery.patch({
      packetId: "pkt-basis-3",
      basisId: "basis-2",
      nextBasisId: "basis-3",
      patch: mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Delivered Basis 3",
      }),
    }),
  );
  return line.history().restoreExact();
}

test("native and external collection definitions converge on one line model across refresh, patch, delivery, and restore", async () => {
  const mod = await loadResourceModule();
  try {
    const restoreState = { active: false };
    const signalNamespace = createFakeSignalNamespace("root", {
      current_branch() {
        return {
          id: 88n,
          name: "external-compatibility",
          parent_branch_id: 11n,
          head_snapshot_id: 177n,
        };
      },
      restore_branch_snapshot_by_id() {
        restoreState.active = true;
      },
    });
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const nativeFamily = resource.collection(
      createNativeCollectionDeclaration(mod, restoreState),
    );
    const externalFamily = resource.compatibility.collection(
      createExternalCollectionDefinition(mod, restoreState),
    );
    const nativeLine = nativeFamily.line({ workspaceId: "demo" });
    const externalLine = externalFamily.line({ workspaceId: "demo" });
    assert.equal(nativeLine.descriptor().compatibility, undefined);
    assert.deepEqual(externalLine.descriptor().compatibility, {
      kind: "externalDefinition",
      version: "forge-resource-external-v1",
      definitionId: "external-demo-collection",
      requestContract: "native-v1",
      reconciliationContract: "collection-v1",
    });

    const nativeRestore = runMixedHistory(nativeLine, mod);
    restoreState.active = false;
    const externalRestore = runMixedHistory(externalLine, mod);

    assert.deepEqual(nativeRestore, {
      kind: "restored",
      mode: "SameRuntimeBranchExact",
      branchId: 88,
      snapshotId: 177,
      basisCurrentId: "basis-3",
      basisAdvanceCount: 2,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
    assert.deepEqual(externalRestore, nativeRestore);
    assert.deepEqual(
      projectExternalConvergenceDigest(externalLine),
      projectExternalConvergenceDigest(nativeLine),
    );
  } finally {
    await mod.cleanup();
  }
});

test("external resource definitions deny incompatible version or request contracts before materialization", async () => {
  const mod = await loadResourceModule();
  try {
    const resource = mod.createResourceNamespace(createFakeSignalNamespace(), {});

    assert.throws(
      () =>
        resource.compatibility.detail({
          version: "forge-resource-external-v0",
          family: "detail",
          definitionId: "bad-version",
          requestContract: "native-v1",
          reconciliationContract: "none",
          declaration: {
            params: mod.resourceParams(),
            normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
            load: ({ id }) => ({ id }),
          },
        }),
      /version must be "forge-resource-external-v1"/,
    );

    assert.throws(
      () =>
        resource.compatibility.detail({
          version: "forge-resource-external-v1",
          family: "detail",
          definitionId: "bad-request-contract",
          requestContract: "legacy-v0",
          reconciliationContract: "none",
          declaration: {
            params: mod.resourceParams(),
            normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
            load: ({ id }) => ({ id }),
          },
        }),
      /requestContract "native-v1"/,
    );

    assert.throws(
      () =>
        resource.compatibility.detail({
          version: "forge-resource-external-v1",
          family: "detail",
          definitionId: "",
          requestContract: "native-v1",
          reconciliationContract: "none",
          declaration: {
            params: mod.resourceParams(),
            normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
            load: ({ id }) => ({ id }),
          },
        }),
      /require non-empty definitionId/,
    );
  } finally {
    await mod.cleanup();
  }
});

test("external resource definitions deny unknown reconciliation contracts explicitly", async () => {
  const mod = await loadResourceModule();
  try {
    const resource = mod.createResourceNamespace(createFakeSignalNamespace(), {});

    assert.throws(
      () =>
        resource.compatibility.collection({
          version: "forge-resource-external-v1",
          family: "collection",
          definitionId: "bad-collection-contract",
          requestContract: "native-v1",
          reconciliationContract: "paged-v1",
          declaration: {
            params: mod.resourceParams(),
            normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
            itemIdentity: (item) => item.id,
            reconcile: mod.resourceCollectionShape({
              items: (value) => value.items,
              replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
            }),
            load: ({ id }) => ({ items: [{ id }] }),
          },
        }),
      /reconciliationContract "collection-v1"/,
    );

    assert.throws(
      () =>
        resource.compatibility.paged({
          version: "forge-resource-external-v1",
          family: "paged",
          definitionId: "bad-paged-contract",
          requestContract: "native-v1",
          reconciliationContract: "collection-v1",
          declaration: {
            params: mod.resourceParams(),
            normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
            itemIdentity: (item) => item.id,
            accumulatePage: (existing, next) => ({
              items: [...existing.items, ...next.items],
            }),
            reconcile: mod.resourceCollectionShape({
              items: (value) => value.items,
              replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
            }),
            load: ({ id }) => ({ items: [{ id }] }),
          },
        }),
      /reconciliationContract "paged-v1"/,
    );
  } finally {
    await mod.cleanup();
  }
});
