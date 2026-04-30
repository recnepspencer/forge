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
  const source = `import init, { clockCapability, createSignals, hostCapabilityPlan, onlineCapability, persistenceCapability, viewportCapability, visibilityCapability } from "${packageName}";
import * as reactApi from "${packageName}/react";

await init();
let visibilityState = "visible";
let visibilityListener = null;
let viewportState = { width: 1280, height: 720 };
let viewportListener = null;
let onlineState = "online";
let onlineListener = null;
let clockTick = 0;
let persistedDraft = { mode: "draft", revision: 1 };
const signals = createSignals({
  hostCapabilities: hostCapabilityPlan({
    visibility: visibilityCapability({
      source: {
        current() {
          return visibilityState;
        },
        subscribe(listener) {
          visibilityListener = listener;
          return () => {
            visibilityListener = null;
          };
        },
      },
      compatibility: "LiveOnly",
    }),
    viewport: viewportCapability({
      source: {
        current() {
          return viewportState;
        },
        subscribe(listener) {
          viewportListener = listener;
          return () => {
            viewportListener = null;
          };
        },
      },
    }),
    online: onlineCapability({
      source: {
        current() {
          return onlineState;
        },
        subscribe(listener) {
          onlineListener = listener;
          return () => {
            onlineListener = null;
          };
        },
      },
    }),
    clock: clockCapability({
      source: {
        current() {
          return clockTick;
        },
      },
      pollMs: 5,
    }),
    persistence: persistenceCapability({
      source: {
        current() {
          return persistedDraft;
        },
      },
    }),
  }),
});
const count = signals.input(1, { id: "count" });
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
const visibleLabel = signals.computed(
  () => (signals.host.visibility?.isVisible() ? "visible" : "hidden"),
  { id: "visibleLabel" },
);
const viewportLabel = signals.computed(
  () => (signals.host.viewport?.width() ?? 0) + "x" + (signals.host.viewport?.height() ?? 0),
  { id: "viewportLabel" },
);
const onlineLabel = signals.computed(
  () => (signals.host.online?.isOnline() ? "online" : "offline"),
  { id: "onlineLabel" },
);
const clockLabel = signals.computed(
  () => (signals.host.clock?.now() ?? 0) + count(),
  { id: "clockLabel" },
);
const persistenceLabel = signals.computed(
  () => signals.host.persistence?.value().revision ?? 0,
  { id: "persistenceLabel" },
);
  visibleLabel();
  viewportLabel();
  onlineLabel();
  clockLabel();
  viewportState = { width: 1440, height: 900 };
  viewportListener?.();
  visibilityState = "hidden";
  visibilityListener?.();
  onlineState = "offline";
  onlineListener?.();
  clockTick = 5;
  await new Promise((resolve) => setTimeout(resolve, 15));
  persistedDraft = { mode: "draft", revision: 2 };
  signals.host.persistence?.commit();
  signals.host.persistence?.commit();
  await Promise.resolve();
signals.transaction((tx) => {
  tx.set(count, 2);
});
const history = signals.history();
const branch = history.current_branch();
const previewBranch = history.create_branch("preview");
const replay = history.replay_for_branch(branch.id);
const snapshot = history.snapshot();
const branchSnapshot = history.branch_snapshot(branch.id);
const branchEnvelope = history.branch_snapshot_envelope(branch.id);
const adapters = signals.adapters();
const runtimeEnvelope = adapters.exportRuntimeEnvelope();
const transportReport = adapters.hostCapabilityTransportReport(runtimeEnvelope);
const restoredExact = createSignals();
restoredExact.adapters().restoreExactRuntimeEnvelope(runtimeEnvelope);
const portableImport = createSignals();
let portableImportError = null;
try {
  portableImport.adapters().replaceRuntimeEnvelope(runtimeEnvelope);
} catch (error) {
  portableImportError = {
    code: error?.code ?? null,
    message: error?.message ?? String(error),
  };
}
const portableImportDiagnostics = portableImport.diagnostics();
const portableImportLatestHostCapabilityEvent =
  portableImportDiagnostics.latestHostCapabilityEvent();
const portableImportRecentHostCapabilityEvents =
  portableImportDiagnostics.recentHostCapabilityEvents();
const portableImportHostCapabilityReport =
  portableImportDiagnostics.hostCapabilityReport();
const portableImportDeniedCallbackIds =
  portableImportRecentHostCapabilityEvents.flatMap((event) => event?.deniedCallbackIds ?? []);
const portableImportPerformanceSummary = portableImportDiagnostics.performanceSummary();
const unavailableVisibleLabel =
  runtimeEnvelope.definitions.unavailableCallbacks.find((artifact) => artifact.id === "visibleLabel");
const unavailableViewportLabel =
  runtimeEnvelope.definitions.unavailableCallbacks.find((artifact) => artifact.id === "viewportLabel");
const unavailableOnlineLabel =
  runtimeEnvelope.definitions.unavailableCallbacks.find((artifact) => artifact.id === "onlineLabel");
const unavailableClockLabel =
  runtimeEnvelope.definitions.unavailableCallbacks.find((artifact) => artifact.id === "clockLabel");
const unavailablePersistenceLabel =
  runtimeEnvelope.definitions.unavailableCallbacks.find((artifact) => artifact.id === "persistenceLabel");
const specialist = signals.specialist();
const specialistGraphSummary = specialist.graphSummary();
const specialistEvaluateDirty = specialist.evaluateDirty();
const diagnostics = signals.diagnostics();
const performanceSummary = diagnostics.performanceSummary();
const hostCapabilityReport = diagnostics.hostCapabilityReport();
const latestHostCapabilityEvent = diagnostics.latestHostCapabilityEvent();
const recentHostCapabilityEvents = diagnostics.recentHostCapabilityEvents();
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
  visibleLabel: visibleLabel(),
  viewportLabel: viewportLabel(),
  onlineLabel: onlineLabel(),
  clockLabel: clockLabel(),
  persistenceLabel: persistenceLabel(),
  visibilityState: signals.host.visibility?.state() ?? null,
  visibilityCompatibility: signals.host.visibility?.descriptor().compatibility ?? null,
  viewportSize: signals.host.viewport?.size() ?? null,
  viewportCompatibility: signals.host.viewport?.descriptor().compatibility ?? null,
  onlineState: signals.host.online?.state() ?? null,
  onlineCompatibility: signals.host.online?.descriptor().compatibility ?? null,
  clockNow: signals.host.clock?.now() ?? null,
  clockCompatibility: signals.host.clock?.descriptor().compatibility ?? null,
  persistenceValue: signals.host.persistence?.value() ?? null,
  persistenceCompatibility: signals.host.persistence?.descriptor().compatibility ?? null,
  runtimeEnvelopeRestoreMode: runtimeEnvelope.runtimeEnvelopeRestoreMode,
  runtimeEnvelopeUnavailableHostCapabilityCompatibility:
    unavailableVisibleLabel?.hostCapabilityReads[0]?.compatibility ?? null,
  runtimeEnvelopeUnavailablePortableOutcome:
    unavailableVisibleLabel?.hostCapabilityTransports[0]?.portableImportOutcome ?? null,
  runtimeEnvelopeUnavailableExactRestoreOutcome:
    unavailableVisibleLabel?.hostCapabilityTransports[0]?.exactRestoreOutcome ?? null,
  runtimeEnvelopeUnavailablePortableReason:
    unavailableVisibleLabel?.hostCapabilityTransports[0]?.portableImportReason ?? null,
  runtimeEnvelopeUnavailableOnlineCompatibility:
    unavailableOnlineLabel?.hostCapabilityReads[0]?.compatibility ?? null,
  runtimeEnvelopeUnavailableViewportCompatibility:
    unavailableViewportLabel?.hostCapabilityReads[0]?.compatibility ?? null,
  runtimeEnvelopeUnavailableViewportPortableOutcome:
    unavailableViewportLabel?.hostCapabilityTransports[0]?.portableImportOutcome ?? null,
  runtimeEnvelopeUnavailableViewportPortableReason:
    unavailableViewportLabel?.hostCapabilityTransports[0]?.portableImportReason ?? null,
  runtimeEnvelopeUnavailableOnlinePortableOutcome:
    unavailableOnlineLabel?.hostCapabilityTransports[0]?.portableImportOutcome ?? null,
  runtimeEnvelopeUnavailableOnlinePortableReason:
    unavailableOnlineLabel?.hostCapabilityTransports[0]?.portableImportReason ?? null,
  runtimeEnvelopeUnavailableClockCompatibility:
    unavailableClockLabel?.hostCapabilityReads[0]?.compatibility ?? null,
  runtimeEnvelopeUnavailableClockPortableOutcome:
    unavailableClockLabel?.hostCapabilityTransports[0]?.portableImportOutcome ?? null,
  runtimeEnvelopeUnavailableClockPortableReason:
    unavailableClockLabel?.hostCapabilityTransports[0]?.portableImportReason ?? null,
  runtimeEnvelopeUnavailablePersistenceCompatibility:
    unavailablePersistenceLabel?.hostCapabilityReads[0]?.compatibility ?? null,
  runtimeEnvelopeUnavailablePersistencePortableOutcome:
    unavailablePersistenceLabel?.hostCapabilityTransports[0]?.portableImportOutcome ?? null,
  runtimeEnvelopeUnavailableCallbackCount:
    runtimeEnvelope.definitions.unavailableCallbacks.length,
  runtimeEnvelopeUnavailableCurrentReads: unavailableVisibleLabel?.currentReads ?? [],
  latestHostCapabilityEventKind: latestHostCapabilityEvent?.kind ?? null,
  latestHostCapabilityEventQueuedCount: latestHostCapabilityEvent?.queuedInvalidationCount ?? null,
  recentHostCapabilityEventCount: recentHostCapabilityEvents.length,
  hostCapabilityReadCount: performanceSummary.hostCapabilityReadCount ?? null,
  hostCapabilityPollCount: performanceSummary.hostCapabilityPollCount ?? null,
  hostCapabilityNoOpPollCount: performanceSummary.hostCapabilityNoOpPollCount ?? null,
  hostCapabilityManualCommitCount: performanceSummary.hostCapabilityManualCommitCount ?? null,
  hostCapabilityNoOpManualCommitCount: performanceSummary.hostCapabilityNoOpManualCommitCount ?? null,
  hostCapabilityInvalidationCount: performanceSummary.hostCapabilityInvalidationCount ?? null,
  hostCapabilityInvalidationBatchFlushCount: performanceSummary.hostCapabilityInvalidationBatchFlushCount ?? null,
  hostCapabilityReevaluationCount: performanceSummary.hostCapabilityReevaluationCount ?? null,
  hostCapabilityInvalidationTouchedNodeCount: performanceSummary.hostCapabilityInvalidationTouchedNodeCount ?? null,
  hostCapabilityCompatibilityDenialCount: performanceSummary.hostCapabilityCompatibilityDenialCount ?? null,
  hostCapabilityUnavailabilityArtifactCount: performanceSummary.hostCapabilityUnavailabilityArtifactCount ?? null,
  hostCapabilityBroadFanoutDenialCount: performanceSummary.hostCapabilityBroadFanoutDenialCount ?? null,
  hostCapabilityReportDigest: hostCapabilityReport.digest,
  hostCapabilityReportLineageDigest: hostCapabilityReport.lineageDigest,
  hostCapabilityReportBreadthDigest: hostCapabilityReport.breadthDigest,
  hostCapabilityReportFamilyCount: hostCapabilityReport.families.length,
  hostCapabilityReportLineageCount: hostCapabilityReport.lineage.length,
  hostCapabilityReportMaxTouchedNodes: hostCapabilityReport.breadth.maxTouchedNodes,
  hostCapabilityReportMaxReevaluatedNodes: hostCapabilityReport.breadth.maxReevaluatedNodes,
  transportReportDigest: transportReport.digest,
  transportReportUnavailableArtifactCount: transportReport.totals.unavailableArtifactCount,
  transportReportDeniedFamilyCount: transportReport.totals.deniedFamilyCount,
  transportReportUnavailableFamilyCount: transportReport.totals.unavailableFamilyCount,
  portableImportLatestHostCapabilityEventKind:
    portableImportLatestHostCapabilityEvent?.kind ?? null,
  portableImportLatestHostCapabilityEventQueuedCount:
    portableImportLatestHostCapabilityEvent?.queuedInvalidationCount ?? null,
  portableImportLatestHostCapabilityEventDeniedIds:
    portableImportLatestHostCapabilityEvent?.deniedCallbackIds ?? [],
  portableImportDeniedCallbackIds,
  portableImportRecentHostCapabilityEventCount:
    portableImportRecentHostCapabilityEvents.length,
  portableImportHostCapabilityCompatibilityDenialCount:
    portableImportPerformanceSummary.hostCapabilityCompatibilityDenialCount ?? null,
  portableImportHostCapabilityReportDigest:
    portableImportHostCapabilityReport.digest,
  portableImportHostCapabilityReportLineageDigest:
    portableImportHostCapabilityReport.lineageDigest,
  portableImportHostCapabilityReportFamilyCount:
    portableImportHostCapabilityReport.families.length,
  branchIdType: typeof branch.id,
  replayFrameCount: replay.frames.length,
  replayHasCallback: replay.frames.some((frame) => frame.callback?.id === "doubled"),
  snapshotBranchId: snapshot.snapshot.meta.branch_id,
  branchSnapshotBranchId: branchSnapshot.meta.branch_id,
  branchSnapshotRestoreMode: branchSnapshot.snapshotRestoreMode,
  branchEnvelopeRestoreMode: branchEnvelope.snapshotEnvelopeRestoreMode,
  exportedPolicyPreset: runtimeEnvelope.definitions.policy.preset,
  snapshotPolicyTier: snapshot.snapshot.meta.runtime_policy.tier,
  snapshotReplayHead: snapshot.snapshot.meta.replay_head,
  snapshotExplanationRetention: snapshot.snapshot.meta.artifact_retention.explanation_retention,
  restoredExactDoubled: restoredExact.read("doubled"),
  portableImportErrorCode: portableImportError?.code ?? null,
  portableImportErrorMessage: portableImportError?.message ?? null,
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
  assert.equal(result.visibleLabel, "hidden", "runtime smoke should reevaluate host capability-backed computed values after capability invalidation");
  assert.equal(result.viewportLabel, "1440x900", "runtime smoke should reevaluate viewport-backed computed values after push-driven invalidation");
  assert.equal(result.onlineLabel, "offline", "runtime smoke should reevaluate the second admitted host-capability family after invalidation");
  assert.equal(result.clockLabel, 7, "runtime smoke should reevaluate the polled clock host-capability family after invalidation");
  assert.equal(result.visibilityState, "hidden", "host capability visibility state should be readable from the product surface");
  assert.equal(result.visibilityCompatibility, "LiveOnly", "host capability descriptors should expose typed compatibility posture");
  assert.deepEqual(result.viewportSize, { width: 1440, height: 900 }, "viewport host capability size should be readable from the product surface");
  assert.equal(result.viewportCompatibility, "Reattachable", "viewport host capability descriptors should expose the reattachable default posture");
  assert.equal(result.onlineState, "offline", "online host capability state should be readable from the product surface");
  assert.equal(result.onlineCompatibility, "Reattachable", "online host capability descriptors should expose the reattachable default posture");
  assert.equal(result.clockNow, 5, "clock host capability state should be readable from the product surface");
  assert.equal(result.clockCompatibility, "SnapshotPortable", "clock host capability descriptors should expose the snapshot-portable default posture");
  assert.deepEqual(
    result.persistenceValue,
    { mode: "draft", revision: 2 },
    "persistence host capability state should be readable from the product surface after an explicit manual commit",
  );
  assert.equal(
    result.persistenceCompatibility,
    "ImportDenied",
    "persistence host capability descriptors should expose the import-denied default posture",
  );
  assert.equal(
    result.runtimeEnvelopeRestoreMode,
    "SameRuntimeExact",
    "runtime envelope artifacts should explicitly say that attached restore tokens are same-runtime exact restore lanes",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailableHostCapabilityCompatibility,
    "LiveOnly",
    "runtime envelope export should expose typed host capability read artifacts for host-backed callbacks",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailablePortableOutcome,
    "Denied",
    "portable runtime-envelope import should expose typed denial posture for live-only host capabilities",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailableViewportCompatibility,
    "Reattachable",
    "runtime envelope export should preserve the viewport family's typed compatibility posture",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailableViewportPortableOutcome,
    "Unavailable",
    "reattachable viewport host capabilities should export an unavailable portable-import posture instead of denial",
  );
  assert.equal(
    typeof result.runtimeEnvelopeUnavailableViewportPortableReason,
    "string",
    "runtime envelope export should expose a typed portable-import explanation for viewport host capabilities",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailableOnlineCompatibility,
    "Reattachable",
    "runtime envelope export should preserve the second family's typed compatibility posture",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailableOnlinePortableOutcome,
    "Unavailable",
    "reattachable host capabilities should export an unavailable portable-import posture instead of denial",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailableClockCompatibility,
    "SnapshotPortable",
    "runtime envelope export should preserve the clock family's typed compatibility posture",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailableClockPortableOutcome,
    "Unavailable",
    "snapshot-portable host capabilities should export an unavailable portable-import posture instead of denial",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailablePersistenceCompatibility,
    "ImportDenied",
    "runtime envelope export should preserve the persistence family's typed compatibility posture",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailablePersistencePortableOutcome,
    "Denied",
    "import-denied host capabilities should export a denied portable-import posture",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailableExactRestoreOutcome,
    "Live",
    "attached exact restore tokens should keep same-runtime exact restore distinct from portable import denial",
  );
  assert.equal(
    typeof result.runtimeEnvelopeUnavailablePortableReason,
    "string",
    "runtime envelope export should expose a typed portable-import explanation for unavailable host capabilities",
  );
  assert.equal(
    typeof result.runtimeEnvelopeUnavailableOnlinePortableReason,
    "string",
    "runtime envelope export should expose a typed portable-import explanation for reattachable host capabilities",
  );
  assert.equal(
    result.runtimeEnvelopeUnavailableCurrentReads.some((read) => String(read).startsWith("__forgeSignal.host.")),
    false,
    "runtime envelope export should not leak hidden framework host backing ids through public callback currentReads",
  );
  assert.equal(result.hostCapabilityInvalidationCount, 5, "performanceSummary should expose host capability invalidation count");
  assert.equal(result.hostCapabilityReadCount >= 15, true, "performanceSummary should expose host capability read count across callback and direct host reads");
  assert.equal(result.hostCapabilityPollCount > 0, true, "performanceSummary should expose polling activity for polled host families");
  assert.equal(result.hostCapabilityNoOpPollCount >= 0, true, "performanceSummary should expose no-op poll count for polled host families");
  assert.equal(
    result.hostCapabilityManualCommitCount,
    2,
    "performanceSummary should expose manual-commit activity for manually committed host families",
  );
  assert.equal(
    result.hostCapabilityNoOpManualCommitCount,
    1,
    "performanceSummary should expose no-op manual commit suppression for manually committed host families",
  );
  assert.equal(result.hostCapabilityUnavailabilityArtifactCount, 5, "performanceSummary should expose exported host capability unavailability artifact count");
  assert.equal(result.hostCapabilityBroadFanoutDenialCount, 0, "performanceSummary should expose host capability broad-fanout denial count even when no denial mode is active");
  assert.equal(typeof result.hostCapabilityReportDigest, "string", "diagnostics should expose a canonical host capability report digest");
  assert.equal(typeof result.hostCapabilityReportLineageDigest, "string", "diagnostics should expose a canonical host capability lineage digest");
  assert.equal(typeof result.hostCapabilityReportBreadthDigest, "string", "diagnostics should expose a canonical host capability breadth digest");
  assert.equal(result.hostCapabilityReportFamilyCount >= 5, true, "diagnostics host capability reports should preserve mixed-family event attribution");
  assert.equal(result.hostCapabilityReportLineageCount >= 5, true, "diagnostics host capability reports should retain a mixed-family event lineage in short package smoke runs");
  assert.equal(result.hostCapabilityReportLineageCount <= 32, true, "diagnostics host capability reports should keep lineage retention bounded");
  assert.equal(result.hostCapabilityReportMaxTouchedNodes >= 1, true, "diagnostics host capability breadth reports should expose touched-node breadth");
  assert.equal(result.hostCapabilityReportMaxReevaluatedNodes >= 1, true, "diagnostics host capability breadth reports should expose reevaluation breadth");
  assert.equal(typeof result.transportReportDigest, "string", "adapters should expose a canonical host capability transport report digest");
  assert.equal(result.transportReportUnavailableArtifactCount, result.runtimeEnvelopeUnavailableCallbackCount, "transport reports should count all unavailable callback artifacts in the exported runtime envelope");
  assert.equal(result.transportReportDeniedFamilyCount, 2, "transport reports should distinguish denied host-capability families");
  assert.equal(result.transportReportUnavailableFamilyCount, 3, "transport reports should distinguish unavailable host-capability families");
  assert.equal(result.hostCapabilityInvalidationBatchFlushCount, 5, "performanceSummary should expose batched host invalidation flush count");
  assert.equal(result.hostCapabilityReevaluationCount, 10, "performanceSummary should expose reevaluation breadth driven by host invalidation");
  assert.equal(
    typeof result.hostCapabilityInvalidationTouchedNodeCount,
    "number",
    "performanceSummary should expose host invalidation touched-node breadth",
  );
  assert.equal(result.latestHostCapabilityEventKind, "InvalidationNoOpSuppressed", "the exporting runtime should retain the latest host-capability lifecycle event, including no-op manual commits");
  assert.equal(result.latestHostCapabilityEventQueuedCount, 1, "host invalidation events should retain queued batch breadth per family");
  assert.equal(result.recentHostCapabilityEventCount >= 6, true, "the exporting runtime should retain host invalidation event history across push, polled, and manual-commit families");
  assert.equal(result.branchIdType, "number", "branch handles should expose numeric ids");
  assert.equal(result.replayFrameCount > 0, true, "branch replay should expose retained frames");
  assert.equal(result.replayHasCallback, true, "branch replay should preserve callback metadata");
  assert.equal(result.snapshotBranchId, 0, "snapshot envelope should serialize structured snapshot metadata");
  assert.equal(result.branchSnapshotBranchId, 0, "branch snapshot should serialize structured snapshot metadata");
  assert.equal(
    result.branchSnapshotRestoreMode,
    "SameRuntimeExact",
    "branch snapshot artifacts should explicitly mark same-runtime exact restore posture",
  );
  assert.equal(
    result.branchEnvelopeRestoreMode,
    "SameRuntimeExact",
    "branch snapshot envelope artifacts should explicitly mark same-runtime exact restore posture",
  );
  assert.equal(typeof result.exportedPolicyPreset, "string", "runtime envelope definitions should expose typed policy presets");
  assert.equal(typeof result.snapshotPolicyTier, "string", "snapshot metadata should expose typed runtime policy tiers");
  assert.equal(
    result.snapshotReplayHead === null || typeof result.snapshotReplayHead === "number",
    true,
    "snapshot metadata should expose a typed replay-head cursor or null",
  );
  assert.equal(typeof result.snapshotExplanationRetention, "string", "snapshot artifact retention policy should expose typed retention categories");
  assert.equal(result.restoredExactDoubled, 4, "exact runtime-envelope restore should restore callback-computed committed truth through the JS boundary");
  assert.equal(
    result.portableImportErrorCode,
    "computeCallbackUnavailableForRuntimeEnvelopeImport",
    "portable runtime-envelope import should surface a typed denial code for unavailable callback-backed host capability state",
  );
  assert.equal(
    typeof result.portableImportErrorMessage,
    "string",
    "portable runtime-envelope import denial should stay self-describing",
  );
  assert.equal(
    result.portableImportErrorMessage.includes("runtime envelope import cannot restore callback-backed nodes without live callback registrations"),
    true,
    "portable runtime-envelope import denial should explain why live callback-backed host capability state cannot travel",
  );
  assert.equal(
    result.portableImportLatestHostCapabilityEventKind,
    "PortableImportDenied",
    "the importing runtime should record a typed host-capability denial event",
  );
  assert.equal(
    result.portableImportLatestHostCapabilityEventQueuedCount,
    0,
    "portable import denial events should not masquerade as queued invalidations",
  );
  assert.deepEqual(
    result.portableImportLatestHostCapabilityEventDeniedIds,
    ["visibleLabel"],
    "the latest portable import denial event should stay family-scoped instead of flattening denied families together",
  );
  assert.deepEqual(
    result.portableImportDeniedCallbackIds.sort(),
    ["persistenceLabel", "visibleLabel"],
    "portable import denial event history should identify all denied callback nodes across denied host-capability families",
  );
  assert.equal(
    result.portableImportRecentHostCapabilityEventCount,
    2,
    "the importing runtime should retain its host-capability denial event history",
  );
  assert.equal(
    result.portableImportHostCapabilityCompatibilityDenialCount,
    2,
    "the importing runtime performance summary should expose host capability compatibility denial count",
  );
  assert.equal(typeof result.portableImportHostCapabilityReportDigest, "string", "import-side diagnostics should expose a canonical host capability report digest");
  assert.equal(typeof result.portableImportHostCapabilityReportLineageDigest, "string", "import-side diagnostics should expose a canonical host capability lineage digest");
  assert.equal(result.portableImportHostCapabilityReportFamilyCount, 2, "import-side host capability reports should remain family-scoped to denied families");
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
  const source = `import { clockCapability, createSignals, hostCapabilityPlan, onlineCapability, persistenceCapability, viewportCapability, visibilityCapability } from "${packageName}";
