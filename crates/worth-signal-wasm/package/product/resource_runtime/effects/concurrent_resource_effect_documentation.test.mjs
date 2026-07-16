import assert from "node:assert/strict";
import { access, readFile } from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";
import { createEffectLine, titlePatch } from "./resource_effect_dag_fixture.mjs";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const crateDir = path.resolve(testDir, "..", "..", "..", "..");
const workspaceDir = path.resolve(crateDir, "..", "..");
const docsDir = path.join(crateDir, "docs");
const canonicalDoc = path.join(
  docsDir,
  "resources",
  "effects",
  "concurrency-and-dependencies.md",
);

test("documented ten-effect example converges to canonical truth", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "docs-concurrent-effects");
    const line = createEffectLine(runtime);
    const admissions = await Promise.all(Array.from({ length: 10 }, (_, index) =>
      line.patch(titlePatch(runtime, index, `optimistic-${index}`))));
    const effectIds = admissions.map((admission) => admission.effectId);

    for (const index of [7, 2, 9, 0, 5, 4, 1, 8, 3, 6]) {
      const options = { responseId: `docs:response:${index}` };
      if (index % 2 === 0) {
        await line.effects().confirm(effectIds[index], options);
      } else {
        await line.effects().reject(effectIds[index], options);
      }
    }

    assert.deepEqual(
      line.value().items.map((item) => item.title),
      Array.from({ length: 10 }, (_, index) =>
        index % 2 === 0 ? `optimistic-${index}` : `loaded-${index}`),
    );
    assert.equal(line.effects().open().length, 0);
    assert.equal(line.effects().projection().kind, "canonical");
    assert.equal(line.effects().counters().openEffectCount, 0);
    assert.equal(line.effects().counters().dependencyIndexKeyCount, 0);
    assert.equal(line.effects().counters().locusIndexKeyCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("documentation claims match the packaged concurrent effect surface", async () => {
  const [doc, dagTypes, patchTypes] = await Promise.all([
    readFile(canonicalDoc, "utf8"),
    readFile(path.join(crateDir, "package", "types", "resource", "resource_effect_branch_dag.d.ts"), "utf8"),
    readFile(path.join(crateDir, "package", "types", "resource", "resource_patch_delivery_surface.d.ts"), "utf8"),
  ]);

  for (const api of [
    "resourcePatch.dependsOn",
    "line.effects().get",
    "line.effects().open",
    "line.effects().confirm",
    "line.effects().reject",
    "line.history().rollbackEffect",
  ]) {
    assert.match(doc, new RegExp(escapeRegExp(api)));
  }
  assert.match(dagTypes, /readonly envelope: ResourceEffectEnvelope/);
  assert.match(dagTypes, /readonly kind: "responseRecorded"/);
  assert.match(dagTypes, /readonly kind: "duplicateSettlement"/);
  assert.match(dagTypes, /confirm\([\s\S]*ResourceEffectSettlementResult/);
  assert.match(dagTypes, /reject\([\s\S]*ResourceEffectSettlementResult/);
  assert.match(patchTypes, /effectId: string/);
  assert.doesNotMatch(doc, /restore (?:the )?shared snapshot/i);
  assert.doesNotMatch(doc, /current branch as (?:the )?effect/i);
});

test("concurrency docs are linked and available to the in-app docs browser", async () => {
  const entryPoints = [
    path.join(docsDir, "learn", "feature-index.md"),
    path.join(docsDir, "resources", "overview.md"),
    path.join(docsDir, "resources", "effects", "README.md"),
  ];
  for (const entryPoint of entryPoints) {
    assert.match(
      await readFile(entryPoint, "utf8"),
      /concurrency-and-dependencies\.md/,
      `${entryPoint} must link the concurrency guide`,
    );
  }

  for (const docPath of [canonicalDoc, ...entryPoints]) {
    await assertLocalMarkdownLinksResolve(docPath);
  }

  const docsContent = await readFile(
    path.join(workspaceDir, "apps", "worth-signals", "src", "state", "docsContent.ts"),
    "utf8",
  );
  const demo = await readFile(
    path.join(workspaceDir, "apps", "worth-signals", "src", "ui", "ResourcesSection.tsx"),
    "utf8",
  );
  assert.match(docsContent, /worth-signal-wasm\/docs\/\*\*\/\*\.md/);
  assert.match(demo, /#\/docs\/resources\/effects\/concurrency-and-dependencies/);
});

async function assertLocalMarkdownLinksResolve(docPath) {
  const text = await readFile(docPath, "utf8");
  const links = [...text.matchAll(/\[[^\]]+\]\(([^)]+)\)/g)]
    .map((match) => match[1].split("#", 1)[0])
    .filter((href) => href && !/^(?:https?:|mailto:)/u.test(href));
  for (const href of links) {
    await access(path.resolve(path.dirname(docPath), href));
  }
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
