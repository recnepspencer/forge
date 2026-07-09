import { stableValueDigest } from "../values/value_paths.js";
import { settlementTimedOut } from "./policy_timing.js";

export function collaborationLane(policy, collaboration, settlements, nowMs) {
  if (collaboration.declared) {
    if (collaboration.posture === "unavailable") {
      return baseLane("collaboration", "collaboration", policy.scope, policy, "unavailable", {
        reason: collaboration.reason,
        token: collaboration.digest,
        acknowledgementRequired: policy.unavailableAcknowledgement === "required",
      });
    }
    if (collaboration.posture === "settling") {
      return acknowledgedOrSettlingLane(
        "collaboration",
        "collaboration",
        policy,
        collaboration.branchId ?? collaboration.lockOwnerId ?? "collaboration",
        collaboration.remoteUpdateDigest ?? collaboration.digest,
        settlements,
        nowMs,
        collaboration.remoteUpdateDigest ?? collaboration.digest,
      );
    }
    if (collaboration.posture === "blocked") {
      return baseLane("collaboration", "collaboration", policy.scope, policy, "busy", {
        reason: collaboration.reason,
        target: collaboration.lockOwnerId ?? collaboration.branchId,
        token: collaboration.digest,
      });
    }
    return baseLane("collaboration", "collaboration", policy.scope, policy, "ready", {
      reason: collaboration.reason,
      target: collaboration.branchId ?? null,
      token: collaboration.digest,
    });
  }
  return externalLane("collaboration", policy, settlements, nowMs);
}

export function externalLane(lane, policy, settlements, nowMs) {
  const laneId = lane;
  const current = settlements.externalLane(laneId);
  if (!current) {
    return baseLane(laneId, lane, policy.scope, policy, "ready", {
      reason: `${lane} presentation has no pending visible work`,
    });
  }
  if (current.status === "settling") {
    return acknowledgedOrSettlingLane(
      laneId,
      lane,
      policy,
      current.target ?? lane,
      current.token,
      settlements,
      nowMs,
      current.observedAtMs,
    );
  }
  return baseLane(laneId, lane, policy.scope, policy, current.status, {
    target: current.target,
    token: current.token,
    reason: current.reason,
    acknowledgementRequired: current.status === "unavailable"
      ? policy.unavailableAcknowledgement === "required"
      : false,
  });
}

export function navigationLanes(policy, navigation, actionDeclarations, stepDeclarations, settlements, nowMs) {
  const lanes = [
    controllerLocalNavigationLane(policy, navigation, settlements, nowMs),
    ...stepDeclarations
      .filter((step) => step.routeCoupled === true)
      .map((step) =>
        baseLane(`navigation:step:${step.id}`, "navigation", "step", policy, "unavailable", {
          target: step.id,
          reason: "route-coupled step presentation requires route authority outside controller-local navigation",
          token: stableValueDigest({ step: step.id, routeCoupled: true }),
          acknowledgementRequired: policy.unavailableAcknowledgement === "required",
        })),
    ...actionDeclarations
      .filter((action) => action.step?.routeCoupled === true)
      .map((action) =>
        baseLane(`navigation:action:${action.id}`, "navigation", policy.scope, policy, "unavailable", {
          target: action.id,
          reason: "route-coupled step action presentation requires route authority outside controller-local navigation",
          token: stableValueDigest({ action: action.id, routeCoupled: true }),
          acknowledgementRequired: policy.unavailableAcknowledgement === "required",
        })),
  ];
  return lanes.filter(Boolean);
}

