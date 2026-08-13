/**
 * Anti-theatre: reverting empty-root spec authoring must make QMS-shaped
 * composition fail with the old importGraph requirement.
 *
 * Run this file alone (or with --test-concurrency=1). It patches package-src
 * in place; concurrent suites that loadSignalsModule can race the patch.
 */
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const here = path.dirname(fileURLToPath(import.meta.url));
const crateRoot = path.resolve(here, "../../../../../");
const forgeRoot = path.resolve(crateRoot, "../..");
const emptyRootSpec = path.join(here, "worker_first_empty_root_spec_authoring.test.mjs");
const importedSpec = path.join(here, "worker_first_callable_spec.test.mjs");
const specNamespaceRel =
  "package-src/product/entrypoint/worker_first_explicit_spec_namespace.ts";

function runNamedTest(testFile, namePattern, timeoutMs = 20_000) {
  const env = { ...process.env };
  delete env.NODE_TEST_CONTEXT;
  delete env.NODE_OPTIONS;
  const result = spawnSync(
    process.execPath,
    [
      "--experimental-wasm-modules",
      "--test",
      "--test-force-exit",
      `--test-timeout=${timeoutMs}`,
      `--test-name-pattern=${namePattern}`,
      testFile,
    ],
    { cwd: forgeRoot, encoding: "utf8", env },
  );
  return {
    status: result.status,
    out: `${result.stdout ?? ""}${result.stderr ?? ""}`,
  };
}

function withPatchedFile(relativePath, mutate, fn) {
  const absolute = path.join(crateRoot, relativePath);
  const original = readFileSync(absolute, "utf8");
  const patched = mutate(original);
  assert.notEqual(patched, original, `patch must change ${relativePath}`);
  writeFileSync(absolute, patched, "utf8");
  try {
    return fn();
  } finally {
    writeFileSync(absolute, original, "utf8");
  }
}

function disableEmptyRootSpecAuthoring(source) {
  // Force every empty-root authoring gate onto the import-bind path.
  return source.replaceAll(
    "if (rootSession.peekActiveImportContext() === null) {",
    "if (false && rootSession.peekActiveImportContext() === null) {",
  );
}

test("MUTATION: drop empty-root spec authoring => QMS composition ATTACK fails", () => {
  withPatchedFile(specNamespaceRel, disableEmptyRootSpecAuthoring, () => {
    const result = runNamedTest(
      emptyRootSpec,
      "worker-first empty root QMS-shaped spec.input composition does not require importGraph",
    );
    assert.notEqual(result.status, 0, result.out);
    assert.match(
      result.out,
      /active imported graph|binds only to input ids from the active imported graph/u,
    );
  });
});

test("MUTATION: drop empty-root spec authoring => full lane ATTACK fails", () => {
  withPatchedFile(specNamespaceRel, disableEmptyRootSpecAuthoring, () => {
    const result = runNamedTest(
      emptyRootSpec,
      "worker-first empty root admits full signals.spec authoring lane",
    );
    assert.notEqual(result.status, 0, result.out);
    assert.match(
      result.out,
      /active imported graph|binds only to/u,
    );
  });
});

test("MUTATION: empty-root authoring patch must not break imported-graph bind lane", () => {
  // Sanity: the production file with authoring enabled still passes import bind proofs.
  const result = runNamedTest(
    importedSpec,
    "default worker-first root exposes explicit spec handles over the active imported graph",
    30_000,
  );
  assert.equal(result.status, 0, result.out);
});