import {
  createReactSignalsStore,
  useOutputValue,
  useSignalValue,
  useSignalsDiagnostics,
} from "${packageName}/react";

let visibilityState: "visible" | "hidden" = "visible";
let viewportState = { width: 1280, height: 720 };
let onlineState: "online" | "offline" = "online";
let clockTick = 0;
let persistedDraft = { mode: "draft" as const, revision: 1 };
const signals = createSignals({
  hostCapabilities: hostCapabilityPlan({
    visibility: visibilityCapability({
      source: {
        current() {
          return visibilityState;
        },
        subscribe() {
          return () => {};
        },
      },
    }),
    viewport: viewportCapability({
      source: {
        current() {
          return viewportState;
        },
        subscribe() {
          return () => {};
        },
      },
    }),
    online: onlineCapability({
      source: {
        current() {
          return onlineState;
        },
        subscribe() {
          return () => {};
        },
      },
    }),
    clock: clockCapability({
      source: {
        current() {
          return clockTick;
        },
      },
      pollMs: 5,
    }),
    persistence: persistenceCapability({
      source: {
        current() {
          return persistedDraft;
        },
      },
    }),
  }),
});
const count = signals.input(1, { id: "count" });
const hostViewport = signals.host.viewport;
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
const hostVisibility = signals.host.visibility;
const hostOnline = signals.host.online;
const hostClock = signals.host.clock;
const hostPersistence = signals.host.persistence;
const viewportLabel = signals.computed(
  () => (hostViewport?.width() ?? 0) + "x" + (hostViewport?.height() ?? 0),
  { id: "viewportLabel" },
);
const persistenceLabel = signals.computed(
  () => hostPersistence?.value().revision ?? 0,
  { id: "persistenceLabel" },
);
const panel = signals.output(() => ({
  count: count(),
  doubled: doubled(),
}), { id: "panel" });
const store = createReactSignalsStore(signals);
persistedDraft = { mode: "draft", revision: 2 };
const persistenceCommit = hostPersistence?.commit();
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
const latestHostCapabilityEvent = diagnostics.latestHostCapabilityEvent();
const recentHostCapabilityEvents = diagnostics.recentHostCapabilityEvents();
const hostCapabilityReport = diagnostics.hostCapabilityReport();
const performanceSummary = diagnostics.performanceSummary();
const delivered = latestObservation?.observation.delivered_event_count;
const callbackNodeIds = latestFlow?.callbackNodes.map((node) => node.id) ?? [];
const callbackHostCapabilityCompatibility =
  latestFlow?.callbackNodes[0]?.hostCapabilityReads[0]?.compatibility ??
  latestObservation?.callbackNodes[0]?.hostCapabilityReads[0]?.compatibility ??
  null;
