import type {
  LineageSummary,
  ReplayFrameSummary,
  RunSummary,
  SignalRuntime,
  WhySummary,
} from "@WORTH/signal";

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
  tileCount: number;
  tileColumns: number;
  tileRows: number;
  dirtyTiles: number;
  uploadedTiles: number;
  uploadSpans: number;
  uploadBytes: number;
  changedDetails: number;
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

export type ScreenBounds = {
  left: number;
  top: number;
  right: number;
  bottom: number;
};

export type TileDetailOccupancy = {
  detail: string;
  tileIndices: number[];
  gridColumns: number;
  gridRows: number;
  bounds?: ScreenBounds | null;
};

export type SceneOccupancySnapshot = Record<string, TileDetailOccupancy>;

export type HudModel = {
  frameIndex: number;
  raysMarched: number;
  averageSteps: number;
  hits: number;
  misses: number;
  renderMs: number;
  tileCount: number;
  tileColumns: number;
  tileRows: number;
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
  dirtyTiles: number;
  uploadedTiles: number;
  uploadSpans: number;
  uploadBytes: number;
  changedDetails: number;
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

export type RenderTileCoord = {
  column: number;
  row: number;
};

export type RenderTileGridModel = {
  columns: number;
  rows: number;
  tileCount: number;
  tileWidth: number;
  tileHeight: number;
};

export type RenderTileModel = {
  column: number;
  row: number;
  left: number;
  top: number;
  width: number;
  height: number;
  centerX: number;
  centerY: number;
  radialWeight: number;
  lightWeight: number;
  gearWeight: number;
};

export type RenderTileGeometryLayerModel = {
  bodyFace: number;
  toothBand: number;
  bore: number;
};

export type RenderTileLightingLayerModel = {
  shadow: number;
  specular: number;
  reflection: number;
};

export type RenderTileEnvironmentLayerModel = {
  background: number;
  floor: number;
};

export type RenderTileUploadLayerModel = {
  red: number;
  green: number;
  blue: number;
  alpha: number;
};

export type RenderTileUploadRect = {
  row: number;
  startColumn: number;
  width: number;
  height: number;
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
  tileGrid: RenderTileGridModel;
  tileUploadBuffer: Float32Array;
  fullComposeUpload: boolean;
  dirtyTileIndices: number[];
  dirtyTileRects: RenderTileUploadRect[];
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
  initialRender: RenderUpdate | null;
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

export type MergeSemanticsView = {
  strategyName: string | null;
  strategyBasis: string | null;
  mergeBaseName: string | null;
  mergeBaseBasis: string | null;
  conflictPolicyName: string | null;
  conflictPolicyBasis: string | null;
  conflictIsolationName: string | null;
  conflictIsolationBasis: string | null;
  identityMatcherName: string | null;
  identityMatcherBasis: string | null;
  sourceOnlyPolicyName: string | null;
  sourceOnlyPolicyBasis: string | null;
  deletionPolicyName: string | null;
  deletionPolicyBasis: string | null;
};

export type ConflictIsolationRecordView = {
  sourceNode: string;
  targetNode: string | null;
  granularity: string;
  isolatedAspects: string[];
};

export type ConflictIsolationView = {
  policyName: string | null;
  policyDigest: string | null;
  policyBasis: string | null;
  expansionBreadth: number;
  witnessGranularity: string | null;
  witnessConflictRecordCount: number;
  isolatedRegionCount: number;
  hostDeclaredRegionCount: number;
  conservativeExpandedNodeCount: number;
  records: ConflictIsolationRecordView[];
};

export type IdentityCorrespondenceView = {
  sourceNode: string;
  targetNode: string | null;
  status: string;
  basis: string | null;
  candidateCount: number;
  candidateTargetNodes: string[];
  admissibilityRejection: string | null;
};

export type MergeRecordView = {
  sourceNode: string;
  targetNode: string | null;
  action: string;
  basis: string;
  identityBasis: string | null;
  identityStatus: string | null;
  identityCandidateCount: number;
  resolvedConflictKinds: string[];
};

export type AspectPolicyRecordView = {
  aspect: string;
  policyName: string;
  policyBasis: string;
  affectedSourceNodes: string[];
};

export type AspectDecisionRecordView = {
  aspect: string;
  sourceNode: string;
  targetNode: string | null;
  policyName: string;
  policyBasis: string;
  outcome: string;
};

export type MergePlanProofView = {
  proofSchemaVersion: string | null;
  registryBundleDigest: string | null;
  planDigest: string | null;
  semanticsDigest: string | null;
  loweredStrategyBundleDigest: string | null;
  selectedStrategyDigest: string | null;
  selectedMergeBaseDigest: string | null;
  selectedConflictPolicyDigest: string | null;
  selectedConflictIsolationDigest: string | null;
  selectedIdentityMatcherDigest: string | null;
  selectedSourceOnlyPolicyDigest: string | null;
  selectedDeletionPolicyDigest: string | null;
};

export type MergeResultProofView = {
  proofSchemaVersion: string | null;
  registryBundleDigest: string | null;
  resultDigest: string | null;
  semanticsDigest: string | null;
  loweredStrategyBundleDigest: string | null;
  lineageDigest: string | null;
  selectedStrategyDigest: string | null;
  selectedMergeBaseDigest: string | null;
  selectedConflictPolicyDigest: string | null;
  selectedConflictIsolationDigest: string | null;
  selectedIdentityMatcherDigest: string | null;
  selectedSourceOnlyPolicyDigest: string | null;
  selectedDeletionPolicyDigest: string | null;
};

export type BranchStateProofView = {
  proofSchemaVersion: string | null;
  branchId: number | null;
  branchName: string | null;
  snapshotId: number | null;
  stateDigest: string | null;
};

export type ReplayParityProofView = {
  proofSchemaVersion: string | null;
  expectedBranchId: number | null;
  expectedBranchName: string | null;
  expectedSnapshotId: number | null;
  expectedStateDigest: string | null;
  replayedBranchId: number | null;
  replayedBranchName: string | null;
  replayedSnapshotId: number | null;
  replayedStateDigest: string | null;
  parity: boolean | null;
  mismatchClasses: string[];
};

export type ReplayArtifactProofView = {
  proofSchemaVersion: string | null;
  parity: boolean | null;
  mismatchClasses: string[];
  replayedLoweredStrategyBundleDigest: string | null;
  replayedMergePlanDigest: string | null;
  replayedMergeResultDigest: string | null;
  replayedLineageDigest: string | null;
  replayedBranchStateDigest: string | null;
  replayedRegistryBundleDigest: string | null;
};

export type ScenarioMode = "manual-gear" | "adversarial-gear-merge";
export type DiagnosticsTier = "webDevelopment" | "development" | "forensic" | "kernel";
export type ScenarioStatus = "idle" | "scripted" | "planned" | "merged" | "replayed";

export type ScenarioProofArtifacts = {
  proofSchemaVersion: string | null;
  schemaDigest: string | null;
  registryBundleDigest: string | null;
  loweredStrategyBundleDigest: string | null;
  semanticsDigest: string | null;
  mergePlanDigest: string | null;
  mergeResultDigest: string | null;
  lineageDigest: string | null;
  mergedBranchStateDigest: string | null;
  replayedLoweredStrategyBundleDigest: string | null;
  replayedMergePlanDigest: string | null;
  replayedMergeResultDigest: string | null;
  replayedLineageDigest: string | null;
  replayBranchStateDigest: string | null;
  replayParity: boolean | null;
  replayMismatchClasses: string[];
};

export type ScenarioState = {
  mode: ScenarioMode;
  status: ScenarioStatus;
  diagnosticsTier: DiagnosticsTier;
  lastAction: string;
  inspectedNodes: string[];
  steps: string[];
  proof: ScenarioProofArtifacts | null;
};

export type MergePlan = {
  sourceBranchId: number | null;
  targetBranchId: number | null;
  mergeKind: string | null;
  divergence: string | null;
  mergeStrategy: string | null;
  sourceSnapshotId: number | null;
  targetSnapshotIdBefore: number | null;
  candidateCount: number;
  sharedNodeCount: number;
  expandedNodeCount: number;
  supportNodeCount: number;
  nodePlanCount: number;
  adoptionCount: number;
  hasResolutionPlan: boolean;
  semantics: MergeSemanticsView;
  identity: {
    targetCandidateCount: number;
    sourceLookupCount: number;
    ambiguousMatchCount: number;
    rejectedAdmissibilityCount: number;
    records: IdentityCorrespondenceView[];
  };
  deletion: {
    targetOnlyCount: number;
    rejectedTargetOnlyCount: number;
    targetOnlyNodes: string[];
  };
  conflictIsolation: ConflictIsolationView;
  aspectPolicies: AspectPolicyRecordView[];
  aspectDecisions: AspectDecisionRecordView[];
  proof: MergePlanProofView | null;
};

export type MergeResult = {
  sourceBranchId: number | null;
  targetBranchId: number | null;
  mergeKind: string | null;
  divergence: string | null;
  mergeStrategy: string | null;
  mergedSnapshotId: number | null;
  targetSnapshotIdBefore: number | null;
  targetSnapshotIdAfter: number | null;
  sourceSnapshotId: number | null;
  recordCount: number;
  adoptedCount: number;
  introducedCount: number;
  replacedCount: number;
  preservedTargetCount: number;
  equivalentUnchangedCount: number;
  skippedNonAdoptableCount: number;
  conflictCount: number;
  hasResolutionPlan: boolean;
  semantics: MergeSemanticsView;
  counters: {
    sourceSliceBreadth: number;
    proofMinimalOverlapBreadth: number;
    conservativeOverlapExpansionBreadth: number;
    finalCandidateBreadth: number;
    reconciliationBreadth: number;
    targetOnlyCount: number;
    identityTargetCandidatesIndexed: number;
    identitySourceLookups: number;
    identityAmbiguousMatchCount: number;
    identityRejectedAdmissibilityCount: number;
    conflictIsolationRecordCount: number;
    conflictIsolationExpansionBreadth: number;
  };
  identity: {
    records: IdentityCorrespondenceView[];
  };
  deletion: {
    targetOnlyCount: number;
    rejectedTargetOnlyCount: number;
    targetOnlyNodes: string[];
  };
  conflictIsolation: ConflictIsolationView;
  aspectPolicies: AspectPolicyRecordView[];
  aspectDecisions: AspectDecisionRecordView[];
  records: MergeRecordView[];
  proof: MergeResultProofView | null;
};
