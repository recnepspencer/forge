import type { RunSummary } from "@WORTH/signal";

import type {
  BranchInspect,
  DiagnosticsTier,
  MergePlan,
  MergeResult,
  ScenarioState,
} from "../gear-scene/core/types";
import type { ReviewManualSelections, WorkerSnapshot } from "../gear-scene/worker/protocol";

export function createEmptySnapshot(): WorkerSnapshot {
  return {
    ready: false,
    graphNodes: 0,
    branches: [],
    activeBranchId: null,
    latestSummary: null,
    mergePlan: null,
    mergeResult: null,
    timeline: [],
    timelineIndex: 0,
    inspect: null,
    mergeReview: null,
    scenario: null,
    error: null,
    debugStatus: "worker idle",
  };
}

const EMPTY_SNAPSHOT = createEmptySnapshot();

export type DemoShellSignals = {
  graphNodes: number;
  branches: WorkerSnapshot["branches"];
  activeBranch: WorkerSnapshot["branches"][number] | null;
  hasFeatureBranch: boolean;
  activeBranchId: WorkerSnapshot["activeBranchId"];
  latestSummary: RunSummary | null;
  suppressionPercent: string;
  mergePlan: MergePlan | null;
  mergeResult: MergeResult | null;
  timeline: WorkerSnapshot["timeline"];
  timelineIndex: number;
  inspect: BranchInspect | null;
  mergeReview: WorkerSnapshot["mergeReview"];
  scenario: ScenarioState | null;
  error: string | null;
  debugStatus: string | null;
  diagnosticsTier: DiagnosticsTier;
  frameVersion: number;
  tracedNode: string | null;
  controlsOpen: boolean;
  walkthroughOpen: boolean;
  walkthroughIndex: number;
  reviewPolicyLane: string;
  reviewManualSelections: ReviewManualSelections;
};

export type DemoShellSignalKey = keyof DemoShellSignals;

export const SHELL_SIGNAL_BINDINGS: Record<
  DemoShellSignalKey,
  { id: string; initial: DemoShellSignals[DemoShellSignalKey] }
> = {
  graphNodes: { id: "uiGraphNodes", initial: EMPTY_SNAPSHOT.graphNodes },
  branches: { id: "uiBranches", initial: EMPTY_SNAPSHOT.branches },
  activeBranch: { id: "uiActiveBranch", initial: null as WorkerSnapshot["branches"][number] | null },
  hasFeatureBranch: { id: "uiHasFeatureBranch", initial: false },
  activeBranchId: { id: "uiActiveBranchId", initial: EMPTY_SNAPSHOT.activeBranchId },
  latestSummary: { id: "uiLatestSummary", initial: EMPTY_SNAPSHOT.latestSummary as RunSummary | null },
  suppressionPercent: { id: "uiSuppressionPercent", initial: "0.0" },
  mergePlan: { id: "uiMergePlan", initial: EMPTY_SNAPSHOT.mergePlan as MergePlan | null },
  mergeResult: { id: "uiMergeResult", initial: EMPTY_SNAPSHOT.mergeResult as MergeResult | null },
  timeline: { id: "uiTimeline", initial: EMPTY_SNAPSHOT.timeline },
  timelineIndex: { id: "uiTimelineIndex", initial: EMPTY_SNAPSHOT.timelineIndex },
  inspect: { id: "uiInspect", initial: EMPTY_SNAPSHOT.inspect as BranchInspect | null },
  mergeReview: { id: "uiMergeReview", initial: EMPTY_SNAPSHOT.mergeReview },
  scenario: { id: "uiScenario", initial: EMPTY_SNAPSHOT.scenario as ScenarioState | null },
  error: { id: "uiError", initial: EMPTY_SNAPSHOT.error },
  debugStatus: { id: "uiDebugStatus", initial: EMPTY_SNAPSHOT.debugStatus ?? null },
  diagnosticsTier: { id: "uiDiagnosticsTier", initial: "webDevelopment" as DiagnosticsTier },
  frameVersion: { id: "uiFrameVersion", initial: 0 },
  tracedNode: { id: "uiTracedNode", initial: null as string | null },
  controlsOpen: { id: "uiControlsOpen", initial: true },
  walkthroughOpen: { id: "uiWalkthroughOpen", initial: false },
  walkthroughIndex: { id: "uiWalkthroughIndex", initial: 0 },
  reviewPolicyLane: { id: "uiReviewPolicyLane", initial: "current" },
  reviewManualSelections: {
    id: "uiReviewManualSelections",
    initial: {
      teeth: "source",
      outerRadius: "source",
      innerRadius: "source",
      thickness: "source",
      lightIntensity: "source",
      lightPosition: "source",
      rotation: "source",
      camera: "source",
    } as ReviewManualSelections,
  },
};

export const SHELL_SIGNAL_KEYS = Object.keys(SHELL_SIGNAL_BINDINGS) as DemoShellSignalKey[];
