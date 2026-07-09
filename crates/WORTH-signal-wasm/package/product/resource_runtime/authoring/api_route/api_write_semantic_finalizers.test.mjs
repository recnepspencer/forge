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

test("api.url(...).verb(\"POST\").update(...) keeps update semantics while switching transport method", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const taskResponse = signals.resource.response.array({
      itemId: (item) => item.id,
      aspects: signals.resource.response.objectAspects()({
        title: "title",
        status: "status",
      }),
    });
    const tasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
      .response(taskResponse)
      .list({
        load: ({ workspaceId }) => [{ id: `${workspaceId}:1`, title: "Task", status: "open" }],
      });
    const line = tasks.line({ workspaceId: "demo" });

    const saveTask = signals.api({}).url("/tasks/:taskId").verb("POST")
      .response(signals.resource.response.detail()({
        status: "status",
        warnings: "warnings",
      }))
      .update({
        reconciles: [{
          family: tasks,
          params: () => ({ workspaceId: "demo" }),
          fallback: "partialReconciliation",
          collection: { kind: "item" },
        }],
        diagnostics: [{ kind: "warnings", field: "warnings" }],
        load: ({ taskId }) => ({
          id: String(taskId),
          title: "Task",
          status: "done",
          warnings: ["status changed"],
        }),
      });

    const saveLine = saveTask.line({ taskId: "demo:1", body: { status: "done" } });
    const mutationResponse = saveLine.mutationResponse();

    assert.equal(saveLine.request().method, "POST");
    assert.equal(mutationResponse.targets[0].reconciliation.kind, "item");
    assert.equal(mutationResponse.executionArtifacts[0].kind, "exactCollectionItem");
    assert.equal(line.value()[0].status, "done");
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

test("api.url(...).verb(\"POST\").remove(...) keeps remove semantics while switching transport method", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const taskResponse = signals.resource.response.collection({
      itemId: (item) => item.id,
      summaries: signalsMod.resourceValueSummaries({
        total: {
          read: (value) => value.total,
          write: (value, total) => ({ ...value, total }),
        },
      }),
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
    });
    const tasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
      .response(taskResponse)
      .list({
        load: () => ({
          items: [{ id: "t1", title: "First" }, { id: "t2", title: "Second" }],
          total: 2,
        }),
      });
    const line = tasks.line({ workspaceId: "demo" });

    const archiveTask = signals.api({}).url("/tasks/:taskId/archive").verb("POST")
      .response(signals.resource.response.summary()())
      .remove({
        reconciles: [{
          family: tasks,
          params: () => ({ workspaceId: "demo" }),
          fallback: "deletionUnavailable",
          collection: {
            kind: "delete",
            itemId: ({ taskId }) => String(taskId),
          },
        }, {
          family: tasks,
          params: () => ({ workspaceId: "demo" }),
          fallback: "refetchRequired",
          summary: { kind: "summary", summary: "total" },
        }],
        load: () => 1,
      });

    const archiveLine = archiveTask.line({ taskId: "t1" });
    const mutationResponse = archiveLine.mutationResponse();

    assert.equal(archiveLine.request().method, "POST");
    assert.equal(mutationResponse.targets[0].reconciliation.kind, "delete");
    assert.equal(mutationResponse.executionArtifacts[0].kind, "exactCollectionDelete");
    assert.deepEqual(line.value(), { items: [{ id: "t2", title: "Second" }], total: 1 });
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

test("api.url(...).mutation({ semantics, method }) admits honest update-like POST authoring", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const taskResponse = signals.resource.response.array({
      itemId: (item) => item.id,
      aspects: signals.resource.response.objectAspects()({
        status: "status",
      }),
    });
    const tasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
      .response(taskResponse)
      .list({
        load: ({ workspaceId }) => [{ id: `${workspaceId}:1`, status: "open" }],
      });
    const saveTask = signals.api({}).url("/tasks/:taskId")
      .response(signals.resource.response.detail()({
        status: "status",
      }))
      .mutation({
        semantics: "update",
        method: "POST",
        reconciles: [{
          family: tasks,
          params: () => ({ workspaceId: "demo" }),
          fallback: "partialReconciliation",
          collection: { kind: "item" },
        }],
        load: ({ taskId }) => ({ id: String(taskId), status: "done" }),
      });

    const saveLine = saveTask.line({ taskId: "demo:1", body: { status: "done" } });
    assert.equal(saveLine.request().method, "POST");
    assert.equal(saveLine.mutationResponse().targets[0].reconciliation.kind, "item");
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).command({ semantics, method }) admits fallback-only command routes without pretending to be CRUD", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const tasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
      .response(signals.resource.response.array({
        itemId: (item) => item.id,
        aspects: signals.resource.response.objectAspects()({
          status: "status",
        }),
      }))
      .list({
        load: ({ workspaceId }) => [{ id: `${workspaceId}:1`, status: "open" }],
      });

    const archiveTask = signals.api({}).url("/tasks/:taskId/archive")
      .response(signals.resource.response.summary()())
      .command({
        semantics: "relationshipUpdate",
        method: "POST",
        reconciles: [{
          family: tasks,
          params: () => ({ workspaceId: "demo" }),
          fallback: "refetchRequired",
        }],
        load: () => 1,
      });

    const archiveLine = archiveTask.line({ taskId: "demo:1", body: { archived: true } });
    const mutationResponse = archiveLine.mutationResponse();
    assert.equal(archiveLine.request().method, "POST");
    assert.equal(mutationResponse.source, 'api.url("/tasks/:taskId/archive").response(...).command(...)');
    assert.equal(mutationResponse.targets[0].reconciliation, null);

    assert.throws(
      () =>
        signals.api({}).url("/tasks/:taskId/archive")
          .response(signals.resource.response.summary()())
          .command({
            semantics: "command",
            method: "POST",
            reconciles: [{
              family: tasks,
              params: () => ({ workspaceId: "demo" }),
              fallback: "partialReconciliation",
              collection: { kind: "item" },
            }],
            load: () => 1,
          }),
      /only admits fallback-only reconciles/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("plain write routes deny reconciles(...) outside the mutation response lane", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const userRead = runtime.signals.api({}).url("/users/:userId").detail({
      load: ({ userId }) => ({ id: userId }),
    });

    assert.throws(
      () =>
        runtime.signals.api({}).url("/users/:userId").update({
          reconciles: [
            {
              family: userRead,
              params: ({ userId }) => ({ userId }),
              fallback: "refetchRequired",
            },
          ],
          load: ({ userId, body }) => ({ id: userId, name: body.name }),
        }),
      /owns reconciles\(\.\.\.\) only in the mutation response lane/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("plain write routes deny identity(...) outside the mutation response lane", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    assert.throws(
      () =>
        runtime.signals.api({}).url("/users").create({
          identity: {
            submitted: ({ body }) => body.id,
            canonical: (value) => value.id,
          },
          load: ({ body }) => body,
        }),
      /owns identity\(\.\.\.\) only in the mutation response lane/,
    );
  } finally {
    await runtime.cleanup();
  }
});
