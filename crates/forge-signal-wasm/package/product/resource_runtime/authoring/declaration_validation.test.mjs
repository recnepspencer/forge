import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";

test("family declaration validation keeps detail, collection, and paged shapes distinct", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { mod, resource } = runtime;

    assert.throws(
      () =>
        resource.detail({
          params: mod.resourceParams(),
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          itemIdentity: (item) => item.id,
          load: ({ productId }) => ({ id: productId }),
        }),
      /must not declare itemIdentity/,
    );

    assert.throws(
      () =>
        resource.collection({
          params: mod.resourceParams(),
          normalizeParams: ({ workspaceId }) =>
            mod.resourceParamIdentity({ workspaceId }, workspaceId),
          load: ({ workspaceId }) => [{ id: workspaceId }],
        }),
      /require itemIdentity/,
    );

    assert.throws(
      () =>
        resource.paged({
          params: mod.resourceParams(),
          normalizeParams: ({ workspaceId }) =>
            mod.resourceParamIdentity({ workspaceId }, workspaceId),
          itemIdentity: (item) => item.id,
          load: ({ workspaceId }) => [{ id: workspaceId }],
        }),
      /require accumulatePage/,
    );

    assert.throws(
      () =>
        resource.detail({
          params: mod.resourceParams(),
          reconcile: mod.resourceCollectionShape({
            items: (value) => value.items,
            replaceItems: (value, nextItems) => ({ ...value, items: nextItems }),
          }),
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          load: ({ productId }) => ({ id: productId }),
        }),
      /must not declare reconcile/,
    );

    assert.throws(
      () =>
        resource.collection({
          params: mod.resourceParams(),
          normalizeParams: ({ workspaceId }) =>
            mod.resourceParamIdentity({ workspaceId }, workspaceId),
          itemIdentity: (item) => item.id,
          reconcile: {
            items: (value) => value.items,
            replaceItems: (value, nextItems) => ({ ...value, items: nextItems }),
          },
          load: ({ workspaceId }) => [{ id: workspaceId }],
        }),
      /reconcile created with resourceCollectionShape/,
    );

    assert.throws(
      () =>
        resource.collection({
          params: mod.resourceParams(),
          normalizeParams: ({ workspaceId }) =>
            mod.resourceParamIdentity({ workspaceId }, workspaceId),
          itemIdentity: (item) => item.id,
          reconcile: mod.resourceCollectionShape({
            items: (value) => value.items,
            replaceItems: (value, nextItems) => ({ ...value, items: nextItems }),
            summaries: {
              total: {
                read: (value) => value.total,
                write: (value, total) => ({ ...value, total }),
              },
            },
          }),
          load: ({ workspaceId }) => ({
            items: [{ id: workspaceId }],
            total: 1,
          }),
        }),
      /resourceCollectionShape\(\.\.\.\) requires summaries created with resourceValueSummaries/,
    );

    assert.throws(
      () =>
        resource.paged({
          params: mod.resourceParams(),
          normalizeParams: ({ workspaceId }) =>
            mod.resourceParamIdentity({ workspaceId }, workspaceId),
          itemIdentity: (item) => item.id,
          accumulatePage: (existing, next) => [...existing, ...next],
          reconcile: mod.resourceCollectionShape({
            items: (value) => value.items,
            replaceItems: (value, nextItems) => ({ ...value, items: nextItems }),
            aspects: {
              title: {
                read: (item) => item.title,
                write: (item, value) => ({ ...item, title: value }),
              },
            },
          }),
          load: ({ workspaceId }) => ({
            items: [{ id: workspaceId, title: workspaceId }],
          }),
        }),
      /resourceCollectionShape\(\.\.\.\) requires aspects created with resourceItemAspects/,
    );

    assert.throws(
      () =>
        resource.detail({
          params: mod.resourceParams(),
          policy: { name: "stable" },
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          load: ({ productId }) => ({ id: productId }),
        }),
      /policy created with resourcePolicyProfiles/,
    );

    assert.throws(
      () =>
        resource.detail({
          params: mod.resourceParams(),
          auth: { kind: "authenticated" },
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          load: ({ productId }) => ({ id: productId }),
        }),
      /auth created with resourceAuth/,
    );

    assert.throws(
      () =>
        resource.detail({
          params: mod.resourceParams(),
          requestContext: { headers: { "x-trace-id": "trace-1" } },
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          load: ({ productId }) => ({ id: productId }),
        }),
      /requestContext created with resourceRequestContext/,
    );

    assert.throws(
      () =>
        resource.detail({
          params: mod.resourceParams(),
          continuation: { kind: "redirect", returnTo: "/done" },
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          load: ({ productId }) => ({ id: productId }),
        }),
      /continuation created with resourceContinuation/,
    );

    assert.throws(
      () =>
        resource.detail({
          params: mod.resourceParams(),
          processingJob: { kind: "poll" },
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          load: ({ productId }) => ({ id: productId }),
        }),
      /processingJob created with resourceProcessingJob/,
    );

    assert.throws(
      () =>
        resource.detail({
          params: mod.resourceParams(),
          uploadTransport: {
            kind: "signed",
            method: "PUT",
            finalizeRequired: true,
          },
          normalizeParams: ({ productId }) =>
            mod.resourceParamIdentity({ productId }, productId),
          load: ({ productId }) => ({ id: productId }),
        }),
      /uploadTransport created with resourceUploadTransport/,
    );
  } finally {
    await runtime.cleanup();
  }
});
