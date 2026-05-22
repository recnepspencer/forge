import assert from "node:assert/strict";
import test from "node:test";
import { readdir, readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const crateDir = path.resolve(testDir, "..", "..", "..", "..", "..");
const readmePath = path.join(crateDir, "README.md");
const docsDir = path.join(crateDir, "docs");

test("README and published package docs teach await createSignals() as the normal lane", async () => {
  const docPaths = [readmePath, ...await collectMarkdownFiles(docsDir)];
  const docs = await Promise.all(docPaths.map((docPath) => readFile(docPath, "utf8")));

  for (const text of docs) {
    assert.doesNotMatch(text, /const signals = createSignals\(\);/);
    assert.doesNotMatch(text, /createSignals\(\)\.importGraph/);
    if (text.includes("createSignals(")) {
      assert.match(text, /\bawait\s+createSignals\(/);
    }
    if (
      text.includes("signals.compatibilityApp()")
      || text.includes("signals.compatibilityRuntime()")
    ) {
      assert.match(text, /deployment:\s*"mainThreadCompatibility"/);
    }
  }

  const readme = docs[0];
  assert.doesNotMatch(readme, /signals\.compatibilityApp\(\)/);
  assert.doesNotMatch(readme, /signals\.compatibilityRuntime\(\)/);

  const hostCapabilityDoc = docs.find((text) => (
    text.includes("# Host Capabilities")
    && text.includes("ambientHostReadDenialArtifact")
  ));
  assert.match(hostCapabilityDoc ?? "", /readDenialCount/);
  assert.match(hostCapabilityDoc ?? "", /dependencyRefreshFailureCount/);
  assert.match(hostCapabilityDoc ?? "", /computeCallbackMissingHostCapabilityReadDenied/);
  assert.match(hostCapabilityDoc ?? "", /computeCallbackDetachedHostCapabilityReadDenied/);
});

async function collectMarkdownFiles(rootDir) {
  const entries = await readdir(rootDir, { withFileTypes: true });
  const collected = [];
  for (const entry of entries) {
    const fullPath = path.join(rootDir, entry.name);
    if (entry.isDirectory()) {
      collected.push(...await collectMarkdownFiles(fullPath));
      continue;
    }
    if (entry.isFile() && fullPath.endsWith(".md")) {
      collected.push(fullPath);
    }
  }
  return collected;
}
