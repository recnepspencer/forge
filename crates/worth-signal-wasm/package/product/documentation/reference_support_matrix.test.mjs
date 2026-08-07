import assert from "node:assert/strict";
import { readFile, stat } from "node:fs/promises";
import path from "node:path";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";
import { fileURLToPath } from "node:url";

import { loadSignalsModule } from "../signals_runtime/module_loading/load_signals_module.mjs";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const crateDir = path.resolve(testDir, "..", "..", "..");
const docsDir = path.join(crateDir, "docs");

async function readDoc(relativePath) {
  return readFile(path.join(docsDir, relativePath), "utf8");
}

function localMarkdownLinks(text) {
  return [...text.matchAll(/\[[^\]]+\]\(([^)]+)\)/gu)]
    .map((match) => match[1].split("#", 1)[0])
    .filter((href) => href && !href.startsWith("http://") && !href.startsWith("https://"));
}

test("phase 9 reference pages preserve the published entrypoint and status boundary", async () => {
  const [overview, status, entrypoints, results, construction, callable, compatibility] =
    await Promise.all([
      readDoc("reference/README.md"),
      readDoc("reference/support-status.md"),
      readDoc("reference/package-entrypoints-and-contracts.md"),
      readDoc("reference/typed-results-and-unavailability.md"),
      readDoc("api-reference/construction.md"),
      readDoc("api-reference/callable-signals.md"),
      readDoc("api-reference/compatibility-surface.md"),
    ]);

  assert.match(overview, /runtime result rather than product support/u);
  for (const term of ["Stable", "Mixed", "Deferred", "Unsupported", "Compatibility-only"]) {
    assert.match(status, new RegExp(`\\b${term}\\b`, "u"));
  }
  assert.match(status, /`unavailable` is a runtime outcome/u);
  assert.match(
    status,
    /createCallableSignals\(\).*Always selects `mainThreadCompatibility`/u,
  );
  assert.match(status, /does not\s+mean deprecated/u);
  assert.match(status, /Host And Bundler Asset Loading/u);
  assert.match(status, /Vite 8\+ with `worker\.format: "es"`/u);
  assert.match(status, /Vite 7 \+ `createSignals\(\{ assets \}\)`/u);
  assert.match(status, /must return \*\*404\*\*, never SPA/u);
  assert.match(status, /optimizeDeps\.exclude/u);
  assert.match(status, /Compatibility-only workaround/u);

  assert.match(entrypoints, /worth-signals-wasm\/react/u);
  assert.match(entrypoints, /worth-signals-wasm\/wasm/u);
  assert.match(entrypoints, /worth-signals-wasm\/worker/u);
  assert.match(entrypoints, /worth-signals-wasm\/raw_surface\.js/u);
  assert.match(entrypoints, /signalsCompatibilityAssertionFailed/u);
  assert.match(entrypoints, /a handle belongs to the runtime that created\s+it/iu);

  assert.match(results, /Recovery object is advice|recovery object is advice/iu);
  assert.match(results, /Do not collapse these reads into one `isValid` boolean/u);
  assert.match(results, /manufacturing a successful-looking artifact/u);

  assert.match(construction, /defaults to `"workerFirst"`/u);
  assert.match(construction, /workerUnavailableConstruction/u);
  assert.match(construction, /There is no hidden fallback|there is no hidden fallback/iu);
  assert.match(callable, /RunSummary \| Promise<RunSummary>/u);
  assert.match(callable, /not a remote database transaction/u);
  assert.match(compatibility, /forces compatibility deployment/u);
  assert.match(compatibility, /default export initializes the lower-level Wasm module/u);

  const allText = [overview, status, entrypoints, results, construction, callable, compatibility]
    .join("\n");
  assert.doesNotMatch(allText, /automatically falls back to the main thread/iu);
  assert.doesNotMatch(allText, /Local Truth is durable shared authority/iu);
  assert.doesNotMatch(allText, /`debugName` is structural identity/iu);
});

test("phase 9 reference links resolve to checked-in pages", async () => {
  const relativePaths = [
    "reference/README.md",
    "reference/support-status.md",
    "reference/package-entrypoints-and-contracts.md",
    "reference/typed-results-and-unavailability.md",
    "api-reference/construction.md",
    "api-reference/callable-signals.md",
    "api-reference/compatibility-surface.md",
  ];

  for (const relativePath of relativePaths) {
    const absolutePath = path.join(docsDir, relativePath);
    const text = await readFile(absolutePath, "utf8");
    for (const href of localMarkdownLinks(text)) {
      const target = await stat(path.resolve(path.dirname(absolutePath), href));
      assert.equal(target.isFile(), true, `${relativePath} -> ${href}`);
    }
  }
});

test("the runtime proves the construction and compatibility claims used by the reference", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = undefined;
  const {
    createCallableSignals,
    createSignals,
    explainCreateSignalsConstruction,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });

  let compatibility = null;
  try {
    const explanation = explainCreateSignalsConstruction();
    assert.equal(explanation.requestedDeployment, "workerFirst");
    assert.equal(explanation.selectedFamily, "workerUnavailable");
    assert.equal(explanation.selectedDeployment, null);

    await assert.rejects(
      () => createSignals(),
      (error) => {
        assert.equal(error?.artifactFamily, "workerUnavailableConstruction");
        assert.equal(error?.reason, "workerConstructorUnavailable");
        assert.equal(
          error?.compatibilityRecovery?.deployment,
          "mainThreadCompatibility",
        );
        return true;
      },
    );

    compatibility = await createCallableSignals({ deployment: "workerFirst" });
    const contract = compatibility.contract();
    assert.equal(contract.surfaceFamily, "mainThreadCompatibilityCallable");
    assert.equal(contract.surfaceVersion, "1");
    assert.equal(contract.deployment, "mainThreadCompatibility");
    assert.deepEqual(contract.capabilities, {
      callableSurface: true,
      scopedAuthoring: true,
      specNamespace: true,
      workerRuntime: false,
    });
    assert.throws(
      () => compatibility.assertCompatibility({ requires: ["workerRuntime"] }),
      (error) => {
        assert.equal(error?.code, "signalsCompatibilityAssertionFailed");
        assert.deepEqual(error?.missingCapabilities, ["workerRuntime"]);
        return true;
      },
    );
  } finally {
    compatibility?.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first construction explanation and contract agree on the admitted deployment", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createSignals,
    explainCreateSignalsConstruction,
    cleanup,
  } = await loadSignalsModule({ rawSurface: "real" });

  let signals = null;
  try {
    const explanation = explainCreateSignalsConstruction();
    assert.equal(explanation.requestedDeployment, "workerFirst");
    assert.equal(explanation.selectedFamily, "workerFirst");
    assert.equal(explanation.selectedDeployment, "workerFirst");
    assert.equal(explanation.compatibilityRecovery, null);

    signals = await createSignals();
    const contract = signals.contract();
    assert.equal(contract.surfaceFamily, "workerFirstCallable");
    assert.equal(contract.deployment, explanation.selectedDeployment);
    assert.equal(contract.capabilities.workerRuntime, true);
  } finally {
    signals?.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});
