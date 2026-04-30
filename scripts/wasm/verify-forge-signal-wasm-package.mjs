import { execFile } from "node:child_process";
import { copyFile, mkdtemp, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import process from "node:process";
import assert from "node:assert/strict";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

async function runNpm(args, options) {
  if (process.platform === "win32") {
    const command = `npm ${args.join(" ")}`;
    return execFileAsync(
      "cmd.exe",
      ["/d", "/s", "/c", command],
      options,
    );
  }
  return execFileAsync("npm", args, options);
}

const pkgDir = path.resolve(process.argv[2] ?? "crates/forge-signal-wasm/pkg");
const packageJsonPath = path.join(pkgDir, "package.json");

function normalizeTarEntries(stdout) {
  return stdout
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean)
    .map((entry) => entry.replaceAll("\\", "/"));
}

function tarballFileName(packageName, version) {
  const normalizedName = packageName
    .replace(/^@/, "")
    .replace(/\//g, "-");
  return `${normalizedName}-${version}.tgz`;
}

async function installSmokeDependencies(tempDir, tarballPath) {
  const localTarballPath = path.join(tempDir, path.basename(tarballPath));
  await copyFile(tarballPath, localTarballPath);

  await runNpm(["init", "-y"], { cwd: tempDir });
  await runNpm(["pkg", "set", "type=module"], { cwd: tempDir });
  await runNpm(
    ["install", path.basename(localTarballPath), "react", "typescript"],
    { cwd: tempDir },
  );
}

async function runRuntimeSmoke(tempDir, packageName) {
  const smokeRuntimePath = path.join(tempDir, "smoke.mjs");
  const source = `import init, { createSignals } from "${packageName}";
import * as reactApi from "${packageName}/react";

await init();
const signals = createSignals();
const count = signals.input(1, { id: "count" });
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
signals.transaction((tx) => {
  tx.set(count, 2);
});
const history = signals.history();
const branch = history.current_branch();
const previewBranch = history.create_branch("preview");
const replay = history.replay_for_branch(branch.id);
const snapshot = history.snapshot();
const branchSnapshot = history.branch_snapshot(branch.id);
const adapters = signals.adapters();
const runtimeEnvelope = adapters.exportRuntimeEnvelope();
const restored = createSignals();
restored.adapters().replaceRuntimeEnvelope(runtimeEnvelope);
const specialist = signals.specialist();
const specialistGraphSummary = specialist.graphSummary();
const specialistEvaluateDirty = specialist.evaluateDirty();
const previewPlan = history.plan_merge_policy_preview({
  source_branch_id: previewBranch.id,
  target_branch_id: branch.id,
});
const previewPlanProof = history.plan_merge_policy_preview_with_proof({
  source_branch_id: previewBranch.id,
  target_branch_id: branch.id,
});
const previewResult = history.merge_branches_policy_preview({
  source_branch_id: previewBranch.id,
  target_branch_id: branch.id,
});

const summary = {
  hasInit: typeof init === "function",
  hasCreateSignals: typeof createSignals === "function",
  reactKeys: Object.keys(reactApi).sort(),
  doubled: doubled(),
  branchIdType: typeof branch.id,
  replayFrameCount: replay.frames.length,
  replayHasCallback: replay.frames.some((frame) => frame.callback?.id === "doubled"),
  snapshotBranchId: snapshot.snapshot.meta.branch_id,
  branchSnapshotBranchId: branchSnapshot.meta.branch_id,
  exportedPolicyPreset: runtimeEnvelope.definitions.policy.preset,
  snapshotPolicyTier: snapshot.snapshot.meta.runtime_policy.tier,
  snapshotReplayHead: snapshot.snapshot.meta.replay_head,
  snapshotExplanationRetention: snapshot.snapshot.meta.artifact_retention.explanation_retention,
  restoredDoubled: restored.read("doubled"),
  specialistGraphProfile: specialistGraphSummary.profile,
  specialistTouchedNodes: specialistEvaluateDirty.touchedNodes,
  previewBranchId: previewBranch.id,
  previewPlanSource: previewPlan.source_branch_id,
  previewPlanStrategy: previewPlan.selected_semantics.strategy_name,
  previewPlanResolution: previewPlan.resolution_plan?.divergence ?? null,
  previewPlanNodeMapIsArray: Array.isArray(previewPlan.node_map),
  previewPlanNodePlansAreTyped:
    Array.isArray(previewPlan.node_plan) &&
    previewPlan.node_plan.every((entry) => typeof entry.decision === "string"),
  previewPlanAdoptionCoreIsTyped:
    Array.isArray(previewPlan.adoption_core) &&
    previewPlan.adoption_core.every((entry) => typeof entry.source_node === "string"),
  previewPlanAdoptionPolicyIsTyped:
    Array.isArray(previewPlan.adoption_policy) &&
    previewPlan.adoption_policy.every((entry) => typeof entry.runtime_artifact === "string"),
  previewPlanDigest: previewPlanProof.proof.planDigest,
  previewResultCounter: previewResult.counters.replay_event_count,
  previewResultRecordsAreTyped:
    Array.isArray(previewResult.records) &&
    previewResult.records.every(
      (record) => typeof record.source_node === "string" && typeof record.action === "string",
    ),
};

console.log(JSON.stringify(summary));
`;
  await writeFile(smokeRuntimePath, source, "utf8");
  const { stdout } = await execFileAsync("node", [smokeRuntimePath], { cwd: tempDir });
  const result = JSON.parse(stdout.trim());

  assert.equal(result.hasInit, true, "root default init export should exist");
  assert.equal(result.hasCreateSignals, true, "root createSignals export should exist");
  assert.deepEqual(
    result.reactKeys,
    [
      "createReactSignalsStore",
      "useOutputValue",
      "useSignalValue",
      "useSignalsDiagnostics",
    ],
    "react subpath should export the expected public API",
  );
  assert.equal(result.doubled, 4, "runtime smoke should evaluate callback-first computed values");
  assert.equal(result.branchIdType, "number", "branch handles should expose numeric ids");
  assert.equal(result.replayFrameCount > 0, true, "branch replay should expose retained frames");
  assert.equal(result.replayHasCallback, true, "branch replay should preserve callback metadata");
  assert.equal(result.snapshotBranchId, 0, "snapshot envelope should serialize structured snapshot metadata");
  assert.equal(result.branchSnapshotBranchId, 0, "branch snapshot should serialize structured snapshot metadata");
  assert.equal(typeof result.exportedPolicyPreset, "string", "runtime envelope definitions should expose typed policy presets");
  assert.equal(typeof result.snapshotPolicyTier, "string", "snapshot metadata should expose typed runtime policy tiers");
  assert.equal(
    result.snapshotReplayHead === null || typeof result.snapshotReplayHead === "number",
    true,
    "snapshot metadata should expose a typed replay-head cursor or null",
  );
  assert.equal(typeof result.snapshotExplanationRetention, "string", "snapshot artifact retention policy should expose typed retention categories");
  assert.equal(result.restoredDoubled, 4, "runtime envelope round-trip should restore callback-computed committed truth through the JS boundary");
  assert.equal(typeof result.specialistGraphProfile, "string", "specialist graph summaries should expose typed graph profiles");
  assert.equal(typeof result.specialistTouchedNodes, "number", "specialist evaluateDirty should expose typed run summaries");
  assert.equal(
    result.previewPlanSource,
    result.previewBranchId,
    "history preview plans should accept numeric branch ids",
  );
  assert.equal(typeof result.previewPlanStrategy, "string", "history preview plans should expose typed selected semantics");
  assert.equal(result.previewPlanNodeMapIsArray, true, "history preview plans should expose a stable node-map entry array");
  assert.equal(result.previewPlanNodePlansAreTyped, true, "history preview plans should expose typed node-plan decisions");
  assert.equal(result.previewPlanAdoptionCoreIsTyped, true, "history preview plans should expose typed adoption core entries");
  assert.equal(result.previewPlanAdoptionPolicyIsTyped, true, "history preview plans should expose typed adoption carry policies");
  assert.equal(typeof result.previewPlanDigest, "string", "history preview proof envelopes should expose typed proof digests");
  assert.equal(typeof result.previewResultCounter, "number", "history preview results should expose typed merge counters");
  assert.equal(result.previewResultRecordsAreTyped, true, "history preview records should expose typed string node identities when present");
}

async function runTypeSmoke(tempDir, packageName) {
  const smokeTypePath = path.join(tempDir, "smoke.ts");
  const tscJsPath = path.join(tempDir, "node_modules", "typescript", "bin", "tsc");
  const source = `import { createSignals } from "${packageName}";
import {
  createReactSignalsStore,
  useOutputValue,
  useSignalValue,
  useSignalsDiagnostics,
} from "${packageName}/react";

const signals = createSignals();
const count = signals.input(1, { id: "count" });
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
const panel = signals.output(() => ({
  count: count(),
  doubled: doubled(),
}), { id: "panel" });
const store = createReactSignalsStore(signals);
const adapters = signals.adapters();
const runtimeEnvelope = adapters.exportRuntimeEnvelope();
adapters.replaceRuntimeEnvelope(runtimeEnvelope);
const runtimeProof = adapters.runtimeProofReport();
const restoredBranchId = runtimeEnvelope.snapshot.snapshot.meta.branch_id;
const snapshotExplanationRetention =
  runtimeEnvelope.snapshot.snapshot.meta.artifact_retention.explanation_retention;
const checkpointImage = runtimeEnvelope.snapshot.snapshot.checkpoint_image;
const diagnosticGraph = runtimeEnvelope.snapshot.snapshot.diagnostic_graph;
const history = signals.history();
const specialist = signals.specialist();
const currentBranch = history.current_branch();
const previewBranch = history.create_branch("preview");
const branchReplay = history.replay_for_branch(currentBranch.id);
const branchSnapshot = history.branch_snapshot(currentBranch.id);
const branchEnvelope = history.branch_snapshot_envelope(currentBranch.id);
const specialistGraphSummary = specialist.graphSummary();
const specialistEvaluateDirty = specialist.evaluateDirty();
history.restore_snapshot(branchEnvelope);
history.restore_branch_snapshot(currentBranch.id, branchSnapshot);
const branchProof = history.branch_state_proof(currentBranch.id);
const parityProof = history.replay_parity_proof(currentBranch.id, currentBranch.id);
const artifactProof = history.replay_artifact_proof({
  proofSchemaVersion: runtimeProof.proofSchemaVersion,
  registryBundleDigest: runtimeProof.registryBundleDigest,
  loweredStrategyBundleDigest: null,
  mergePlanDigest: null,
  mergeResultDigest: null,
  lineageDigest: null,
  branchStateDigest: branchProof.stateDigest,
}, currentBranch.id);
const previewPlan = history.plan_merge_policy_preview({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const previewPlanProof = history.plan_merge_policy_preview_with_proof({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const previewResult = history.merge_branches_policy_preview({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const previewResultProof = history.merge_branches_policy_preview_with_proof({
  source_branch_id: previewBranch.id,
  target_branch_id: currentBranch.id,
});
const diagnostics = signals.diagnostics();
const latestObservation = diagnostics.latestObservation();
const latestFlow = diagnostics.latestFlow();
const delivered = latestObservation?.observation.delivered_event_count;
const callbackNodeIds = latestFlow?.callbackNodes.map((node) => node.id) ?? [];
const proofVersion = runtimeProof.proofSchemaVersion;
const exportedPolicyPreset = runtimeEnvelope.definitions.policy.preset;
const snapshotPolicyTier = runtimeEnvelope.snapshot.snapshot.meta.runtime_policy.tier;
const snapshotReplayHead = runtimeEnvelope.snapshot.snapshot.meta.replay_head;
const replayHasCallback = branchReplay.frames.some((frame) => frame.callback?.id === "doubled");
const specialistGraphProfile = specialistGraphSummary.profile;
const specialistTouchedNodes = specialistEvaluateDirty.touchedNodes;
const artifactParity = artifactProof.parity;
const previewPlanSource = previewPlan.source_branch_id;
const previewPlanStrategy = previewPlan.selected_semantics.strategy_name;
const previewPlanResolution = previewPlan.resolution_plan?.divergence ?? null;
const previewPlanNodeMapEntry = previewPlan.node_map[0]?.source_node ?? null;
const previewPlanDecision = previewPlan.node_plan[0]?.decision ?? null;
const previewPlanAdoptionSource = previewPlan.adoption_core[0]?.source_node ?? null;
const previewPlanCarryPolicy = previewPlan.adoption_policy[0]?.runtime_artifact ?? null;
const previewPlanDigest = previewPlanProof.proof.planDigest;
const previewResultTarget = previewResult.target_branch;
const previewResultRecordNode = previewResult.records[0]?.source_node ?? null;
const previewResultCounter = previewResult.counters.replay_event_count;
const previewResultDigest = previewResultProof.proof.resultDigest;
const panelValue = signals.read(panel);
const panelView = useOutputValue<{ count: number; doubled: number }>(panel, store);
const countView = useSignalValue<number>(count, store);
const doubledView = useSignalValue<number>(doubled, store);
const diagnosticsView = useSignalsDiagnostics(store);

void delivered;
void callbackNodeIds;
void runtimeEnvelope;
void runtimeProof;
void restoredBranchId;
void snapshotExplanationRetention;
void checkpointImage;
void diagnosticGraph;
void proofVersion;
void exportedPolicyPreset;
void snapshotPolicyTier;
void snapshotReplayHead;
void history;
void specialist;
void currentBranch;
void previewBranch;
void branchReplay;
void branchSnapshot;
void branchEnvelope;
void branchProof;
void parityProof;
void artifactProof;
void replayHasCallback;
void specialistGraphProfile;
void specialistTouchedNodes;
void artifactParity;
void previewPlan;
void previewPlanProof;
void previewResult;
void previewResultProof;
void previewPlanSource;
void previewPlanStrategy;
void previewPlanResolution;
void previewPlanNodeMapEntry;
void previewPlanDecision;
void previewPlanAdoptionSource;
void previewPlanCarryPolicy;
void previewPlanDigest;
void previewResultTarget;
void previewResultRecordNode;
void previewResultCounter;
void previewResultDigest;
void panelValue;
void panelView;
void countView;
void doubledView;
void diagnosticsView;
`;
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

async function assertDocsStayOnCurrentPackageStory(pkgDir, packageName) {
  const docsDir = path.join(pkgDir, "docs");
  const docNames = await readdir(docsDir);
  const docTexts = await Promise.all(
    docNames
      .filter((name) => name.endsWith(".md"))
      .map(async (name) => readFile(path.join(docsDir, name), "utf8")),
  );
  const joinedDocs = docTexts.join("\n");
  const readme = await readFile(path.join(pkgDir, "README.md"), "utf8");

  assert.equal(
    readme.includes("forge-signal-wasm-dev"),
    false,
    "package README must not refer to the obsolete forge-signal-wasm-dev package name",
  );
  assert.equal(
    joinedDocs.includes("forge-signal-wasm-dev"),
    false,
    "package docs must not refer to the obsolete forge-signal-wasm-dev package name",
  );
  assert.equal(
    readme.includes("npm install forge-signal-wasm"),
    true,
    "package README should teach the public install command",
  );
  assert.equal(
    joinedDocs.includes(packageName),
    true,
    "prepared docs should mention the package lane they are proving",
  );
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
    "package/product/handles.js",
    "package/product/specialist.js",
    "package/product/transactions.js",
    "package/types/model.d.ts",
    "package/types/raw_surface.d.ts",
    "package/types/callable_surface.d.ts",
    "package/react/index.js",
    "package/react/index.d.ts",
  ];

  for (const entry of requiredEntries) {
    assert(
      entries.includes(entry),
      `packed tarball is missing required entry ${entry}`,
    );
  }

  await assertDocsStayOnCurrentPackageStory(pkgDir, packageJson.name);

  const tempDir = await mkdtemp(path.join(tmpdir(), "forge-signal-wasm-proof-"));

  try {
    await installSmokeDependencies(tempDir, tarballPath);
    await runRuntimeSmoke(tempDir, packageJson.name);
    await runTypeSmoke(tempDir, packageJson.name);
  } finally {
    await rm(tempDir, { recursive: true, force: true });
  }

  console.log(`Verified ${packageJson.name}@${packageJson.version} from ${pkgDir}`);
}

await main();
