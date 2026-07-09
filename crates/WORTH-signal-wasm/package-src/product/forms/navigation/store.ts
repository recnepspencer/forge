import { stableValueDigest } from "../values/value_paths.js";
import { createNavigationContext, resolveControllerLocalNavigation } from "./semantics.js";

export function createNavigationStore() {
  let nextArtifactId = 1;
  let currentStepId = null;
  let visitedStepIds = [];
  let skippedStepIds = [];
  const history = [];
  return Object.freeze({
    report(stepArtifacts) {
      const context = createNavigationContext(stepArtifacts, currentStepId, visitedStepIds, skippedStepIds);
      currentStepId = context.currentStepId;
      visitedStepIds = [...context.visitedStepIds];
      skippedStepIds = [...context.skippedStepIds];
      return context;
    },
    applyStepAction(actionPlan, stepArtifacts) {
      const context = createNavigationContext(stepArtifacts, currentStepId, visitedStepIds, skippedStepIds);
      currentStepId = context.currentStepId;
      visitedStepIds = [...context.visitedStepIds];
      skippedStepIds = [...context.skippedStepIds];
      const transition = resolveControllerLocalNavigation(actionPlan.step, context, actionPlan.id);
      const artifact = navigationArtifact(nextArtifactId++, actionPlan, transition);
      history.push(artifact);
      if (transition.resultKind === "navigated") {
        currentStepId = transition.toStepId;
        visitedStepIds = [...transition.visitedStepIds];
        skippedStepIds = [...transition.skippedStepIds];
      }
      return artifact;
    },
    history() {
      return Object.freeze([...history]);
    },
  });
}

function navigationArtifact(artifactId, actionPlan, transition) {
  const artifact = {
    kind: "navigationTransition",
    artifactId,
    observedAtMs: Date.now(),
    action: actionPlan.id,
    command: actionPlan.step.command,
    stepId: actionPlan.step.stepId,
    routeCoupled: actionPlan.step.routeCoupled,
    resultKind: transition.resultKind,
    fromStepId: transition.fromStepId ?? null,
    toStepId: transition.toStepId ?? null,
    visitedStepIds: transition.visitedStepIds ?? Object.freeze([]),
    skippedStepIds: transition.skippedStepIds ?? Object.freeze([]),
    blockers: transition.blockers ?? Object.freeze([]),
    reason: transition.reason,
    token: transition.token ?? null,
  };
  return Object.freeze({
    ...artifact,
    navigationDigest: stableValueDigest(artifact),
  });
}
