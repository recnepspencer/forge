import {
  createSignalRuntime,
  define,
  expr,
  keyed,
  policy,
  type BranchMergePlan,
  type BranchMergeResult,
  type BranchStateProofReport,
  type MergePlanProofReport,
  type MergeResultProofReport,
  type ReplayArtifactProofInput,
  type ReplayArtifactProofReport,
  type ReplayParityProofReport,
  type ReplayFrameSummary,
  type RuntimeEnvelope,
  type RuntimeProofReport,
  type SignalRuntime,
  type WhySummary,
} from "@forge/signal";

import { defaultSceneState, renderScene } from "./renderer";
import {
  type ScenePatch,
  type BranchInspect,
  type BranchId,
  type GearDimensionsModel,
  type GearProfileModel,
  type GearTopologyModel,
  type GearMeshModel,
  type GearToothModel,
  type HudModel,
  type LightingModel,
  type BranchStateProofView,
  type MergePlan,
  type MergePlanProofView,
  type MergeRecordView,
  type MergeResult,
  type MergeResultProofView,
  type MergeSemanticsView,
  type IdentityCorrespondenceView,
  type ReplayParityProofView,
  type ReplayArtifactProofView,
  type RenderAspects,
  type RenderUpdate,
  type ScenarioProofArtifacts,
  type SceneRuntimeBundle,
  type SceneState,
  type ViewportProjectionModel,
  type ViewportShadingModel,
} from "./types";

const DEFAULT_POLICY = policy.preset("webDevelopment");

const CAMERA_SOURCE_IDS = ["cameraX", "cameraY", "cameraZ", "cameraYaw", "cameraPitch"] as const;
const LIGHT_SOURCE_IDS = ["lightX", "lightY", "lightZ", "lightIntensity"] as const;
const ASPECT_RECIPE_IDS = [
  "gearDimensionsModel",
  "gearProfileModel",
  "gearTopologyModel",
  "gearMeshModel",
  "lightingModel",
  "viewportProjectionModel",
  "viewportShadingModel",
] as const;
const HUD_SOURCE_IDS = [
  "hudFrameIndex",
  "hudRaysMarched",
  "hudAverageSteps",
  "hudHits",
  "hudMisses",
  "hudRenderMs",
  "hudTouchedNodes",
  "hudNodesEvaluated",
  "hudNodesSuppressed",
  "hudTotalNanos",
] as const;

type RuntimeProgress = (phase: string, detail?: string) => void;
const frameCounters = new WeakMap<SignalRuntime, number>();

export async function createSceneRuntime(progress?: RuntimeProgress): Promise<SceneRuntimeBundle> {
  progress?.("runtime:init", "creating wasm runtime");
  const runtime = await createSignalRuntime();
  progress?.("runtime:policy", "setting runtime policy");
  runtime.setRuntimePolicy(DEFAULT_POLICY);

  const scene = defaultSceneState();
  progress?.("runtime:sources", "defining scene sources");
  defineSceneSources(runtime, scene);
  progress?.("runtime:aspects", "defining aspect recipes");
  defineAspectRecipes(runtime);
  progress?.("runtime:hud-sources", "defining hud sources");
  defineHudSources(runtime);
  progress?.("runtime:hud-recipe", "defining hud recipe");
  defineHudRecipe(runtime);

  progress?.("runtime:first-render", "rendering initial branch");
  await renderBranch(runtime, runtime.history().currentBranch().id, progress);
  frameCounters.set(runtime, 1);
  progress?.("runtime:ready", "scene runtime ready");
  return { runtime };
}

export function ensureFeatureBranch(runtime: SignalRuntime): BranchId {
  const existing = runtime.history().branches().find((branch) => branch.name === "what-if");
  if (existing) {
    return existing.id;
  }
  return runtime.history().createBranch("what-if").id;
}

export function branchCount(runtime: SignalRuntime): number {
  return runtime.history().branches().length;
}

export function totalGraphNodes(runtime: SignalRuntime): number {
  void runtime;
  return STATIC_NODE_COUNT;
}

export function displayBranchId(branchId: BranchId): string {
  return String(branchId);
}

export function readBranchSummary(
  runtime: SignalRuntime,
  branchId: BranchId,
  progress?: RuntimeProgress,
) {
  const history = runtime.history();
  const before = history.currentBranch().id;
  progress?.("read:branch-summary:switch-start", `switching to branch ${branchId}`);
  history.switchBranch(branchId);
  progress?.("read:branch-summary:switch-done", `on branch ${branchId}`);

  const branch = history.currentBranch();
  const state = readSceneState(runtime, progress);
  const hud = {
    frameIndex: frameCounters.get(runtime) ?? 0,
    raysMarched: 0,
    averageSteps: 0,
    hits: 0,
    misses: 0,
    renderMs: 0,
    touchedNodes: 0,
    nodesEvaluated: 0,
    nodesSuppressed: 0,
    totalNanos: 0,
    cameraX: state.camera.x,
    cameraY: state.camera.y,
    cameraZ: state.camera.z,
    lightX: state.light.x,
    lightY: state.light.y,
    lightZ: state.light.z,
  } satisfies HudModel;

  history.switchBranch(before);
  progress?.("read:branch-summary:return", `restored branch ${before}`);

  return {
    id: branch.id,
    name: branch.name,
    state,
    hud,
  };
}

export function readBranchInspect(runtime: SignalRuntime, branchId: BranchId): BranchInspect {
  return readBranchInspectForNode(runtime, branchId, "hudModel");
}

export function readBranchInspectForNode(
  runtime: SignalRuntime,
  branchId: BranchId,
  nodeId: string,
): BranchInspect {
  const history = runtime.history();
  const before = history.currentBranch().id;
  history.switchBranch(branchId);
  const inspectId = nodeId;
  const replay = history.replayFor(inspectId).frames.slice(-10) as ReplayFrameSummary[];
  const why = runtime.diagnostics().why(inspectId) as WhySummary;
  const lineage = history.lineageFor(inspectId);
  history.switchBranch(before);
  return { selectedNode: nodeId, replay, why, lineage };
}

