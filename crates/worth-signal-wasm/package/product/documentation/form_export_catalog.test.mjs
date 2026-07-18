import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import {
  assignSurfacesToCoverageGroups,
  collectPublicContractInventory,
} from "./public_surface_inventory.mjs";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const crateDir = path.resolve(testDir, "..", "..", "..");
const catalogPath = path.join(crateDir, "docs", "api-reference", "form-export-catalog.md");
const policyPath = path.join(crateDir, "docs", "metadata", "public-surface-policy.json");

function catalogEntries(markdown) {
  return [...markdown.matchAll(/^- `([^`]+)` — (.+)$/gmu)].map((match) => ({
    description: match[2],
    name: match[1],
  }));
}

test("every public Forms declaration has exactly one useful catalog description", async () => {
  const [catalog, policyText] = await Promise.all([
    readFile(catalogPath, "utf8"),
    readFile(policyPath, "utf8"),
  ]);
  const policy = JSON.parse(policyText);
  const inventory = collectPublicContractInventory(crateDir);
  const assignments = assignSurfacesToCoverageGroups(inventory.surfaces, policy);
  const expectedSurfaces = assignments.assignments.get("forms");
  const entries = catalogEntries(catalog);

  assert.equal(entries.length, expectedSurfaces.length);
  assert.equal(new Set(entries.map((entry) => entry.name)).size, entries.length, "catalog names must be unique");
  assert.deepEqual(
    entries.map((entry) => entry.name).sort(),
    expectedSurfaces.map((surface) => surface.exportName).sort(),
    "the catalog must change whenever the public Forms declaration census changes",
  );

  for (const entry of entries) {
    assert.equal(entry.description.length >= 35, true, `${entry.name} needs a useful one-line description`);
    assert.match(entry.description, /\.$/u, `${entry.name} description must be a complete sentence`);
  }

  const sourceFiles = new Set(expectedSurfaces.map((surface) => surface.source));
  for (const sourceFile of sourceFiles) {
    assert.equal(catalog.includes(`Source declaration: \`${sourceFile}\``), true, `${sourceFile} is not discoverable`);
  }

  assert.match(catalog, /deeper details are discoverable in source code/i);
  assert.match(catalog, /ordinary application entry point remains `signals\.form\(\.\.\.\)`/i);
  assert.match(catalog, /\*\*Ask your AI agent:\*\*/u);
  assert.match(catalog, /Propose 2-3 design patterns, their requirements,/u);
});
