import assert from "node:assert/strict";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import process from "node:process";
import { buildRuntimeSmokeSource } from "./verify-forge-signal-wasm-runtime-smoke-source.mjs";
import { buildTypeSmokeSource } from "./verify-forge-signal-wasm-type-smoke-source.mjs";
import {
  assertDocsStayOnCurrentPackageStory,
  execFileAsync,
  installSmokeDependencies,
  normalizeTarEntries,
  runNpm,
  tarballFileName,
} from "./verify-forge-signal-wasm-package-support.mjs";

const pkgDir = path.resolve(process.argv[2] ?? "crates/forge-signal-wasm/pkg");
const packageJsonPath = path.join(pkgDir, "package.json");

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
  assert.equal(result.nameValue, "Ada");
  assert.equal(result.namingGraphInputId, "name");
  assert.equal(result.namingGraphOutputId, "naming.publicDisplayName");
  assert.equal(result.namingGraphDescriptor.sourceId, "displayLabel");
  assert.equal(result.namingGraphDescriptor.outputName, "publicDisplayName");
  assert.equal(result.namingGraphCompatibilityOutputId, "naming.publicDisplayName");
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
    "--module", "NodeNext",
    "--moduleResolution", "NodeNext",
    "--skipLibCheck",
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
  assert.equal(packageJson.types, "./forge_signal_wasm.d.ts");
  assert.equal(packageJson.exports["."].import, "./index.js");
  assert.equal(packageJson.exports["./react"].import, "./react/index.js");

  await rm(tarballPath, { force: true });
  await runNpm(["pack"], { cwd: pkgDir });

  const { stdout: tarStdout } = await execFileAsync("tar", ["-tf", expectedTarballName], { cwd: pkgDir });
  const entries = normalizeTarEntries(tarStdout);
  const requiredEntries = [
    "package/index.js",
    "package/index.d.ts",
    "package/raw_surface.js",
    "package/product/signals.js",
    "package/product/host_capabilities.js",
    "package/product/handles.js",
    "package/product/specialist.js",
    "package/product/transactions.js",
    "package/types/callable_surface.d.ts",
    "package/types/controller_surface.d.ts",
    "package/types/diagnostics.d.ts",
    "package/types/graph_surface.d.ts",
    "package/types/model.d.ts",
    "package/types/raw_surface.d.ts",
    "package/forge_signal_wasm.d.ts",
    "package/react/index.js",
    "package/react/index.d.ts",
    "package/README.md",
    "package/docs/consuming_the_package.md",
    "package/docs/app_surface_reference.md",
    "package/docs/diagnostics_and_history_reference.md",
  ];
  for (const requiredEntry of requiredEntries) {
    assert.equal(
      entries.includes(requiredEntry),
      true,
      `expected tarball to contain ${requiredEntry}`,
    );
  }

  const tempDir = await mkdtemp(path.join(tmpdir(), "forge-signal-wasm-package-verify-"));
  try {
    await installSmokeDependencies(tempDir, tarballPath);
    await runRuntimeSmoke(tempDir, packageJson.name);
    await runTypeSmoke(tempDir, packageJson.name);
    await assertDocsStayOnCurrentPackageStory(pkgDir, packageJson.name);
  } finally {
    await rm(tempDir, { recursive: true, force: true });
    await rm(tarballPath, { force: true });
  }
}

await main();
