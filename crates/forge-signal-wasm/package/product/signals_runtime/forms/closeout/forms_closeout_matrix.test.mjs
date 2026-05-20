import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const matrixPath = path.resolve(
  "docs/metadata/forms-closeout-matrix.json",
);
const inventoryPath = path.resolve(
  "docs/metadata/forms-feature-doc-inventory.json",
);

test("forms closeout matrix ties product families to machine-checkable evidence", () => {
  const matrix = JSON.parse(fs.readFileSync(matrixPath, "utf8"));
  const inventory = JSON.parse(fs.readFileSync(inventoryPath, "utf8"));

  assert.deepEqual(matrix.proofLanes, [
    "runtime",
    "typeSurface",
    "docs",
    "diagnosticsHistory",
    "resourceBranch",
    "renderer",
    "performance",
    "closeout",
  ]);
  assert.equal(matrix.rows.length, 5);
  assert.equal(inventory.features.length, 11);

  const docsRow = matrix.rows.find((row) => row.family === "docsAndCloseout");
  assert.ok(docsRow);
  assert.ok(docsRow.runtime.includes(
    "package/product/signals_runtime/forms/closeout/forms_feature_docs_happy_path.test.mjs",
  ));

  const resourceRow = matrix.rows.find((row) => row.family === "resourceLineAndCollaboration");
  assert.ok(resourceRow.resourceBranch.includes(
    "package/product/signals_runtime/forms/resource_source/state/form_resource_visible_selection_readback.test.mjs",
  ));
  assert.ok(resourceRow.renderer.includes(
    "package/product/signals_runtime/forms/resource_source/state/form_attachment_transfer_projection.test.mjs",
  ));

  for (const row of matrix.rows) {
    assert.equal(typeof row.family, "string");
    assert.ok(row.family.length > 0);

    for (const lane of matrix.proofLanes) {
      assert.ok(Array.isArray(row[lane]), `${row.family} ${lane} is not an array`);
      for (const ref of row[lane]) {
        assert.equal(typeof ref, "string");
        assert.ok(fs.existsSync(path.resolve(ref)), `${row.family} missing ${ref}`);
      }
    }
  }
});
