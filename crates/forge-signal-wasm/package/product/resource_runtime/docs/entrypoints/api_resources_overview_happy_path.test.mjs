import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

const overviewPath = path.resolve(
  "crates/forge-signal-wasm/docs/resources/overview.md",
);

test("API resources overview entrypoint points at the default lane and its feature homes", async () => {
  const overview = fs.readFileSync(overviewPath, "utf8");
  const runtime = await createRealRequestRuntime();
  try {
    const api = runtime.signals.api({
      baseUrl: "/api",
    });
    const userDetail = api.url("/users/:userId").detail({
      load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
    });

    const line = userDetail.line({ userId: "u1" });

    assert.equal(line.summary().current.status.kind, "fulfilled");
    assert.equal(line.summary().request.target.url, "/api/users/u1");

    assert.match(overview, /signals\.api\(\.\.\.\)/);
    assert.match(overview, /line\.summary\(\)/);
    assert.match(overview, /signals\.resource\.response\.array\(\.\.\.\)/);
    assert.match(overview, /signals\.resource\.response\.objectItems<T>\(\)\(\.\.\.\)/);
    assert.match(overview, /signals\.resource\.response\.collection<T>\(\)\(\.\.\.\)/);
    assert.match(overview, /\.\/transfers\.md/);
    assert.match(overview, /\.\/downloads\.md/);
    assert.match(overview, /\.\/branch-native-effects\.md/);
    assert.match(overview, /\.\/external-delivery-and-compatibility\.md/);
    assert.match(overview, /\.\.\/learn\/recipes\.md/);
  } finally {
    await runtime.cleanup();
  }
});
