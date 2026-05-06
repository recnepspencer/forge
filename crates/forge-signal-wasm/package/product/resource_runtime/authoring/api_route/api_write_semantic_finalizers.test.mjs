import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";
import { normalizeRouteLineArtifact } from "./route_line_artifact_proof.mjs";

test("api.url(...).create(...) lowers to the same write-shaped detail truth as a raw detail declaration", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const apiCreate = signals.api({
      headers: {
        authorization: "Bearer shared",
      },
    }).url("/users").create({
      load: ({ body }) => ({ id: body.userId, name: body.name }),
    });
    const rawCreate = signals.resource.detail({
      params: signalsMod.resourceParams(),
      method: "POST",
      requestBody: ({ body }) => body,
      requestContext: signalsMod.resourceRequestContext({
        headers: {
          authorization: "Bearer shared",
        },
      }),
      normalizeParams: ({ body }) =>
        signalsMod.resourceParamIdentity(
          { body },
          '/users#body={"name":"Ada","userId":"u1"}',
        ),
      load: ({ body }) => ({ id: body.userId, name: body.name }),
    });

    const apiLine = apiCreate.line({
      body: {
        userId: "u1",
        name: "Ada",
      },
    });
    const rawLine = rawCreate.line({
      body: {
        userId: "u1",
        name: "Ada",
      },
    });

    assert.deepEqual(normalizeRouteLineArtifact(apiLine), normalizeRouteLineArtifact(rawLine));
    assert.equal(apiLine.request().method, "POST");
    assert.deepEqual(apiLine.request().body, {
      userId: "u1",
      name: "Ada",
    });
    assert.deepEqual(apiLine.request().target, {
      baseUrl: null,
      requestPath: "/users",
      url: "/users",
    });
    assert.equal(apiLine.diagnostics().request.method, "POST");
    assert.equal(apiLine.diagnostics().request.bodyPresent, true);
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).update(...) preserves path-param identity while making body identity explicit", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const apiUpdate = signals.api({}).url("/users/:userId").update({
      load: ({ userId, body }) => ({ id: userId, name: body.name }),
    });
    const rawUpdate = signals.resource.detail({
      params: signalsMod.resourceParams(),
      method: "PUT",
      requestBody: ({ body }) => body,
      normalizeParams: ({ userId, body }) =>
        signalsMod.resourceParamIdentity(
          { userId, body },
          '/users/u1#body={"name":"Grace"}',
        ),
      load: ({ userId, body }) => ({ id: userId, name: body.name }),
    });

    const apiLine = apiUpdate.line({
      userId: "u1",
      body: {
        name: "Grace",
      },
    });
    const rawLine = rawUpdate.line({
      userId: "u1",
      body: {
        name: "Grace",
      },
    });

    assert.deepEqual(normalizeRouteLineArtifact(apiLine), normalizeRouteLineArtifact(rawLine));
    assert.equal(
      apiLine.descriptor().canonicalParams.canonicalKey,
      '/users/u1#body={"name":"Grace"}',
    );
    assert.deepEqual(apiLine.request().target, {
      baseUrl: null,
      requestPath: "/users/u1",
      url: "/users/u1",
    });
    assert.equal(apiLine.request().method, "PUT");
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).remove(...) stays in the same grammar and lowers to delete-shaped request truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const apiRemove = signals.api({}).url("/users/:userId").remove({
      load: ({ userId }) => ({ removed: userId }),
    });
    const rawRemove = signals.resource.detail({
      params: signalsMod.resourceParams(),
      method: "DELETE",
      normalizeParams: ({ userId }) =>
        signalsMod.resourceParamIdentity(
          { userId },
          `/users/${encodeURIComponent(String(userId))}`,
        ),
      load: ({ userId }) => ({ removed: userId }),
    });

    const apiLine = apiRemove.line({ userId: "u1" });
    const rawLine = rawRemove.line({ userId: "u1" });

    assert.deepEqual(normalizeRouteLineArtifact(apiLine), normalizeRouteLineArtifact(rawLine));
    assert.equal(apiLine.request().method, "DELETE");
    assert.equal(apiLine.request().body, null);
    assert.equal(apiLine.diagnostics().request.bodyPresent, false);
  } finally {
    await runtime.cleanup();
  }
});

test("custom action routes can stay inside the write grammar without falling back to raw declarations", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const exportUsers = runtime.signals.api({
      baseUrl: "/api",
    }).url("/users/export").create({
      load: ({ body }) => ({ jobId: body.jobId }),
    });

    const line = exportUsers.line({
      body: {
        jobId: "job-1",
      },
    });

    assert.equal(line.request().method, "POST");
    assert.deepEqual(line.request().target, {
      baseUrl: "/api",
      requestPath: "/users/export",
      url: "/api/users/export",
    });
    assert.equal(
      line.descriptor().canonicalParams.canonicalKey,
      '/users/export#body={"jobId":"job-1"}',
    );
  } finally {
    await runtime.cleanup();
  }
});
