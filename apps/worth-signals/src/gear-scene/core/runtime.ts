import {
  createSignalRuntime,
  define,
  expr,
  keyed,
  policy,
  type BranchMergePlan,
  type BranchMergeResult,
  type BranchStateProofReport,
  type ChangedRegion,
  type MergePlanProofReport,
  type MergeResultProofReport,
  type MergePolicyPreviewRequest,
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
  RENDER_HEIGHT,
  RENDER_WIDTH,
  type RenderAspects,
  type RenderTileCoord,
  type RenderTileGridModel,
  type RenderTileEnvironmentLayerModel,
  type RenderTileUploadLayerModel,
  type RenderTileGeometryLayerModel,
  type RenderTileLightingLayerModel,
  type RenderTileModel,
  type RenderTileUploadRect,
  type RenderUpdate,
  type ScenarioProofArtifacts,
  type SceneOccupancySnapshot,
  type SceneRuntimeBundle,
  type SceneState,
  type ScreenBounds,
  type TileDetailOccupancy,
  type ViewportProjectionModel,
  type ViewportShadingModel,
} from "./types";

const DEFAULT_POLICY = policy.preset("webDevelopment");
const FULL_TILE_GRID_COLUMNS = 200;
const FULL_TILE_GRID_ROWS = 200;
const BOOT_TILE_GRID_COLUMNS = 200;
const BOOT_TILE_GRID_ROWS = 200;

const CAMERA_SOURCE_IDS = ["cameraX", "cameraY", "cameraZ", "cameraYaw", "cameraPitch"] as const;
const LIGHT_SOURCE_IDS = ["lightX", "lightY", "lightZ", "lightIntensity"] as const;
const TILE_SOURCE_IDS = ["renderTileColumns", "renderTileRows"] as const;
const ASPECT_RECIPE_IDS = [
  "gearDimensionsModel",
  "gearProfileModel",
  "gearTopologyModel",
  "gearMeshModel",
  "lightingModel",
  "viewportProjectionModel",
  "viewportShadingModel",
  "renderTileGridModel",
] as const;
const HUD_SOURCE_IDS = [
  "hudFrameIndex",
  "hudRaysMarched",
  "hudAverageSteps",
  "hudHits",
  "hudMisses",
  "hudRenderMs",
  "hudTileCount",
  "hudTileColumns",
  "hudTileRows",
  "hudTouchedNodes",
  "hudNodesEvaluated",
  "hudNodesSuppressed",
  "hudTotalNanos",
  "hudDirtyTiles",
  "hudUploadedTiles",
  "hudUploadSpans",
  "hudUploadBytes",
  "hudChangedDetails",
] as const;
const UPLOAD_LAYER_FIELDS = [
  "red",
  "green",
  "blue",
  "alpha",
] as const;

type RuntimeProgress = (phase: string, detail?: string) => void;
type CreateSceneRuntimeOptions = {
  renderInitial?: boolean;
  tileGrid?: { columns: number; rows: number };
};
type RenderGlobalAspectsModel = {
  dimensions: GearDimensionsModel;
  profile: GearProfileModel;
  topology: GearTopologyModel;
  mesh: GearMeshModel;
  lighting: LightingModel;
  projection: ViewportProjectionModel;
  shading: ViewportShadingModel;
  tileGrid: RenderTileGridModel;
};
const frameCounters = new WeakMap<SignalRuntime, number>();

export async function createSceneRuntime(
  progress?: RuntimeProgress,
  options?: CreateSceneRuntimeOptions,
): Promise<SceneRuntimeBundle> {
  const renderInitial = options?.renderInitial ?? true;
  const tileGrid = options?.tileGrid ?? {
    columns: BOOT_TILE_GRID_COLUMNS,
    rows: BOOT_TILE_GRID_ROWS,
  };
  progress?.("runtime:init", "creating wasm runtime");
  const runtime = await createSignalRuntime();
  progress?.("runtime:policy", "setting runtime policy");
  runtime.setRuntimePolicy(DEFAULT_POLICY);

  const scene = defaultSceneState();
  progress?.("runtime:sources", "defining scene sources");
  defineSceneSources(runtime, scene, tileGrid);
  progress?.("runtime:aspects", "defining aspect recipes");
  defineAspectRecipes(runtime);
  progress?.("runtime:hud-sources", "defining hud sources");
  defineHudSources(runtime, tileGrid);
  progress?.("runtime:hud-recipe", "defining hud recipe");
  defineHudRecipe(runtime);

  let initialRender: RenderUpdate | null = null;
  if (renderInitial) {
    progress?.("runtime:first-render", "rendering initial branch");
    const branch = runtime.history().currentBranch();
    const tileCount = tileGrid.columns * tileGrid.rows;
    if (tileCount > 2_400) {
      initialRender = renderInteractivePreview(branch.id, branch.name, scene);
    } else {
      initialRender = await renderBranch(runtime, branch.id, progress);
    }
  }
  progress?.("runtime:ready", "scene runtime ready");
  return { runtime, initialRender };
}

export function hasStressTileGrid(runtime: SignalRuntime): boolean {
  const { columns, rows } = currentTileGrid(runtime);
  return columns === FULL_TILE_GRID_COLUMNS && rows === FULL_TILE_GRID_ROWS;
}

