import type { BranchId, DiagnosticsTier, ScenePatch, ScenarioMode } from "../gear-scene/core/types";
import type { ReviewManualSelections, WorkerCommand } from "../gear-scene/worker/protocol";
import { demoSession } from "./demo_session";
import {
  closeWalkthrough,
  nextWalkthrough,
  openWalkthrough,
  prevWalkthrough,
  setDiagnosticsTier as writeDiagnosticsTier,
  setReviewManualSelections as writeReviewManualSelections,
  setReviewPolicyLane as writeReviewPolicyLane,
  setTracedNode as writeTracedNode,
  toggleControls,
} from "./ui_mutations";

export const demoActions = {
  command(command: WorkerCommand) {
    demoSession.command(command);
  },
  branch() {
    demoSession.command({ type: "branch" });
  },
  mergeNow() {
    demoSession.command({ type: "merge" });
  },
  activateBranch(branchId: BranchId) {
    demoSession.command({ type: "activateBranch", branchId });
  },
  applyScenePatch(branchId: BranchId, patch: ScenePatch, label?: string) {
    demoSession.command({ type: "setScenePatch", branchId, patch, label });
  },
  inspectNode(branchId: BranchId, nodeId: string) {
    const app = demoSession.getApp();
    if (!app) return;
    writeTracedNode(app, nodeId);
    demoSession.command({ type: "inspectNode", branchId, nodeId });
  },
  setTracedNode(nodeId: string | null) {
    const app = demoSession.getApp();
    if (!app) return;
    writeTracedNode(app, nodeId);
  },
  scrub(index: number) {
    demoSession.command({ type: "scrub", index });
  },
  jumpToTrace(index: number) {
    demoSession.jumpToTrace(index);
  },
  toggleControls() {
    const app = demoSession.getApp();
    if (!app) return;
    toggleControls(app);
  },
  openWalkthrough() {
    const app = demoSession.getApp();
    if (!app) return;
    openWalkthrough(app);
  },
  closeWalkthrough() {
    const app = demoSession.getApp();
    if (!app) return;
    closeWalkthrough(app);
  },
  nextWalkthrough(maxIndex: number) {
    const app = demoSession.getApp();
    if (!app) return;
    nextWalkthrough(app, maxIndex);
  },
  prevWalkthrough() {
    const app = demoSession.getApp();
    if (!app) return;
    prevWalkthrough(app);
  },
  setReviewPolicyLane(lane: string) {
    const app = demoSession.getApp();
    if (!app) return;
    writeReviewPolicyLane(app, lane);
  },
  setReviewManualSelections(selections: ReviewManualSelections) {
    const app = demoSession.getApp();
    if (!app) return;
    writeReviewManualSelections(app, selections);
    demoSession.command({ type: "setReviewManualSelections", selections });
  },
  setScenarioMode(mode: ScenarioMode) {
    demoSession.command({ type: "setScenarioMode", mode });
  },
  setDiagnosticsTier(tier: DiagnosticsTier) {
    const app = demoSession.getApp();
    if (!app) return;
    writeDiagnosticsTier(app, tier);
    demoSession.getWorkerClient()?.post({ type: "setDiagnosticsTier", tier });
  },
  runScenario() {
    demoSession.command({ type: "runAdversarialMergeScenario" });
  },
  planScenarioMerge() {
    demoSession.command({ type: "planScenarioMerge" });
  },
  executeScenarioMerge() {
    demoSession.command({ type: "executeScenarioMerge" });
  },
  replayScenarioMerge() {
    demoSession.command({ type: "replayScenarioMerge" });
  },
  getFrame(branchId: BranchId) {
    return demoSession.getFrame(branchId);
  },
  getReviewFrame(frameId: string) {
    return demoSession.getReviewFrame(frameId);
  },
};
