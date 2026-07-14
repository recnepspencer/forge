import type { ScenePatch } from "../gear-scene/core/types";
import { demoState, useDemoSignal } from "./ui_state";

export function useWorkspaceView() {
  const branches = useDemoSignal(demoState.branches.branches);
  const activeBranch = useDemoSignal(demoState.branches.activeBranch);
  const latestSummary = useDemoSignal(demoState.branches.latestSummary);
  const suppression = useDemoSignal(demoState.branches.suppressionPercent);
  const graphNodes = useDemoSignal(demoState.branches.graphNodes);
  const frameVersion = useDemoSignal(demoState.branches.frameVersion);
  const timeline = useDemoSignal(demoState.timeline.timeline);
  const timelineIndex = useDemoSignal(demoState.timeline.timelineIndex);
  const inspect = useDemoSignal(demoState.inspection.inspect);
  const tracedNode = useDemoSignal(demoState.inspection.tracedNode);
  const controlsOpen = useDemoSignal(demoState.controls.controlsOpen);

  function applyActiveBranchPatch(patch: ScenePatch, label: string) {
    if (!activeBranch) return;
    demoState.transport.applyScenePatch(activeBranch.id, patch, label);
  }

  function inspectActiveBranchNode(nodeId: string) {
    if (!activeBranch) return;
    demoState.inspection.inspectNode(activeBranch.id, nodeId);
  }

  return {
    branches,
    activeBranch,
    latestSummary,
    suppression,
    graphNodes,
    frameVersion,
    timeline,
    timelineIndex,
    inspect,
    tracedNode,
    controlsOpen,
    applyActiveBranchPatch,
    inspectActiveBranchNode,
  };
}

export function useReviewView() {
  const mergePlan = useDemoSignal(demoState.merge.mergePlan);
  const mergeResult = useDemoSignal(demoState.merge.mergeResult);
  const mergeReview = useDemoSignal(demoState.merge.mergeReview);
  const walkthroughOpen = useDemoSignal(demoState.controls.walkthroughOpen);
  const walkthroughIndex = useDemoSignal(demoState.controls.walkthroughIndex);
  const reviewPolicyLane = useDemoSignal(demoState.controls.reviewPolicyLane);
  const reviewManualSelections = useDemoSignal(demoState.controls.reviewManualSelections);

  return {
    mergePlan,
    mergeResult,
    mergeReview,
    walkthroughOpen,
    walkthroughIndex,
    reviewPolicyLane,
    reviewManualSelections,
  };
}

export function useScenarioView() {
  const hasFeatureBranch = useDemoSignal(demoState.branches.hasFeatureBranch);
  const scenario = useDemoSignal(demoState.scenario.scenario);
  const diagnosticsTier = useDemoSignal(demoState.scenario.diagnosticsTier);
  const error = useDemoSignal(demoState.status.error);
  const debugStatus = useDemoSignal(demoState.status.debugStatus);

  return {
    hasFeatureBranch,
    scenario,
    diagnosticsTier,
    error,
    debugStatus,
  };
}
