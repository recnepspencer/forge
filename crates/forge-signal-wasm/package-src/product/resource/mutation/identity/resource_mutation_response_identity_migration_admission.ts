import {
  createIdentityMigrationTargetIdentityDigest,
} from "./resource_mutation_response_identity_migration_digests.js";

function applyIdentityMigrationSiblingConflictPolicy(plannedTargets) {
  const conflictingTargetIds = readConflictingExactTargetIds(plannedTargets);
  if (conflictingTargetIds.size === 0) {
    return Object.freeze(plannedTargets);
  }
  return Object.freeze(
    plannedTargets.map((target) =>
      conflictingTargetIds.has(target.publicTarget.targetId)
        ? createSiblingConflictDeniedIdentityMigrationTarget(target, plannedTargets)
        : target),
  );
}

function applyIdentityMigrationAtomicityPolicy(declaration, plannedTargets) {
  const exactTargetCount = plannedTargets.filter((target) =>
    target.publicTarget.execution.kind === "exactResidentLine").length;
  const fallbackTargets = plannedTargets.filter((target) =>
    target.publicTarget.outcome === "fallback");
  if (
    declaration.atomicity === "partialAllowed"
    || exactTargetCount === 0
    || fallbackTargets.length === 0
  ) {
    return Object.freeze(plannedTargets);
  }
  return Object.freeze(
    plannedTargets.map((target) =>
      target.publicTarget.execution.kind === "exactResidentLine"
        ? createAtomicityDeniedIdentityMigrationTarget(target, fallbackTargets)
        : target),
  );
}

function readIdentityMigrationPartialAdmission(declaration, plannedTargets) {
  const exactTargetCount = plannedTargets.filter((target) =>
    target.publicTarget.execution.kind === "exactResidentLine").length;
  const fallbackTargetCount = plannedTargets.filter((target) =>
    target.publicTarget.outcome === "fallback").length;
  if (exactTargetCount === 0 || fallbackTargetCount === 0) {
    return "notNeeded";
  }
  return declaration.atomicity === "partialAllowed" ? "admitted" : "denied";
}

function readConflictingExactTargetIds(plannedTargets) {
  const exactTargets = plannedTargets.filter((target) =>
    target.publicTarget.execution.kind === "exactResidentLine");
  const targetIds = new Set();
  const targetsByDestination = new Map();
  for (const target of exactTargets) {
    const destinationKey = [
      target.publicTarget.family.kind,
      target.publicTarget.family.familyId,
      target.preparedExecution.nextCanonicalParamIdentity.canonicalKey,
    ].join("|");
    const siblingTargets = targetsByDestination.get(destinationKey) ?? [];
    siblingTargets.push(target);
    targetsByDestination.set(destinationKey, siblingTargets);
  }
  for (const siblingTargets of targetsByDestination.values()) {
    if (siblingTargets.length < 2) {
      continue;
    }
    for (const target of siblingTargets) {
      targetIds.add(target.publicTarget.targetId);
    }
  }
  return targetIds;
}

function createSiblingConflictDeniedIdentityMigrationTarget(targetRecord, plannedTargets) {
  const siblingCount = plannedTargets.filter((target) =>
    target !== targetRecord
    && target.publicTarget.execution.kind === "exactResidentLine"
    && target.publicTarget.family.kind === targetRecord.publicTarget.family.kind
    && target.publicTarget.family.familyId === targetRecord.publicTarget.family.familyId
    && target.preparedExecution.nextCanonicalParamIdentity.canonicalKey
      === targetRecord.preparedExecution.nextCanonicalParamIdentity.canonicalKey).length;
  const detail = [
    readIdentityMigrationAdmissionLabel(targetRecord.publicTarget),
    "stays in",
    `${targetRecord.publicTarget.fallback} posture because ${siblingCount + 1} sibling migration target(s) claim canonical destination`,
    targetRecord.preparedExecution.nextCanonicalParamIdentity.canonicalKey,
  ].join(" ");
  return createDeniedIdentityMigrationTarget(targetRecord, detail);
}

function createAtomicityDeniedIdentityMigrationTarget(targetRecord, fallbackTargets) {
  const detail = [
    readIdentityMigrationAdmissionLabel(targetRecord.publicTarget),
    "stays in",
    `${targetRecord.publicTarget.fallback} posture because identity.atomicity=allOrNone does not admit partial migration`,
    `after ${fallbackTargets.length} sibling target(s) fell back`,
  ].join(" ");
  return createDeniedIdentityMigrationTarget(targetRecord, detail);
}

function createDeniedIdentityMigrationTarget(targetRecord, detail) {
  return Object.freeze({
    publicTarget: Object.freeze({
      ...targetRecord.publicTarget,
      outcome: "fallback",
      detail,
      execution: Object.freeze({
        kind: "fallback",
        fallback: targetRecord.publicTarget.fallback,
        detail,
      }),
      targetDigest: createIdentityMigrationTargetIdentityDigest(
        targetRecord.publicTarget,
        targetRecord.publicTarget.line,
        "fallback",
        null,
      ),
    }),
    preparedExecution: Object.freeze({ kind: "fallback" }),
  });
}

function readIdentityMigrationAdmissionLabel(target) {
  const lineLabel =
    `${target.family.kind} ${target.family.familyId} line ${target.line.canonicalKey}`;
  if (target.scope.kind === "residentLine") {
    return lineLabel;
  }
  if (target.scope.kind === "summary") {
    return `summary ${target.scope.summary} on ${lineLabel}`;
  }
  if (target.scope.kind === "visibleSelection") {
    return `visibleSelection on ${lineLabel}`;
  }
  return `detailChild region ${target.scope.region} on ${lineLabel}`;
}

export {
  applyIdentityMigrationAtomicityPolicy,
  applyIdentityMigrationSiblingConflictPolicy,
  readIdentityMigrationPartialAdmission,
};
