import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { formsDocsRoot as docsDir } from "./forms_docs_root.mjs";

const inventoryPath = path.join(docsDir, "metadata/forms-feature-doc-inventory.json");
const publicManifestPath = path.join(docsDir, "metadata/public-documentation.json");

function readDoc(name) {
  return fs.readFileSync(path.join(docsDir, name), "utf8");
}

test("forms feature doc inventory covers every form page and every canonical public guide", () => {
  const inventory = JSON.parse(fs.readFileSync(inventoryPath, "utf8"));
  const publicManifest = JSON.parse(fs.readFileSync(publicManifestPath, "utf8"));
  const overview = readDoc("forms/index.md");

  assert.equal(inventory.schemaVersion, 2);
  assert.ok(Array.isArray(inventory.entrypoints));
  assert.ok(Array.isArray(inventory.features));
  assert.equal(inventory.features.length, 13);

  for (const entrypoint of inventory.entrypoints) {
    assert.equal(typeof entrypoint.id, "string");
    assert.ok(fs.existsSync(path.join(docsDir, entrypoint.doc)), entrypoint.doc);
  }

  for (const feature of inventory.features) {
    assert.equal(typeof feature.id, "string");
    assert.ok(fs.existsSync(path.join(docsDir, feature.canonicalDoc)), feature.canonicalDoc);
    assert.ok(Array.isArray(feature.docRoots));
  }

  const entrypointDocs = new Set(inventory.entrypoints.map((entrypoint) => entrypoint.doc));
  const docRoots = inventory.features.flatMap((feature) => feature.docRoots);
  const formsRoot = path.join(docsDir, "forms");
  const formDocs = walkMarkdown(formsRoot).map((doc) => (
    path.relative(docsDir, doc).replaceAll(path.sep, "/")
  ));

  for (const doc of formDocs) {
    const docText = readDoc(doc);
    assert.equal(
      entrypointDocs.has(doc) || docRoots.some((root) => doc.startsWith(root)),
      true,
      `orphaned Forms documentation: ${doc}`,
    );
    assert.doesNotMatch(
      docText,
      /const\s+\w+\s*=\s*form\.executeAction\(/u,
      `${doc} reads the action union without awaiting it`,
    );
  }

  const publicForms = publicManifest.sections.find((section) => section.id === "forms");
  assert.ok(publicForms);
  const publicPaths = new Set(publicForms.items.map((item) => `${item.path}.md`));
  for (const feature of inventory.features) {
    assert.equal(publicPaths.has(feature.canonicalDoc), true, `${feature.id} is absent from Forms navigation`);
  }

  assert.match(overview, /small path is the framework/i);
  assert.match(overview, /does not render controls/i);
});

function walkMarkdown(root) {
  return fs.readdirSync(root, { withFileTypes: true }).flatMap((entry) => {
    const entryPath = path.join(root, entry.name);
    if (entry.isDirectory()) return walkMarkdown(entryPath);
    return entry.isFile() && entry.name.endsWith(".md") ? [entryPath] : [];
  });
}
