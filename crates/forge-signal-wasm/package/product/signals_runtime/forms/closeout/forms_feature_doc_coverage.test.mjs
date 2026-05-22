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
  const overview = readDoc("forms/overview.md");

  assert.ok(Array.isArray(inventory.entrypoints));
  assert.ok(Array.isArray(inventory.features));
  assert.equal(inventory.features.length, 11);

  for (const entrypoint of inventory.entrypoints) {
    assert.equal(typeof entrypoint.id, "string");
    assert.ok(fs.existsSync(path.join(docsDir, entrypoint.doc)), entrypoint.doc);
  }

  for (const feature of inventory.features) {
    assert.equal(typeof feature.id, "string");
    assert.ok(fs.existsSync(path.join(docsDir, feature.doc)), feature.doc);
    const docName = path.basename(feature.doc);
    assert.match(featureIndex, new RegExp(docName.replace(".", "\\.")));
    assert.match(readme, new RegExp(feature.doc.replace(".", "\\.")));
    assert.match(overview, new RegExp(docName.replace(".", "\\.")));
  }

  assert.match(startHere, /Forms Overview/);
  assert.match(recipes, /Recipe: Ordinary Local Form/);
  assert.match(recipes, /Recipe: Resource-Backed Form/);
  assert.match(recipes, /Recipe: Async Validation/);
  assert.match(recipes, /Recipe: Host Facts And Generated Layout/);
  assert.match(recipes, /Recipe: Collaboration Posture/);
  assert.match(recipes, /Recipe: Submit Lifecycle With Canonical Fulfillment/);
});
