import assert from "node:assert/strict";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import process from "node:process";
import { buildRuntimeSmokeSource } from "./verify-worth-signals-wasm-runtime-smoke-source.mjs";
import { buildTypeSmokeSource } from "./verify-worth-signals-wasm-type-smoke-source.mjs";
import {
  assertDocsStayOnCurrentPackageStory,
  execFileAsync,
  installSmokeDependencies,
  normalizeTarEntries,
  runNpm,
  tarballFileName,
} from "./verify-worth-signals-wasm-package-support.mjs";
import { BUNDLED_JS_FILE_CAP } from "./bundle_worth_signals_wasm_entries.mjs";
import { measureJsFootprint } from "./measure-worth-signals-wasm-js-footprint.mjs";
import { verifyAdditionalConsumers } from "./verify-worth-signals-wasm-consumers.mjs";
import { buildAbortSmokeSource } from "./verify-worth-signals-wasm-abort-smoke-source.mjs";
import { assertPublishedWasmSizeContract } from "./verify-worth-signals-wasm-size-asserts.mjs";

const pkgDir = path.resolve(process.argv[2] ?? "crates/worth-signal-wasm/pkg");
const packageJsonPath = path.join(pkgDir, "package.json");

async function runAbortSmoke(tempDir, packageName) {
  const smokePath = path.join(tempDir, "abort-smoke.mjs");
  await writeFile(smokePath, buildAbortSmokeSource(packageName), "utf8");
  const { stdout } = await execFileAsync("node", [smokePath], { cwd: tempDir });
  const result = JSON.parse(stdout.trim());
  assert.equal(
    result.rawDenialKind,
    "js-error",
    "raw createRawSignals().read denial must be a JS/wasm-bindgen error, not a trap",
  );
  assert.equal(
    result.productDenialKind,
    "js-error",
    "product signals.read denial must be a JS/wasm-bindgen error, not a trap",
  );
  assert.equal(result.rawDenialLooksLikeBoundaryError, true);
  assert.equal(result.stillReadable, true);
}