const latestHostCapabilityEventKind = latestHostCapabilityEvent?.kind ?? null;
const latestHostCapabilityQueuedCount = latestHostCapabilityEvent?.queuedInvalidationCount ?? 0;
const latestHostCapabilityDeniedIds = latestHostCapabilityEvent?.deniedCallbackIds ?? [];
const hostCapabilityLineageDigest = hostCapabilityReport.lineageDigest;
const hostCapabilityBreadthDigest = hostCapabilityReport.breadthDigest;
const hostCapabilityLineageEntry = hostCapabilityReport.lineage[0] ?? null;
const hostCapabilityBreadthFamily = hostCapabilityReport.breadth.families[0] ?? null;
const hostCapabilityReadCount = performanceSummary.hostCapabilityReadCount ?? 0;
const hostCapabilityReevaluationCount = performanceSummary.hostCapabilityReevaluationCount ?? 0;
const hostCapabilityCompatibilityDenialCount =
  performanceSummary.hostCapabilityCompatibilityDenialCount ?? 0;
const hostCapabilityPollCount = performanceSummary.hostCapabilityPollCount ?? 0;
const hostCapabilityNoOpPollCount = performanceSummary.hostCapabilityNoOpPollCount ?? 0;
const visibilityMode = hostVisibility?.state() ?? "hidden";
const visibilityDescriptor = hostVisibility?.descriptor();
const viewportSize = hostViewport?.size() ?? { width: 0, height: 0 };
const viewportWidth = hostViewport?.width() ?? 0;
const viewportHeight = hostViewport?.height() ?? 0;
const viewportDescriptor = hostViewport?.descriptor();
const onlineMode = hostOnline?.state() ?? "offline";
const onlineDescriptor = hostOnline?.descriptor();
const onlineFlag = hostOnline?.isOnline() ?? false;
const clockNow = hostClock?.now() ?? 0;
const clockDescriptor = hostClock?.descriptor();
const persistenceValue = hostPersistence?.value() ?? { mode: "draft", revision: 0 };
const persistenceDescriptor = hostPersistence?.descriptor();
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
void callbackHostCapabilityCompatibility;
void latestHostCapabilityEventKind;
void latestHostCapabilityQueuedCount;
void latestHostCapabilityDeniedIds;
void recentHostCapabilityEvents;
void hostCapabilityLineageDigest;
void hostCapabilityBreadthDigest;
void hostCapabilityLineageEntry;
void hostCapabilityBreadthFamily;
void hostCapabilityReadCount;
void hostCapabilityReevaluationCount;
void hostCapabilityCompatibilityDenialCount;
void hostCapabilityPollCount;
void hostCapabilityNoOpPollCount;
void visibilityState;
void visibilityMode;
void visibilityDescriptor;
void onlineState;
void onlineMode;
void onlineDescriptor;
void onlineFlag;
void clockTick;
void clockNow;
void clockDescriptor;
void persistedDraft;
void hostPersistence;
void persistenceLabel;
void persistenceCommit;
void persistenceValue;
void persistenceDescriptor;
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
void viewportState;
void hostViewport;
void viewportLabel;
void viewportSize;
void viewportWidth;
void viewportHeight;
void viewportDescriptor;
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
    "package/product/host_capabilities.js",
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