export function readSceneState(runtime: SignalRuntime, progress?: RuntimeProgress): SceneState {
  progress?.("read:scene:cameraX:start", "reading cameraX");
  const cameraX = runtime.read<number>("cameraX");
  progress?.("read:scene:cameraX:done", "read cameraX");
  progress?.("read:scene:cameraY:start", "reading cameraY");
  const cameraY = runtime.read<number>("cameraY");
  progress?.("read:scene:cameraY:done", "read cameraY");
  progress?.("read:scene:cameraZ:start", "reading cameraZ");
  const cameraZ = runtime.read<number>("cameraZ");
  progress?.("read:scene:cameraZ:done", "read cameraZ");
  progress?.("read:scene:cameraYaw:start", "reading cameraYaw");
  const cameraYaw = runtime.read<number>("cameraYaw");
  progress?.("read:scene:cameraYaw:done", "read cameraYaw");
  progress?.("read:scene:cameraPitch:start", "reading cameraPitch");
  const cameraPitch = runtime.read<number>("cameraPitch");
  progress?.("read:scene:cameraPitch:done", "read cameraPitch");
  progress?.("read:scene:lightX:start", "reading lightX");
  const lightX = runtime.read<number>("lightX");
  progress?.("read:scene:lightX:done", "read lightX");
  progress?.("read:scene:lightY:start", "reading lightY");
  const lightY = runtime.read<number>("lightY");
  progress?.("read:scene:lightY:done", "read lightY");
  progress?.("read:scene:lightZ:start", "reading lightZ");
  const lightZ = runtime.read<number>("lightZ");
  progress?.("read:scene:lightZ:done", "read lightZ");
  progress?.("read:scene:lightIntensity:start", "reading lightIntensity");
  const lightIntensity = runtime.read<number>("lightIntensity");
  progress?.("read:scene:lightIntensity:done", "read lightIntensity");
  progress?.("read:scene:gearTeeth:start", "reading gearTeeth");
  const gearTeeth = runtime.read<number>("gearTeeth");
  progress?.("read:scene:gearTeeth:done", "read gearTeeth");
  progress?.("read:scene:gearOuterRadius:start", "reading gearOuterRadius");
  const gearOuterRadius = runtime.read<number>("gearOuterRadius");
  progress?.("read:scene:gearOuterRadius:done", "read gearOuterRadius");
  progress?.("read:scene:gearInnerRadius:start", "reading gearInnerRadius");
  const gearInnerRadius = runtime.read<number>("gearInnerRadius");
  progress?.("read:scene:gearInnerRadius:done", "read gearInnerRadius");
  progress?.("read:scene:gearThickness:start", "reading gearThickness");
  const gearThickness = runtime.read<number>("gearThickness");
  progress?.("read:scene:gearThickness:done", "read gearThickness");
  progress?.("read:scene:gearRotation:start", "reading gearRotation");
  const gearRotation = runtime.read<number>("gearRotation");
  progress?.("read:scene:gearRotation:done", "read gearRotation");
  return {
    camera: {
      x: cameraX,
      y: cameraY,
      z: cameraZ,
      yaw: cameraYaw,
      pitch: cameraPitch,
    },
    light: {
      x: lightX,
      y: lightY,
      z: lightZ,
      intensity: lightIntensity,
    },
    gear: {
      teeth: gearTeeth,
      outerRadius: gearOuterRadius,
      innerRadius: gearInnerRadius,
      thickness: gearThickness,
      rotation: gearRotation,
    },
  };
}

export async function updateScene(
  runtime: SignalRuntime,
  branchId: BranchId,
  patch: ScenePatch,
  progress?: RuntimeProgress,
): Promise<RenderUpdate> {
  const history = runtime.history();
  const before = history.currentBranch().id;
  history.switchBranch(branchId);

  const currentState = readSceneState(runtime, progress);
  const nextState = mergeSceneState(currentState, patch);
  const result = await renderCurrentBranch(
    runtime,
    nextState,
    buildScenePatchOps(patch),
    progress,
  );
  history.switchBranch(before);
  return result;
}

export async function renderBranch(
  runtime: SignalRuntime,
  branchId: BranchId,
  progress?: RuntimeProgress,
): Promise<RenderUpdate> {
  const history = runtime.history();
  const before = history.currentBranch().id;
  history.switchBranch(branchId);
  const state = readSceneState(runtime, progress);
  const result = await renderCurrentBranch(runtime, state, [], progress);
  history.switchBranch(before);
  return result;
}

export function readRenderAspects(runtime: SignalRuntime): RenderAspects {
  const dimensions = runtime.read<GearDimensionsModel>("gearDimensionsModel");
  const profile = runtime.read<GearProfileModel>("gearProfileModel");
  const topology = runtime.read<GearTopologyModel>("gearTopologyModel");
  const mesh = runtime.read<GearMeshModel>("gearMeshModel");

  const teethCount = Math.max(dimensions.teeth, 1);
  const teeth: GearToothModel[] = [];
  const step = (Math.PI * 2) / teethCount;
  for (let i = 0; i < teethCount; i++) {
    teeth.push({
      index: i,
      startAngle: i * step,
      midAngle: (i + 0.5) * step,
      endAngle: (i + 1) * step,
      rootRadius: profile.rootRadius,
      tipRadius: profile.tipRadius,
      thickness: dimensions.thickness,
    });
  }

  return {
    dimensions,
    profile,
    topology,
    mesh,
    teeth,
    lighting: runtime.read<LightingModel>("lightingModel"),
    projection: runtime.read<ViewportProjectionModel>("viewportProjectionModel"),
    shading: runtime.read<ViewportShadingModel>("viewportShadingModel"),
  };
}

export function planMerge(
  runtime: SignalRuntime,
  sourceBranchId: BranchId,
  targetBranchId: BranchId,
): MergePlan {
  const envelope = runtime.history().planMergeBranchesDetailedWithProof(sourceBranchId, targetBranchId);
  return projectMergePlan(envelope?.plan, envelope?.proof);
}

export async function executeMerge(
  runtime: SignalRuntime,
  sourceBranchId: BranchId,
  targetBranchId: BranchId,
): Promise<MergeResult> {
  const envelope = runtime.history().mergeBranchesDetailedWithProof(sourceBranchId, targetBranchId);
  const result = projectMergeResult(envelope?.result, envelope?.proof);
  await renderBranch(runtime, targetBranchId);
  return result;
}

export function readBranchStateProof(runtime: SignalRuntime, branchId: BranchId): BranchStateProofView {
  const proof = runtime.history().branchStateProof(branchId) as BranchStateProofReport;
  return {
    proofSchemaVersion: asStringOrNull(proof?.proofSchemaVersion),
    branchId: asNumberOrNull(proof?.branchId),
    branchName: asStringOrNull(proof?.branchName),
    snapshotId: asNumberOrNull(proof?.snapshotId),
    stateDigest: asStringOrNull(proof?.stateDigest),
  };
}

export function readReplayParityProof(
  runtime: SignalRuntime,
  expectedBranchId: BranchId,
  replayedBranchId: BranchId,
): ReplayParityProofView {
  const proof = runtime.history().replayParityProof(
    expectedBranchId,
    replayedBranchId,
  ) as ReplayParityProofReport;
  return {
    proofSchemaVersion: asStringOrNull(proof?.proofSchemaVersion),
    expectedBranchId: asNumberOrNull(proof?.expectedBranchId),
    expectedBranchName: asStringOrNull(proof?.expectedBranchName),
    expectedSnapshotId: asNumberOrNull(proof?.expectedSnapshotId),
    expectedStateDigest: asStringOrNull(proof?.expectedStateDigest),
    replayedBranchId: asNumberOrNull(proof?.replayedBranchId),
    replayedBranchName: asStringOrNull(proof?.replayedBranchName),
    replayedSnapshotId: asNumberOrNull(proof?.replayedSnapshotId),
    replayedStateDigest: asStringOrNull(proof?.replayedStateDigest),
    parity: typeof proof?.parity === "boolean" ? proof.parity : null,
    mismatchClasses: Array.isArray(proof?.mismatchClasses)
      ? proof.mismatchClasses.map((value: unknown) => String(value))
      : [],
  };
}

