import { tx, type SignalApp } from "@forge/signal";

import type { WorkerSnapshot } from "../gear-scene/worker/protocol";
import { normalizeWorkerSnapshot } from "./shell_signal_normalization";

function computeSuppressionPercent(summary: { nodesEvaluated: number } | null, graphNodes: number): string {
  if (!summary || graphNodes === 0) return "0.0";
  const untouched = Math.max(graphNodes - summary.nodesEvaluated, 0);
  return ((untouched / graphNodes) * 100).toFixed(1);
}

export function applySnapshotToShellSignals(app: SignalApp, snapshot: WorkerSnapshot) {
  const normalized = normalizeWorkerSnapshot(snapshot);
  const activeBranch =
    normalized.branches.find((branch) => branch.id === normalized.activeBranchId)
    ?? normalized.branches[0]
    ?? null;
  const hasFeatureBranch = normalized.branches.some((branch) => branch.name === "what-if");
  const suppressionPercent = computeSuppressionPercent(normalized.latestSummary, normalized.graphNodes);

  app.batch([
    tx.set("uiReady", normalized.ready),
    tx.set("uiGraphNodes", normalized.graphNodes),
    tx.set("uiBranches", normalized.branches),
    tx.set("uiActiveBranch", activeBranch),
    tx.set("uiHasFeatureBranch", hasFeatureBranch),
    tx.set("uiActiveBranchId", normalized.activeBranchId),
    tx.set("uiLatestSummary", normalized.latestSummary),
    tx.set("uiSuppressionPercent", suppressionPercent),
    tx.set("uiMergePlan", normalized.mergePlan),
    tx.set("uiMergeResult", normalized.mergeResult),
    tx.set("uiTimeline", normalized.timeline),
    tx.set("uiTimelineIndex", normalized.timelineIndex),
    tx.set("uiInspect", normalized.inspect),
    tx.set("uiScenario", normalized.scenario),
    tx.set("uiError", normalized.error),
    tx.set("uiDebugStatus", normalized.debugStatus ?? null),
  ]);
}
