import assert from "node:assert/strict";
import { readFile, readdir, stat } from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const crateDir = path.resolve(testDir, "..", "..", "..");
const docsDir = path.join(crateDir, "docs");
const manifestPath = path.join(docsDir, "metadata", "public-documentation.json");
const allowedStatuses = new Set(["compatibility", "mixed", "stable"]);
const expectedSectionIds = [
  "core", "forms", "integrations", "local-truth",
  "reference", "resources", "router", "start",
];
const expectedFeatureIds = [
  "aspects", "compatibility-surface", "construction", "diagnostics", "forms",
  "graphs-and-controllers", "history", "host-capabilities", "inputs-and-computed",
  "linked-state", "local-truth", "react", "resources", "router", "transactions",
];
const forbiddenPublicPathParts = ["closeout", "crosswalk", "milestone", "resource-contracts"];
const mojibake = /\u00c2|\u00c3|\u00e2(?:\u0080|\u20ac|\u201a|\u2020|\u02c6)/u;

async function readManifest() {
  return JSON.parse(await readFile(manifestPath, "utf8"));
}

async function assertFileExists(filePath, context) {
  try {
    const fileStat = await stat(filePath);
    assert.equal(fileStat.isFile(), true, context);
  } catch (error) {
    assert.fail(`${context}: ${error.message}`);
  }
}

function localMarkdownLinks(text) {
  return [...text.matchAll(/\[[^\]]+\]\(([^)]+)\)/gu)]
    .map((match) => match[1])
    .filter((href) => href && !/^[a-z]+:/iu.test(href) && !href.startsWith("#"));
}

function linkedFilePath(articlePath, href) {
  const withoutFragment = href.split("#", 1)[0].split("?", 1)[0];
  return path.resolve(path.dirname(articlePath), withoutFragment);
}

function linkedDocSubpath(articleSubpath, href) {
  const withoutFragment = href.split("#", 1)[0].split("?", 1)[0];
  if (!withoutFragment.endsWith(".md")) return null;
  return path.posix
    .normalize(path.posix.join(path.posix.dirname(articleSubpath), withoutFragment))
    .replace(/\.md$/u, "");
}

function canonicalRedirectTarget(redirectBySource, requestedPath) {
  const visited = new Set();
  let current = requestedPath;
  while (redirectBySource.has(current)) {
    assert.equal(visited.has(current), false, `redirect cycle at ${current}`);
    visited.add(current);
    current = redirectBySource.get(current);
  }
  return current;
}

async function markdownInventory() {
  const entries = await readdir(docsDir, { recursive: true });
  return new Map(entries
    .filter((entry) => entry.endsWith(".md"))
    .map((entry) => {
      const normalized = entry.replaceAll("\\", "/");
      return [normalized.replace(/\.md$/u, ""), path.join(docsDir, entry)];
    }));
}

async function discoverPublicCorpus(manifest, inventory) {
  const redirectBySource = new Map(manifest.redirects.map(({ from, to }) => [from, to]));
  const pending = manifest.sections.flatMap((section) => section.items.map((item) => item.path));
  const discovered = new Set();

  while (pending.length > 0) {
    const articleSubpath = canonicalRedirectTarget(redirectBySource, pending.shift());
    if (discovered.has(articleSubpath)) continue;
    const articlePath = inventory.get(articleSubpath);
    assert.ok(articlePath, `missing public doc ${articleSubpath}`);
    discovered.add(articleSubpath);
    const text = await readFile(articlePath, "utf8");
    for (const href of localMarkdownLinks(text)) {
      const linkedSubpath = linkedDocSubpath(articleSubpath, href);
      if (linkedSubpath) pending.push(linkedSubpath);
    }
  }
  return discovered;
}

