import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const readmePath = path.resolve(
  "crates/forge-signal-wasm/docs/README.md",
);

test("README entrypoint points readers at the feature-first happy path", async () => {
  const readme = fs.readFileSync(readmePath, "utf8");

  assert.match(readme, /start_here\.md/);
  assert.match(readme, /learn\/feature-index\.md/);
  assert.match(readme, /learn\/recipes\.md/);
  assert.match(readme, /resources\/transfers\.md/);
  assert.match(readme, /resources\/downloads\.md/);
  assert.match(readme, /resources\/line-inspection\.md/);
  assert.match(readme, /resources\/branch-native-effects\.md/);
  assert.match(readme, /resources\/raw-escape-hatch\.md/);
  assert.match(readme, /the one feature page that matches your task/i);
});
