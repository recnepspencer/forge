import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("raw escape hatch doc happy path keeps manual family authoring usable and line-centered", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const userDetail = runtime.signals.resource.detail({
      params: runtime.signalsMod.resourceParams(),
      normalizeParams: ({ userId }) =>
        runtime.signalsMod.resourceParamIdentity({ userId }, `/users/${userId}`),
      load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
    });

    const line = userDetail.line({ userId: "u1" });

    assert.deepEqual(line.value(), { id: "u1", name: "User u1" });
    assert.equal(line.summary().request.target.requestPath, null);
  } finally {
    await runtime.cleanup();
  }
});
