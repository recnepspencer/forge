import assert from "node:assert/strict";
import test from "node:test";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const crateDir = path.resolve(testDir, "..", "..", "..", "..", "..");
const readmePath = path.join(crateDir, "README.md");
const installGuidePath = path.join(crateDir, "docs", "package", "install-and-publish.md");

test("README and package install guide share the canonical explicit compatibility recovery path", async () => {
  const [readme, installGuide] = await Promise.all([
    readFile(readmePath, "utf8"),
    readFile(installGuidePath, "utf8"),
  ]);

  for (const text of [readme, installGuide]) {
    assert.match(text, /artifactFamily !== "workerUnavailableConstruction"/);
    assert.match(text, /deployment:\s*"mainThreadCompatibility"/);
    assert.match(
      text,
      /(Do not assume `createSignals\(\)` silently falls back|recover explicitly instead of expecting\s+the package to fall back on its own)/,
    );
  }
});
