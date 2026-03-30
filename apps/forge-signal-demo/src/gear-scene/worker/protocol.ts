import type { MergePlan, MergeResult, ScenePatch, SceneState, HudModel, BranchId, BranchInspect } from "../core/types";
import type { RunSummary } from "@forge/signal";

export type BranchFrame = {
  branchId: BranchId;
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
  label: string;
  frameIndex: number;
  activeBranchName: string | null;
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
  | { type: "activateBranch"; branchId: BranchId }
  | { type: "scrub"; index: number }
  | { type: "setScenePatch"; patch: ScenePatch; label?: string };

export type WorkerEvent =
  | { type: "snapshot"; snapshot: WorkerSnapshot; frames: BranchFrame[] }
  | { type: "error"; error: string }
  | WorkerDebugEvent;