test("public documentation manifest is a complete, curated authority map", async () => {
  const manifest = await readManifest();
  assert.equal(manifest.schemaVersion, 1);
  assert.equal(manifest.product, "worth-signals-wasm");

  const sectionIds = manifest.sections.map((section) => section.id);
  assert.equal(new Set(sectionIds).size, sectionIds.length, "section ids must be unique");
  assert.deepEqual([...sectionIds].sort(), expectedSectionIds, "public sections cannot disappear silently");

  const publicItems = manifest.sections.flatMap((section) => section.items);
  const publicPaths = publicItems.map((item) => item.path);
  assert.equal(new Set(publicPaths).size, publicPaths.length, "public paths must be unique");
  for (const item of publicItems) {
    await assertFileExists(path.join(docsDir, `${item.path}.md`), `missing public doc ${item.path}`);
    assert.equal(
      forbiddenPublicPathParts.some((part) => item.path.includes(part)),
      false,
      `engineering material leaked into public navigation: ${item.path}`,
    );
  }

  const redirectSources = manifest.redirects.map((redirect) => redirect.from);
  assert.equal(new Set(redirectSources).size, redirectSources.length, "redirect sources must be unique");
  assert.equal(redirectSources.some((source) => publicPaths.includes(source)), false);

  const featureIds = manifest.features.map((feature) => feature.id);
  const canonicalPaths = manifest.features.map((feature) => feature.canonicalPath);
  assert.equal(new Set(featureIds).size, featureIds.length, "feature ids must be unique");
  assert.equal(new Set(canonicalPaths).size, canonicalPaths.length, "canonical topics must be unique");
  assert.deepEqual([...featureIds].sort(), expectedFeatureIds, "feature truth entries cannot disappear silently");

  for (const feature of manifest.features) {
    assert.equal(publicPaths.includes(feature.canonicalPath), true, `${feature.id} lacks a two-click canonical page`);
    assert.equal(allowedStatuses.has(feature.status), true, `${feature.id} has an unknown status`);
    assert.equal(feature.publicEntrypoints.length > 0, true, `${feature.id} lacks entry points`);
    assert.equal(feature.truthOwner.length > 20, true, `${feature.id} lacks an honest truth owner`);
    assert.equal(feature.limits.length > 0, true, `${feature.id} lacks current limits`);
    assert.equal(feature.evidence.length > 0, true, `${feature.id} lacks runtime evidence`);
    for (const evidencePath of feature.evidence) {
      await assertFileExists(path.join(crateDir, evidencePath), `missing evidence ${evidencePath}`);
    }
  }
});

test("the public corpus has no parallel pages, orphan pages, or broken redirects", async () => {
  const [manifest, inventory] = await Promise.all([readManifest(), markdownInventory()]);
  const publicCorpus = await discoverPublicCorpus(manifest, inventory);
  const redirectBySource = new Map(manifest.redirects.map(({ from, to }) => [from, to]));

  assert.deepEqual(
    [...publicCorpus].sort(),
    [...inventory.keys()].sort(),
    "every Markdown page must be reachable from the curated public navigation",
  );
  for (const [source, target] of redirectBySource) {
    assert.equal(inventory.has(source), false, `legacy page still exists beside its redirect: ${source}`);
    const canonicalTarget = canonicalRedirectTarget(redirectBySource, target);
    assert.equal(publicCorpus.has(canonicalTarget), true, `redirect target is not public: ${source} -> ${target}`);
  }
});

test("every public page is titled, encoding-clean, and internally linked", async () => {
  const [manifest, inventory] = await Promise.all([readManifest(), markdownInventory()]);
  const publicCorpus = await discoverPublicCorpus(manifest, inventory);
  const redirectSources = new Set(manifest.redirects.map((redirect) => redirect.from));

  for (const articleSubpath of publicCorpus) {
    const articlePath = inventory.get(articleSubpath);
    const text = await readFile(articlePath, "utf8");
    assert.match(text, /^#\s+\S/m, `${articleSubpath} lacks a title`);
    assert.doesNotMatch(text, mojibake, `${articleSubpath} contains mojibake`);
    for (const href of localMarkdownLinks(text)) {
      await assertFileExists(linkedFilePath(articlePath, href), `broken link ${articleSubpath} -> ${href}`);
      const linkedSubpath = linkedDocSubpath(articleSubpath, href);
      if (linkedSubpath) {
        assert.equal(redirectSources.has(linkedSubpath), false, `internal link still uses legacy URL: ${articleSubpath} -> ${linkedSubpath}`);
      }
    }
  }
});

test("the publishable README is synchronized with the crate authority", async () => {
  const [crateReadme, packageReadme] = await Promise.all([
    readFile(path.join(crateDir, "README.md"), "utf8"),
    readFile(path.join(crateDir, "pkg", "README.md"), "utf8"),
  ]);
  assert.equal(packageReadme, crateReadme);
});
