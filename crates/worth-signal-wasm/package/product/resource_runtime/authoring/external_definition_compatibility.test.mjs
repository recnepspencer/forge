import assert from "node:assert/strict";
import test from "node:test";

import {
  createBranchHead,
  createRealResourceNamespace,
  createRealResourceRuntime,
  installHistoryOverrides,
} from "../runtime_fixture/real_resource_signals.mjs";
import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";

async function settleRuntime() {
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

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
    version: "worth-resource-external-v1",
    family: "collection",
    definitionId: "external-demo-collection",
    requestContract: "native-v1",
    reconciliationContract: "collection-v1",
    declaration: createNativeCollectionDeclaration(mod, restoreState),
  };
}

function projectExternalConvergenceDigest(line) {
  const history = line.history();
  return {
    value: line.value(),
    status: line.status(),
    freshness: line.freshness(),
    requestBasisId: line.request().context.basisId,
    diagnostics: line.diagnosticsSummary(),
    basis: history.basis,
    availability: history.availability,
    lifecycleLength: history.lifecycle.length,
    lastLifecycleEvent: history.lifecycle.at(-1)?.event ?? null,
  };
}

function runMixedHistory(line, delivery, mod) {
  line.patch(
    mod.resourcePatch.itemAspect({
      itemId: "demo:1",
      aspect: "title",
      value: "Locally Patched",
    }),
  );
  line.refresh();
  line.deliver(
    delivery.replace({
      packetId: "pkt-basis-2",
      basisId: "basis-1",
      nextBasisId: "basis-2",
      nextValue: {
        items: [{ id: "demo:1", title: "Delivered Basis 2" }],
      },
    }),
  );
  line.deliver(
    delivery.patch({
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

test(
  "native and external collection definitions converge on one line model across refresh, patch, delivery, and restore",
  { concurrency: false },
  async () => {
  const runtime = await createRealResourceRuntime();
  let phase = "setup";
  try {
    const mod = runtime.resourceMod;
    const restoreState = { active: false };
    const branch = createBranchHead(runtime.signals, "external-compatibility");
    await settleRuntime();
    const snapshotId = Number(
      runtime.signals.history().branch_snapshot_id(BigInt(branch.id)),
    );
    const uninstallRestoreHook = installHistoryOverrides(runtime.signals, {
      restore_branch_snapshot_by_id(history, branchId, targetSnapshotId) {
        restoreState.active = true;
        return history.restore_branch_snapshot_by_id(branchId, targetSnapshotId);
      },
    });
    const resource = createRealResourceNamespace(mod, runtime.signals);
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
      version: "worth-resource-external-v1",
      definitionId: "external-demo-collection",
      requestContract: "native-v1",
      reconciliationContract: "collection-v1",
    });

    phase = "native restore";
    const nativeRestore = runMixedHistory(nativeLine, mod.resourceDelivery, mod);
    restoreState.active = false;
    phase = "external restore";
    const externalRestore = runMixedHistory(
      externalLine,
      resource.compatibility.delivery,
      mod,
    );

    phase = "assert restore";
    assert.deepEqual(nativeRestore, {
      kind: "restored",
      mode: "SameRuntimeBranchExact",
      branchId: branch.id,
      snapshotId,
      basisCurrentId: "basis-3",
      basisAdvanceCount: 2,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
    assert.deepEqual(externalRestore, nativeRestore);
    phase = "read digests";
    const nativeDigest = projectExternalConvergenceDigest(nativeLine);
    const externalDigest = projectExternalConvergenceDigest(externalLine);
    phase = "assert digests";
    assert.deepEqual(externalDigest.basis, nativeDigest.basis);
    assert.deepEqual(externalDigest.availability, nativeDigest.availability);
    assert.equal(externalDigest.lifecycleLength, nativeDigest.lifecycleLength);
    assert.equal(externalDigest.lastLifecycleEvent, nativeDigest.lastLifecycleEvent);
    assert.equal(externalDigest.requestBasisId, "basis-3");
    assert.deepEqual(externalDigest.status, {
      kind: "fulfilled",
      operation: "restore",
    });
    assert.deepEqual(externalDigest.value, {
      items: [{ id: "demo:1", title: "Restored Snapshot" }],
    });
    assert.equal(externalDigest.diagnostics.activity.lastOperation, "restore");
    phase = "free lines";
    nativeLine.free();
    externalLine.free();
    uninstallRestoreHook();
    phase = "settle";
    await settleRuntime();
  } catch (error) {
    throw new Error(`external compatibility phase failed: ${phase}`, {
      cause: error,
    });
  } finally {
    await runtime.cleanup();
  }
  },
);

test("external resource definitions deny incompatible version or request contracts before materialization", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;

    assert.throws(
      () =>
        resource.compatibility.detail({
          version: "worth-resource-external-v0",
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
      /version must be "worth-resource-external-v1"/,
    );

    assert.throws(
      () =>
        resource.compatibility.detail({
          version: "worth-resource-external-v1",
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
          version: "worth-resource-external-v1",
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
    await runtime.cleanup();
  }
});

test("external resource definitions deny unknown reconciliation contracts explicitly", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;

    assert.throws(
      () =>
        resource.compatibility.collection({
          version: "worth-resource-external-v1",
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
          version: "worth-resource-external-v1",
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
    await runtime.cleanup();
  }
});
