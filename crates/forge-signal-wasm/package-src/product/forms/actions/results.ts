import { stableValueDigest } from "../values/value_paths.js";
import { recoveryActionsForBlockers } from "./recovery.js";

export function createActionAttemptStore() {
  let nextAttemptId = 1;
  const history = [];
  return Object.freeze({
    attempt(plan) {
      const duplicate = latestAttemptForPlan(history, plan);
      const result = nextActionResult(plan, duplicate, nextAttemptId);
      nextAttemptId += 1;
      if (result.supersededAttemptId !== undefined) {
        history.push(actionResultArtifact({
          attemptId: nextAttemptId,
          plan,
          resultKind: "superseded",
          reason: "action attempt was superseded by a newer attempt",
          supersededAttemptId: result.supersededAttemptId,
          supersededByAttemptId: result.attemptId,
        }));
        nextAttemptId += 1;
      }
      history.push(result);
      return result;
    },
    history() {
      return Object.freeze([...history]);
    },
  });
}

function nextActionResult(plan, duplicate, attemptId) {
  if (!plan.readiness.canRun) {
    return actionResultArtifact({
      attemptId,
      plan,
      resultKind: "denied",
      reason: "action plan is not ready",
      blockers: plan.readiness.blockers,
    });
  }
  if (isInertEmptyPatchAction(plan)) {
    return actionResultArtifact({
      attemptId,
      plan,
      resultKind: "noOp",
      reason: "action plan has no semantic patch to apply",
    });
  }
  if (duplicate) {
    return repeatedAttemptResult(plan, duplicate, attemptId);
  }
  return actionResultArtifact({
    attemptId,
    plan,
    resultKind: "accepted",
    reason: "action plan accepted",
  });
}

function isInertEmptyPatchAction(plan) {
  return (
    plan.patch.empty &&
    plan.patch.policy === "allowEmpty" &&
    plan.effectPolicy === "none" &&
    plan.hostEffect === null &&
    plan.kind !== "step"
  );
}

function repeatedAttemptResult(plan, duplicate, attemptId) {
  if (plan.idempotency === "collapse") {
    return actionResultArtifact({
      attemptId,
      plan,
      resultKind: "noOp",
      reason: "matching action attempt collapsed into the existing attempt",
      collapsedIntoAttemptId: duplicate.attemptId,
      repeatedAttempt: "collapse",
    });
  }
  if (plan.idempotency === "deny") {
    const blocker = Object.freeze({
      kind: "idempotency:duplicate",
      action: plan.id,
      reason: "matching action attempt is already recorded",
    });
    return actionResultArtifact({
      attemptId,
      plan,
      resultKind: "denied",
      reason: blocker.reason,
      blockers: Object.freeze([blocker]),
      repeatedAttempt: "deny",
    });
  }
  if (plan.idempotency === "queue") {
    return actionResultArtifact({
      attemptId,
      plan,
      resultKind: "accepted",
      reason: "matching action attempt queued after the existing attempt",
      queuePosition: queuedAttemptCount(plan, duplicate),
      repeatedAttempt: "queue",
    });
  }
  if (plan.idempotency === "supersede") {
    return actionResultArtifact({
      attemptId,
      plan,
      resultKind: "accepted",
      reason: "matching action attempt supersedes the previous attempt",
      supersededAttemptId: duplicate.attemptId,
      repeatedAttempt: "supersede",
    });
  }
  return actionResultArtifact({
    attemptId,
    plan,
    resultKind: "accepted",
    reason: "action plan accepted as a repeated independent attempt",
  });
}

function actionResultArtifact(options) {
  const blockers = options.blockers ?? Object.freeze([]);
  const artifact = {
    kind: "actionResult",
    attemptId: options.attemptId,
    action: options.plan.id,
    actionKind: options.plan.kind,
    resultKind: options.resultKind,
    planDigest: options.plan.planDigest,
    idempotency: options.plan.idempotency,
    destructive: options.plan.destructive,
    reason: options.reason,
    blockers,
    recoveryActions: recoveryActionsForBlockers(blockers),
    proof: options.plan.proof,
    patch: options.plan.patch,
    collapsedIntoAttemptId: options.collapsedIntoAttemptId,
    supersededAttemptId: options.supersededAttemptId,
    supersededByAttemptId: options.supersededByAttemptId,
    queuePosition: options.queuePosition,
    repeatedAttempt: options.repeatedAttempt ?? "none",
  };
  return Object.freeze({
    ...artifact,
    resultDigest: stableValueDigest(artifact),
  });
}

function latestAttemptForPlan(history, plan) {
  return [...history]
    .reverse()
    .find((attempt) => (
      attempt.action === plan.id &&
      attempt.planDigest === plan.planDigest &&
      attempt.resultKind !== "denied" &&
      attempt.resultKind !== "noOp" &&
      attempt.resultKind !== "superseded"
    ));
}

function queuedAttemptCount(plan, duplicate) {
  return duplicate.queuePosition === undefined ? 1 : duplicate.queuePosition + 1;
}
