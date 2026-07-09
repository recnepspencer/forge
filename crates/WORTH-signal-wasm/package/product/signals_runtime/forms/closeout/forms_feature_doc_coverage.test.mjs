import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { formsDocsRoot as docsDir } from "./forms_docs_root.mjs";

const inventoryPath = path.join(docsDir, "metadata/forms-feature-doc-inventory.json");

function readDoc(name) {
  return fs.readFileSync(path.join(docsDir, name), "utf8");
}

test("forms feature doc inventory stays complete and wired into entrypoints and recipes", () => {
  const inventory = JSON.parse(fs.readFileSync(inventoryPath, "utf8"));
  const readme = readDoc("README.md");
  const startHere = readDoc("start_here.md");
  const featureIndex = readDoc("learn/feature-index.md");
  const recipes = readDoc("learn/recipes.md");
  const overview = readDoc("forms/index.md");

  assert.ok(Array.isArray(inventory.entrypoints));
  assert.ok(Array.isArray(inventory.features));
  assert.equal(inventory.features.length, 19);

  for (const entrypoint of inventory.entrypoints) {
    assert.equal(typeof entrypoint.id, "string");
    assert.ok(fs.existsSync(path.join(docsDir, entrypoint.doc)), entrypoint.doc);
  }

  for (const feature of inventory.features) {
    assert.equal(typeof feature.id, "string");
    assert.ok(fs.existsSync(path.join(docsDir, feature.doc)), feature.doc);
  }

  assert.match(readme, /forms\/getting-started\/README\.md/);
  assert.match(readme, /forms\/state\/README\.md/);
  assert.match(readme, /forms\/changes\/README\.md/);
  assert.match(readme, /forms\/validation\/README\.md/);
  assert.match(featureIndex, /forms\/changes\/patching-complex-edit-forms\.md/);
  assert.match(featureIndex, /forms\/validation\/async-validation\.md/);
  assert.match(overview, /patch a complex edit form/i);
  assert.match(startHere, /Forms Overview/);
  assert.match(recipes, /Recipe: Ordinary Local Form/);
  assert.match(recipes, /Recipe: Async Validation/);
  assert.match(recipes, /Recipe: Complex Edit Form With Nested Patch Truth/);
});