export function readReplayArtifactProof(
  runtime: SignalRuntime,
  expected: ReplayArtifactProofInput,
  replayedBranchId: BranchId,
): ReplayArtifactProofView {
  const proof = runtime.history().replayArtifactProof(
    expected,
    replayedBranchId,
  ) as ReplayArtifactProofReport;
  return {
    proofSchemaVersion: asStringOrNull(proof?.proofSchemaVersion),
    parity: typeof proof?.parity === "boolean" ? proof.parity : null,
    mismatchClasses: Array.isArray(proof?.mismatchClasses)
      ? proof.mismatchClasses.map((value: unknown) => String(value))
      : [],
    replayedLoweredStrategyBundleDigest: asStringOrNull(
      proof?.replayed?.loweredStrategyBundleDigest,
    ),
    replayedMergePlanDigest: asStringOrNull(proof?.replayed?.mergePlanDigest),
    replayedMergeResultDigest: asStringOrNull(proof?.replayed?.mergeResultDigest),
    replayedLineageDigest: asStringOrNull(proof?.replayed?.lineageDigest),
    replayedBranchStateDigest: asStringOrNull(proof?.replayed?.branchStateDigest),
    replayedRegistryBundleDigest: asStringOrNull(proof?.replayed?.registryBundleDigest),
  };
}

export function readRuntimeProofReport(runtime: SignalRuntime): RuntimeProofReport {
  return runtime.adapters().runtimeProofReport() as RuntimeProofReport;
}

export function buildScenarioProofArtifacts(params: {
  runtime: SignalRuntime;
  mergePlan: MergePlan | null;
  mergeResult: MergeResult | null;
  activeBranchId: BranchId | null;
  previousProof?: ScenarioProofArtifacts | null;
}): ScenarioProofArtifacts {
  const { runtime, mergePlan, mergeResult, activeBranchId, previousProof = null } = params;
  const runtimeProof = readRuntimeProofReport(runtime);
  const semanticsDigest = mergeResult?.proof?.semanticsDigest ?? mergePlan?.proof?.semanticsDigest ?? null;
  const mergePlanDigest = mergePlan?.proof?.planDigest ?? null;
  const mergeResultDigest = mergeResult?.proof?.resultDigest ?? null;
  const lineageDigest = mergeResult?.proof?.lineageDigest ?? null;
  const mergedBranchStateDigest =
    mergeResult != null && activeBranchId != null
      ? readBranchStateProof(runtime, activeBranchId).stateDigest
      : null;

  return {
    proofSchemaVersion:
      mergeResult?.proof?.proofSchemaVersion
      ?? mergePlan?.proof?.proofSchemaVersion
      ?? runtimeProof.proofSchemaVersion
      ?? null,
    schemaDigest: runtimeProof.schemaRegistryDigest ?? null,
    registryBundleDigest: runtimeProof.registryBundleDigest ?? null,
    loweredStrategyBundleDigest:
      mergeResult?.proof?.loweredStrategyBundleDigest
      ?? mergePlan?.proof?.loweredStrategyBundleDigest
      ?? null,
    semanticsDigest,
    mergePlanDigest,
    mergeResultDigest,
    lineageDigest,
    mergedBranchStateDigest,
    replayedLoweredStrategyBundleDigest: previousProof?.replayedLoweredStrategyBundleDigest ?? null,
    replayedMergePlanDigest: previousProof?.replayedMergePlanDigest ?? null,
    replayedMergeResultDigest: previousProof?.replayedMergeResultDigest ?? null,
    replayedLineageDigest: previousProof?.replayedLineageDigest ?? null,
    replayBranchStateDigest: previousProof?.replayBranchStateDigest ?? null,
    replayParity: previousProof?.replayParity ?? null,
    replayMismatchClasses: previousProof?.replayMismatchClasses ?? [],
  };
}

export function buildReplayScenarioProofArtifacts(params: {
  runtime: SignalRuntime;
  replayedBranchId: BranchId | null;
  previousProof?: ScenarioProofArtifacts | null;
}) {
  const { runtime, replayedBranchId, previousProof = null } = params;
  if (previousProof == null || replayedBranchId == null) {
    return null;
  }

  return readReplayArtifactProof(
    runtime,
    {
      proofSchemaVersion: previousProof.proofSchemaVersion ?? "legacy-unknown",
      registryBundleDigest: previousProof.registryBundleDigest,
      loweredStrategyBundleDigest: previousProof.loweredStrategyBundleDigest,
      mergePlanDigest: previousProof.mergePlanDigest,
      mergeResultDigest: previousProof.mergeResultDigest,
      lineageDigest: previousProof.lineageDigest,
      branchStateDigest: previousProof.mergedBranchStateDigest ?? "",
    },
    replayedBranchId,
  );
}

