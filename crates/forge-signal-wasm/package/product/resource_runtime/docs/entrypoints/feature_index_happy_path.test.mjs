import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const featureIndexPath = path.resolve(
  "crates/forge-signal-wasm/docs/learn/feature-index.md",
);

test("feature index entrypoint keeps the shipped feature map discoverable", async () => {
  const featureIndex = fs.readFileSync(featureIndexPath, "utf8");

  assert.match(featureIndex, /multipart upload/i);
  assert.match(featureIndex, /multipart downloads/i);
  assert.match(featureIndex, /line\.summary\(\)/);
  assert.match(featureIndex, /resource-contracts\/history-and-restore\.md/);
  assert.match(featureIndex, /resources\/branch-native-effects\.md/);
  assert.match(featureIndex, /resources\/external-delivery-and-compatibility\.md/);
  assert.match(featureIndex, /resources\/raw-escape-hatch\.md/);
  assert.match(featureIndex, /\.\/recipes\.md/);
});