export async function hydrateRuntimeToStressGrid(
  runtime: SignalRuntime,
  progress?: RuntimeProgress,
): Promise<RenderUpdate | null> {
  if (hasStressTileGrid(runtime)) {
    return null;
  }

  progress?.("runtime:hydrate-grid:start", "upgrading runtime to full stress grid");
  runtime.transaction([
    { kind: "set", id: "renderTileColumns", value: FULL_TILE_GRID_COLUMNS },
    { kind: "set", id: "renderTileRows", value: FULL_TILE_GRID_ROWS },
    { kind: "set", id: "hudTileCount", value: FULL_TILE_GRID_COLUMNS * FULL_TILE_GRID_ROWS },
    { kind: "set", id: "hudTileColumns", value: FULL_TILE_GRID_COLUMNS },
    { kind: "set", id: "hudTileRows", value: FULL_TILE_GRID_ROWS },
  ]);
  runtime.clearKeyedFamilyCache("renderTileCoord");
  seedRenderTileSources(runtime, FULL_TILE_GRID_COLUMNS, FULL_TILE_GRID_ROWS);
  branchOccupancyStore(runtime).clear();
  progress?.("runtime:hydrate-grid:seeded", "full stress tile sources registered");
  const prewarmStartedAt = performance.now();
  runtime.prewarmKeyedGrid(
    "renderTileUploadLayerModel",
    FULL_TILE_GRID_COLUMNS,
    FULL_TILE_GRID_ROWS,
  );
  progress?.(
    "runtime:hydrate-grid:prewarmed",
    `upload layer prewarmed in ${(performance.now() - prewarmStartedAt).toFixed(2)} ms`,
  );

  const update = await renderBranch(runtime, runtime.history().currentBranch().id, progress);
  progress?.("runtime:hydrate-grid:ready", "full stress grid active");
  return update;
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
  const { columns, rows } = currentTileGrid(runtime);
  return STATIC_NODE_COUNT + columns * rows * 2;
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
    tileCount: currentTileGrid(runtime).columns * currentTileGrid(runtime).rows,
    tileColumns: currentTileGrid(runtime).columns,
    tileRows: currentTileGrid(runtime).rows,
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
    dirtyTiles: 0,
    uploadedTiles: 0,
    uploadSpans: 0,
    uploadBytes: 0,
    changedDetails: 0,
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
  progress?.("read:scene:start", "reading scene sources");
  const scalarStartedAt = performance.now();
  const state = runtime.read<SceneState>("sceneStateModel");
  progress?.(
    "read:scene:values",
    `scene scalar reads ready in ${(performance.now() - scalarStartedAt).toFixed(2)} ms`,
  );
  progress?.("read:scene:done", "scene sources ready");
  return state;
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
  const tileGrid = currentTileGrid(runtime);
  const nextState = mergeSceneState(currentState, patch);
  const previousOccupancy = getBranchOccupancySnapshot(runtime, branchId) ?? buildSceneOccupancySnapshot(currentState, tileGrid.columns, tileGrid.rows);
  const nextOccupancy = buildSceneOccupancySnapshot(nextState, tileGrid.columns, tileGrid.rows);
  const result = await renderCurrentBranch(
    runtime,
    nextState,
    buildScenePatchOps(currentState, nextState, patch, previousOccupancy, nextOccupancy, tileGrid),
    progress,
    nextOccupancy,
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

export function renderInteractivePreview(
  branchId: BranchId,
  branchName: string,
  state: SceneState,
  previousHud?: HudModel | null,
): RenderUpdate {
  const frameIndex = (previousHud?.frameIndex ?? 0) + 1;
  const aspects = derivePreviewRenderAspects(state, BOOT_TILE_GRID_COLUMNS, BOOT_TILE_GRID_ROWS);
  const rendered = renderScene(state, aspects);
  rendered.stats.frameIndex = frameIndex;
  const hud: HudModel = {
    frameIndex,
    raysMarched: rendered.stats.raysMarched,
    averageSteps: rendered.stats.averageSteps,
    hits: rendered.stats.hits,
    misses: rendered.stats.misses,
    renderMs: rendered.stats.renderMs,
    tileCount: rendered.stats.tileCount,
    tileColumns: rendered.stats.tileColumns,
    tileRows: rendered.stats.tileRows,
    touchedNodes: previousHud?.touchedNodes ?? 0,
    nodesEvaluated: previousHud?.nodesEvaluated ?? 0,
    nodesSuppressed: previousHud?.nodesSuppressed ?? 0,
    totalNanos: previousHud?.totalNanos ?? 0,
    cameraX: state.camera.x,
    cameraY: state.camera.y,
    cameraZ: state.camera.z,
    lightX: state.light.x,
    lightY: state.light.y,
    lightZ: state.light.z,
    dirtyTiles: rendered.stats.dirtyTiles,
    uploadedTiles: rendered.stats.uploadedTiles,
    uploadSpans: rendered.stats.uploadSpans,
    uploadBytes: rendered.stats.uploadBytes,
    changedDetails: rendered.stats.changedDetails,
  };

  return {
    summary: {
      touchedNodes: previousHud?.touchedNodes ?? 0,
      nodesEvaluated: previousHud?.nodesEvaluated ?? 0,
      nodesRecomputed: 0,
      nodesSuppressed: previousHud?.nodesSuppressed ?? 0,
      plansBuilt: 0,
      stagesExecuted: 0,
      totalNanos: String(previousHud?.totalNanos ?? 0),
      evaluationNanos: "0",
      commitNanos: "0",
    },
    branchId,
    branchName,
    state,
    hud,
    frame: rendered.frame,
    stats: rendered.stats,
  };
}

export function readRenderAspects(
  runtime: SignalRuntime,
  dirtyTileIndices?: number[],
  progress?: RuntimeProgress,
): RenderAspects {
  const globalsStartedAt = performance.now();
  const globals = runtime.read<RenderGlobalAspectsModel>("renderGlobalAspectsModel");
  progress?.(
    "render:aspects:globals",
    `global aspect reads ready in ${(performance.now() - globalsStartedAt).toFixed(2)} ms`,
  );
  const {
    dimensions,
    profile,
    topology,
    mesh,
    lighting,
    projection,
    shading,
    tileGrid,
  } = globals;
  const fullComposeUpload = dirtyTileIndices == null;
  const nextDirtyTileIndices =
    dirtyTileIndices ?? Array.from({ length: tileGrid.tileCount }, (_, index) => index);
  const dirtyTileRects = fullComposeUpload
    ? [{ row: 0, startColumn: 0, width: tileGrid.columns, height: tileGrid.rows }]
    : coalesceDirtyTileRectangles(nextDirtyTileIndices, tileGrid.columns);
  const tileReadStartedAt = performance.now();
  const tileUploadBuffer = readPackedUploadBuffer(
    runtime,
    tileGrid.columns,
    tileGrid.rows,
    dirtyTileRects,
    fullComposeUpload,
  );
  progress?.(
    "render:aspects:tiles",
    `tile upload reads ready in ${(performance.now() - tileReadStartedAt).toFixed(2)} ms (${fullComposeUpload ? tileGrid.tileCount : nextDirtyTileIndices.length} tiles)`,
  );

  const assembleStartedAt = performance.now();
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
  progress?.(
    "render:aspects:assemble",
    `aspect assembly ready in ${(performance.now() - assembleStartedAt).toFixed(2)} ms`,
  );

  return {
    dimensions,
    profile,
    topology,
    mesh,
    teeth,
    lighting,
    projection,
    shading,
    tileGrid,
    tileUploadBuffer,
    fullComposeUpload,
    dirtyTileIndices: nextDirtyTileIndices,
    dirtyTileRects,
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

export function planMergePolicyPreview(
  runtime: SignalRuntime,
  request: MergePolicyPreviewRequest,
): MergePlan {
  const envelope = runtime.history().planMergePolicyPreviewDetailedWithProof(request);
  return projectMergePlan(envelope?.plan, envelope?.proof);
}

export function executeMergePolicyPreview(
  runtime: SignalRuntime,
  request: MergePolicyPreviewRequest,
): MergeResult {
  const envelope = runtime.history().mergeBranchesPolicyPreviewDetailedWithProof(request);
  return projectMergeResult(envelope?.result, envelope?.proof);
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

function defineSceneSources(
  runtime: SignalRuntime,
  scene: SceneState,
  tileGrid: { columns: number; rows: number },
) {
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
  runtime.defineSource(define.source<number>("renderTileColumns").initial(tileGrid.columns));
  runtime.defineSource(define.source<number>("renderTileRows").initial(tileGrid.rows));
  runtime.defineSourceFamily(define.sourceFamily<number>("gearToothIndex").initial(0));
  runtime.defineSourceFamily(define.sourceFamily<RenderTileCoord>("renderTileCoord").initial({ column: 0, row: 0 }));
  seedGearToothSources(runtime, scene.gear.teeth);
  seedRenderTileSources(runtime, tileGrid.columns, tileGrid.rows);
}

function defineHudSources(runtime: SignalRuntime, tileGrid: { columns: number; rows: number }) {
  runtime.defineSource(define.source<number>("hudFrameIndex").initial(0));
  runtime.defineSource(define.source<number>("hudRaysMarched").initial(0));
  runtime.defineSource(define.source<number>("hudAverageSteps").initial(0));
  runtime.defineSource(define.source<number>("hudHits").initial(0));
  runtime.defineSource(define.source<number>("hudMisses").initial(0));
  runtime.defineSource(define.source<number>("hudRenderMs").initial(0));
  runtime.defineSource(define.source<number>("hudTileCount").initial(tileGrid.columns * tileGrid.rows));
  runtime.defineSource(define.source<number>("hudTileColumns").initial(tileGrid.columns));
  runtime.defineSource(define.source<number>("hudTileRows").initial(tileGrid.rows));
  runtime.defineSource(define.source<number>("hudTouchedNodes").initial(0));
  runtime.defineSource(define.source<number>("hudNodesEvaluated").initial(0));
  runtime.defineSource(define.source<number>("hudNodesSuppressed").initial(0));
  runtime.defineSource(define.source<number>("hudTotalNanos").initial(0));
  runtime.defineSource(define.source<number>("hudDirtyTiles").initial(0));
  runtime.defineSource(define.source<number>("hudUploadedTiles").initial(0));
  runtime.defineSource(define.source<number>("hudUploadSpans").initial(0));
  runtime.defineSource(define.source<number>("hudUploadBytes").initial(0));
  runtime.defineSource(define.source<number>("hudChangedDetails").initial(0));
}

function defineHudRecipe(runtime: SignalRuntime) {
  runtime.defineRecipe(
    define
      .recipe<HudModel>("hudModel")
      .reads(
        ...CAMERA_SOURCE_IDS,
        ...LIGHT_SOURCE_IDS,
        ...TILE_SOURCE_IDS,
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
          tileCount: expr.read("hudTileCount"),
          tileColumns: expr.read("hudTileColumns"),
          tileRows: expr.read("hudTileRows"),
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
          dirtyTiles: expr.read("hudDirtyTiles"),
          uploadedTiles: expr.read("hudUploadedTiles"),
          uploadSpans: expr.read("hudUploadSpans"),
          uploadBytes: expr.read("hudUploadBytes"),
          changedDetails: expr.read("hudChangedDetails"),
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
  const shading = expr.read<ViewportShadingModel>("viewportShadingModel");

  runtime.defineRecipe(
    define
      .recipe<SceneState>("sceneStateModel")
      .reads(
        "cameraX",
        "cameraY",
        "cameraZ",
        "cameraYaw",
        "cameraPitch",
        "lightX",
        "lightY",
        "lightZ",
        "lightIntensity",
        "gearTeeth",
        "gearOuterRadius",
        "gearInnerRadius",
        "gearThickness",
        "gearRotation",
      )
      .expr(
        expr.object<SceneState>({
          camera: expr.object({
            x: expr.read("cameraX"),
            y: expr.read("cameraY"),
            z: expr.read("cameraZ"),
            yaw: expr.read("cameraYaw"),
            pitch: expr.read("cameraPitch"),
          }),
          light: expr.object({
            x: expr.read("lightX"),
            y: expr.read("lightY"),
            z: expr.read("lightZ"),
            intensity: expr.read("lightIntensity"),
          }),
          gear: expr.object({
            teeth: expr.read("gearTeeth"),
            outerRadius: expr.read("gearOuterRadius"),
            innerRadius: expr.read("gearInnerRadius"),
            thickness: expr.read("gearThickness"),
            rotation: expr.read("gearRotation"),
          }),
        }),
      )
      .identityExact(),
  );

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
      .reads("lightIntensity", "gearDimensionsModel", "viewportProjectionModel")
      .expr(
        expr.object<ViewportShadingModel>({
          ambient: expr.sum(
            expr.value(0.24),
            expr.multiply(
              expr.divide(expr.value(1), expr.sum(expr.value(1), expr.read("lightIntensity"))),
              expr.value(0.16),
            ),
          ),
          diffuseBoost: expr.sum(expr.value(0.46), expr.multiply(expr.read("lightIntensity"), expr.value(0.22))),
          edgeContrast: expr.clamp(expr.sum(expr.value(0.9), expr.multiply(expr.get(dims, "rimWidth"), expr.value(0.8))), expr.value(0.8), expr.value(1.8)),
          shadowOpacity: expr.clamp(expr.sum(expr.value(0.12), expr.multiply(expr.read("lightIntensity"), expr.value(0.08))), expr.value(0.1), expr.value(0.34)),
          floorGridOpacity: expr.clamp(expr.sum(expr.value(0.06), expr.multiply(expr.get(projection, "perspectiveScale"), expr.value(0.008))), expr.value(0.05), expr.value(0.18)),
          specularPower: expr.clamp(
            expr.sum(
              expr.value(10),
              expr.multiply(
                expr.sum(expr.value(0.5), expr.multiply(expr.read("lightIntensity"), expr.value(0.18))),
                expr.value(4),
              ),
            ),
            expr.value(8),
            expr.value(24),
          ),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipe(
    define
      .recipe<RenderTileGridModel>("renderTileGridModel")
      .reads("renderTileColumns", "renderTileRows")
      .expr(
        expr.object<RenderTileGridModel>({
          columns: expr.read("renderTileColumns"),
          rows: expr.read("renderTileRows"),
          tileCount: expr.multiply(expr.read("renderTileColumns"), expr.read("renderTileRows")),
          tileWidth: expr.divide(expr.value(640), expr.read("renderTileColumns")),
          tileHeight: expr.divide(expr.value(360), expr.read("renderTileRows")),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipe(
    define
      .recipe<RenderGlobalAspectsModel>("renderGlobalAspectsModel")
      .reads(
        "gearDimensionsModel",
        "gearProfileModel",
        "gearTopologyModel",
        "gearMeshModel",
        "lightingModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "renderTileGridModel",
      )
      .expr(
        expr.object<RenderGlobalAspectsModel>({
          dimensions: dims,
          profile,
          topology,
          mesh,
          lighting,
          projection,
          shading,
          tileGrid: expr.read<RenderTileGridModel>("renderTileGridModel"),
        }),
      )
      .identityExact(),
  );

  const tileGrid = expr.read<RenderTileGridModel>("renderTileGridModel");
  const tileCoord = expr.read<RenderTileCoord>("renderTileCoord");
  runtime.defineRecipeFamily(
    define
      .recipeFamily<RenderTileModel>("renderTileModel")
      .reads(
        "renderTileGridModel",
        "gearOuterRadius",
        "lightIntensity",
        "lightIntensity",
        "lightZ",
        "cameraZ",
        "cameraPitch",
        keyed.read("renderTileCoord"),
      )
      .expr(
        expr.object<RenderTileModel>({
          column: expr.get(tileCoord, "column"),
          row: expr.get(tileCoord, "row"),
          left: expr.multiply(expr.get(tileCoord, "column"), expr.get(tileGrid, "tileWidth")),
          top: expr.multiply(expr.get(tileCoord, "row"), expr.get(tileGrid, "tileHeight")),
          width: expr.get(tileGrid, "tileWidth"),
          height: expr.get(tileGrid, "tileHeight"),
          centerX: expr.divide(
            expr.sum(
              expr.multiply(expr.get(tileCoord, "column"), expr.get(tileGrid, "tileWidth")),
              expr.multiply(expr.get(tileGrid, "tileWidth"), expr.value(0.5)),
            ),
            expr.value(640),
          ),
          centerY: expr.divide(
            expr.sum(
              expr.multiply(expr.get(tileCoord, "row"), expr.get(tileGrid, "tileHeight")),
              expr.multiply(expr.get(tileGrid, "tileHeight"), expr.value(0.5)),
            ),
            expr.value(360),
          ),
          radialWeight: expr.clamp(
            expr.subtract(
              expr.value(1.18),
              expr.sum(
                expr.abs(expr.subtract(
                  expr.divide(
                    expr.sum(
                      expr.multiply(expr.get(tileCoord, "column"), expr.get(tileGrid, "tileWidth")),
                      expr.multiply(expr.get(tileGrid, "tileWidth"), expr.value(0.5)),
                    ),
                    expr.value(640),
                  ),
                  expr.value(0.5),
                )),
                expr.abs(expr.subtract(
                  expr.divide(
                    expr.sum(
                      expr.multiply(expr.get(tileCoord, "row"), expr.get(tileGrid, "tileHeight")),
                      expr.multiply(expr.get(tileGrid, "tileHeight"), expr.value(0.5)),
                    ),
                    expr.value(360),
                  ),
                  expr.value(0.54),
                )),
              ),
            ),
            expr.value(0),
            expr.value(1),
          ),
          lightWeight: expr.clamp(
            expr.divide(
              expr.read("lightIntensity"),
              expr.sum(expr.value(1), expr.abs(expr.read("lightZ"))),
            ),
            expr.value(0),
            expr.value(1.5),
          ),
          gearWeight: expr.clamp(
            expr.sum(
              expr.multiply(expr.read("gearOuterRadius"), expr.value(0.48)),
              expr.multiply(expr.read("lightIntensity"), expr.value(0.12)),
              expr.multiply(
                expr.divide(expr.value(1), expr.sum(expr.value(1), expr.abs(expr.read("cameraZ")))),
                expr.value(0.22),
              ),
              expr.multiply(expr.abs(expr.read("cameraPitch")), expr.value(0.06)),
            ),
            expr.value(0),
            expr.value(2),
          ),
        }),
      )
      .identityExact(),
  );
  runtime.defineRecipeFamily(
    define
      .recipeFamily<RenderTileGeometryLayerModel>("renderTileGeometryLayerModel")
      .reads(
        "renderTileGridModel",
        "gearTeeth",
        "gearOuterRadius",
        "gearInnerRadius",
        "lightIntensity",
        "cameraZ",
        keyed.read("renderTileCoord"),
      )
      .expr(
        expr.object<RenderTileGeometryLayerModel>({
          bodyFace: expr.clamp(
            expr.sum(
              expr.multiply(
                expr.clamp(
                  expr.subtract(
                    expr.value(1),
                    expr.multiply(
                      expr.abs(expr.subtract(
                        expr.divide(
                          expr.sum(
                            expr.multiply(expr.get(tileCoord, "row"), expr.get(tileGrid, "tileHeight")),
                            expr.multiply(expr.get(tileGrid, "tileHeight"), expr.value(0.5)),
                          ),
                          expr.value(360),
                        ),
                        expr.value(0.54),
                      )),
                      expr.value(2.8),
                    ),
                  ),
                  expr.value(0),
                  expr.value(1),
                ),
                expr.value(0.62),
              ),
              expr.multiply(expr.read("gearOuterRadius"), expr.value(0.22)),
              expr.multiply(expr.read("lightIntensity"), expr.value(0.08)),
            ),
            expr.value(0),
            expr.value(1),
          ),
          toothBand: expr.clamp(
            expr.sum(
              expr.multiply(expr.read("gearTeeth"), expr.value(0.02)),
              expr.multiply(
                expr.divide(expr.value(1), expr.sum(expr.value(1), expr.abs(expr.read("cameraZ")))),
                expr.value(0.28),
              ),
              expr.multiply(expr.read("gearOuterRadius"), expr.value(0.12)),
            ),
            expr.value(0),
            expr.value(1),
          ),
          bore: expr.clamp(
            expr.sum(
              expr.multiply(
                expr.divide(expr.read("gearInnerRadius"), expr.max(expr.read("gearOuterRadius"), expr.value(0.01))),
                expr.value(0.9),
              ),
              expr.multiply(expr.value(1), expr.value(0.08)),
            ),
            expr.value(0),
            expr.value(1),
          ),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipeFamily(
    define
      .recipeFamily<RenderTileLightingLayerModel>("renderTileLightingLayerModel")
      .reads(
        keyed.read("renderTileCoord"),
        "renderTileGridModel",
        "gearOuterRadius",
      )
      .expr(
        expr.object<RenderTileLightingLayerModel>({
          shadow: expr.clamp(
            expr.sum(
              expr.multiply(
                expr.clamp(
                  expr.divide(
                    expr.sum(
                      expr.multiply(expr.get(tileCoord, "row"), expr.get(tileGrid, "tileHeight")),
                      expr.multiply(expr.get(tileGrid, "tileHeight"), expr.value(0.5)),
                    ),
                    expr.value(RENDER_HEIGHT),
                  ),
                  expr.value(0),
                  expr.value(1),
                ),
                expr.value(0.38),
              ),
              expr.multiply(
                expr.clamp(
                  expr.divide(
                    expr.sum(
                      expr.multiply(expr.get(tileCoord, "column"), expr.get(tileGrid, "tileWidth")),
                      expr.multiply(expr.get(tileGrid, "tileWidth"), expr.value(0.5)),
                    ),
                    expr.value(RENDER_WIDTH),
                  ),
                  expr.value(0),
                  expr.value(1),
                ),
                expr.value(0.08),
              ),
            ),
            expr.value(0),
            expr.value(1),
          ),
          specular: expr.clamp(
            expr.sum(
              expr.multiply(
                expr.clamp(
                  expr.subtract(
                    expr.value(1),
                    expr.abs(
                      expr.subtract(
                        expr.divide(
                          expr.sum(
                            expr.multiply(expr.get(tileCoord, "column"), expr.get(tileGrid, "tileWidth")),
                            expr.multiply(expr.get(tileGrid, "tileWidth"), expr.value(0.5)),
                          ),
                          expr.value(RENDER_WIDTH),
                        ),
                        expr.value(0.44),
                      ),
                    ),
                  ),
                  expr.value(0),
                  expr.value(1),
                ),
                expr.value(0.32),
              ),
              expr.multiply(
                expr.clamp(
                  expr.subtract(
                    expr.value(1),
                    expr.abs(
                      expr.subtract(
                        expr.divide(
                          expr.sum(
                            expr.multiply(expr.get(tileCoord, "row"), expr.get(tileGrid, "tileHeight")),
                            expr.multiply(expr.get(tileGrid, "tileHeight"), expr.value(0.5)),
                          ),
                          expr.value(RENDER_HEIGHT),
                        ),
                        expr.value(0.28),
                      ),
                    ),
                  ),
                  expr.value(0),
                  expr.value(1),
                ),
                expr.value(0.42),
              ),
              expr.multiply(expr.read("gearOuterRadius"), expr.value(0.12)),
            ),
            expr.value(0),
            expr.value(1),
          ),
          reflection: expr.clamp(
            expr.sum(
              expr.multiply(
                expr.clamp(
                  expr.subtract(
                    expr.divide(
                      expr.sum(
                        expr.multiply(expr.get(tileCoord, "row"), expr.get(tileGrid, "tileHeight")),
                        expr.multiply(expr.get(tileGrid, "tileHeight"), expr.value(0.5)),
                      ),
                      expr.value(RENDER_HEIGHT),
                    ),
                    expr.value(0.62),
                  ),
                  expr.value(0),
                  expr.value(1),
                ),
                expr.value(0.34),
              ),
            ),
            expr.value(0),
            expr.value(1),
          ),
        }),
      )
      .identityExact(),
  );

  runtime.defineRecipeFamily(
    define
      .recipeFamily<RenderTileEnvironmentLayerModel>("renderTileEnvironmentLayerModel")
      .reads(
        keyed.read("renderTileCoord"),
        "renderTileGridModel",
      )
      .expr(
        expr.object<RenderTileEnvironmentLayerModel>({
          background: expr.clamp(
            expr.sum(
              expr.multiply(
                expr.subtract(
                  expr.value(1),
                  expr.divide(
                    expr.sum(
                      expr.multiply(expr.get(tileCoord, "row"), expr.get(tileGrid, "tileHeight")),
                      expr.multiply(expr.get(tileGrid, "tileHeight"), expr.value(0.5)),
                    ),
                    expr.value(RENDER_HEIGHT),
                  ),
                ),
                expr.value(0.34),
              ),
              expr.multiply(
                expr.clamp(
                  expr.subtract(
                    expr.value(1),
                    expr.abs(
                      expr.subtract(
                        expr.divide(
                          expr.sum(
                            expr.multiply(expr.get(tileCoord, "column"), expr.get(tileGrid, "tileWidth")),
                            expr.multiply(expr.get(tileGrid, "tileWidth"), expr.value(0.5)),
                          ),
                          expr.value(RENDER_WIDTH),
                        ),
                        expr.value(0.5),
                      ),
                    ),
                  ),
                  expr.value(0),
                  expr.value(1),
                ),
                expr.value(0.18),
              ),
            ),
            expr.value(0),
            expr.value(1),
          ),
          floor: expr.clamp(
            expr.sum(
              expr.multiply(
                expr.clamp(
                  expr.subtract(
                    expr.divide(
                      expr.sum(
                        expr.multiply(expr.get(tileCoord, "row"), expr.get(tileGrid, "tileHeight")),
                        expr.multiply(expr.get(tileGrid, "tileHeight"), expr.value(0.5)),
                      ),
                      expr.value(RENDER_HEIGHT),
                    ),
                    expr.value(0.56),
                  ),
                  expr.value(0.05),
                  expr.value(0.18),
                ),
                expr.value(2.4),
              ),
              expr.multiply(
                expr.clamp(
                  expr.subtract(
                    expr.value(1),
                    expr.abs(
                      expr.subtract(
                        expr.divide(
                          expr.sum(
                            expr.multiply(expr.get(tileCoord, "column"), expr.get(tileGrid, "tileWidth")),
                            expr.multiply(expr.get(tileGrid, "tileWidth"), expr.value(0.5)),
                          ),
                          expr.value(RENDER_WIDTH),
                        ),
                        expr.value(0.5),
                      ),
                    ),
                  ),
                  expr.value(0),
                  expr.value(1),
                ),
                expr.value(0.06),
              ),
            ),
            expr.value(0),
            expr.value(1),
          ),
        }),
      )
      .identityExact(),
  );

  const tileGeometryLayer = expr.read<RenderTileGeometryLayerModel>("renderTileGeometryLayerModel");
  const tileLightingLayer = expr.read<RenderTileLightingLayerModel>("renderTileLightingLayerModel");
  const tileEnvironmentLayer = expr.read<RenderTileEnvironmentLayerModel>("renderTileEnvironmentLayerModel");
  runtime.defineRecipeFamily(
    define
      .recipeFamily<RenderTileUploadLayerModel>("renderTileUploadLayerModel")
      .reads(
        keyed.read("renderTileGeometryLayerModel"),
        keyed.read("renderTileLightingLayerModel"),
        keyed.read("renderTileEnvironmentLayerModel"),
      )
      .expr(
        expr.object<RenderTileUploadLayerModel>({
          red: expr.clamp(
            expr.sum(
              expr.multiply(expr.get(tileGeometryLayer, "bodyFace"), expr.value(0.72)),
              expr.multiply(expr.get(tileLightingLayer, "reflection"), expr.value(0.18)),
            ),
            expr.value(0),
            expr.value(1),
          ),
          green: expr.clamp(
            expr.sum(
              expr.multiply(expr.get(tileGeometryLayer, "toothBand"), expr.value(0.6)),
              expr.multiply(expr.get(tileGeometryLayer, "bore"), expr.value(0.4)),
            ),
            expr.value(0),
            expr.value(1),
          ),
          blue: expr.clamp(
            expr.sum(
              expr.multiply(expr.get(tileLightingLayer, "specular"), expr.value(0.62)),
              expr.multiply(expr.get(tileLightingLayer, "reflection"), expr.value(0.38)),
            ),
            expr.value(0),
            expr.value(1),
          ),
          alpha: expr.clamp(
            expr.sum(
              expr.multiply(expr.get(tileEnvironmentLayer, "background"), expr.value(0.55)),
              expr.multiply(expr.get(tileEnvironmentLayer, "floor"), expr.value(0.45)),
              expr.multiply(expr.get(tileLightingLayer, "shadow"), expr.value(0.18)),
            ),
            expr.value(0),
            expr.value(1),
          ),
        }),
      )
      .identityExact(),
  );
}

async function renderCurrentBranch(
  runtime: SignalRuntime,
  state: SceneState,
  scenePatchOps: ScenePatchOp[],
  progress?: RuntimeProgress,
  nextOccupancy?: SceneOccupancySnapshot,
): Promise<RenderUpdate> {
  const renderStartedAt = performance.now();
  const history = runtime.history();
  const branch = history.currentBranch();
  const frameIndex = (frameCounters.get(runtime) ?? 0) + 1;
  frameCounters.set(runtime, frameIndex);
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
    const transactionStartedAt = performance.now();
    sceneSummary = runtime.transaction(scenePatchOps);
    progress?.(
      "render:tx-scene-done",
      `evaluated ${sceneSummary.nodesEvaluated} nodes in ${(performance.now() - transactionStartedAt).toFixed(2)} ms`,
    );
  }
  progress?.("render:aspects:start", "reading render aspects");
  const dirtyTileIndices = dirtyTileIndicesFromScenePatchOps(
    scenePatchOps,
    currentTileGrid(runtime).columns,
  );
  const nextAspectsStartedAt = performance.now();
  const aspects = readRenderAspects(runtime, dirtyTileIndices, progress);
  const nextAspectsMs = performance.now() - nextAspectsStartedAt;
  progress?.(
    "render:aspects:done",
    `render aspects ready in ${nextAspectsMs.toFixed(2)} ms (${dirtyTileIndices?.length ?? aspects.tileGrid.tileCount} tiles)`,
  );
  progress?.("render:js-start", `rendering frame ${frameIndex}`);
  const jsRenderStartedAt = performance.now();
  const rendered = renderScene(state, aspects);
  const jsRenderMs = performance.now() - jsRenderStartedAt;
  progress?.("render:js-done", `js render ${jsRenderMs.toFixed(2)} ms`);
  rendered.stats.frameIndex = frameIndex;

  const graphSummary = summarizeAspectGraph(scenePatchOps, aspects, sceneSummary);
  rendered.stats.changedDetails = changedDetailCount(scenePatchOps);

  const hud: HudModel = {
    frameIndex,
    raysMarched: rendered.stats.raysMarched,
    averageSteps: rendered.stats.averageSteps,
    hits: rendered.stats.hits,
    misses: rendered.stats.misses,
    renderMs: rendered.stats.renderMs,
    tileCount: rendered.stats.tileCount,
    tileColumns: rendered.stats.tileColumns,
    tileRows: rendered.stats.tileRows,
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
    dirtyTiles: rendered.stats.dirtyTiles,
    uploadedTiles: rendered.stats.uploadedTiles,
    uploadSpans: rendered.stats.uploadSpans,
    uploadBytes: rendered.stats.uploadBytes,
    changedDetails: rendered.stats.changedDetails,
  };

  setBranchOccupancySnapshot(
    runtime,
    branch.id,
    nextOccupancy ?? buildSceneOccupancySnapshot(state, aspects.tileGrid.columns, aspects.tileGrid.rows),
  );
  progress?.(
    "render:summary",
    JSON.stringify({
      frameIndex,
      totalMs: Number((performance.now() - renderStartedAt).toFixed(2)),
      previousAspectsMs: 0,
      transactionMs: scenePatchOps.length > 0 ? Number((Number(sceneSummary.totalNanos || 0) / 1_000_000).toFixed(2)) : 0,
      nextAspectsMs: Number(nextAspectsMs.toFixed(2)),
      jsRenderMs: Number(jsRenderMs.toFixed(2)),
      dirtyTiles: rendered.stats.dirtyTiles,
      uploadedTiles: rendered.stats.uploadedTiles,
      uploadSpans: rendered.stats.uploadSpans,
      uploadBytes: rendered.stats.uploadBytes,
    }),
  );

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

function branchOccupancyStore(runtime: SignalRuntime) {
  let store = occupancySnapshots.get(runtime);
  if (!store) {
    store = new Map<BranchId, SceneOccupancySnapshot>();
    occupancySnapshots.set(runtime, store);
  }
  return store;
}

function getBranchOccupancySnapshot(runtime: SignalRuntime, branchId: BranchId) {
  return branchOccupancyStore(runtime).get(branchId) ?? null;
}

function setBranchOccupancySnapshot(runtime: SignalRuntime, branchId: BranchId, snapshot: SceneOccupancySnapshot) {
  branchOccupancyStore(runtime).set(branchId, snapshot);
}

type ScenePatchOp =
  | { kind: "set"; id: string; value: number }
  | { kind: "setWithRegions"; id: string; value: number; changedRegions: ChangedRegion[] };

const occupancySnapshots = new WeakMap<SignalRuntime, Map<BranchId, SceneOccupancySnapshot>>();
const retainedUploadBuffers = new WeakMap<
  SignalRuntime,
  { columns: number; rows: number; buffer: Float32Array }
>();

function buildScenePatchOps(
  current: SceneState,
  next: SceneState,
  patch: ScenePatch,
  previousOccupancy?: SceneOccupancySnapshot | null,
  nextOccupancy?: SceneOccupancySnapshot | null,
  tileGrid: { columns: number; rows: number } = { columns: FULL_TILE_GRID_COLUMNS, rows: FULL_TILE_GRID_ROWS },
): ScenePatchOp[] {
  const regionMap = buildScenePatchRegions(
    current,
    next,
    patch,
    previousOccupancy ?? buildSceneOccupancySnapshot(current, tileGrid.columns, tileGrid.rows),
    nextOccupancy ?? buildSceneOccupancySnapshot(next, tileGrid.columns, tileGrid.rows),
  );
  const ops: ScenePatchOp[] = [];

  function pushSet(id: string, value: number, regions: ChangedRegion[]) {
    if (regions.length > 0) {
      ops.push({ kind: "setWithRegions", id, value, changedRegions: regions });
      return;
    }
    ops.push({ kind: "set", id, value });
  }

  if (patch.camera) {
    const cameraRegions = regionMap.camera;
    if (patch.camera.x !== undefined) pushSet("cameraX", patch.camera.x, cameraRegions);
    if (patch.camera.y !== undefined) pushSet("cameraY", patch.camera.y, cameraRegions);
    if (patch.camera.z !== undefined) pushSet("cameraZ", patch.camera.z, cameraRegions);
    if (patch.camera.yaw !== undefined) pushSet("cameraYaw", patch.camera.yaw, cameraRegions);
    if (patch.camera.pitch !== undefined) pushSet("cameraPitch", patch.camera.pitch, cameraRegions);
  }

  if (patch.light) {
    const lightRegions = regionMap.light;
    if (patch.light.x !== undefined) pushSet("lightX", patch.light.x, lightRegions);
    if (patch.light.y !== undefined) pushSet("lightY", patch.light.y, lightRegions);
    if (patch.light.z !== undefined) pushSet("lightZ", patch.light.z, lightRegions);
    if (patch.light.intensity !== undefined) {
      pushSet("lightIntensity", patch.light.intensity, lightRegions);
    }
  }

  if (patch.gear) {
    const gearRegions = regionMap.gear;
    if (patch.gear.teeth !== undefined) pushSet("gearTeeth", patch.gear.teeth, gearRegions);
    if (patch.gear.outerRadius !== undefined) {
      pushSet("gearOuterRadius", patch.gear.outerRadius, gearRegions);
    }
    if (patch.gear.innerRadius !== undefined) {
      pushSet("gearInnerRadius", patch.gear.innerRadius, gearRegions);
    }
    if (patch.gear.thickness !== undefined) {
      pushSet("gearThickness", patch.gear.thickness, gearRegions);
    }
    if (patch.gear.rotation !== undefined) {
      pushSet("gearRotation", patch.gear.rotation, regionMap.motion);
    }
  }

  return ops;
}

function readPackedUploadBuffer(
  runtime: SignalRuntime,
  columns: number,
  rows: number,
  rectangles: RenderTileUploadRect[],
  fullUpload: boolean,
): Float32Array {
  const retained = retainedUploadBuffer(runtime, columns, rows);
  if (fullUpload) {
    retained.set(
      runtime.readKeyedGridPackedFields(
        "renderTileUploadLayerModel",
        columns,
        rows,
        [...UPLOAD_LAYER_FIELDS],
      ),
    );
    return retained;
  }

  for (const rectangle of rectangles) {
    const clamped = clampTileUploadRect(rectangle, columns, rows);
    if (!clamped) {
      continue;
    }
    const rectValues = runtime.readKeyedRectPackedFields(
      "renderTileUploadLayerModel",
      columns,
      rows,
      clamped.row,
      clamped.startColumn,
      clamped.width,
      clamped.height,
      [...UPLOAD_LAYER_FIELDS],
    );
    writeRectangleIntoPackedBuffer(retained, columns, clamped, rectValues);
  }
  return retained;
}

function retainedUploadBuffer(runtime: SignalRuntime, columns: number, rows: number): Float32Array {
  const existing = retainedUploadBuffers.get(runtime);
  const requiredLength = columns * rows * UPLOAD_LAYER_FIELDS.length;
  if (
    existing
    && existing.columns === columns
    && existing.rows === rows
    && existing.buffer.length === requiredLength
  ) {
    return existing.buffer;
  }
  const next = {
    columns,
    rows,
    buffer: createPackedTileBuffer(requiredLength),
  };
  retainedUploadBuffers.set(runtime, next);
  return next.buffer;
}

function createPackedTileBuffer(length: number): Float32Array {
  if (
    typeof SharedArrayBuffer !== "undefined"
    && typeof globalThis.crossOriginIsolated === "boolean"
    && globalThis.crossOriginIsolated
  ) {
    return new Float32Array(new SharedArrayBuffer(length * Float32Array.BYTES_PER_ELEMENT));
  }
  return new Float32Array(length);
}

function writeRectangleIntoPackedBuffer(
  target: Float32Array,
  columns: number,
  rectangle: RenderTileUploadRect,
  values: Float32Array,
) {
  const stride = UPLOAD_LAYER_FIELDS.length;
  const rowWidth = rectangle.width * stride;
  for (let rowOffset = 0; rowOffset < rectangle.height; rowOffset += 1) {
    const sourceStart = rowOffset * rowWidth;
    const targetStart =
      ((rectangle.row + rowOffset) * columns + rectangle.startColumn) * stride;
    target.set(values.subarray(sourceStart, sourceStart + rowWidth), targetStart);
  }
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

function seedRenderTileSources(runtime: SignalRuntime, columns: number, rows: number) {
  runtime.seedKeyedGridCoords("renderTileCoord", columns, rows);
}

function dirtyTileIndicesFromScenePatchOps(
  scenePatchOps: ScenePatchOp[],
  columns: number,
): number[] | undefined {
  if (scenePatchOps.length === 0) {
    return undefined;
  }
  const touched = new Set<number>();
  for (const op of scenePatchOps) {
    if (op.kind !== "setWithRegions") {
      continue;
    }
    for (const region of op.changedRegions) {
      const match = /^tile-(\d+)-(\d+)$/.exec(region.partition);
      if (!match) {
        continue;
      }
      const column = Number(match[1]);
      const row = Number(match[2]);
      if (!Number.isFinite(column) || !Number.isFinite(row)) {
        continue;
      }
      touched.add(row * columns + column);
    }
  }
  return touched.size > 0 ? Array.from(touched).sort((a, b) => a - b) : [];
}

function coalesceDirtyTileSpans(dirtyTileIndices: number[], columns: number) {
  const sorted = [...dirtyTileIndices].sort((left, right) => left - right);
  const spans: Array<{ row: number; startColumn: number; width: number }> = [];
  for (const tileIndex of sorted) {
    const row = Math.floor(tileIndex / columns);
    const column = tileIndex % columns;
    const previous = spans[spans.length - 1];
    if (
      previous
      && previous.row === row
      && previous.startColumn + previous.width === column
    ) {
      previous.width += 1;
      continue;
    }
    spans.push({ row, startColumn: column, width: 1 });
  }
  return spans;
}

function coalesceDirtyTileRectangles(
  dirtyTileIndices: number[],
  columns: number,
): RenderTileUploadRect[] {
  const spans = coalesceDirtyTileSpans(dirtyTileIndices, columns);
  const rectangles: RenderTileUploadRect[] = [];
  for (const span of spans) {
    const previous = rectangles[rectangles.length - 1];
    if (
      previous
      && previous.startColumn === span.startColumn
      && previous.width === span.width
      && previous.row + previous.height === span.row
    ) {
      previous.height += 1;
      continue;
    }
    rectangles.push({
      row: span.row,
      startColumn: span.startColumn,
      width: span.width,
      height: 1,
    });
  }
  return rectangles;
}

function clampTileUploadRect(
  rectangle: RenderTileUploadRect,
  columns: number,
  rows: number,
): RenderTileUploadRect | null {
  if (rectangle.row >= rows || rectangle.startColumn >= columns) {
    return null;
  }
  const width = Math.min(rectangle.width, columns - rectangle.startColumn);
  const height = Math.min(rectangle.height, rows - rectangle.row);
  if (width <= 0 || height <= 0) {
    return null;
  }
  return {
    row: rectangle.row,
    startColumn: rectangle.startColumn,
    width,
    height,
  };
}

function changedDetailCount(scenePatchOps: ScenePatchOp[]): number {
  const details = new Set<string>();
  for (const op of scenePatchOps) {
    if (op.kind !== "setWithRegions") {
      continue;
    }
    for (const region of op.changedRegions) {
      if (region.detail) {
        details.add(region.detail);
      }
    }
  }
  return details.size;
}

function derivePreviewRenderAspects(
  state: SceneState,
  columns: number,
  rows: number,
): RenderAspects {
  const dimensions: GearDimensionsModel = {
    teeth: state.gear.teeth,
    outerRadius: state.gear.outerRadius,
    innerRadius: state.gear.innerRadius,
    thickness: state.gear.thickness,
    rotation: state.gear.rotation,
    rimWidth: state.gear.outerRadius - state.gear.innerRadius,
    boreRatio: state.gear.innerRadius / Math.max(state.gear.outerRadius, 0.0001),
  };

  const rootRadius = Math.max(
    dimensions.innerRadius + 0.2,
    dimensions.outerRadius - 0.18,
  );
  const profile: GearProfileModel = {
    toothStep: (Math.PI * 2) / Math.max(dimensions.teeth, 1),
    rootRadius,
    tipRadius: dimensions.outerRadius,
    shoulderRadius:
      dimensions.outerRadius
      - (dimensions.outerRadius - rootRadius) * 0.45,
    toothDepth: clamp(
      (dimensions.outerRadius - dimensions.innerRadius) * 0.34,
      0.08,
      0.18,
    ),
    profilePointCount: dimensions.teeth * 6,
  };

  const topology: GearTopologyModel = {
    toothCount: dimensions.teeth,
    ringSegments: Math.max(dimensions.teeth * 4, 64),
    silhouetteBands: Math.max(Math.floor(dimensions.thickness * 12), 3),
    profilePointCount: profile.profilePointCount,
  };

  const mesh: GearMeshModel = {
    topFaceTriangles: profile.profilePointCount * 2,
    sideTriangles: profile.profilePointCount * 2,
    boreTriangles: topology.ringSegments * 2,
    triangleCount: profile.profilePointCount * 4 + topology.ringSegments * 2,
    outerRingCount: profile.profilePointCount,
    innerRingCount: topology.ringSegments,
  };

  const teeth: GearToothModel[] = [];
  const toothStep = (Math.PI * 2) / Math.max(dimensions.teeth, 1);
  for (let index = 0; index < Math.max(dimensions.teeth, 1); index += 1) {
    teeth.push({
      index,
      startAngle: index * toothStep,
      midAngle: (index + 0.5) * toothStep,
      endAngle: (index + 1) * toothStep,
      rootRadius: profile.rootRadius,
      tipRadius: profile.tipRadius,
      thickness: dimensions.thickness,
    });
  }

  const lighting: LightingModel = {
    x: state.light.x,
    y: state.light.y,
    z: state.light.z,
    intensity: state.light.intensity,
    falloff: 1 / (1 + state.light.intensity),
    highlightBoost: 0.5 + state.light.intensity * 0.18,
  };

  const projection: ViewportProjectionModel = {
    focalLength: 480 + Math.abs(state.camera.z) * 8,
    cameraDistance: Math.sqrt(
      state.camera.x * state.camera.x
      + state.camera.y * state.camera.y
      + state.camera.z * state.camera.z,
    ),
    centerLift: state.camera.pitch * 0.28,
    perspectiveScale: mesh.triangleCount / 100,
  };

  const shading: ViewportShadingModel = {
    ambient: 0.24 + (1 / (1 + state.light.intensity)) * 0.16,
    diffuseBoost: 0.46 + state.light.intensity * 0.22,
    edgeContrast: clamp(0.9 + dimensions.rimWidth * 0.8, 0.8, 1.8),
    shadowOpacity: clamp(0.12 + state.light.intensity * 0.08, 0.1, 0.34),
    floorGridOpacity: clamp(0.06 + projection.perspectiveScale * 0.008, 0.05, 0.18),
    specularPower: clamp(10 + lighting.highlightBoost * 4, 8, 24),
  };

  const tileGrid: RenderTileGridModel = {
    columns,
    rows,
    tileCount: columns * rows,
    tileWidth: RENDER_WIDTH / columns,
    tileHeight: RENDER_HEIGHT / rows,
  };

  const tileUploadBuffer = new Float32Array(tileGrid.tileCount * UPLOAD_LAYER_FIELDS.length);
  for (let row = 0; row < rows; row += 1) {
    for (let column = 0; column < columns; column += 1) {
      const tileIndex = row * columns + column;
      const baseIndex = tileIndex * UPLOAD_LAYER_FIELDS.length;
      const normX = (column * tileGrid.tileWidth + tileGrid.tileWidth * 0.5) / RENDER_WIDTH;
      const normY = (row * tileGrid.tileHeight + tileGrid.tileHeight * 0.5) / RENDER_HEIGHT;
      const bodyFace = clamp(
        clamp(1 - Math.abs(normY - 0.54) * 2.8, 0, 1) * 0.62
          + dimensions.outerRadius * 0.22
          + state.light.intensity * 0.08,
        0,
        1,
      );
      const toothBand = clamp(
        dimensions.teeth * 0.02
          + (1 / (1 + Math.abs(state.camera.z))) * 0.28
          + dimensions.outerRadius * 0.12,
        0,
        1,
      );
      const bore = clamp(
        (dimensions.innerRadius / Math.max(dimensions.outerRadius, 0.01)) * 0.9 + 0.08,
        0,
        1,
      );
      const shadow = clamp(normY * 0.38 + normX * 0.08, 0, 1);
      const specular = clamp(
        clamp(1 - Math.abs(normX - 0.44), 0, 1) * 0.32
          + clamp(1 - Math.abs(normY - 0.28), 0, 1) * 0.42
          + dimensions.outerRadius * 0.12,
        0,
        1,
      );
      const reflection = clamp(clamp(normY - 0.62, 0, 1) * 0.34, 0, 1);
      const background = clamp((1 - normY) * 0.34 + clamp(1 - Math.abs(normX - 0.5), 0, 1) * 0.18, 0, 1);
      const floor = clamp(clamp(normY - 0.56, 0.05, 0.18) * 2.4 + clamp(1 - Math.abs(normX - 0.5), 0, 1) * 0.06, 0, 1);

      tileUploadBuffer[baseIndex] = clamp(bodyFace * 0.72 + reflection * 0.18, 0, 1);
      tileUploadBuffer[baseIndex + 1] = clamp(toothBand * 0.6 + bore * 0.4, 0, 1);
      tileUploadBuffer[baseIndex + 2] = clamp(specular * 0.62 + reflection * 0.38, 0, 1);
      tileUploadBuffer[baseIndex + 3] = clamp(background * 0.55 + floor * 0.45 + shadow * 0.18, 0, 1);
    }
  }

  return {
    dimensions,
    profile,
    topology,
    mesh,
    teeth,
    lighting,
    projection,
    shading,
    tileGrid,
    tileUploadBuffer,
    fullComposeUpload: true,
    dirtyTileIndices: Array.from({ length: tileGrid.tileCount }, (_, tileIndex) => tileIndex),
    dirtyTileRects: [{ row: 0, startColumn: 0, width: columns, height: rows }],
  };
}

const STATIC_NODE_COUNT =
  CAMERA_SOURCE_IDS.length +
  LIGHT_SOURCE_IDS.length +
  TILE_SOURCE_IDS.length +
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

function buildScenePatchRegions(
  _current: SceneState,
  _next: SceneState,
  patch: ScenePatch,
  previousOccupancy: SceneOccupancySnapshot,
  nextOccupancy: SceneOccupancySnapshot,
) {
  const gearRegions = patch.gear
    ? occupancyUnionRegions(previousOccupancy, nextOccupancy, ["tooth-band", "body-face", "inner-ring", "bore"])
    : [];
  const lightRegions = patch.light ? [] : [];
  const motionRegions = patch.gear?.rotation !== undefined ? [] : [];
  const cameraRegions = patch.camera
    ? occupancyUnionRegions(previousOccupancy, nextOccupancy, ["background", "floor", "specular", "shadow", "reflection", "tooth-band", "body-face", "bore"])
    : [];

  return {
    gear: gearRegions,
    light: lightRegions,
    motion: motionRegions,
    camera: cameraRegions,
  };
}

function fullFrameBounds(): ScreenBounds {
  return { left: 0, top: 0, right: RENDER_WIDTH, bottom: RENDER_HEIGHT };
}

function projectGearBounds(state: SceneState): ScreenBounds {
  const scale = 150 / Math.max(Math.abs(state.camera.z) + 1.4, 1);
  const radius = Math.max(state.gear.outerRadius * scale, 32);
  const centerX = RENDER_WIDTH * 0.5 + state.camera.x * 20 - state.camera.yaw * 42;
  const centerY = RENDER_HEIGHT * (0.52 + state.camera.pitch * 0.12) - state.camera.y * 18;
  return inflateBounds(
    {
      left: centerX - radius * 1.16,
      top: centerY - radius * 1.16,
      right: centerX + radius * 1.16,
      bottom: centerY + radius * 1.16,
    },
    18,
  );
}

function projectBoreBounds(state: SceneState): ScreenBounds {
  const scale = 150 / Math.max(Math.abs(state.camera.z) + 1.4, 1);
  const radius = Math.max(state.gear.innerRadius * scale, 10);
  const centerX = RENDER_WIDTH * 0.5 + state.camera.x * 20 - state.camera.yaw * 42;
  const centerY = RENDER_HEIGHT * (0.52 + state.camera.pitch * 0.12) - state.camera.y * 18;
  return inflateBounds(
    {
      left: centerX - radius * 1.05,
      top: centerY - radius * 1.05,
      right: centerX + radius * 1.05,
      bottom: centerY + radius * 1.05,
    },
    8,
  );
}

function projectLightBounds(state: SceneState): ScreenBounds {
  const gear = projectGearBounds(state);
  const biasX = state.light.x * 14;
  const biasY = -state.light.y * 10;
  return inflateBounds(
    {
      left: gear.left + biasX * 0.4,
      top: gear.top + biasY * 0.4,
      right: gear.right + biasX,
      bottom: gear.bottom + biasY * 0.2,
    },
    34 + state.light.intensity * 8,
  );
}

function projectShadowBounds(state: SceneState): ScreenBounds {
  const gear = projectGearBounds(state);
  const offsetX = -state.light.x * 18;
  const offsetY = Math.abs(state.light.z) * 10 + 26;
  return inflateBounds(
    {
      left: gear.left + offsetX - 24,
      top: gear.bottom + offsetY - 28,
      right: gear.right + offsetX + 24,
      bottom: gear.bottom + offsetY + 46,
    },
    24,
  );
}

function buildSceneOccupancySnapshot(
  state: SceneState,
  columns: number,
  rows: number,
): SceneOccupancySnapshot {
  const geometry = geometryOccupancyBounds(state);
  const lighting = lightingOccupancyBounds(state);
  const motion = motionOccupancyBounds(state);
  const environment = environmentOccupancyBounds(state);
  const occupancy: SceneOccupancySnapshot = {};
  const exactTileIndices = exactDetailTileIndices(state, columns, rows);
  for (const [detail, bounds] of Object.entries({
    ...geometry,
    ...lighting,
    ...motion,
    ...environment,
  })) {
    occupancy[detail] = {
      detail,
      tileIndices: exactTileIndices[detail] ?? tileIndicesForBounds(bounds, columns, rows),
      gridColumns: columns,
      gridRows: rows,
      bounds,
    } satisfies TileDetailOccupancy;
  }
  return occupancy;
}

function geometryOccupancyBounds(state: SceneState) {
  const gear = projectGearBounds(state);
  const bore = projectBoreBounds(state);
  const toothBand = ringBandBounds(gear, 0.72, 1.08);
  const innerRing = ringBandBounds(gear, 0.38, 0.68);
  const bodyFace = ringBandBounds(gear, 0.18, 0.74);
  return {
    "tooth-band": toothBand,
    "body-face": bodyFace,
    "inner-ring": innerRing,
    bore,
  } satisfies Record<string, ScreenBounds>;
}

function lightingOccupancyBounds(state: SceneState) {
  const light = projectLightBounds(state);
  const shadow = projectShadowBounds(state);
  const highlight = cropBounds(light, 0.18, 0.12, 0.88, 0.72);
  const reflection = reflectShadowBounds(shadow);
  return {
    specular: light,
    highlight,
    shadow,
    reflection,
  } satisfies Record<string, ScreenBounds>;
}

function motionOccupancyBounds(state: SceneState) {
  const gear = projectGearBounds(state);
  const light = projectLightBounds(state);
  return {
    "tooth-band": ringBandBounds(gear, 0.7, 1.08),
    "body-face": ringBandBounds(gear, 0.24, 0.84),
    specular: cropBounds(light, 0.08, 0.08, 0.92, 0.86),
  } satisfies Record<string, ScreenBounds>;
}

function environmentOccupancyBounds(state: SceneState) {
  const lighting = lightingOccupancyBounds(state);
  const geometry = geometryOccupancyBounds(state);
  return {
    background: fullFrameBounds(),
    floor: floorBounds(state),
    specular: lighting.specular,
    shadow: lighting.shadow,
    reflection: lighting.reflection,
    "tooth-band": geometry["tooth-band"],
    "body-face": geometry["body-face"],
    bore: geometry.bore,
  } satisfies Record<string, ScreenBounds>;
}

function occupancyUnionRegions(
  previous: SceneOccupancySnapshot,
  next: SceneOccupancySnapshot,
  details: string[],
): ChangedRegion[] {
  const regions: ChangedRegion[] = [];
  for (const detail of details) {
    const touched = new Set<number>([
      ...(previous[detail]?.tileIndices ?? []),
      ...(next[detail]?.tileIndices ?? []),
    ]);
    const columns = previous[detail]?.gridColumns
      ?? next[detail]?.gridColumns
      ?? FULL_TILE_GRID_COLUMNS;
    for (const tileIndex of touched) {
      const column = tileIndex % columns;
      const row = Math.floor(tileIndex / columns);
      regions.push({ partition: `tile-${column}-${row}`, detail });
    }
  }
  return uniqueRegions(regions);
}

function tileIndicesForBounds(
  bounds: ScreenBounds,
  columns: number,
  rows: number,
): number[] {
  const tileWidth = RENDER_WIDTH / columns;
  const tileHeight = RENDER_HEIGHT / rows;
  const left = clampInt(Math.floor(bounds.left / tileWidth), 0, columns - 1);
  const right = clampInt(Math.floor(bounds.right / tileWidth), 0, columns - 1);
  const top = clampInt(Math.floor(bounds.top / tileHeight), 0, rows - 1);
  const bottom = clampInt(Math.floor(bounds.bottom / tileHeight), 0, rows - 1);
  const indices: number[] = [];
  for (let row = top; row <= bottom; row += 1) {
    for (let column = left; column <= right; column += 1) {
      indices.push(row * columns + column);
    }
  }
  return indices;
}

function exactDetailTileIndices(
  state: SceneState,
  columns: number,
  rows: number,
): Record<string, number[]> {
  const gear = projectGearBounds(state);
  const bore = projectBoreBounds(state);
  const light = projectLightBounds(state);
  const shadow = projectShadowBounds(state);
  const reflection = reflectShadowBounds(shadow);
  const floor = floorBounds(state);
  const center = projectedGearCenter(state);
  const outerRadius = Math.max((gear.right - gear.left) * 0.5, 1);
  const innerRadius = Math.max((bore.right - bore.left) * 0.5, 1);
  const toothInnerRadius = outerRadius * 0.72;
  const bodyOuterRadius = outerRadius * 0.74;
  const innerRingInnerRadius = outerRadius * 0.38;
  const innerRingOuterRadius = outerRadius * 0.68;
  const toothStep = (Math.PI * 2) / Math.max(state.gear.teeth, 1);
  const toothHalfWidth = toothStep * 0.34;
  const detailTiles = new Map<string, Set<number>>();

  forEachTileCenter(columns, rows, (tileIndex, x, y) => {
    const dx = x - center.x;
    const dy = y - center.y;
    const distance = Math.hypot(dx, dy);
      const angle = normalizeAngle(Math.atan2(dy, dx));

    if (distance >= innerRadius && distance <= bodyOuterRadius) {
      addDetailTile(detailTiles, "body-face", tileIndex);
    }
    if (distance >= innerRingInnerRadius && distance <= innerRingOuterRadius) {
      addDetailTile(detailTiles, "inner-ring", tileIndex);
    }
    if (distance <= innerRadius) {
      addDetailTile(detailTiles, "bore", tileIndex);
    }
    if (distance >= toothInnerRadius && distance <= outerRadius * 1.08) {
      const nearestTooth = Math.round(angle / toothStep);
      const toothCenter = nearestTooth * toothStep;
      const delta = shortestAngularDistance(angle, toothCenter);
      if (Math.abs(delta) <= toothHalfWidth) {
        addDetailTile(detailTiles, "tooth-band", tileIndex);
      }
    }

    if (pointInBounds(x, y, light)) {
      addDetailTile(detailTiles, "specular", tileIndex);
    }
    if (pointInBounds(x, y, cropBounds(light, 0.18, 0.12, 0.88, 0.72))) {
      addDetailTile(detailTiles, "highlight", tileIndex);
    }
    if (pointInBounds(x, y, shadow)) {
      addDetailTile(detailTiles, "shadow", tileIndex);
    }
    if (pointInBounds(x, y, reflection)) {
      addDetailTile(detailTiles, "reflection", tileIndex);
    }
    if (pointInBounds(x, y, floor)) {
      addDetailTile(detailTiles, "floor", tileIndex);
    }
    addDetailTile(detailTiles, "background", tileIndex);
  });

  return Object.fromEntries(
    Array.from(detailTiles.entries()).map(([detail, indices]) => [
      detail,
      Array.from(indices).sort((left, right) => left - right),
    ]),
  );
}

function forEachTileCenter(
  columns: number,
  rows: number,
  visit: (tileIndex: number, x: number, y: number) => void,
) {
  const tileWidth = RENDER_WIDTH / columns;
  const tileHeight = RENDER_HEIGHT / rows;
  for (let row = 0; row < rows; row += 1) {
    for (let column = 0; column < columns; column += 1) {
      const tileIndex = row * columns + column;
      visit(
        tileIndex,
        column * tileWidth + tileWidth * 0.5,
        row * tileHeight + tileHeight * 0.5,
      );
    }
  }
}

function addDetailTile(
  detailTiles: Map<string, Set<number>>,
  detail: string,
  tileIndex: number,
) {
  let tiles = detailTiles.get(detail);
  if (!tiles) {
    tiles = new Set<number>();
    detailTiles.set(detail, tiles);
  }
  tiles.add(tileIndex);
}

function projectedGearCenter(state: SceneState) {
  return {
    x: RENDER_WIDTH * 0.5 + state.camera.x * 20 - state.camera.yaw * 42,
    y: RENDER_HEIGHT * (0.52 + state.camera.pitch * 0.12) - state.camera.y * 18,
  };
}

function currentTileGrid(runtime: SignalRuntime) {
  return {
    columns: runtime.read<number>("renderTileColumns"),
    rows: runtime.read<number>("renderTileRows"),
  };
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}

function pointInBounds(x: number, y: number, bounds: ScreenBounds): boolean {
  return x >= bounds.left && x <= bounds.right && y >= bounds.top && y <= bounds.bottom;
}

function normalizeAngle(angle: number): number {
  const wrapped = angle % (Math.PI * 2);
  return wrapped < 0 ? wrapped + Math.PI * 2 : wrapped;
}

function shortestAngularDistance(from: number, to: number): number {
  let delta = from - to;
  while (delta > Math.PI) {
    delta -= Math.PI * 2;
  }
  while (delta < -Math.PI) {
    delta += Math.PI * 2;
  }
  return delta;
}

function ringBandBounds(bounds: ScreenBounds, _innerScale: number, outerScale: number): ScreenBounds {
  const centerX = (bounds.left + bounds.right) * 0.5;
  const centerY = (bounds.top + bounds.bottom) * 0.5;
  const halfWidth = (bounds.right - bounds.left) * 0.5;
  const halfHeight = (bounds.bottom - bounds.top) * 0.5;
  return {
    left: clampNumber(centerX - halfWidth * outerScale, 0, RENDER_WIDTH),
    top: clampNumber(centerY - halfHeight * outerScale, 0, RENDER_HEIGHT),
    right: clampNumber(centerX + halfWidth * outerScale, 0, RENDER_WIDTH),
    bottom: clampNumber(centerY + halfHeight * outerScale, 0, RENDER_HEIGHT),
  };
}

function cropBounds(bounds: ScreenBounds, x0: number, y0: number, x1: number, y1: number): ScreenBounds {
  const width = bounds.right - bounds.left;
  const height = bounds.bottom - bounds.top;
  return {
    left: clampNumber(bounds.left + width * x0, 0, RENDER_WIDTH),
    top: clampNumber(bounds.top + height * y0, 0, RENDER_HEIGHT),
    right: clampNumber(bounds.left + width * x1, 0, RENDER_WIDTH),
    bottom: clampNumber(bounds.top + height * y1, 0, RENDER_HEIGHT),
  };
}

function reflectShadowBounds(shadow: ScreenBounds): ScreenBounds {
  const centerY = (shadow.top + shadow.bottom) * 0.5;
  const height = shadow.bottom - shadow.top;
  return inflateBounds(
    {
      left: shadow.left + 12,
      top: centerY + 10,
      right: shadow.right - 12,
      bottom: centerY + height * 0.82,
    },
    10,
  );
}

function floorBounds(state: SceneState): ScreenBounds {
  const horizon = RENDER_HEIGHT * (0.68 + state.camera.pitch * 0.04);
  return {
    left: 0,
    top: clampNumber(horizon, 0, RENDER_HEIGHT),
    right: RENDER_WIDTH,
    bottom: RENDER_HEIGHT,
  };
}

function inflateBounds(bounds: ScreenBounds, padding: number): ScreenBounds {
  return {
    left: clampNumber(bounds.left - padding, 0, RENDER_WIDTH),
    top: clampNumber(bounds.top - padding, 0, RENDER_HEIGHT),
    right: clampNumber(bounds.right + padding, 0, RENDER_WIDTH),
    bottom: clampNumber(bounds.bottom + padding, 0, RENDER_HEIGHT),
  };
}

function uniqueRegions(regions: ChangedRegion[]): ChangedRegion[] {
  const seen = new Set<string>();
  return regions.filter((region) => {
    const key = `${region.partition}:${region.detail ?? ""}`;
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function clampInt(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

function clampNumber(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

function summarizeAspectGraph(
  scenePatchOps: ScenePatchOp[],
  next: RenderAspects,
  sceneSummary: Awaited<ReturnType<SignalRuntime["transaction"]>>,
) {
  const changedAspects = changedAspectCount(scenePatchOps);

  const sourceChanges = scenePatchOps.length;
  const hudNodes = 1;
  const touchedTileNodes = uniqueTouchedTilePartitions(scenePatchOps).size;
  const totalGraphNodes =
    STATIC_NODE_COUNT + next.tileGrid.columns * next.tileGrid.rows * 2;
  const touchedNodes = Math.min(sourceChanges + changedAspects + hudNodes + touchedTileNodes, totalGraphNodes);
  const nodesEvaluated = touchedNodes;
  const nodesSuppressed = Math.max(totalGraphNodes - nodesEvaluated, 0);

  return {
    ...sceneSummary,
    touchedNodes,
    nodesEvaluated,
    nodesSuppressed,
  };
}

function changedAspectCount(scenePatchOps: ScenePatchOp[]): number {
  if (scenePatchOps.length === 0) {
    return 0;
  }
  const aspects = new Set<string>();
  for (const op of scenePatchOps) {
    aspects.add(op.id);
    if (op.kind !== "setWithRegions") {
      continue;
    }
    for (const region of op.changedRegions) {
      if (
        region.detail === "tooth-band"
        || region.detail === "body-face"
        || region.detail === "inner-ring"
        || region.detail === "bore"
      ) {
        aspects.add("geometry");
      } else if (
        region.detail === "specular"
        || region.detail === "highlight"
        || region.detail === "shadow"
        || region.detail === "reflection"
      ) {
        aspects.add("lighting");
      } else if (region.detail === "background" || region.detail === "floor") {
        aspects.add("environment");
      }
    }
  }
  return aspects.size;
}

function uniqueTouchedTilePartitions(scenePatchOps: ScenePatchOp[]): Set<string> {
  const touched = new Set<string>();
  for (const op of scenePatchOps) {
    if (op.kind !== "setWithRegions") continue;
    for (const region of op.changedRegions) {
      touched.add(region.partition);
    }
  }
  return touched;
}

export function exportRuntimeEnvelope(runtime: SignalRuntime): RuntimeEnvelope {
  return runtime.adapters().exportRuntimeEnvelope();
}

export function replaceRuntimeEnvelope(runtime: SignalRuntime, envelope: RuntimeEnvelope) {
  runtime.adapters().replaceRuntimeEnvelope(envelope);
}
