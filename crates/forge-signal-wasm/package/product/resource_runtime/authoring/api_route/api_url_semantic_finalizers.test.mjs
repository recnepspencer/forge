import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";
import { normalizeRouteLineArtifact } from "./route_line_artifact_proof.mjs";

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

    assert.deepEqual(normalizeRouteLineArtifact(apiLine), normalizeRouteLineArtifact(rawLine));
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
      normalizeRouteLineArtifact(list.line({ workspaceId: "demo" })),
      normalizeRouteLineArtifact(rawList.line({ workspaceId: "demo" })),
    );
    assert.deepEqual(
      normalizeRouteLineArtifact(paged.line({ workspaceId: "demo" })),
      normalizeRouteLineArtifact(rawPaged.line({ workspaceId: "demo" })),
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
      /does not admit undeclared path param "search"/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).params(...) lowers request params into one explicit member shape and canonical identity", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const apiList = signals.api({})
      .url("/workspaces/:workspaceId/tasks")
      .params()
      .list({
        itemIdentity: (item) => item.id,
        load: ({ workspaceId, params }) => [
          { id: `${workspaceId}:${params.search ?? "all"}:${params.page ?? 1}` },
        ],
      });
    const rawList = signals.resource.collection({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId, params }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId, params },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/tasks?page=${encodeURIComponent(String(params.page))}&search=${encodeURIComponent(String(params.search))}`,
        ),
      itemIdentity: (item) => item.id,
      load: ({ workspaceId, params }) => [
        { id: `${workspaceId}:${params.search ?? "all"}:${params.page ?? 1}` },
      ],
    });

    const apiLine = apiList.line({
      workspaceId: "demo",
      params: {
        search: "ada",
        page: 2,
      },
    });
    const rawLine = rawList.line({
      workspaceId: "demo",
      params: {
        search: "ada",
        page: 2,
      },
    });

    assert.deepEqual(normalizeRouteLineArtifact(apiLine), normalizeRouteLineArtifact(rawLine));
    assert.equal(
      apiLine.descriptor().canonicalParams.canonicalKey,
      "/workspaces/demo/tasks?page=2&search=ada",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...) composes request target information from inherited baseUrl and resolved route path", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detail = runtime.signals.api({
      baseUrl: "/api",
    }).scope({
      baseUrl: "/v2",
    }).url("/users/:userId").params().detail({
      load: ({ userId, params }) => ({
        id: `${userId}:${params.search ?? "all"}`,
      }),
    });

    const line = detail.line({
      userId: "u1",
      params: {
        search: "ada",
      },
    });

    assert.deepEqual(line.request().target, {
      baseUrl: "/api/v2",
      requestPath: "/users/u1?search=ada",
      url: "/api/v2/users/u1?search=ada",
    });
    assert.deepEqual(line.request().sources.baseUrl, {
      sources: ["apiRoot.baseUrl", "apiScope[1].baseUrl"],
    });
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
    assert.throws(
      () => runtime.signals.api({}).url("/users/:userId/:userId"),
      /must not repeat path param "userId"/,
    );
    assert.throws(
      () => runtime.signals.api({}).url("/reports/:params").params(),
      /would collide with the request params lane/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...) rejects malformed route structure before lowering", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    assert.throws(
      () => runtime.signals.api({}).url("users"),
      /routes must start with \//,
    );
    assert.throws(
      () => runtime.signals.api({}).url("/users/"),
      /must not contain empty path segments/,
    );
    assert.throws(
      () => runtime.signals.api({}).url("/users//roles"),
      /must not contain empty path segments/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).params(...) rejects missing or malformed request-param input at runtime", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const list = runtime.signals.api({})
      .url("/users")
      .params()
      .list({
        itemIdentity: (item) => item.id,
        load: ({ params }) => [{ id: params.search ?? "all" }],
      });

    assert.throws(
      () => list.line({}),
      /requires an explicit params object/,
    );
    assert.throws(
      () => list.line({ params: [] }),
      /requires params to be a plain object/,
    );
  } finally {
    await runtime.cleanup();
  }
});