function projectMergePlan(plan: BranchMergePlan | null | undefined, proof: MergePlanProofReport | null | undefined): MergePlan {
  return {
    sourceBranchId: asNumberOrNull(plan?.source_branch_id),
    targetBranchId: asNumberOrNull(plan?.target_branch_id),
    mergeKind: asStringOrNull(plan?.merge_kind),
    divergence: asStringOrNull(plan?.divergence),
    mergeStrategy: asStringOrNull(plan?.merge_strategy),
    sourceSnapshotId: asNumberOrNull(plan?.source_snapshot_id),
    targetSnapshotIdBefore: asNumberOrNull(plan?.target_snapshot_id_before),
    candidateCount: countEntries(plan?.planned_candidates?.nodes),
    sharedNodeCount: countEntries(plan?.proof_minimal_overlap?.shared_nodes),
    expandedNodeCount: countEntries(plan?.conservative_overlap?.expanded_nodes),
    supportNodeCount: countEntries(plan?.conservative_overlap?.support_nodes),
    nodePlanCount: countEntries(plan?.node_plan),
    adoptionCount: countEntries(plan?.adoption_core),
    hasResolutionPlan: Boolean(plan?.resolution_plan),
    semantics: projectSemantics(plan),
    identity: {
      targetCandidateCount: asNumber(plan?.identity_correspondence?.target_candidate_count),
      sourceLookupCount: asNumber(plan?.identity_correspondence?.source_lookup_count),
      ambiguousMatchCount: asNumber(plan?.identity_correspondence?.ambiguous_match_count),
      rejectedAdmissibilityCount: asNumber(
        plan?.identity_correspondence?.rejected_admissibility_count,
      ),
      records: projectIdentityRecords(plan?.identity_correspondence?.records),
    },
    deletion: {
      targetOnlyCount: asNumber(plan?.deletion_plan?.target_only_count),
      rejectedTargetOnlyCount: asNumber(plan?.deletion_plan?.rejected_target_only_count),
      targetOnlyNodes: projectNodeIds(plan?.deletion_plan?.target_only_nodes),
    },
    conflictIsolation: {
      policyName: asStringOrNull(plan?.conflict_isolation_plan?.selected_policy_name),
      policyDigest: asStringOrNull(plan?.conflict_isolation_plan?.selected_policy_digest),
      policyBasis: asStringOrNull(plan?.conflict_isolation_plan?.selected_policy_basis),
      expansionBreadth: asNumber(plan?.conflict_isolation_plan?.expansion_breadth),
      witnessGranularity: asStringOrNull(plan?.conflict_isolation_plan?.witness?.granularity),
      witnessConflictRecordCount: asNumber(
        plan?.conflict_isolation_plan?.witness?.conflict_record_count,
      ),
      isolatedRegionCount: asNumber(
        plan?.conflict_isolation_plan?.region_summary?.isolated_region_count,
      ),
      hostDeclaredRegionCount: asNumber(
        plan?.conflict_isolation_plan?.region_summary?.host_declared_region_count,
      ),
      conservativeExpandedNodeCount: asNumber(
        plan?.conflict_isolation_plan?.conservative_expansion?.expanded_node_count,
      ),
      records: projectConflictIsolationRecords(plan?.conflict_isolation_plan?.records),
    },
    aspectPolicies: projectAspectPolicyRecords(plan?.aspect_policy_plan?.records),
    aspectDecisions: projectAspectDecisionRecords(plan?.aspect_decision_plan?.records),
    proof: projectMergePlanProof(proof),
  };
}

function projectMergeResult(result: BranchMergeResult | null | undefined, proof: MergeResultProofReport | null | undefined): MergeResult {
  return {
    sourceBranchId: asNumberOrNull(result?.source_branch),
    targetBranchId: asNumberOrNull(result?.target_branch),
    mergeKind: asStringOrNull(result?.merge_kind),
    divergence: asStringOrNull(result?.divergence),
    mergeStrategy: asStringOrNull(result?.merge_strategy),
    mergedSnapshotId: asNumberOrNull(result?.merged_snapshot_id),
    targetSnapshotIdBefore: asNumberOrNull(result?.target_snapshot_id_before),
    targetSnapshotIdAfter: asNumberOrNull(result?.target_snapshot_id_after),
    sourceSnapshotId: asNumberOrNull(result?.source_snapshot_id),
    recordCount: countEntries(result?.records),
    adoptedCount: asNumber(result?.counters?.adopted_count),
    introducedCount: asNumber(result?.counters?.introduced_node_count),
    replacedCount: asNumber(result?.counters?.replaced_count),
    preservedTargetCount: asNumber(result?.counters?.preserved_target_count),
    equivalentUnchangedCount: asNumber(result?.counters?.equivalent_unchanged_count),
    skippedNonAdoptableCount: asNumber(result?.counters?.skipped_non_adoptable_count),
    conflictCount: Array.isArray(result?.records)
      ? result.records.reduce(
          (total: number, record: any) => total + countEntries(record?.resolved_conflict_kinds),
          0,
        )
      : 0,
    hasResolutionPlan: Boolean(result?.resolution_plan),
    semantics: projectSemantics(result),
    counters: {
      sourceSliceBreadth: asNumber(result?.counters?.source_slice_breadth),
      proofMinimalOverlapBreadth: asNumber(result?.counters?.proof_minimal_overlap_breadth),
      conservativeOverlapExpansionBreadth: asNumber(
        result?.counters?.conservative_overlap_expansion_breadth,
      ),
      finalCandidateBreadth: asNumber(result?.counters?.final_candidate_breadth),
      reconciliationBreadth: asNumber(result?.counters?.reconciliation_breadth),
      targetOnlyCount: asNumber(result?.counters?.target_only_count),
      identityTargetCandidatesIndexed: asNumber(
        result?.counters?.identity_target_candidates_indexed,
      ),
      identitySourceLookups: asNumber(result?.counters?.identity_source_lookups),
      identityAmbiguousMatchCount: asNumber(result?.counters?.identity_ambiguous_match_count),
      identityRejectedAdmissibilityCount: asNumber(
        result?.counters?.identity_rejected_admissibility_count,
      ),
      conflictIsolationRecordCount: asNumber(
        result?.counters?.conflict_isolation_record_count,
      ),
      conflictIsolationExpansionBreadth: asNumber(
        result?.counters?.conflict_isolation_expansion_breadth,
      ),
    },
    identity: {
      records: projectIdentityRecords(result?.identity_correspondence?.records),
    },
    deletion: {
      targetOnlyCount: asNumber(result?.deletion_plan?.target_only_count),
      rejectedTargetOnlyCount: asNumber(result?.deletion_plan?.rejected_target_only_count),
      targetOnlyNodes: projectNodeIds(result?.deletion_plan?.target_only_nodes),
    },
    conflictIsolation: {
      policyName: asStringOrNull(result?.conflict_isolation_plan?.selected_policy_name),
      policyDigest: asStringOrNull(result?.conflict_isolation_plan?.selected_policy_digest),
      policyBasis: asStringOrNull(result?.conflict_isolation_plan?.selected_policy_basis),
      expansionBreadth: asNumber(result?.conflict_isolation_plan?.expansion_breadth),
      witnessGranularity: asStringOrNull(result?.conflict_isolation_plan?.witness?.granularity),
      witnessConflictRecordCount: asNumber(
        result?.conflict_isolation_plan?.witness?.conflict_record_count,
      ),
      isolatedRegionCount: asNumber(
        result?.conflict_isolation_plan?.region_summary?.isolated_region_count,
      ),
      hostDeclaredRegionCount: asNumber(
        result?.conflict_isolation_plan?.region_summary?.host_declared_region_count,
      ),
      conservativeExpandedNodeCount: asNumber(
        result?.conflict_isolation_plan?.conservative_expansion?.expanded_node_count,
      ),
      records: projectConflictIsolationRecords(result?.conflict_isolation_plan?.records),
    },
    aspectPolicies: projectAspectPolicyRecords(result?.aspect_policy_plan?.records),
    aspectDecisions: projectAspectDecisionRecords(result?.aspect_decision_plan?.records),
    records: projectMergeRecords(result?.records),
    proof: projectMergeResultProof(proof),
  };
}

