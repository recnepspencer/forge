import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../signals_runtime/module_loading/load_signals_module.mjs";

test("shared route path grammar preserves api request-param and body companion lanes", async () => {
  const { importProductModule, cleanup } = await loadSignalsModule();

  try {
    const apiRoutePattern = await importProductModule("api/route/api_route_pattern.js");
    const requestParamsModule = await importProductModule("api/route/api_route_request_params.js");

    const readPattern = apiRoutePattern.parseApiRoutePattern("/workspaces/:workspaceId/tasks");
    const readParamsState = requestParamsModule.withDeclaredApiRouteRequestParams();
    const readBoundParams = apiRoutePattern.createRouteBoundParams(
      readPattern,
      readParamsState,
      {
        workspaceId: "w1",
        params: { q: "draft", page: 2 },
      },
      "forbidden",
    );
    assert.equal(
      apiRoutePattern.createRouteRequestPath(readPattern, readBoundParams),
      "/workspaces/w1/tasks",
    );

    const writePattern = apiRoutePattern.parseApiRoutePattern("/tasks/:taskId");
    const writeBoundParams = apiRoutePattern.createRouteBoundParams(
      writePattern,
      requestParamsModule.createApiRouteRequestParamsState(),
      {
        taskId: "t1",
        body: { title: "Updated" },
      },
      "required",
    );
    assert.equal(
      apiRoutePattern.createRouteRequestPath(writePattern, writeBoundParams),
      "/tasks/t1",
    );

    assert.throws(
      () => apiRoutePattern.createRouteBoundParams(
        writePattern,
        requestParamsModule.createApiRouteRequestParamsState(),
        {
          taskId: "t1",
          params: { q: "nope" },
          body: { title: "Updated" },
        },
        "required",
      ),
      /undeclared path param "params"/,
    );
    assert.throws(
      () => apiRoutePattern.createRouteRequestPath(
        readPattern,
        {
          ...readBoundParams,
          extra: "nope",
        },
      ),
      /undeclared path param "extra"/,
    );
  } finally {
    await cleanup();
  }
});
