import { demoActions } from "./demo_actions";
import { demoReads } from "./demo_reads";
import { useDemoSignal } from "./use_demo_signal";

export { useDemoSignal };

export const demoState = {
  branches: {
    branches: demoReads.branches,
    activeBranch: demoReads.activeBranch,
    hasFeatureBranch: demoReads.hasFeatureBranch,
    graphNodes: demoReads.graphNodes,
    latestSummary: demoReads.latestSummary,
    suppressionPercent: demoReads.suppressionPercent,
    frameVersion: demoReads.frameVersion,
  },
  controls: {
    controlsOpen: demoReads.controlsOpen,
    walkthroughOpen: demoReads.walkthroughOpen,
    walkthroughIndex: demoReads.walkthroughIndex,
    toggleControls: demoActions.toggleControls,
    openWalkthrough: demoActions.openWalkthrough,
    closeWalkthrough: demoActions.closeWalkthrough,
    nextWalkthrough: demoActions.nextWalkthrough,
    prevWalkthrough: demoActions.prevWalkthrough,
  },
  inspection: {
    inspect: demoReads.inspect,
    tracedNode: demoReads.tracedNode,
    setTracedNode: demoActions.setTracedNode,
    inspectNode: demoActions.inspectNode,
    jumpToTrace: demoActions.jumpToTrace,
  },
  merge: {
    mergePlan: demoReads.mergePlan,
    mergeResult: demoReads.mergeResult,
    mergeNow: demoActions.mergeNow,
  },
  scenario: {
    scenario: demoReads.scenario,
    diagnosticsTier: demoReads.diagnosticsTier,
    setMode: demoActions.setScenarioMode,
    setDiagnosticsTier: demoActions.setDiagnosticsTier,
    run: demoActions.runScenario,
    plan: demoActions.planScenarioMerge,
    execute: demoActions.executeScenarioMerge,
    replay: demoActions.replayScenarioMerge,
  },
  status: {
    error: demoReads.error,
    debugStatus: demoReads.debugStatus,
  },
  timeline: {
    timeline: demoReads.timeline,
    timelineIndex: demoReads.timelineIndex,
    scrub: demoActions.scrub,
  },
  transport: {
    command: demoActions.command,
    activateBranch: demoActions.activateBranch,
    inspectNode: demoActions.inspectNode,
    scrub: demoActions.scrub,
    applyScenePatch: demoActions.applyScenePatch,
    branch: demoActions.branch,
    merge: demoActions.mergeNow,
    runScenario: demoActions.runScenario,
    planScenarioMerge: demoActions.planScenarioMerge,
    executeScenarioMerge: demoActions.executeScenarioMerge,
    replayScenarioMerge: demoActions.replayScenarioMerge,
    setScenarioMode: demoActions.setScenarioMode,
    setDiagnosticsTier: demoActions.setDiagnosticsTier,
    getFrame: demoActions.getFrame,
  },
};
