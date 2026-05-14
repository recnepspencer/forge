import { stableValueDigest } from "../values/value_paths.js";

const NAVIGABLE_STEP_POSTURES = new Set(["active", "optional", "blocked", "skipped"]);

export function createNavigationContext(stepArtifacts, currentStepId, visitedStepIds, skippedStepIds) {
  const localSteps = stepArtifacts
    .filter((step) => step.routeCoupled !== true)
    .slice()
    .sort((left, right) => left.order - right.order);
  const localStepIds = localSteps.map((step) => step.id);
  const current = resolveCurrentStepId(localSteps, currentStepId);
  const visited = sanitizeStepIdSet(localStepIds, visitedStepIds);
  if (current !== null) {
    visited.add(current);
  }
  const skipped = sanitizeStepIdSet(localStepIds, skippedStepIds);
  return Object.freeze({
    currentStepId: current,
    localSteps,
    localStepIds: Object.freeze(localStepIds),
    visitedStepIds: Object.freeze([...visited]),
    skippedStepIds: Object.freeze([...skipped]),
  });
}

export function controllerLocalNavigationBlockers(stepAction, context, actionId) {
  if (!stepAction || stepAction.routeCoupled === true) {
    return Object.freeze([]);
  }
  if (stepAction.command === "custom") {
    return Object.freeze([navigationBlocker("action:deferred", actionId, "custom controller-local step actions remain deferred until custom navigation integration exists")]);
  }
  if (stepAction.command === "next" || stepAction.command === "back" || stepAction.command === "skip") {
    if (context.currentStepId !== stepAction.stepId) {
      return Object.freeze([navigationBlocker("navigation:notCurrentStep", actionId, `step action ${actionId} can only run while ${stepAction.stepId} is the current controller-local step`)]);
    }
  }
  if (stepAction.command === "next" || stepAction.command === "skip") {
    if (nextLocalStepId(context, stepAction.stepId) === null) {
      return Object.freeze([navigationBlocker("navigation:noNextStep", actionId, `step action ${actionId} has no next controller-local step to navigate to`)]);
    }
    return Object.freeze([]);
  }
  if (stepAction.command === "back") {
    if (previousVisitedStepId(context, stepAction.stepId) === null) {
      return Object.freeze([navigationBlocker("navigation:noBackStep", actionId, `step action ${actionId} has no previous visited controller-local step to navigate to`)]);
    }
    return Object.freeze([]);
  }
  const target = stepArtifactFor(context, stepAction.stepId);
  if (!target) {
    return Object.freeze([navigationBlocker("navigation:unavailableTarget", actionId, `step action ${actionId} targets an undeclared controller-local step`)]);
  }
  if (target.posture === "removed") {
    return Object.freeze([navigationBlocker("navigation:removedTarget", actionId, `step action ${actionId} targets a removed controller-local step`)]);
  }
  if (target.posture === "unavailable") {
    return Object.freeze([navigationBlocker("navigation:unavailableTarget", actionId, `step action ${actionId} targets an unavailable controller-local step`)]);
  }
  return Object.freeze([]);
}

export function resolveControllerLocalNavigation(stepAction, context, actionId) {
  const blockers = controllerLocalNavigationBlockers(stepAction, context, actionId);
  if (blockers.length > 0) {
    return Object.freeze({
      resultKind: "blocked",
      blockers,
      reason: blockers[0].reason,
    });
  }
  const fromStepId = context.currentStepId;
  const visited = new Set(context.visitedStepIds);
  const skipped = new Set(context.skippedStepIds);
  let toStepId = null;
  let reason = "";
  if (stepAction.command === "next") {
    toStepId = nextLocalStepId(context, stepAction.stepId);
    reason = `${actionId} navigated to the next controller-local step`;
  } else if (stepAction.command === "back") {
    toStepId = previousVisitedStepId(context, stepAction.stepId);
    reason = `${actionId} navigated to the previous visited controller-local step`;
  } else if (stepAction.command === "jump") {
    toStepId = stepAction.stepId;
    reason = `${actionId} jumped to ${stepAction.stepId}`;
  } else if (stepAction.command === "skip") {
    skipped.add(stepAction.stepId);
    toStepId = nextLocalStepId({
      ...context,
      skippedStepIds: Object.freeze([...skipped]),
    }, stepAction.stepId);
    reason = `${actionId} skipped ${stepAction.stepId} and advanced to the next controller-local step`;
  } else if (stepAction.command === "revisit") {
    skipped.delete(stepAction.stepId);
    toStepId = stepAction.stepId;
    reason = `${actionId} revisited ${stepAction.stepId}`;
  } else {
    return Object.freeze({
      resultKind: "blocked",
      blockers,
      reason: `${actionId} has no admitted controller-local navigation semantics`,
    });
  }
  if (toStepId !== null) {
    visited.add(toStepId);
  }
  const transition = {
    fromStepId,
    toStepId,
    visitedStepIds: Object.freeze([...visited]),
    skippedStepIds: Object.freeze([...skipped]),
    reason,
    token: stableValueDigest({
      actionId,
      command: stepAction.command,
      fromStepId,
      toStepId,
      visitedStepIds: [...visited],
      skippedStepIds: [...skipped],
    }),
  };
  return Object.freeze({
    resultKind: "navigated",
    ...transition,
  });
}

function navigationBlocker(kind, action, reason) {
  return Object.freeze({ kind, action, reason });
}

function resolveCurrentStepId(localSteps, currentStepId) {
  if (currentStepId !== null && localSteps.some((step) => step.id === currentStepId && NAVIGABLE_STEP_POSTURES.has(step.posture))) {
    return currentStepId;
  }
  return localSteps.find((step) => NAVIGABLE_STEP_POSTURES.has(step.posture))?.id ?? null;
}

function sanitizeStepIdSet(localStepIds, stepIds) {
  const allowed = new Set(localStepIds);
  return new Set((stepIds ?? []).filter((stepId) => allowed.has(stepId)));
}

function stepArtifactFor(context, stepId) {
  return context.localSteps.find((step) => step.id === stepId) ?? null;
}

function nextLocalStepId(context, stepId) {
  const index = context.localSteps.findIndex((step) => step.id === stepId);
  if (index < 0) {
    return null;
  }
  for (let cursor = index + 1; cursor < context.localSteps.length; cursor += 1) {
    const candidate = context.localSteps[cursor];
    if (candidate.posture === "removed" || candidate.posture === "unavailable") {
      continue;
    }
    if (context.skippedStepIds.includes(candidate.id)) {
      continue;
    }
    return candidate.id;
  }
  return null;
}

function previousVisitedStepId(context, stepId) {
  const index = context.localSteps.findIndex((step) => step.id === stepId);
  if (index < 0) {
    return null;
  }
  for (let cursor = index - 1; cursor >= 0; cursor -= 1) {
    const candidate = context.localSteps[cursor];
    if (context.visitedStepIds.includes(candidate.id)) {
      return candidate.id;
    }
  }
  return null;
}
