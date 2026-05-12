import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const featureIndexPath = path.resolve(
  "crates/forge-signal-wasm/docs/feature_index.md",
);

test("feature index entrypoint keeps the shipped feature map discoverable", async () => {
  const featureIndex = fs.readFileSync(featureIndexPath, "utf8");

  assert.match(featureIndex, /multipart upload/i);
  assert.match(featureIndex, /multipart downloads/i);
  assert.match(featureIndex, /line\.summary\(\)/);
  assert.match(featureIndex, /feature_history_and_restore\.md/);
  assert.match(featureIndex, /feature_branch_native_resource_effects\.md/);
  assert.match(featureIndex, /feature_external_delivery_and_compatibility\.md/);
  assert.match(featureIndex, /feature_raw_escape_hatch\.md/);
  assert.match(featureIndex, /resource_recipes\.md/);
});
