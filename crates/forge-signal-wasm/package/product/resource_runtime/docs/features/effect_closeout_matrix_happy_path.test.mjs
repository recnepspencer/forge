import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

const docPath = path.resolve(
  "crates/forge-signal-wasm/docs/resource-contracts/closeout-matrix.md",
);

test("effect closeout matrix doc covers profile proof lanes", async () => {
  const doc = fs.readFileSync(docPath, "utf8");

  assert.match(doc, /closeoutMatrix/);
  assert.match(doc, /ResourceEffectCloseoutMatrix/);
  assert.match(doc, /proof lanes/i);
  assert.match(doc, /unsupported rows/i);

  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    const matrix = signals.resource.effects.closeoutMatrix(
      signals.resource.effects.branchNative(),
    );
    const families = new Set(matrix.rows.map((row) => row.effectFamily));

    assert.equal(matrix.profileName, "branchNative");
    assert.ok(matrix.proofLanes.includes("runtime"));
    assert.ok(matrix.proofLanes.includes("branchMerge"));
    assert.ok(families.has("optimisticWrite"));
    assert.ok(families.has("mergeRebase"));
    assert.ok(families.has("diagnosticsHistory"));
  } finally {
    await runtime.cleanup();
  }
});