async function runRuntimeSmoke(tempDir, packageName) {
  const smokeRuntimePath = path.join(tempDir, "smoke.mjs");
  const source = buildRuntimeSmokeSource(packageName);
  await writeFile(smokeRuntimePath, source, "utf8");
  const { stdout } = await execFileAsync("node", [smokeRuntimePath], { cwd: tempDir });
  const result = JSON.parse(stdout.trim());

  assert.equal(result.hasInit, true);
  assert.equal(result.hasCreateSignals, true);
  assert.equal(result.reactKeys.includes("createReactSignalsStore"), true);
  assert.equal(result.reactKeys.includes("useSignalValue"), true);
  assert.equal(result.doubled, 4);
  assert.equal(result.visibleLabel, "hidden");
  assert.equal(result.viewportLabel, "1440x900");
  assert.equal(result.onlineLabel, "offline");
  assert.equal(result.clockLabel, 7);
  assert.equal(result.persistenceLabel, 2);
  assert.equal(result.visibilityState, "hidden");
  assert.deepEqual(result.localDraftValue, {
    title: "Ready to ship",
    done: true,
    status: "queued",
  });
  assert.equal(result.nameValue, "Ada");
  assert.notEqual(result.nameOpaqueId, "name");
  assert.equal(result.firstShippingOptionId, "ground");
  assert.equal(result.firstShippingOptionDebugName, "firstShippingOption");
  assert.equal(result.preservedShippingAfterSourceChangeId, "air");
  assert.equal(result.preservedShippingAfterRelinkId, "air");
  assert.equal(result.preservedShippingAfterResetId, "air");
  assert.equal(result.firstShippingAfterResetId, "sea");
  assert.equal(result.firstShippingAfterRelinkId, "sea");
  assert.equal(result.preservedShippingAfterFallbackRelinkId, "sea");
  assert.equal(result.linkedSelectionAfterGraphResetId, "ready");
  assert.equal(result.linkedRevisionAfterSecondGraphResetId, "review");
  assert.equal(result.namingGraphInputId, result.nameOpaqueId);
  assert.equal(result.namingGraphOutputId, "naming.publicDisplayName");
  assert.notEqual(result.displayLabelOpaqueId, "displayLabel");
  assert.equal(result.namingGraphDescriptor.sourceId, result.displayLabelOpaqueId);
  assert.equal(result.namingGraphDescriptor.outputName, "publicDisplayName");
  assert.equal(result.namingGraphCompatibilityOutputId, "naming.publicDisplayName");
  assert.deepEqual(
    result.requirednessInputDescriptors.map((descriptor) => ({
      inputName: descriptor.inputName,
      authority: descriptor.authority,
      requiredness: descriptor.requiredness,
    })),
    [
      { inputName: "serverValue", authority: "readOnly", requiredness: "required" },
      { inputName: "draftValue", authority: "writable", requiredness: "optional" },
    ],
  );
  assert.equal(result.requirednessServerAuthority.requiredness, "required");
  assert.equal(result.requirednessDraftAuthority.requiredness, "optional");
  assert.equal(result.itemDetailGraphId, "itemDetail");
  assert.equal(result.itemDetailGraphInputNames.includes("serverItemData"), true);
  assert.equal(result.itemDetailGraphOutputNames.includes("submitReadiness"), true);
  assert.equal(result.itemDetailGraphInputKeys.includes("serverItemData"), true);
  assert.equal(result.itemDetailGraphReadKeys.includes("submitReadiness"), true);
  assert.equal(result.itemDetailGraphOperationalWriteId, "itemDetail.editSession.serverItemData");
  assert.equal(result.itemDetailGraphOperationalPatchId, "itemDetail.editSession.draftEdits");
  assert.equal(result.itemDetailGraphOperationalAuthority, "writable");
  assert.equal(result.itemDetailGraphOperationalSupportsPatch, true);
  assert.equal(result.itemDetailGraphOperationalServerState, "done");
  assert.equal(result.itemDetailGraphOperationalDraftTitle, "Ready to ship");
  assert.equal(result.itemDetailGraphOperationalDraftReviewState, "approved");
  assert.deepEqual(result.itemDetailGraphResetDraftKeys, []);
  assert.equal(result.itemDetailGraphResetServerValue, null);
  assert.deepEqual(result.row0ScopePath, ["rows", "rows.row-0"]);
  assert.equal(typeof result.row0ScopeParent, "string");
  assert.equal(result.row0SignalCanonicalId, "rows.row-0.count");
  assert.equal(result.row0SignalGraphId, null);
  assert.equal(result.row0SignalRootScopeId, "rows");
  assert.equal(result.row1SignalCanonicalId, "rows.row-1.count");
  assert.equal(result.row0HandleId, "rows.row-0.count");
  assert.equal(result.row1HandleId, "rows.row-1.count");
  assert.equal(result.itemDetailGraphCompatibilityInputId, "itemDetail.editSession.serverItemData");
  assert.equal(result.itemDetailGraphCompatibilityContractInputId, "itemDetail.editSession.serverItemData");
  assert.equal(result.itemDetailGraphCompatibilityOutputId, "itemDetail.submitReadiness");
  assert.equal(result.itemDetailGraphWhyId, "itemDetail.submitReadiness");
  assert.equal(result.itemDetailGraphInputWhyId, "itemDetail.editSession.serverItemData");
  assert.equal(result.itemDetailGraphReplayFrameCount >= 1, true);
  assert.equal(result.itemDetailGraphInputReplayFrameCount >= 1, true);
  assert.deepEqual(
    result.itemDetailGraphDependencyInputNames.sort(),
    ["draftEdits", "serverItemData"],
  );
  assert.deepEqual(
    result.itemDetailGraphDependencySourceIds.sort(),
    [
      "itemDetail.editSession.draftEdits",
      "itemDetail.editSession.serverItemData",
    ],
  );
  assert.equal(result.itemDetailGraphContractSummaryOutputCount, 3);
  assert.deepEqual(
    result.itemDetailGraphContractDeltaAddedOutputs,
    ["effectiveItemData", "dirtyState", "submitReadiness"],
  );
  assert.equal(result.importedItemDetailGraphOutputId, "itemDetail.submitReadiness");
  assert.equal(result.importedItemDetailGraphReadiness, null);
  assert.equal(result.importedItemDetailGraphInputCount, null);
  assert.equal(result.importedItemDetailGraphContractInputId, "itemDetail.editSession.serverItemData");
  assert.equal(result.importedItemDetailGraphHistoryChanged, false);
  assert.equal(result.importedItemDetailGraphHistoryRestoreMode, "SameRuntimeExact");
  assert.equal(result.itemDetailGraphImportPortableMode, "Denied");
  assert.equal(result.itemDetailGraphImportHydrateMode, "Deferred");
  assert.equal(result.importedItemDetailGraphExactRestoreMode, "SameRuntimeExact");
  assert.equal(result.pageModalPageInputId, "itemWorkspace.page.serverItemData");
  assert.equal(result.pageModalModalInputId, "itemWorkspace.modal.serverItemData");
  assert.equal(result.pageModalPageOutputId, "itemWorkspace.pageEffectiveItemData");
  assert.equal(result.pageModalModalOutputId, "itemWorkspace.modalEffectiveItemData");
  assert.equal(result.taskEditorGraphInputId, "taskEditor.form.serverValue");
  assert.equal(result.taskEditorGraphRouteParamsId, "taskEditor.resource.routeParams");
  assert.equal(result.taskEditorGraphOutputId, "taskEditor.submitAvailability");
  assert.equal(result.taskEditorGraphPatchId, "taskEditor.form.draftValue");
  assert.equal(result.taskEditorGraphPatchedStatus, "published");
  assert.equal(result.taskEditorGraphRouteParamTaskId, "task-8");
  assert.equal(result.taskEditorGraphInputWhyId, "taskEditor.resource.routeParams");
  assert.equal(result.taskEditorGraphOutputWhyId, "taskEditor.submitAvailability");
  assert.equal(result.taskEditorGraphInputReplayFrameCount >= 1, true);
  assert.equal(result.taskEditorGraphOutputReplayFrameCount >= 1, true);
  assert.equal(result.taskEditorGraphCompatibilityInputId, "taskEditor.resource.routeParams");
  assert.equal(result.taskEditorGraphCompatibilityOutputId, "taskEditor.submitAvailability");
  assert.equal(result.authorityGraphReadOnlyAuthority, "readOnly");
  assert.equal(result.authorityGraphImportedAuthority, "imported");
  assert.equal(result.authorityGraphWritablePatchId, "taskAuthority.authority.draftValue");
  assert.equal(result.authorityGraphInputId, "taskAuthority.authority.serverValue");
  assert.equal(result.authorityGraphOutputId, "taskAuthority.effectiveValue");
  assert.equal(result.authorityGraphDraftTitle, "Queued");
  assert.equal(result.authorityGraphTaskId, "task-7");
  assert.equal(result.authorityGraphInputWhyId, "taskAuthority.authority.externalParams");
  assert.equal(result.authorityGraphOutputReplayFrameCount >= 1, true);
  assert.equal(result.authorityGraphCompatibilityInputId, "taskAuthority.authority.externalParams");
}

