import type {
  MergePlan,
  MergeResult,
  ScenePatch,
  SceneState,
  HudModel,
  BranchId,
  BranchInspect,
  DiagnosticsTier,
  ScenarioMode,
  ScenarioState,
} from "../core/types";
import type { RunSummary } from "@WORTH/signal";

export type BranchFrame = {
  branchId: BranchId;
  width: number;
  height: number;
  bitmap: ImageBitmap;
};

export type ReviewFrame = {
  id: string;
  width: number;
  height: number;
  bitmap: ImageBitmap;
};

export type BranchSummary = {
  id: BranchId;
  name: string;
  state: SceneState;
  hud: HudModel;
};

export type TimelineEntry = {
  id: string;
  parentIds: string[];
  branchName: string | null;
  kind: "normal" | "branch" | "merge";
  label: string;
  frameIndex: number;
  activeBranchName: string | null;
  branchCount: number;
  snapshotId?: number | null;
  primaryNode: string;
  touchedNodes: string[];
};

export type MergeReviewPolicyPreview = {
  id: string;
  label: string;
  accent: string;
  description: string;
  plan: MergePlan | null;
  frameId: string | null;
  resultState: SceneState | null;
  visualMode: "rendered" | "manual-review";
};

export type ReviewManualSelections = {
  teeth: "source" | "target";
  outerRadius: "source" | "target";
  innerRadius: "source" | "target";
  thickness: "source" | "target";
  lightIntensity: "source" | "target";
  lightPosition: "source" | "target";
  rotation: "source" | "target";
  camera: "source" | "target";
};

export type MergeReviewSnapshot = {
  source: BranchSummary;
  target: BranchSummary;
  merged: BranchSummary;
  sourceFrameId: string;
  targetFrameId: string;
  mergedFrameId: string;
  previews: MergeReviewPolicyPreview[];
};

export type WorkerSnapshot = {
  ready: boolean;
  graphNodes: number;
  branches: BranchSummary[];
  activeBranchId: BranchId | null;
  latestSummary: RunSummary | null;
  mergePlan: MergePlan | null;
  mergeResult: MergeResult | null;
  timeline: TimelineEntry[];
  timelineIndex: number;
  inspect: BranchInspect | null;
  mergeReview: MergeReviewSnapshot | null;
  scenario: ScenarioState | null;
  error: string | null;
  debugStatus?: string | null;
};

export type WorkerDebugEvent = {
  type: "debug";
  phase: string;
  detail?: string;
  elapsedMs?: number;
};

export type WorkerCommand =
  | { type: "init" }
  | { type: "setInputs"; pressed: string[] }
  | { type: "look"; deltaX: number; deltaY: number }
  | { type: "branch" }
  | { type: "merge" }
  | { type: "runAdversarialMergeScenario" }
  | { type: "planScenarioMerge" }
  | { type: "executeScenarioMerge" }
  | { type: "replayScenarioMerge" }
  | { type: "setScenarioMode"; mode: ScenarioMode }
  | { type: "setDiagnosticsTier"; tier: DiagnosticsTier }
  | { type: "setReviewManualSelections"; selections: ReviewManualSelections }
  | { type: "activateBranch"; branchId: BranchId }
  | { type: "inspectNode"; branchId: BranchId; nodeId: string }
  | { type: "scrub"; index: number }
  | { type: "setScenePatch"; branchId: BranchId; patch: ScenePatch; label?: string };

export type WorkerEvent =
  | { type: "snapshot"; snapshot: WorkerSnapshot; frames: BranchFrame[]; reviewFrames: ReviewFrame[] }
  | { type: "error"; error: string }
  | WorkerDebugEvent;
