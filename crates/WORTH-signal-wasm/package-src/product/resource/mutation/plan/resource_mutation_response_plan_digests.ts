import {
  isExactMutationResponseExecutionKind,
} from "../resource_mutation_response_target_execution.js";

function createPlannedMutationResponseTargetIdentityDigest(
  target,
  lineIdentity,
  executionArtifact,
) {
  return [
    target.targetId,
    target.family.kind,
    target.family.familyId,
    lineIdentity.canonicalKey,
    executionArtifact.kind === "fallback"
      ? target.fallback
      : isExactMutationResponseExecutionKind(executionArtifact.kind)
        ? `exact:${executionArtifact.scope}:${
          executionArtifact.itemId
          ?? executionArtifact.summary
          ?? executionArtifact.field
          ?? executionArtifact.region
          ?? executionArtifact.path
          ?? "line"
        }`
        : target.fallback,
  ].join("|");
}

function createMutationResponseTargetDigest(targets) {
  if (targets.length === 0) {
    return "mutation-response-targets|none";
  }
  return `mutation-response-targets|${targets.map((target) =>
    `${target.targetId}:${target.family.kind}:${target.family.familyId}:${target.fallback}`).join(",")}`;
}

function createMutationResponseFallbackDigest(targets) {
  if (targets.length === 0) {
    return "mutation-response-fallbacks|none";
  }
  return `mutation-response-fallbacks|${targets.map((target) =>
    target.execution.kind === "fallback"
      ? `${target.targetId}:${target.fallback.kind}:${target.execution.partial?.digest ?? "none"}:${target.line.canonicalKey}`
      : `${target.targetId}:none:${target.line.canonicalKey}`).join(",")}`;
}

function createPlannedMutationResponseTargetDigest(targets) {
  if (targets.length === 0) {
    return "mutation-response-targets|none";
  }
  return `mutation-response-targets|${targets.map((target) =>
    `${target.targetId}:${target.family.kind}:${target.family.familyId}:${target.line.canonicalKey}:${target.fallback.kind}`).join(",")}`;
}

function createMutationResponseExecutionDigest(executionArtifacts) {
  if (executionArtifacts.length === 0) {
    return "mutation-response-execution|none";
  }
  return `mutation-response-execution|${executionArtifacts.map((artifact) =>
    artifact.kind === "fallback"
      ? `${artifact.targetId}:fallback:${artifact.fallback}:${artifact.partial?.digest ?? "none"}:${artifact.canonicalKey}`
      : `${artifact.targetId}:${artifact.kind}:${artifact.scope}:${
        artifact.itemId
        ?? artifact.summary
        ?? artifact.field
        ?? artifact.region
        ?? artifact.path
        ?? "line"
      }:${artifact.canonicalKey}`).join(",")}`;
}

function createExecutedMutationResponseExecutionDigest(executedArtifacts) {
  if (executedArtifacts.length === 0) {
    return "mutation-response-execution|none";
  }
  return `mutation-response-execution|${executedArtifacts.map((artifact) =>
    artifact.kind === "fallback"
      ? `${artifact.targetId}:fallback:${artifact.fallback}:${artifact.partial?.digest ?? "none"}:${artifact.canonicalKey}`
      : `${artifact.targetId}:${artifact.kind}:${artifact.scope}:${
        artifact.itemId
        ?? artifact.summary
        ?? artifact.field
        ?? artifact.region
        ?? artifact.path
        ?? "line"
      }:${artifact.effectId ?? "none"}:${artifact.canonicalKey}`).join(",")}`;
}

function readAppliedTargetBreadthCounter(executedArtifacts) {
  const appliedTargetBreadth = executedArtifacts.filter(
    (artifact) => isExactMutationResponseExecutionKind(artifact.kind),
  ).length;
  if (appliedTargetBreadth === 0) {
    return Object.freeze({});
  }
  return Object.freeze({
    appliedTargetBreadth,
  });
}

const readFallbackTargetBreadth = (executionArtifacts) =>
  executionArtifacts.filter((artifact) => artifact.kind === "fallback").length;
const readStaleTargetDenialBreadth = (executionArtifacts) =>
  executionArtifacts.filter((artifact) => artifact.staleness !== null).length;

export {
  createExecutedMutationResponseExecutionDigest,
  createMutationResponseExecutionDigest,
  createMutationResponseFallbackDigest,
  createMutationResponseTargetDigest,
  createPlannedMutationResponseTargetDigest,
  createPlannedMutationResponseTargetIdentityDigest,
  readAppliedTargetBreadthCounter,
  readFallbackTargetBreadth,
  readStaleTargetDenialBreadth,
};
