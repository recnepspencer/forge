import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("fetch and write doc happy path covers list params and standard create", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const api = runtime.signals.api({
      baseUrl: "/api",
      headers: {
        authorization: "Bearer shared-token",
      },
    });
    const users = api.url("/users").params().items((item) => item.id).list({
      load: ({ params }) => [{ id: `u:${params.search ?? "all"}`, name: "Ada" }],
    });
    const createUser = api.url("/users").create({
      load: ({ body }) => ({ id: body.userId, name: body.name }),
    });

    const listLine = users.line({ params: { search: "ada" } });
    const createLine = createUser.line({
      body: { userId: "u1", name: "Ada" },
    });

    assert.equal(listLine.value()[0].id, "u:ada");
    assert.deepEqual(createLine.value(), { id: "u1", name: "Ada" });
    assert.equal(createLine.request().method, "POST");
  } finally {
    await runtime.cleanup();
  }
});
