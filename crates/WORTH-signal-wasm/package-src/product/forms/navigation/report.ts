import { stableValueDigest } from "../values/value_paths.js";

export function readNavigationReport(store, stepArtifacts) {
  const state = store.report(stepArtifacts);
  const history = store.history();
  const latest = history.at(-1) ?? null;
  const summary = Object.freeze({
    currentStepId: state.currentStepId,
    localStepIds: Object.freeze(state.localStepIds),
    visitedStepIds: Object.freeze(state.visitedStepIds),
    skippedStepIds: Object.freeze(state.skippedStepIds),
    blockedTransitions: history.filter((entry) => entry.resultKind === "blocked").length,
  });
  const counters = Object.freeze({
    costBasis: "controllerLocalStepNavigationHistoryScan",
    incrementalStatus: "notIncremental",
    localSteps: state.localStepIds.length,
    visitedSteps: state.visitedStepIds.length,
    skippedSteps: state.skippedStepIds.length,
    transitions: history.length,
    blockedTransitions: summary.blockedTransitions,
  });
  return Object.freeze({
    current: Object.freeze({
      stepId: state.currentStepId,
      visitedStepIds: Object.freeze(state.visitedStepIds),
      skippedStepIds: Object.freeze(state.skippedStepIds),
    }),
    latest,
    history,
    summary,
    counters,
    digest: stableValueDigest({ summary, counters, history, latest }),
  });
}
