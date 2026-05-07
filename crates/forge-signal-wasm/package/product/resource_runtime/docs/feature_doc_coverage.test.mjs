import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const docsDir = path.resolve(
  "crates/forge-signal-wasm/docs",
);
const docTestsDir = path.resolve(
  "crates/forge-signal-wasm/package/product/resource_runtime/docs",
);
const inventoryPath = path.join(docsDir, "resource_feature_doc_inventory.json");

function readFile(name) {
  return fs.readFileSync(path.join(docsDir, name), "utf8");
}

test("feature doc inventory stays complete and wired into the main entrypoints", async () => {
  const inventory = JSON.parse(fs.readFileSync(inventoryPath, "utf8"));
  const readme = readFile("README.md");
  const startHere = readFile("start_here.md");
  const featureIndex = readFile("feature_index.md");
  const overview = readFile("api_resources_overview.md");
  const recipes = readFile("resource_recipes.md");

  assert.ok(Array.isArray(inventory.entrypoints));
  assert.ok(Array.isArray(inventory.features));
  assert.ok(inventory.entrypoints.length >= 5);
  assert.ok(inventory.features.length >= 9);

  for (const entrypoint of inventory.entrypoints) {
    assert.equal(typeof entrypoint.id, "string");
    assert.ok(fs.existsSync(path.join(docsDir, entrypoint.doc)), entrypoint.doc);
    assert.ok(fs.existsSync(path.join(docTestsDir, entrypoint.test)), entrypoint.test);
  }

  for (const feature of inventory.features) {
    assert.equal(typeof feature.id, "string");
    assert.ok(fs.existsSync(path.join(docsDir, feature.doc)), feature.doc);
    assert.ok(fs.existsSync(path.join(docTestsDir, feature.test)), feature.test);
    assert.match(featureIndex, new RegExp(feature.doc.replace(".", "\\.")));
    assert.match(readme, new RegExp(feature.doc.replace(".", "\\.")));
    assert.match(startHere, new RegExp(feature.doc.replace(".", "\\.")));
    assert.match(overview, new RegExp(feature.doc.replace(".", "\\.")));
    if (feature.doc !== "feature_line_inspection.md") {
      assert.match(recipes, new RegExp(feature.doc.replace(".", "\\.")));
    }
  }
});
