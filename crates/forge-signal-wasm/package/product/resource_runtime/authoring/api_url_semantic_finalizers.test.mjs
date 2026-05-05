import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

function normalizeLineArtifact(line) {
  return {
    kind: line.descriptor().family.kind,
    canonicalKey: line.descriptor().canonicalParams.canonicalKey,
    canonicalParams: JSON.parse(
      JSON.stringify(line.descriptor().canonicalParams.params),
    ),
    value: JSON.parse(JSON.stringify(line.value())),
    request: normalizeRequest(line.request()),
  };
}

function normalizeRequest(request) {
  const snapshot = JSON.parse(JSON.stringify(request));
  delete snapshot.family.familyId;
  delete snapshot.sources;
  return snapshot;
}

test("api.url(...).detail(...) lowers to the same route-bound family truth as a raw detail declaration", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const apiDetail = signals.api({
      headers: {
        authorization: "Bearer shared",
      },
    }).url("/users/:userId").detail({
      headers: ({ userId }) => ({
        "x-user-id": String(userId),
      }),
      load: ({ userId }) => ({ id: userId }),
    });
    const rawDetail = signals.resource.detail({
      params: signalsMod.resourceParams(),
      requestContext: ({ userId }) =>
        signalsMod.resourceRequestContext({
          headers: {
            authorization: "Bearer shared",
            "x-user-id": String(userId),
          },
        }),
      normalizeParams: ({ userId }) =>
        signalsMod.resourceParamIdentity({ userId }, "/users/u1".replace("u1", encodeURIComponent(String(userId)))),
      load: ({ userId }) => ({ id: userId }),
    });

    const apiLine = apiDetail.line({ userId: "u1" });
    const rawLine = rawDetail.line({ userId: "u1" });

    assert.deepEqual(normalizeLineArtifact(apiLine), normalizeLineArtifact(rawLine));
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).list(...) and paged(...) preserve raw family identity and request truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const list = signals.api({}).url("/workspaces/:workspaceId/tasks").list({
      itemIdentity: (item) => item.id,
      load: ({ workspaceId }) => [{ id: workspaceId }],
    });
    const rawList = signals.resource.collection({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/tasks`,
        ),
      itemIdentity: (item) => item.id,
      load: ({ workspaceId }) => [{ id: workspaceId }],
    });
    const paged = signals.api({}).url("/workspaces/:workspaceId/tasks").paged({
      itemIdentity: (item) => item.id,
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ workspaceId }) => [{ id: workspaceId }],
    });
    const rawPaged = signals.resource.paged({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/tasks`,
        ),
      itemIdentity: (item) => item.id,
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ workspaceId }) => [{ id: workspaceId }],
    });

    assert.deepEqual(
      normalizeLineArtifact(list.line({ workspaceId: "demo" })),
      normalizeLineArtifact(rawList.line({ workspaceId: "demo" })),
    );
    assert.deepEqual(
      normalizeLineArtifact(paged.line({ workspaceId: "demo" })),
      normalizeLineArtifact(rawPaged.line({ workspaceId: "demo" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...) path params form stable canonical identity and deny missing or undeclared params", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detail = runtime.signals.api({})
      .url("/tenants/:tenantId/users/:userId")
      .detail({
        load: ({ tenantId, userId }) => ({ tenantId, userId }),
      });

    const line = detail.line({ tenantId: "acme", userId: "u1" });
    assert.equal(
      line.descriptor().canonicalParams.canonicalKey,
      "/tenants/acme/users/u1",
    );

    assert.throws(
      () => detail.line({ tenantId: "acme" }),
      /missing required path param "userId"/,
    );
    assert.throws(
      () => detail.line({ tenantId: "acme", userId: "u1", search: "x" }),
      /does not admit undeclared param "search" before params\(\.\.\.\) exists/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...) treats literal colon text inside a segment as literal route content", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detail = runtime.signals.api({})
      .url("/reports/export:csv")
      .detail({
        load: () => ({ ok: true }),
      });

    const line = detail.line({});
    assert.equal(
      line.descriptor().canonicalParams.canonicalKey,
      "/reports/export:csv",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...) admits the root route and keeps its canonical identity stable", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detail = runtime.signals.api({})
      .url("/")
      .detail({
        load: () => ({ ok: true }),
      });

    const line = detail.line({});
    assert.equal(line.descriptor().canonicalParams.canonicalKey, "/");
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...) rejects duplicate placeholders and route-lane params ceremony", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    assert.throws(
      () => signals.api({}).url("/users/:userId/:userId"),
      /must not repeat path param "userId"/,
    );
    assert.throws(
      () =>
        signals.api({}).url("/users/:userId").detail({
          params: signalsMod.resourceParams(),
          load: ({ userId }) => ({ id: userId }),
        }),
      /owns params\(\.\.\.\) in the route-first lane/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...) rejects malformed route structure before lowering", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    assert.throws(
      () => signals.api({}).url("/users/"),
      /must not contain empty path segments/,
    );
    assert.throws(
      () => signals.api({}).url("/users//roles"),
      /must not contain empty path segments/,
    );
    assert.throws(
      () => signals.api({}).url("/users/:1bad"),
      /must use :paramName placeholders/,
    );
    assert.throws(
      () => signals.api({}).url("/users/:user-id"),
      /must use :paramName placeholders/,
    );
  } finally {
    await runtime.cleanup();
  }
});
