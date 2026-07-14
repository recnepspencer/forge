import { define, type SignalApp } from "@WORTH/signal";

import type {
  BranchInspect,
  DiagnosticsTier,
  MergePlan,
  MergeResult,
  ScenarioState,
} from "../gear-scene/core/types";
import type { MergeReviewSnapshot, WorkerSnapshot } from "../gear-scene/worker/protocol";
import { createEmptySnapshot } from "./shell_signal_schema";

export function registerShellSignals(app: SignalApp) {
  const empty = createEmptySnapshot();
  app.source(define.source<boolean>("uiReady").initial(empty.ready));
  app.source(define.source<number>("uiGraphNodes").initial(empty.graphNodes));
  app.source(define.source<WorkerSnapshot["branches"]>("uiBranches").initial(empty.branches));
  app.source(define.source<WorkerSnapshot["branches"][number] | null>("uiActiveBranch").initial(null));
  app.source(define.source<boolean>("uiHasFeatureBranch").initial(false));
  app.source(define.source<WorkerSnapshot["activeBranchId"]>("uiActiveBranchId").initial(empty.activeBranchId));
  app.source(define.source("uiLatestSummary").initial(empty.latestSummary));
  app.source(define.source<string>("uiSuppressionPercent").initial("0.0"));
  app.source(define.source<MergePlan | null>("uiMergePlan").initial(empty.mergePlan));
  app.source(define.source<MergeResult | null>("uiMergeResult").initial(empty.mergeResult));
  app.source(define.source<WorkerSnapshot["timeline"]>("uiTimeline").initial(empty.timeline));
  app.source(define.source<number>("uiTimelineIndex").initial(empty.timelineIndex));
  app.source(define.source<BranchInspect | null>("uiInspect").initial(empty.inspect));
  app.source(define.source<MergeReviewSnapshot | null>("uiMergeReview").initial(empty.mergeReview));
  app.source(define.source<ScenarioState | null>("uiScenario").initial(empty.scenario));
  app.source(define.source<string | null>("uiError").initial(empty.error));
  app.source(define.source<string | null>("uiDebugStatus").initial(empty.debugStatus ?? null));
  app.source(define.source<DiagnosticsTier>("uiDiagnosticsTier").initial("webDevelopment"));
  app.source(define.source<number>("uiFrameVersion").initial(0));
  app.source(define.source<string | null>("uiTracedNode").initial(null));
  app.source(define.source<boolean>("uiControlsOpen").initial(true));
  app.source(define.source<boolean>("uiWalkthroughOpen").initial(false));
  app.source(define.source<number>("uiWalkthroughIndex").initial(0));
  app.source(define.source<string>("uiReviewPolicyLane").initial("current"));
  app.source(define.source<{ topology: "source" | "target"; lighting: "source" | "target"; motion: "source" | "target" }>("uiReviewManualSelections").initial({
    topology: "source",
    lighting: "source",
    motion: "source",
  }));
}
