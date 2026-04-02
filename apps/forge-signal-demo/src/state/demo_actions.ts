import type { BranchId, DiagnosticsTier, ScenePatch, ScenarioMode } from "../gear-scene/core/types";
import type { WorkerCommand } from "../gear-scene/worker/protocol";
import { runtimeAccess } from "./runtime_access";

export const demoActions = {
  command(command: WorkerCommand) {
    runtimeAccess.command(command);
  },
  branch() {
    runtimeAccess.command({ type: "branch" });
  },
  mergeNow() {
    runtimeAccess.command({ type: "merge" });
  },
  activateBranch(branchId: BranchId) {
    runtimeAccess.command({ type: "activateBranch", branchId });
  },
  applyScenePatch(branchId: BranchId, patch: ScenePatch, label?: string) {
    runtimeAccess.command({ type: "setScenePatch", branchId, patch, label });
  },
  inspectNode(branchId: BranchId, nodeId: string) {
    runtimeAccess.setTracedNode(nodeId);
    runtimeAccess.command({ type: "inspectNode", branchId, nodeId });
  },
  setTracedNode(nodeId: string | null) {
    runtimeAccess.setTracedNode(nodeId);
  },
  scrub(index: number) {
    runtimeAccess.command({ type: "scrub", index });
  },
  jumpToTrace(index: number) {
    runtimeAccess.jumpToTrace(index);
  },
  toggleControls() {
    runtimeAccess.toggleControls();
  },
  openWalkthrough() {
    runtimeAccess.openWalkthrough();
  },
  closeWalkthrough() {
    runtimeAccess.closeWalkthrough();
  },
  nextWalkthrough(maxIndex: number) {
    runtimeAccess.nextWalkthrough(maxIndex);
  },
  prevWalkthrough() {
    runtimeAccess.prevWalkthrough();
  },
  setScenarioMode(mode: ScenarioMode) {
    runtimeAccess.command({ type: "setScenarioMode", mode });
  },
  setDiagnosticsTier(tier: DiagnosticsTier) {
    runtimeAccess.setDiagnosticsTier(tier);
  },
  runScenario() {
    runtimeAccess.command({ type: "runAdversarialMergeScenario" });
  },
  planScenarioMerge() {
    runtimeAccess.command({ type: "planScenarioMerge" });
  },
  executeScenarioMerge() {
    runtimeAccess.command({ type: "executeScenarioMerge" });
  },
  replayScenarioMerge() {
    runtimeAccess.command({ type: "replayScenarioMerge" });
  },
  getFrame(branchId: BranchId) {
    return runtimeAccess.getFrame(branchId);
  },
};
