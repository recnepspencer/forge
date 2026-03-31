import type {
  LineageSummary,
  MergePlanReport,
  MergeResultReport,
  ReplayFrameSummary,
  RunSummary,
  SignalRuntime,
  WhySummary,
} from "@forge/signal";

export const RENDER_WIDTH = 640;
export const RENDER_HEIGHT = 360;

export type BranchId = number;

export type CameraState = {
  x: number;
  y: number;
  z: number;
  yaw: number;
  pitch: number;
};

export type LightState = {
  x: number;
  y: number;
  z: number;
  intensity: number;
};

export type GearState = {
  teeth: number;
  outerRadius: number;
  innerRadius: number;
  thickness: number;
  rotation: number;
};

export type RenderStats = {
  frameIndex: number;
  raysMarched: number;
  averageSteps: number;
  hits: number;
  misses: number;
  renderMs: number;
};

export type SceneState = {
  camera: CameraState;
  light: LightState;
  gear: GearState;
};

export type ScenePatch = {
  camera?: Partial<CameraState>;
  light?: Partial<LightState>;
  gear?: Partial<GearState>;
};

export type HudModel = {
  frameIndex: number;
  raysMarched: number;
  averageSteps: number;
  hits: number;
  misses: number;
  renderMs: number;
  touchedNodes: number;
  nodesEvaluated: number;
  nodesSuppressed: number;
  totalNanos: number;
  cameraX: number;
  cameraY: number;
  cameraZ: number;
  lightX: number;
  lightY: number;
  lightZ: number;
};

export type GearDimensionsModel = {
  teeth: number;
  outerRadius: number;
  innerRadius: number;
  thickness: number;
  rotation: number;
  rimWidth: number;
  boreRatio: number;
};

export type GearProfileModel = {
  toothStep: number;
  rootRadius: number;
  tipRadius: number;
  shoulderRadius: number;
  toothDepth: number;
  profilePointCount: number;
};

export type GearTopologyModel = {
  toothCount: number;
  ringSegments: number;
  silhouetteBands: number;
  profilePointCount: number;
};

export type GearMeshModel = {
  topFaceTriangles: number;
  sideTriangles: number;
  boreTriangles: number;
  triangleCount: number;
  outerRingCount: number;
  innerRingCount: number;
};

export type GearToothModel = {
  index: number;
  startAngle: number;
  midAngle: number;
  endAngle: number;
  rootRadius: number;
  tipRadius: number;
  thickness: number;
};

export type LightingModel = {
  x: number;
  y: number;
  z: number;
  intensity: number;
  falloff: number;
  highlightBoost: number;
};

export type ViewportProjectionModel = {
  focalLength: number;
  cameraDistance: number;
  centerLift: number;
  perspectiveScale: number;
};

export type ViewportShadingModel = {
  ambient: number;
  diffuseBoost: number;
  edgeContrast: number;
  shadowOpacity: number;
  floorGridOpacity: number;
  specularPower: number;
};

export type RenderAspects = {
  dimensions: GearDimensionsModel;
  profile: GearProfileModel;
  topology: GearTopologyModel;
  mesh: GearMeshModel;
  teeth: GearToothModel[];
  lighting: LightingModel;
  projection: ViewportProjectionModel;
  shading: ViewportShadingModel;
};

export type SceneBranchView = {
  id: BranchId;
  name: string;
  state: SceneState;
  hud: HudModel;
};

export type BranchInspect = {
  selectedNode: string;
  replay: ReplayFrameSummary[];
  why: WhySummary;
  lineage: LineageSummary;
};

export type RuntimeState = {
  runtime: SignalRuntime;
  graphNodes: number;
};

export type SceneRuntimeBundle = {
  runtime: SignalRuntime;
};

export type RenderUpdate = {
  summary: RunSummary;
  branchId: BranchId;
  branchName: string;
  state: SceneState;
  hud: HudModel;
  frame: ImageBitmap;
  stats: RenderStats;
};

export type MergePlan = MergePlanReport;
export type MergeResult = MergeResultReport;