export function exitLane(policy, exit, settlements, nowMs) {
  if (exit.current) {
    if (exit.current.status === "settling") {
      return acknowledgedOrSettlingLane(
        "exit",
        "exit",
        policy,
        exit.current.target ?? "exit",
        exit.current.token,
        settlements,
        nowMs,
        exit.current.observedAtMs,
      );
    }
    return baseLane("exit", "exit", policy.scope, policy, exit.current.status, {
      target: exit.current.target,
      reason: exit.current.reason,
      token: exit.current.token,
      acknowledgementRequired: exit.current.status === "unavailable"
        ? policy.unavailableAcknowledgement === "required"
        : false,
    });
  }
  if (exit.summary.status === "unavailable") {
    return baseLane("exit", "exit", policy.scope, policy, "unavailable", {
      target: exit.summary.activeTarget,
      reason: exit.summary.unavailableReason ?? "exit presentation is unavailable because form truth is unresolved",
      token: exit.digest,
      acknowledgementRequired: policy.unavailableAcknowledgement === "required",
    });
  }
  if (exit.summary.status === "busy") {
    return baseLane("exit", "exit", policy.scope, policy, "busy", {
      target: exit.summary.activeTarget,
      reason: exit.summary.guardKind === "pendingAction"
        ? "exit presentation is waiting for pending actions to settle"
        : "exit presentation is waiting for dirty draft confirmation",
      token: exit.digest,
    });
  }
  return baseLane("exit", "exit", policy.scope, policy, "ready", {
    reason: "exit presentation is settled",
    token: exit.digest,
  });
}

function controllerLocalNavigationLane(policy, navigation, settlements, nowMs) {
  const latest = navigation.latest;
  if (!latest) {
    return baseLane("navigation:local", "navigation", "step", policy, "ready", {
      target: navigation.current.stepId,
      reason: navigation.current.stepId === null
        ? "controller-local navigation has no active step"
        : `controller-local navigation is settled on ${navigation.current.stepId}`,
      token: navigation.digest,
    });
  }
  if (latest.resultKind === "blocked") {
    return baseLane("navigation:local", "navigation", "step", policy, "failed", {
      target: latest.stepId,
      reason: latest.reason,
      token: latest.navigationDigest,
    });
  }
  return acknowledgedOrSettlingLane(
    "navigation:local",
    "navigation",
    { ...policy, scope: "step" },
    latest.toStepId ?? latest.stepId,
    latest.navigationDigest,
    settlements,
    nowMs,
    latest.observedAtMs,
  );
}

export function acknowledgedOrSettlingLane(id, lane, policy, target, token, settlements, nowMs, settlingObservedAtMs, dependencies = null) {
  if (policy.settlementAcknowledgement !== "required") {
    return baseLane(id, lane, policy.scope, policy, "ready", {
      target,
      token,
      reason: `${target} presentation settled without acknowledgement`,
      dependencies,
    });
  }
  const settlement = settlements.settlementFor(id, token);
  if (!settlement && settlementTimedOut(policy, settlingObservedAtMs, nowMs)) {
    const timedOutSettlement = settlements.timeout({
      id,
      lane,
      scope: policy.scope,
      token,
      acknowledgement: { required: true },
      reason: `${target} presentation settlement timed out`,
    }, `${target} presentation settlement timed out`);
    return baseLane(id, lane, policy.scope, policy, "failed", {
      target,
      token,
      reason: `${target} presentation settlement timed out`,
      settlement: timedOutSettlement,
      acknowledgementRequired: true,
      dependencies,
    });
  }
  if (settlement?.resultKind === "acknowledged") {
    return baseLane(id, lane, policy.scope, policy, "ready", {
      target,
      token,
      reason: `${target} presentation settlement was acknowledged`,
      settlement,
      acknowledgementRequired: true,
      dependencies,
    });
  }
  if (settlement?.resultKind === "timedOut") {
    return baseLane(id, lane, policy.scope, policy, "failed", {
      target,
      token,
      reason: `${target} presentation settlement timed out`,
      settlement,
      acknowledgementRequired: true,
      dependencies,
    });
  }
  return baseLane(id, lane, policy.scope, policy, "settling", {
    target,
    token,
    reason: `${target} semantic fulfillment is complete but visible settlement still needs acknowledgement`,
    acknowledgementRequired: true,
    dependencies,
  });
}

export function baseLane(id, lane, scope, policy, status, options) {
  const settlement = options.settlement ?? null;
  return Object.freeze({
    id,
    lane,
    scope,
    target: options.target ?? null,
    status,
    reason: options.reason,
    token: options.token ?? null,
    policy: Object.freeze(policy),
    acknowledgement: Object.freeze({
      required: options.acknowledgementRequired === true,
      status: settlement?.resultKind ?? "pending",
      settlementDigest: settlement?.settlementDigest ?? null,
    }),
    bootstrap: options.bootstrap ?? null,
    dependencies: options.dependencies ?? null,
  });
}
