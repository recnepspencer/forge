import type { DemoShellSignalKey } from "./shell_signal_schema";
import { demoSession } from "./demo_session";

export type SignalSlice<T> = {
  getSnapshot: () => T;
  subscribe: (listener: () => void) => () => void;
};

function signalSlice<K extends DemoShellSignalKey>(key: K): SignalSlice<ReturnType<typeof demoSession.readSignal<K>>> {
  return {
    getSnapshot: () => demoSession.readSignal(key),
    subscribe: (listener) => demoSession.subscribeSignal(key, listener),
  };
}

export const demoReads = {
  branches: signalSlice("branches"),
  activeBranch: signalSlice("activeBranch"),
  hasFeatureBranch: signalSlice("hasFeatureBranch"),
  graphNodes: signalSlice("graphNodes"),
  latestSummary: signalSlice("latestSummary"),
  suppressionPercent: signalSlice("suppressionPercent"),
  frameVersion: signalSlice("frameVersion"),
  mergePlan: signalSlice("mergePlan"),
  mergeResult: signalSlice("mergeResult"),
  timeline: signalSlice("timeline"),
  timelineIndex: signalSlice("timelineIndex"),
  inspect: signalSlice("inspect"),
  mergeReview: signalSlice("mergeReview"),
  tracedNode: signalSlice("tracedNode"),
  scenario: signalSlice("scenario"),
  diagnosticsTier: signalSlice("diagnosticsTier"),
  error: signalSlice("error"),
  debugStatus: signalSlice("debugStatus"),
  controlsOpen: signalSlice("controlsOpen"),
  walkthroughOpen: signalSlice("walkthroughOpen"),
  walkthroughIndex: signalSlice("walkthroughIndex"),
  reviewPolicyLane: signalSlice("reviewPolicyLane"),
  reviewManualSelections: signalSlice("reviewManualSelections"),
};
