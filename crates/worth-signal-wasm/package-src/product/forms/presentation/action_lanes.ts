import { stableValueDigest } from "../values/value_paths.js";
import { acknowledgedOrSettlingLane, baseLane } from "./auxiliary_lanes.js";
import { busyVisibilityStatus, minimumBusyPending } from "./policy_timing.js";

export function actionLanes(policy, catalog, executions, settlements, nowMs, dependencies) {
  return catalog.map((action) => {
    const latest = [...executions].reverse().find((entry) => entry.action === action.id) ?? null;
    if (!latest) {
      return baseLane(`action:${action.id}`, "action", policy.scope, policy, "ready", {
        target: action.id,
        reason: `${action.id} presentation has no pending visible work`,
      });
    }
    if (latest.resultKind === "pending") {
      const status = busyVisibilityStatus(policy, latest.observedAtMs, nowMs);
      return baseLane(`action:${action.id}`, "action", policy.scope, policy, status, {
        target: action.id,
        reason: status === "pending"
          ? `${latest.reason}; busy reveal is intentionally delayed`
          : latest.reason,
        token: latest.executionDigest,
      });
    }
    const priorPending = [...executions].reverse().find((entry) => (
      entry.action === action.id &&
      entry.operationId === latest.operationId &&
      entry.resultKind === "pending"
    )) ?? null;
    if (priorPending && minimumBusyPending(policy, priorPending.observedAtMs, nowMs)) {
      const status = busyVisibilityStatus(policy, priorPending.observedAtMs, nowMs);
      return baseLane(`action:${action.id}`, "action", policy.scope, policy, status, {
        target: action.id,
        reason: status === "pending"
          ? `${action.id} presentation is delaying busy reveal before minimum busy duration starts`
          : `${action.id} presentation is preserving minimum busy duration`,
        token: latest.executionDigest,
      });
    }
    const dependencySummary = actionDependencySummary(policy.settleOn ?? [], dependencies);
    if (latest.resultKind === "fulfilled") {
      if (dependencySummary && dependencySummary.unavailable.length > 0) {
        return baseLane(`action:${action.id}`, "action", policy.scope, policy, "unavailable", {
          target: action.id,
          reason: `${action.id} presentation cannot satisfy declared settlement dependencies`,
          token: latest.executionDigest,
          acknowledgementRequired: policy.unavailableAcknowledgement === "required",
          dependencies: dependencySummary,
        });
      }
      if (dependencySummary && dependencySummary.blocking.length > 0) {
        return baseLane(`action:${action.id}`, "action", policy.scope, policy, "busy", {
          target: action.id,
          reason: `${action.id} presentation is waiting for declared visible settlement dependencies`,
          token: latest.executionDigest,
          dependencies: dependencySummary,
        });
      }
      return acknowledgedOrSettlingLane(
        `action:${action.id}`,
        "action",
        policy,
        action.id,
        latest.executionDigest,
        settlements,
        nowMs,
        latest.observedAtMs,
        dependencySummary,
      );
    }
    if (latest.resultKind === "rejected" || latest.resultKind === "timedOut" || latest.resultKind === "cancelled") {
      return baseLane(`action:${action.id}`, "action", policy.scope, policy, "failed", {
        target: action.id,
        reason: latest.reason,
        token: latest.executionDigest,
        dependencies: dependencySummary,
      });
    }
    return baseLane(`action:${action.id}`, "action", policy.scope, policy, "ready", {
      target: action.id,
      reason: latest.reason,
      token: latest.executionDigest,
      dependencies: dependencySummary,
    });
  });
}

function actionDependencySummary(requiredDependencies, dependencies) {
  if (requiredDependencies.length === 0) {
    return null;
  }
  const required = Object.freeze(requiredDependencies.map((dependency) => dependencyArtifact(dependency, dependencies)));
  const summary = {
    required,
    blocking: Object.freeze(required.filter((dependency) => (
      dependency.status === "pending" ||
      dependency.status === "busy" ||
      dependency.status === "settling"
    ))),
    unavailable: Object.freeze(required.filter((dependency) => dependency.status === "unavailable")),
    satisfied: Object.freeze(required.filter((dependency) => dependency.status === "ready")),
  };
  return Object.freeze({
    ...summary,
    digest: stableValueDigest(summary),
  });
}

function dependencyArtifact(dependency, dependencies) {
  switch (dependency) {
    case "canonicalization":
      return laneDependencyArtifact(dependency, dependencies.canonicalization);
    case "messages":
      return laneDependencyArtifact(dependency, dependencies.messages);
    case "layout":
      return laneDependencyArtifact(dependency, dependencies.layout);
    case "navigation":
      return laneDependencyArtifact(dependency, dependencies.navigation);
    case "handoff":
      return laneDependencyArtifact(dependency, dependencies.handoff);
    case "focusTarget":
      return focusTargetDependencyArtifact(dependencies.focusTarget);
    default:
      return Object.freeze({
        dependency,
        status: "unavailable",
        target: null,
        reason: `${dependency} settlement dependency is not supported`,
        digest: null,
      });
  }
}

function laneDependencyArtifact(dependency, lane) {
  return Object.freeze({
    dependency,
    status: lane.status,
    target: lane.target,
    reason: lane.reason,
    digest: lane.token ?? null,
  });
}

function focusTargetDependencyArtifact(focusTarget) {
  if (focusTarget.posture === "unavailable") {
    return Object.freeze({
      dependency: "focusTarget",
      status: "unavailable",
      target: focusTarget.field,
      reason: focusTarget.reason,
      digest: stableValueDigest(focusTarget),
    });
  }
  if (focusTarget.posture === "none") {
    return Object.freeze({
      dependency: "focusTarget",
      status: "ready",
      target: null,
      reason: focusTarget.reason,
      digest: stableValueDigest(focusTarget),
    });
  }
  return Object.freeze({
    dependency: "focusTarget",
    status: "ready",
    target: focusTarget.target,
    reason: focusTarget.reason,
    digest: stableValueDigest(focusTarget),
  });
}
