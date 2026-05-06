import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("start_here doc happy path materializes a route-first detail line and grouped summary", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const userDetail = runtime.signals.api({
      baseUrl: "/api",
    }).url("/users/:userId").detail({
      load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
    });

    const line = userDetail.line({ userId: "u1" });

    assert.deepEqual(line.value(), { id: "u1", name: "User u1" });
    assert.deepEqual(line.summary().request.target, {
      baseUrl: "/api",
      requestPath: "/users/u1",
      url: "/api/users/u1",
    });
  } finally {
    await runtime.cleanup();
  }
});