function projectMergePlanProof(proof: MergePlanProofReport | null | undefined): MergePlanProofView | null {
  if (!proof) {
    return null;
  }
  return {
    proofSchemaVersion: asStringOrNull(proof?.proofSchemaVersion),
    registryBundleDigest: asStringOrNull(proof?.registryBundleDigest),
    planDigest: asStringOrNull(proof?.planDigest),
    semanticsDigest: asStringOrNull(proof?.semanticsDigest),
    loweredStrategyBundleDigest: asStringOrNull(proof?.loweredStrategyBundleDigest),
    selectedStrategyDigest: asStringOrNull(proof?.selectedStrategyDigest),
    selectedMergeBaseDigest: asStringOrNull(proof?.selectedMergeBaseDigest),
    selectedConflictPolicyDigest: asStringOrNull(proof?.selectedConflictPolicyDigest),
    selectedConflictIsolationDigest: asStringOrNull(proof?.selectedConflictIsolationDigest),
    selectedIdentityMatcherDigest: asStringOrNull(proof?.selectedIdentityMatcherDigest),
    selectedSourceOnlyPolicyDigest: asStringOrNull(proof?.selectedSourceOnlyPolicyDigest),
    selectedDeletionPolicyDigest: asStringOrNull(proof?.selectedDeletionPolicyDigest),
  };
}

function projectMergeResultProof(proof: MergeResultProofReport | null | undefined): MergeResultProofView | null {
  if (!proof) {
    return null;
  }
  return {
    proofSchemaVersion: asStringOrNull(proof?.proofSchemaVersion),
    registryBundleDigest: asStringOrNull(proof?.registryBundleDigest),
    resultDigest: asStringOrNull(proof?.resultDigest),
    semanticsDigest: asStringOrNull(proof?.semanticsDigest),
    loweredStrategyBundleDigest: asStringOrNull(proof?.loweredStrategyBundleDigest),
    lineageDigest: asStringOrNull(proof?.lineageDigest),
    selectedStrategyDigest: asStringOrNull(proof?.selectedStrategyDigest),
    selectedMergeBaseDigest: asStringOrNull(proof?.selectedMergeBaseDigest),
    selectedConflictPolicyDigest: asStringOrNull(proof?.selectedConflictPolicyDigest),
    selectedConflictIsolationDigest: asStringOrNull(proof?.selectedConflictIsolationDigest),
    selectedIdentityMatcherDigest: asStringOrNull(proof?.selectedIdentityMatcherDigest),
    selectedSourceOnlyPolicyDigest: asStringOrNull(proof?.selectedSourceOnlyPolicyDigest),
    selectedDeletionPolicyDigest: asStringOrNull(proof?.selectedDeletionPolicyDigest),
  };
}

function projectSemantics(value: any): MergeSemanticsView {
  return {
    strategyName: asStringOrNull(
      value?.selected_strategy_name ?? value?.selected_semantics?.strategy_name,
    ),
    strategyBasis: asStringOrNull(
      value?.selected_strategy_basis ?? value?.selected_semantics?.strategy_basis,
    ),
    mergeBaseName: asStringOrNull(
      value?.selected_merge_base_name ?? value?.selected_semantics?.merge_base_name,
    ),
    mergeBaseBasis: asStringOrNull(
      value?.selected_merge_base_basis ?? value?.selected_semantics?.merge_base_basis,
    ),
    conflictPolicyName: asStringOrNull(
      value?.selected_conflict_policy_name ?? value?.selected_semantics?.conflict_policy_name,
    ),
    conflictPolicyBasis: asStringOrNull(
      value?.selected_conflict_policy_basis ?? value?.selected_semantics?.conflict_policy_basis,
    ),
    conflictIsolationName: asStringOrNull(
      value?.selected_conflict_isolation_name
      ?? value?.selected_semantics?.conflict_isolation_name,
    ),
    conflictIsolationBasis: asStringOrNull(
      value?.selected_conflict_isolation_basis
      ?? value?.selected_semantics?.conflict_isolation_basis,
    ),
    identityMatcherName: asStringOrNull(
      value?.selected_identity_matcher_name ?? value?.selected_semantics?.identity_matcher_name,
    ),
    identityMatcherBasis: asStringOrNull(
      value?.selected_identity_matcher_basis ?? value?.selected_semantics?.identity_matcher_basis,
    ),
    sourceOnlyPolicyName: asStringOrNull(
      value?.selected_source_only_policy_name ?? value?.selected_semantics?.source_only_policy_name,
    ),
    sourceOnlyPolicyBasis: asStringOrNull(
      value?.selected_source_only_policy_basis ?? value?.selected_semantics?.source_only_policy_basis,
    ),
    deletionPolicyName: asStringOrNull(
      value?.selected_deletion_policy_name ?? value?.selected_semantics?.deletion_policy_name,
    ),
    deletionPolicyBasis: asStringOrNull(
      value?.selected_deletion_policy_basis ?? value?.selected_semantics?.deletion_policy_basis,
    ),
  };
}

function projectIdentityRecords(records: any): IdentityCorrespondenceView[] {
  if (!Array.isArray(records)) {
    return [];
  }
  return records.map((record) => ({
    sourceNode: nodeIdLabel(record?.source_node),
    targetNode: nodeIdLabelOrNull(record?.target_node),
    status: asString(record?.status),
    basis: asStringOrNull(record?.basis),
    candidateCount: asNumber(record?.candidate_count),
    candidateTargetNodes: projectNodeIds(record?.candidate_target_nodes),
    admissibilityRejection: asStringOrNull(record?.admissibility_rejection),
  }));
}

function projectMergeRecords(records: any): MergeRecordView[] {
  if (!Array.isArray(records)) {
    return [];
  }
  return records.map((record) => ({
    sourceNode: nodeIdLabel(record?.source_node),
    targetNode: nodeIdLabelOrNull(record?.target_node),
    action: asString(record?.action),
    basis: asString(record?.basis),
    identityBasis: asStringOrNull(record?.identity_basis),
    identityStatus: asStringOrNull(record?.identity_status),
    identityCandidateCount: asNumber(record?.identity_candidate_count),
    resolvedConflictKinds: Array.isArray(record?.resolved_conflict_kinds)
      ? record.resolved_conflict_kinds.map((kind: unknown) => String(kind))
      : [],
  }));
}

function projectAspectPolicyRecords(records: any) {
  if (!Array.isArray(records)) {
    return [];
  }
  return records.map((record) => ({
    aspect: aspectLabel(record?.aspect),
    policyName: asString(record?.selected_policy_name),
    policyBasis: asString(record?.selected_policy_basis),
    affectedSourceNodes: projectNodeIds(record?.affected_source_nodes),
  }));
}

function projectAspectDecisionRecords(records: any) {
  if (!Array.isArray(records)) {
    return [];
  }
  return records.map((record) => ({
    aspect: aspectLabel(record?.aspect),
    sourceNode: nodeIdLabel(record?.source_node),
    targetNode: nodeIdLabelOrNull(record?.target_node),
    policyName: asString(record?.selected_policy_name),
    policyBasis: asString(record?.selected_policy_basis),
    outcome: asString(record?.outcome),
  }));
}