async function runTypeSmoke(tempDir, packageName) {
  const smokeTypePath = path.join(tempDir, "smoke.ts");
  const tscJsPath = path.join(tempDir, "node_modules", "typescript", "bin", "tsc");
  const source = buildTypeSmokeSource(packageName);
  await writeFile(smokeTypePath, source, "utf8");
  const args = [
    tscJsPath,
    "--noEmit",
    "--strict",
    "--target", "ES2022",
    "--module", "ESNext",
    "--moduleResolution", "Bundler",
    "--skipLibCheck", "false",
    "--lib", "ESNext,DOM,ESNext.Disposable",
    "--jsx", "react-jsx",
    smokeTypePath,
  ];
  await execFileAsync(process.execPath, args, { cwd: tempDir });
}

async function main() {
  const packageJson = JSON.parse(await readFile(packageJsonPath, "utf8"));
  const expectedTarballName = tarballFileName(packageJson.name, packageJson.version);
  const tarballPath = path.join(pkgDir, expectedTarballName);

  assert.equal(packageJson.main, "./index.js");
  assert.equal(packageJson.module, "./index.js");
  assert.equal(packageJson.types, "./index.d.ts");
  assert.equal(packageJson.exports["."].import, "./index.js");
  assert.equal(packageJson.exports["."].types, "./index.d.ts");
  assert.equal(packageJson.exports["./wasm"], "./worth_signal_wasm_bg.wasm");
  assert.equal(
    packageJson.exports["./worker"],
    "./product/entrypoint/bridge/worker_runtime_bridge_worker.js",
  );
  assert.equal(packageJson.exports["./raw"].import, "./raw_surface.js");
  assert.equal(packageJson.exports["./raw"].types, "./raw_surface.d.ts");
  assert.equal(packageJson.exports["./raw_surface.js"].import, "./raw_surface.js");
  assert.equal(packageJson.exports["./raw_surface.js"].types, "./raw_surface.d.ts");
  assert.equal(packageJson.exports["./react"].import, "./react/index.js");
  assert.equal(packageJson.exports["./react"].types, "./react/index.d.ts");
  assert.equal(packageJson.peerDependencies.react, "^18.0.0 || ^19.0.0");
  assert.equal(
    packageJson.peerDependencies["@types/react"],
    undefined,
    "the package must not install a React type version into the consumer",
  );
  assert.equal(packageJson.peerDependenciesMeta.react.optional, true);

  const wasmEntrypointSource = await readFile(
    path.join(pkgDir, "worth_signal_wasm.js"),
    "utf8",
  );
  assert.match(
    wasmEntrypointSource,
    /function assertWasmMagic\(/u,
    "prepared WASM entry must reject non-WASM magic before instantiate",
  );
  assert.match(
    wasmEntrypointSource,
    /received HTML \(prefix/u,
    "prepared WASM entry must diagnose HTML-as-WASM bodies",
  );
  assert.match(
    wasmEntrypointSource,
    /createSignals\(\{\s*assets:/u,
    "HTML-as-WASM diagnostic must remediate via createSignals({ assets })",
  );
  assert.match(
    wasmEntrypointSource,
    /wasmInitPromise = null/u,
    "failed WASM init must clear the in-flight promise so callers can retry",
  );

  const reactEntrySource = await readFile(
    path.join(pkgDir, "react", "index.js"),
    "utf8",
  );
  assert.doesNotMatch(
    reactEntrySource,
    /import\s+(?!\{)[A-Za-z_$][\w$]*(?:\s*,\s*\{[^}]*\})?\s+from\s*["']react["']/u,
    "the ESM adapter must not depend on synthetic React default-import interop",
  );
  assert.match(
    reactEntrySource,
    /from\s*["']react["']/u,
    "the bundled React adapter must import the react peer",
  );

  await assertPublishedWasmSizeContract(pkgDir);

  const footprint = await measureJsFootprint(pkgDir, { pack: false });
  assert.ok(
    footprint.counts.jsFiles <= BUNDLED_JS_FILE_CAP,
    `published JS file count ${footprint.counts.jsFiles} exceeds Track 5 cap ${BUNDLED_JS_FILE_CAP}`,
  );
  assert.ok(
    footprint.counts.productJsFiles <= 8,
    `product JS forest must stay collapsed (got ${footprint.counts.productJsFiles})`,
  );
  const hashedChunks = (footprint.files ?? [])
    .map((file) => file.path)
    .filter((relativePath) =>
      relativePath.startsWith("chunks/") &&
      /(?:^|[-_.])[a-f0-9]{8,}(?:\.js)?$/iu.test(path.basename(relativePath))
    );
  assert.deepEqual(
    hashedChunks,
    [],
    "Track 5 forbids content-hashed chunk filenames (use chunks/[name])",
  );

  const bridgeSource = await readFile(
    path.join(pkgDir, "product/entrypoint/bridge/worker_runtime_bridge.js"),
    "utf8",
  );
  assert.match(
    bridgeSource,
    /new URL\(\s*["']\.\/worker_runtime_bridge_worker\.js["']\s*,\s*import\.meta\.url\s*\)/u,
    "bridge shell must keep colocated worker URL on import.meta.url",
  );
  assert.doesNotMatch(
    bridgeSource,
    /from\s+["']\.\/chunks\//u,
    "bridge shell must not depend on shared chunks (colocated worker URL authority)",
  );

  await rm(tarballPath, { force: true });
  const { stdout: packStdout } = await runNpm(["pack", "--json"], { cwd: pkgDir });
  const packResults = JSON.parse(packStdout);
  const packedFiles = Array.isArray(packResults) && packResults.length > 0
    ? packResults[0].files
    : [];
  const entries = packedFiles.map((file) =>
    `package/${String(file.path).replaceAll("\\", "/")}`
  );
  const requiredEntries = [
    "package/index.js",
    "package/index.d.ts",
    "package/worth_signal_wasm.js",
    "package/worth_signal_wasm_bg.js",
    "package/worth_signal_wasm_bg.wasm",
    "package/product/entrypoint/bridge/worker_runtime_bridge.js",
    "package/product/entrypoint/bridge/worker_runtime_bridge_worker.js",
    "package/raw_surface.d.ts",
    "package/raw_surface.js",
    "package/types/callable_surface.d.ts",
    "package/types/controller_surface.d.ts",
    "package/types/diagnostics.d.ts",
    "package/types/graph_surface.d.ts",
    "package/types/model.d.ts",
    "package/types/raw_surface.d.ts",
    "package/types/resource/api_namespace.d.ts",
    "package/types/resource/api_route_builder.d.ts",
    "package/types/resource/resource_namespace.d.ts",
    "package/react/index.js",
    "package/react/index.d.ts",
    "package/react/model.d.ts",
    "package/README.md",
    "package/docs/start_here.md",
    "package/docs/core/README.md",
    "package/docs/core/aspects.md",
    "package/docs/core/diagnostics.md",
    "package/docs/forms/index.md",
    "package/docs/forms/getting-started/your-first-form.md",
    "package/docs/package/install-and-publish.md",
    "package/docs/local-truth/README.md",
    "package/docs/local-truth/branch-merge.md",
    "package/docs/reference/support-status.md",
    "package/docs/resources/overview.md",
    "package/docs/resources/branch-native-effects.md",
    "package/docs/resources/merge-and-rebase.md",
    "package/docs/resources/json-effects.md",
    "package/docs/resources/effects/effect-envelopes-and-closeout.md",
    "package/docs/resources/verification/response-topology-proof.md",
    "package/docs/router/index.md",
    "package/docs/router/runtime_placement/worker_first_default.md",
    "package/docs/api-reference/route-authoring.md",
  ];
  const forbiddenEntries = [
    "package/worth_signal_wasm.d.ts",
    "package/product/signals.js",
    "package/product/host_capabilities.js",
    "package/product/handles.js",
    "package/react/context.js",
  ];
  for (const requiredEntry of requiredEntries) {
    assert.equal(
      entries.includes(requiredEntry),
      true,
      `expected tarball to contain ${requiredEntry}`,
    );
  }
  for (const forbiddenEntry of forbiddenEntries) {
    assert.equal(
      entries.includes(forbiddenEntry),
      false,
      `expected tarball to exclude ${forbiddenEntry}`,
    );
  }
  const staleProductModules = entries.filter(
    (entry) => entry.startsWith("package/product/") && entry.endsWith(".mjs"),
  );
  assert.deepEqual(
    staleProductModules,
    [],
    "expected tarball to exclude stale product-side .mjs artifacts",
  );
  const deepProductJs = entries.filter(
    (entry) =>
      entry.startsWith("package/product/") &&
      entry.endsWith(".js") &&
      entry !== "package/product/entrypoint/bridge/worker_runtime_bridge.js" &&
      entry !== "package/product/entrypoint/bridge/worker_runtime_bridge_worker.js",
  );
  assert.deepEqual(
    deepProductJs,
    [],
    "unbundled product forest must not ship beside the Track 5 entry shells",
  );

  const tempDir = await mkdtemp(path.join(tmpdir(), "worth-signals-wasm-package-verify-"));
  try {
    await installSmokeDependencies(tempDir, tarballPath);
    await runAbortSmoke(tempDir, packageJson.name);
    await runRuntimeSmoke(tempDir, packageJson.name);
    await runTypeSmoke(tempDir, packageJson.name);
    await assertDocsStayOnCurrentPackageStory(pkgDir, packageJson.name);
    await verifyAdditionalConsumers(tarballPath, packageJson.name);
  } finally {
    await rm(tempDir, { recursive: true, force: true });
    await rm(tarballPath, { force: true });
  }
}

await main();
