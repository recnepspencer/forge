import {
  createSignalRuntime,
  define,
  expr,
  policy,
  type MergePlanReport,
  type MergeResultReport,
  type ReplayFrameSummary,
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
  type HudModel,
  type LightingModel,
  type RenderAspects,
  type RenderUpdate,
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
  progress?.("read:branch-summary:hud-start", "reading hud model");
  const hud = runtime.read<HudModel>("hudModel");
  progress?.("read:branch-summary:hud-done", "read hud model");

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
  const history = runtime.history();
  const before = history.currentBranch().id;
  history.switchBranch(branchId);
  const replay = history.replayFor("hudModel").frames.slice(-10) as ReplayFrameSummary[];
  const why = runtime.diagnostics().why("hudModel") as WhySummary;
  history.switchBranch(before);
  return { replay, why };
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
  const result = await renderCurrentBranch(runtime, nextState, buildScenePatchOps(patch), progress);
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
  const result = await renderCurrentBranch(runtime, readSceneState(runtime, progress), [], progress);
  history.switchBranch(before);
  return result;
}

export function readRenderAspects(runtime: SignalRuntime): RenderAspects {
  return {
    dimensions: runtime.read<GearDimensionsModel>("gearDimensionsModel"),
    profile: runtime.read<GearProfileModel>("gearProfileModel"),
    topology: runtime.read<GearTopologyModel>("gearTopologyModel"),
    mesh: runtime.read<GearMeshModel>("gearMeshModel"),
    lighting: runtime.read<LightingModel>("lightingModel"),
    projection: runtime.read<ViewportProjectionModel>("viewportProjectionModel"),
    shading: runtime.read<ViewportShadingModel>("viewportShadingModel"),
  };
}

export function planMerge(
  runtime: SignalRuntime,
  sourceBranchId: BranchId,
  targetBranchId: BranchId,
): MergePlanReport {
  return runtime.history().planMergeBranches(sourceBranchId, targetBranchId);
}

export async function executeMerge(
  runtime: SignalRuntime,
  sourceBranchId: BranchId,
  targetBranchId: BranchId,
): Promise<MergeResultReport> {
  const result = runtime.history().mergeBranches(sourceBranchId, targetBranchId);
  await renderBranch(runtime, targetBranchId);
  return result;
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
  progress?.("read:hudFrameIndex:start", "reading hudFrameIndex");
  const frameIndex = runtime.read<number>("hudFrameIndex") + 1;
  progress?.("read:hudFrameIndex:done", `frame ${frameIndex}`);
  const previousAspects = readRenderAspects(runtime);
  progress?.("render:tx-scene-start", "committing scene aspects");
  const sceneSummary = runtime.transaction(scenePatchOps);
  progress?.("render:tx-scene-done", `evaluated ${sceneSummary.nodesEvaluated} nodes`);
  progress?.("render:aspects:start", "reading render aspects");
  const aspects = readRenderAspects(runtime);
  progress?.("render:aspects:done", "render aspects ready");
  progress?.("render:js-start", `rendering frame ${frameIndex}`);
  const rendered = renderScene(state, aspects);
  progress?.("render:js-done", `js render ${rendered.stats.renderMs.toFixed(2)} ms`);
  rendered.stats.frameIndex = frameIndex;

  progress?.("render:tx-frame-start", "committing hud stats");
  runtime.transaction([
    {
      kind: "setMany",
      values: [
        { id: "hudFrameIndex", value: frameIndex },
        { id: "hudRaysMarched", value: rendered.stats.raysMarched },
        { id: "hudAverageSteps", value: rendered.stats.averageSteps },
        { id: "hudHits", value: rendered.stats.hits },
        { id: "hudMisses", value: rendered.stats.misses },
        { id: "hudRenderMs", value: rendered.stats.renderMs },
      ],
    },
  ]);
  progress?.("render:tx-frame-done", "hud stats committed");

  progress?.("render:tx-hud-start", "committing derived hud counters");
  runtime.transaction([
    {
      kind: "setMany",
      values: [
        { id: "hudTouchedNodes", value: sceneSummary.touchedNodes },
        { id: "hudNodesEvaluated", value: sceneSummary.nodesEvaluated },
        { id: "hudNodesSuppressed", value: sceneSummary.nodesSuppressed },
        { id: "hudTotalNanos", value: Number(sceneSummary.totalNanos) || 0 },
      ],
    },
  ]);
  progress?.("render:tx-hud-done", "hud counters committed");

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