function projectConflictIsolationRecords(records: any) {
  if (!Array.isArray(records)) {
    return [];
  }
  return records.map((record) => ({
    sourceNode: nodeIdLabel(record?.source_node),
    targetNode: nodeIdLabelOrNull(record?.target_node),
    granularity: asString(record?.granularity),
    isolatedAspects: Array.isArray(record?.isolated_aspects)
      ? record.isolated_aspects.map((value: unknown) => aspectLabel(value))
      : [],
  }));
}

function projectNodeIds(values: any): string[] {
  if (!Array.isArray(values)) {
    return [];
  }
  return values.map((value) => nodeIdLabel(value));
}

function countEntries(value: any): number {
  return Array.isArray(value) ? value.length : 0;
}

function nodeIdLabel(value: any): string {
  if (value && typeof value === "object" && "index" in value && "generation" in value) {
    return `${String((value as { index: unknown }).index)}:${String((value as { generation: unknown }).generation)}`;
  }
  return String(value ?? "");
}

function nodeIdLabelOrNull(value: any): string | null {
  return value == null ? null : nodeIdLabel(value);
}

function aspectLabel(value: any): string {
  if (typeof value === "number") {
    return `aspect-${value}`;
  }
  if (value && typeof value === "object" && "0" in value) {
    return `aspect-${String((value as { 0: unknown })[0])}`;
  }
  return String(value ?? "");
}

function asString(value: unknown): string {
  return value == null ? "" : String(value);
}

function asStringOrNull(value: unknown): string | null {
  return value == null ? null : String(value);
}

function asNumber(value: unknown): number {
  if (typeof value === "number") {
    return value;
  }
  if (typeof value === "bigint") {
    return Number(value);
  }
  if (typeof value === "string" && value.length > 0) {
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : 0;
  }
  return 0;
}

function asNumberOrNull(value: unknown): number | null {
  if (value == null) {
    return null;
  }
  return asNumber(value);
}

function defineSceneSources(runtime: SignalRuntime, scene: SceneState) {
  runtime.defineSource(define.source<number>("cameraX").initial(scene.camera.x));
  runtime.defineSource(define.source<number>("cameraY").initial(scene.camera.y));
  runtime.defineSource(define.source<number>("cameraZ").initial(scene.camera.z));
  runtime.defineSource(define.source<number>("cameraYaw").initial(scene.camera.yaw));
  runtime.defineSource(define.source<number>("cameraPitch").initial(scene.camera.pitch));

  runtime.defineSource(define.source<number>("lightX").initial(scene.light.x));
  runtime.defineSource(define.source<number>("lightY").initial(scene.light.y));
  runtime.defineSource(define.source<number>("lightZ").initial(scene.light.z));
  runtime.defineSource(define.source<number>("lightIntensity").initial(scene.light.intensity));

  runtime.defineSource(define.source<number>("gearTeeth").initial(scene.gear.teeth));
  runtime.defineSource(define.source<number>("gearOuterRadius").initial(scene.gear.outerRadius));
  runtime.defineSource(define.source<number>("gearInnerRadius").initial(scene.gear.innerRadius));
  runtime.defineSource(define.source<number>("gearThickness").initial(scene.gear.thickness));
  runtime.defineSource(define.source<number>("gearRotation").initial(scene.gear.rotation));
  runtime.defineSourceFamily(define.sourceFamily<number>("gearToothIndex").initial(0));
  seedGearToothSources(runtime, scene.gear.teeth);
}

function defineHudSources(runtime: SignalRuntime) {
  runtime.defineSource(define.source<number>("hudFrameIndex").initial(0));
  runtime.defineSource(define.source<number>("hudRaysMarched").initial(0));
  runtime.defineSource(define.source<number>("hudAverageSteps").initial(0));
  runtime.defineSource(define.source<number>("hudHits").initial(0));
  runtime.defineSource(define.source<number>("hudMisses").initial(0));
  runtime.defineSource(define.source<number>("hudRenderMs").initial(0));
  runtime.defineSource(define.source<number>("hudTouchedNodes").initial(0));
  runtime.defineSource(define.source<number>("hudNodesEvaluated").initial(0));
  runtime.defineSource(define.source<number>("hudNodesSuppressed").initial(0));
  runtime.defineSource(define.source<number>("hudTotalNanos").initial(0));
}

function defineHudRecipe(runtime: SignalRuntime) {
  runtime.defineRecipe(
    define
      .recipe<HudModel>("hudModel")
      .reads(
        ...CAMERA_SOURCE_IDS,
        ...LIGHT_SOURCE_IDS,
        ...ASPECT_RECIPE_IDS,
        ...HUD_SOURCE_IDS,
      )
      .expr(
        expr.object<HudModel>({
          frameIndex: expr.read("hudFrameIndex"),
          raysMarched: expr.read("hudRaysMarched"),
          averageSteps: expr.read("hudAverageSteps"),
          hits: expr.read("hudHits"),
          misses: expr.read("hudMisses"),
          renderMs: expr.read("hudRenderMs"),
          touchedNodes: expr.read("hudTouchedNodes"),
          nodesEvaluated: expr.read("hudNodesEvaluated"),
          nodesSuppressed: expr.read("hudNodesSuppressed"),
          totalNanos: expr.read("hudTotalNanos"),
          cameraX: expr.read("cameraX"),
          cameraY: expr.read("cameraY"),
          cameraZ: expr.read("cameraZ"),
          lightX: expr.read("lightX"),
          lightY: expr.read("lightY"),
          lightZ: expr.read("lightZ"),
        }),
      )
      .identityExact(),
  );
}

