import type { BranchId, DiagnosticsTier, ScenarioMode, ScenarioProofArtifacts, ScenarioState } from "../core/types";
import type { SceneState } from "../core/types";

export const DEFAULT_SCENARIO_INSPECT_NODES = [
  "gearTopologyModel",
  "gearMeshModel",
  "gearToothModel::tooth-0",
  "lightingModel",
  "hudModel",
];

export type TimelineKind = "normal" | "branch" | "merge";

export function createScenarioState(
  mode: ScenarioMode,
  status: ScenarioState["status"],
  lastAction: string,
  diagnosticsTier: DiagnosticsTier = "webDevelopment",
): ScenarioState {
  return {
    mode,
    status,
    diagnosticsTier,
    lastAction,
    inspectedNodes: DEFAULT_SCENARIO_INSPECT_NODES,
    steps: [],
    proof: emptyScenarioProof(),
  };
}

export function mergeScenarioState(
  current: ScenarioState | null,
  status: ScenarioState["status"],
  lastAction: string,
): ScenarioState {
  return {
    ...(current ?? createScenarioState("adversarial-gear-merge", status, lastAction)),
    status,
    lastAction,
    mode: current?.mode ?? "adversarial-gear-merge",
    diagnosticsTier: current?.diagnosticsTier ?? "webDevelopment",
    inspectedNodes: current?.inspectedNodes ?? DEFAULT_SCENARIO_INSPECT_NODES,
    steps: current?.steps ?? [],
  };
}

export function withScenarioProof(
  scenario: ScenarioState | null,
  proof: ScenarioProofArtifacts | null,
): ScenarioState | null {
  if (!scenario) {
    return null;
  }
  return {
    ...scenario,
    proof: proof ?? emptyScenarioProof(),
  };
}

export function withScenarioStatus(
  current: ScenarioState | null,
  status: ScenarioState["status"],
  lastAction: string,
  proof: ScenarioProofArtifacts | null,
): ScenarioState {
  return {
    ...mergeScenarioState(current, status, lastAction),
    proof: proof ?? emptyScenarioProof(),
  };
}

export function withReplayScenarioProof(
  current: ScenarioState | null,
  lastAction: string,
  previousProof: ScenarioProofArtifacts | null,
  replayProof: Partial<ScenarioProofArtifacts> | null,
): ScenarioState {
  return {
    ...mergeScenarioState(current, "replayed", lastAction),
    proof: {
      ...(previousProof ?? emptyScenarioProof()),
      ...(replayProof ?? {}),
    },
  };
}

export function withScenarioTier(
  scenario: ScenarioState | null,
  tier: DiagnosticsTier,
  lastAction: string,
): ScenarioState {
  return {
    ...(scenario ?? createScenarioState("manual-gear", "idle", lastAction, tier)),
    diagnosticsTier: tier,
    lastAction,
  };
}

export function emptyScenarioProof(): ScenarioProofArtifacts {
  return {
    proofSchemaVersion: null,
    schemaDigest: null,
    registryBundleDigest: null,
    loweredStrategyBundleDigest: null,
    semanticsDigest: null,
    mergePlanDigest: null,
    mergeResultDigest: null,
    lineageDigest: null,
    mergedBranchStateDigest: null,
    replayedLoweredStrategyBundleDigest: null,
    replayedMergePlanDigest: null,
    replayedMergeResultDigest: null,
    replayedLineageDigest: null,
    replayBranchStateDigest: null,
    replayParity: null,
    replayMismatchClasses: [],
  };
}

export function timelineKindForLabel(label: string): TimelineKind {
  if (label === "branch") return "branch";
  if (label === "merge") return "merge";
  return "normal";
}

export function primaryNodeForLabel(label: string): string {
  switch (label) {
    case "scenario-main-topology":
    case "scenario-feature-topology":
      return "gearTopologyModel";
    case "scenario-main-render":
    case "scenario-feature-render":
      return "lightingModel";
    case "teeth":
      return "gearToothModel::tooth-0";
    case "outer":
    case "inner":
    case "thickness":
    case "rotation":
      return "gearMeshModel";
    case "light":
      return "lightingModel";
    case "boot":
      return "gearMeshModel";
    default:
      return "hudModel";
  }
}

export function touchedNodesForLabel(label: string, state: SceneState): string[] {
  const toothNodes = Array.from(
    { length: Math.min(state.gear.teeth, 6) },
    (_, index) => `gearToothModel::tooth-${index}`,
  );
  switch (label) {
    case "scenario-main-topology":
    case "scenario-feature-topology":
      return [
        "gearTeeth",
        "gearOuterRadius",
        "gearInnerRadius",
        ...toothNodes,
        "gearDimensionsModel",
        "gearProfileModel",
        "gearTopologyModel",
        "gearMeshModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "hudModel",
      ];
    case "scenario-main-render":
    case "scenario-feature-render":
      return [
        "gearRotation",
        "lightIntensity",
        "lightingModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "gearMeshModel",
        "hudModel",
      ];
    case "boot":
      return [
        "gearDimensionsModel",
        "gearProfileModel",
        "gearTopologyModel",
        "gearMeshModel",
        ...toothNodes,
        "lightingModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "hudModel",
      ];
    case "branch":
    case "merge":
      return ["hudModel", "viewportProjectionModel", "viewportShadingModel"];
    case "teeth":
      return [
        "gearTeeth",
        ...toothNodes,
        "gearDimensionsModel",
        "gearProfileModel",
        "gearTopologyModel",
        "gearMeshModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "hudModel",
      ];
    case "outer":
      return [
        "gearOuterRadius",
        "gearDimensionsModel",
        "gearProfileModel",
        "gearTopologyModel",
        "gearMeshModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "hudModel",
      ];
    case "inner":
      return [
        "gearInnerRadius",
        "gearDimensionsModel",
        "gearProfileModel",
        "gearTopologyModel",
        "gearMeshModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "hudModel",
      ];
    case "thickness":
      return [
        "gearThickness",
        "gearDimensionsModel",
        "gearTopologyModel",
        "gearMeshModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "hudModel",
      ];
    case "rotation":
      return [
        "gearRotation",
        "gearDimensionsModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "hudModel",
      ];
    case "light":
      return [
        "lightIntensity",
        "lightingModel",
        "viewportProjectionModel",
        "viewportShadingModel",
        "hudModel",
      ];
    default:
      return ["hudModel"];
  }
}

export function findLastTimelineIndex(
  timeline: Array<{ label: string }>,
  label: string,
): number {
  for (let index = timeline.length - 1; index >= 0; index -= 1) {
    if (timeline[index]?.label === label) {
      return index;
    }
  }
  return -1;
}

export function displayBranchSetSnapshot(
  branches: Array<{ name: string; state: SceneState }>,
): Array<{ name: string; state: SceneState }> {
  return branches.map((branch) => ({
    name: branch.name,
    state: structuredClone(branch.state),
  }));
}

export function branchFrameIds(branches: Map<BranchId, unknown>): BranchId[] {
  return Array.from(branches.keys());
}
