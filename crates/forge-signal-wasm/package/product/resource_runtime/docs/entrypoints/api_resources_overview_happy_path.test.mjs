import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

const overviewPath = path.resolve(
  "crates/forge-signal-wasm/docs/api_resources_overview.md",
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
    assert.match(overview, /feature_transfers\.md/);
    assert.match(overview, /feature_downloads\.md/);
    assert.match(overview, /feature_external_delivery_and_compatibility\.md/);
    assert.match(overview, /resource_recipes\.md/);
  } finally {
    await runtime.cleanup();
  }
});