function defineAspectRecipes(runtime: SignalRuntime) {
  const dims = expr.read<GearDimensionsModel>("gearDimensionsModel");
  const profile = expr.read<GearProfileModel>("gearProfileModel");
  const topology = expr.read<GearTopologyModel>("gearTopologyModel");
  const mesh = expr.read<GearMeshModel>("gearMeshModel");
  const lighting = expr.read<LightingModel>("lightingModel");
  const projection = expr.read<ViewportProjectionModel>("viewportProjectionModel");

  runtime.defineRecipe(
    define
      .recipe<GearDimensionsModel>("gearDimensionsModel")
      .reads("gearTeeth", "gearOuterRadius", "gearInnerRadius", "gearThickness", "gearRotation")
      .expr(
        expr.object<GearDimensionsModel>({
          teeth: expr.read("gearTeeth"),
          outerRadius: expr.read("gearOuterRadius"),
          innerRadius: expr.read("gearInnerRadius"),
          thickness: expr.read("gearThickness"),
          rotation: expr.read("gearRotation"),
          rimWidth: expr.subtract(expr.read("gearOuterRadius"), expr.read("gearInnerRadius")),
          boreRatio: expr.divide(expr.read("gearInnerRadius"), expr.read("gearOuterRadius")),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipe(
    define
      .recipe<GearProfileModel>("gearProfileModel")
      .reads("gearDimensionsModel")
      .expr(
        expr.object<GearProfileModel>({
          toothStep: expr.divide(expr.value(Math.PI * 2), expr.get(dims, "teeth")),
          rootRadius: expr.max(expr.sum(expr.get(dims, "innerRadius"), expr.value(0.2)), expr.subtract(expr.get(dims, "outerRadius"), expr.value(0.18))),
          tipRadius: expr.get(dims, "outerRadius"),
          shoulderRadius: expr.subtract(expr.get(dims, "outerRadius"), expr.multiply(expr.subtract(expr.get(dims, "outerRadius"), expr.max(expr.sum(expr.get(dims, "innerRadius"), expr.value(0.2)), expr.subtract(expr.get(dims, "outerRadius"), expr.value(0.18)))), expr.value(0.45))),
          toothDepth: expr.clamp(expr.multiply(expr.subtract(expr.get(dims, "outerRadius"), expr.get(dims, "innerRadius")), expr.value(0.34)), expr.value(0.08), expr.value(0.18)),
          profilePointCount: expr.multiply(expr.get(dims, "teeth"), expr.value(6)),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipe(
    define
      .recipe<GearTopologyModel>("gearTopologyModel")
      .reads("gearDimensionsModel", "gearProfileModel")
      .expr(
        expr.object<GearTopologyModel>({
          toothCount: expr.get(dims, "teeth"),
          ringSegments: expr.max(expr.multiply(expr.get(dims, "teeth"), expr.value(4)), expr.value(64)),
          silhouetteBands: expr.max(expr.floor(expr.multiply(expr.get(dims, "thickness"), expr.value(12))), expr.value(3)),
          profilePointCount: expr.get(profile, "profilePointCount"),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipe(
    define
      .recipe<GearMeshModel>("gearMeshModel")
      .reads("gearProfileModel", "gearTopologyModel")
      .expr(
        expr.object<GearMeshModel>({
          topFaceTriangles: expr.multiply(expr.get(profile, "profilePointCount"), expr.value(2)),
          sideTriangles: expr.multiply(expr.get(profile, "profilePointCount"), expr.value(2)),
          boreTriangles: expr.multiply(expr.get(topology, "ringSegments"), expr.value(2)),
          triangleCount: expr.sum(expr.multiply(expr.get(profile, "profilePointCount"), expr.value(4)), expr.multiply(expr.get(topology, "ringSegments"), expr.value(2))),
          outerRingCount: expr.get(profile, "profilePointCount"),
          innerRingCount: expr.get(topology, "ringSegments"),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipeFamily(
    define
      .recipeFamily<GearToothModel>("gearToothModel")
      .reads("gearDimensionsModel", "gearProfileModel", keyed.read("gearToothIndex"))
      .expr(
        expr.object<GearToothModel>({
          index: expr.read("gearToothIndex"),
          startAngle: expr.divide(
            expr.multiply(expr.read("gearToothIndex"), expr.value(Math.PI * 2)),
            expr.get(dims, "teeth"),
          ),
          midAngle: expr.divide(
            expr.multiply(expr.sum(expr.read("gearToothIndex"), expr.value(0.5)), expr.value(Math.PI * 2)),
            expr.get(dims, "teeth"),
          ),
          endAngle: expr.divide(
            expr.multiply(expr.sum(expr.read("gearToothIndex"), expr.value(1)), expr.value(Math.PI * 2)),
            expr.get(dims, "teeth"),
          ),
          rootRadius: expr.get(profile, "rootRadius"),
          tipRadius: expr.get(profile, "tipRadius"),
          thickness: expr.get(dims, "thickness"),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipe(
    define
      .recipe<LightingModel>("lightingModel")
      .reads("lightX", "lightY", "lightZ", "lightIntensity")
      .expr(
        expr.object<LightingModel>({
          x: expr.read("lightX"),
          y: expr.read("lightY"),
          z: expr.read("lightZ"),
          intensity: expr.read("lightIntensity"),
          falloff: expr.divide(expr.value(1), expr.sum(expr.value(1), expr.read("lightIntensity"))),
          highlightBoost: expr.sum(expr.value(0.5), expr.multiply(expr.read("lightIntensity"), expr.value(0.18))),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipe(
    define
      .recipe<ViewportProjectionModel>("viewportProjectionModel")
      .reads("cameraX", "cameraY", "cameraZ", "cameraPitch", "gearMeshModel")
      .expr(
        expr.object<ViewportProjectionModel>({
          focalLength: expr.sum(expr.value(480), expr.multiply(expr.abs(expr.read("cameraZ")), expr.value(8))),
          cameraDistance: expr.sqrt(expr.sum(expr.multiply(expr.read("cameraX"), expr.read("cameraX")), expr.multiply(expr.read("cameraY"), expr.read("cameraY")), expr.multiply(expr.read("cameraZ"), expr.read("cameraZ")))),
          centerLift: expr.multiply(expr.read("cameraPitch"), expr.value(0.28)),
          perspectiveScale: expr.divide(expr.get(mesh, "triangleCount"), expr.value(100)),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipe(
    define
      .recipe<ViewportShadingModel>("viewportShadingModel")
      .reads("lightingModel", "gearDimensionsModel", "viewportProjectionModel")
      .expr(
        expr.object<ViewportShadingModel>({
          ambient: expr.sum(expr.value(0.24), expr.multiply(expr.get(lighting, "falloff"), expr.value(0.16))),
          diffuseBoost: expr.sum(expr.value(0.46), expr.multiply(expr.get(lighting, "intensity"), expr.value(0.22))),
          edgeContrast: expr.clamp(expr.sum(expr.value(0.9), expr.multiply(expr.get(dims, "rimWidth"), expr.value(0.8))), expr.value(0.8), expr.value(1.8)),
          shadowOpacity: expr.clamp(expr.sum(expr.value(0.12), expr.multiply(expr.get(lighting, "intensity"), expr.value(0.08))), expr.value(0.1), expr.value(0.34)),
          floorGridOpacity: expr.clamp(expr.sum(expr.value(0.06), expr.multiply(expr.get(projection, "perspectiveScale"), expr.value(0.008))), expr.value(0.05), expr.value(0.18)),
          specularPower: expr.clamp(expr.sum(expr.value(10), expr.multiply(expr.get(lighting, "highlightBoost"), expr.value(4))), expr.value(8), expr.value(24)),
        }),
      )
      .identityExact(),
  );
}

async function renderCurrentBranch(
  runtime: SignalRuntime,
  state: SceneState,
  scenePatchOps: Array<{ kind: "set"; id: string; value: number }>,
  progress?: RuntimeProgress,
): Promise<RenderUpdate> {
  const history = runtime.history();
  const branch = history.currentBranch();
  const frameIndex = (frameCounters.get(runtime) ?? 0) + 1;
  frameCounters.set(runtime, frameIndex);
  const previousAspects = readRenderAspects(runtime);
  let sceneSummary = {
    touchedNodes: 0,
    nodesEvaluated: 0,
    nodesRecomputed: 0,
    nodesSuppressed: 0,
    plansBuilt: 0,
    stagesExecuted: 0,
    totalNanos: "0",
    evaluationNanos: "0",
    commitNanos: "0",
  };
  if (scenePatchOps.length > 0) {
    progress?.("render:tx-scene-start", "committing scene aspects");
    sceneSummary = runtime.transaction(scenePatchOps);
    progress?.("render:tx-scene-done", `evaluated ${sceneSummary.nodesEvaluated} nodes`);
  }
  progress?.("render:aspects:start", "reading render aspects");
  const aspects = readRenderAspects(runtime);
  progress?.("render:aspects:done", "render aspects ready");
  progress?.("render:js-start", `rendering frame ${frameIndex}`);
  const rendered = renderScene(state, aspects);
  progress?.("render:js-done", `js render ${rendered.stats.renderMs.toFixed(2)} ms`);
  rendered.stats.frameIndex = frameIndex;

  const graphSummary = summarizeAspectGraph(scenePatchOps, previousAspects, aspects, sceneSummary);

  const hud: HudModel = {
    frameIndex,
    raysMarched: rendered.stats.raysMarched,
    averageSteps: rendered.stats.averageSteps,
    hits: rendered.stats.hits,
    misses: rendered.stats.misses,
    renderMs: rendered.stats.renderMs,
    touchedNodes: graphSummary.touchedNodes,
    nodesEvaluated: graphSummary.nodesEvaluated,
    nodesSuppressed: graphSummary.nodesSuppressed,
    totalNanos: Number(sceneSummary.totalNanos) || 0,
    cameraX: state.camera.x,
    cameraY: state.camera.y,
    cameraZ: state.camera.z,
    lightX: state.light.x,
    lightY: state.light.y,
    lightZ: state.light.z,
  };

  return {
    summary: graphSummary,
    branchId: branch.id,
    branchName: branch.name,
    state,
    hud,
    frame: rendered.frame,
    stats: rendered.stats,
  };
}

function buildScenePatchOps(patch: ScenePatch) {
  const ops: Array<{ kind: "set"; id: string; value: number }> = [];

  if (patch.camera) {
    if (patch.camera.x !== undefined) ops.push({ kind: "set", id: "cameraX", value: patch.camera.x });
    if (patch.camera.y !== undefined) ops.push({ kind: "set", id: "cameraY", value: patch.camera.y });
    if (patch.camera.z !== undefined) ops.push({ kind: "set", id: "cameraZ", value: patch.camera.z });
    if (patch.camera.yaw !== undefined) ops.push({ kind: "set", id: "cameraYaw", value: patch.camera.yaw });
    if (patch.camera.pitch !== undefined) ops.push({ kind: "set", id: "cameraPitch", value: patch.camera.pitch });
  }

  if (patch.light) {
    if (patch.light.x !== undefined) ops.push({ kind: "set", id: "lightX", value: patch.light.x });
    if (patch.light.y !== undefined) ops.push({ kind: "set", id: "lightY", value: patch.light.y });
    if (patch.light.z !== undefined) ops.push({ kind: "set", id: "lightZ", value: patch.light.z });
    if (patch.light.intensity !== undefined) {
      ops.push({ kind: "set", id: "lightIntensity", value: patch.light.intensity });
    }
  }

  if (patch.gear) {
    if (patch.gear.teeth !== undefined) ops.push({ kind: "set", id: "gearTeeth", value: patch.gear.teeth });
    if (patch.gear.outerRadius !== undefined) {
      ops.push({ kind: "set", id: "gearOuterRadius", value: patch.gear.outerRadius });
    }
    if (patch.gear.innerRadius !== undefined) {
      ops.push({ kind: "set", id: "gearInnerRadius", value: patch.gear.innerRadius });
    }
    if (patch.gear.thickness !== undefined) {
      ops.push({ kind: "set", id: "gearThickness", value: patch.gear.thickness });
    }
    if (patch.gear.rotation !== undefined) {
      ops.push({ kind: "set", id: "gearRotation", value: patch.gear.rotation });
    }
  }

  return ops;
}

function seedGearToothSources(runtime: SignalRuntime, teeth: number) {
  runtime.setKeyedMany(
    "gearToothIndex",
    Array.from({ length: Math.max(1, teeth) }, (_, index) => ({
      key: `tooth-${index}`,
      value: index,
    })),
  );
}

const STATIC_NODE_COUNT =
  CAMERA_SOURCE_IDS.length +
  LIGHT_SOURCE_IDS.length +
  5 +
  ASPECT_RECIPE_IDS.length +
  HUD_SOURCE_IDS.length +
  1;

function mergeSceneState(base: SceneState, patch: ScenePatch): SceneState {
  return {
    camera: {
      ...base.camera,
      ...patch.camera,
    },
    light: {
      ...base.light,
      ...patch.light,
    },
    gear: {
      ...base.gear,
      ...patch.gear,
    },
  };
}

function summarizeAspectGraph(
  scenePatchOps: Array<{ kind: "set"; id: string; value: number }>,
  previous: RenderAspects,
  next: RenderAspects,
  sceneSummary: Awaited<ReturnType<SignalRuntime["transaction"]>>,
) {
  const changedAspects = [
    !shallowNumberRecordEqual(previous.dimensions, next.dimensions),
    !shallowNumberRecordEqual(previous.profile, next.profile),
    !shallowNumberRecordEqual(previous.topology, next.topology),
    !shallowNumberRecordEqual(previous.mesh, next.mesh),
    !shallowNumberRecordEqual(previous.lighting, next.lighting),
    !shallowNumberRecordEqual(previous.projection, next.projection),
    !shallowNumberRecordEqual(previous.shading, next.shading),
  ].filter(Boolean).length;

  const sourceChanges = scenePatchOps.length;
  const hudNodes = 1;
  const touchedNodes = Math.min(sourceChanges + changedAspects + hudNodes, STATIC_NODE_COUNT);
  const nodesEvaluated = touchedNodes;
  const nodesSuppressed = Math.max(STATIC_NODE_COUNT - nodesEvaluated, 0);

  return {
    ...sceneSummary,
    touchedNodes,
    nodesEvaluated,
    nodesSuppressed,
  };
}

function shallowNumberRecordEqual(
  left: Record<string, number>,
  right: Record<string, number>,
): boolean {
  const leftKeys = Object.keys(left);
  const rightKeys = Object.keys(right);
  if (leftKeys.length !== rightKeys.length) {
    return false;
  }
  for (const key of leftKeys) {
    if (left[key] !== right[key]) {
      return false;
    }
  }
  return true;
}

export function exportRuntimeEnvelope(runtime: SignalRuntime): RuntimeEnvelope {
  return runtime.adapters().exportRuntimeEnvelope();
}

export function replaceRuntimeEnvelope(runtime: SignalRuntime, envelope: RuntimeEnvelope) {
  runtime.adapters().replaceRuntimeEnvelope(envelope);
}
