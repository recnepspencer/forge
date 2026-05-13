import {
  createIdentityMigrationFallbackDigest,
  createIdentityMigrationTargetDigest,
  createIdentityMigrationTargetIdentityDigest,
} from "./resource_mutation_response_identity_migration_digests.js";

const RESOURCE_MUTATION_RESPONSE_PREPARED_IDENTITY_MIGRATIONS = Symbol(
  "forgeSignal.resourceMutationResponsePreparedIdentityMigrations",
);

function executePreparedMutationResponseIdentityMigration(identityMigration) {
  if (identityMigration === null) {
    return null;
  }
  const preparedExecutions =
    identityMigration[RESOURCE_MUTATION_RESPONSE_PREPARED_IDENTITY_MIGRATIONS] ?? [];
  const executedTargets = Object.freeze(
    identityMigration.targets.map((target, index) => {
      const preparedExecution = preparedExecutions[index];
      if (preparedExecution.kind !== "exactResidentLine") {
        return target;
      }
      const result = preparedExecution.targetMaterialization.migrateIdentity(
        preparedExecution.nextCanonicalParamIdentity,
      );
      if (result.kind !== "migrated") {
        return createExecutedFallbackIdentityMigrationTarget(target, result);
      }
      return createExecutedExactIdentityMigrationTarget(target, result);
    }),
  );
  return Object.freeze({
    ...identityMigration,
    exactTargetCount: readExactTargetCount(executedTargets),
    targets: executedTargets,
    targetDigest: createIdentityMigrationTargetDigest(executedTargets),
    fallbackDigest: createIdentityMigrationFallbackDigest(
      executedTargets,
      identityMigration.migrationNeeded,
    ),
    fallbackKinds: readIdentityMigrationFallbackKinds(executedTargets),
    executionDigest: createIdentityMigrationExecutionDigest(executedTargets),
    counters: Object.freeze({
      ...identityMigration.counters,
      exactTargetCount: readExactTargetCount(executedTargets),
      requestDescriptorRewriteBreadth: readExactTargetCount(executedTargets),
      lifecycleProofBreadth:
        identityMigration.migrationNeeded === true ? executedTargets.length : 0,
    }),
    digest: [
      identityMigration.declarationDigest,
      identityMigration.submittedIdentityDigest,
      identityMigration.responseIdentityDigest,
      identityMigration.canonicalIdentityDigest,
      createIdentityMigrationTargetDigest(executedTargets),
      createIdentityMigrationFallbackDigest(
        executedTargets,
        identityMigration.migrationNeeded,
      ),
      createIdentityMigrationExecutionDigest(executedTargets),
    ].join("|"),
  });
}

function createExecutedExactIdentityMigrationTarget(target, result) {
  const line = Object.freeze({
    ...target.line,
    canonicalKey: result.nextCanonicalKey,
    runtimeLineId: result.nextRuntimeLineId,
    residency: "resident",
  });
  const detail =
    `${readIdentityMigrationExecutionLabel(target)} migrated from ${result.previousCanonicalKey} to ${result.nextCanonicalKey}`;
  return Object.freeze({
    ...target,
    line,
    outcome: "exactResidentLine",
    detail,
    execution: Object.freeze({
      ...target.execution,
      nextRuntimeLineId: result.nextRuntimeLineId,
      basisId: result.basisId,
      requestPath: result.requestPath,
      outcomeKind: "applied",
      detail,
    }),
    targetDigest: createIdentityMigrationTargetIdentityDigest(
      target,
      line,
      "exactResidentLine",
      null,
    ),
  });
}

function createExecutedFallbackIdentityMigrationTarget(target, result) {
  const detail =
    result.detail
    ?? `${readIdentityMigrationExecutionLabel(target)} could not complete exact identity migration`;
  return Object.freeze({
    ...target,
    outcome: "fallback",
    detail,
    execution: Object.freeze({
      kind: "fallback",
      fallback: target.fallback,
      detail,
    }),
    targetDigest: createIdentityMigrationTargetIdentityDigest(
      target,
      target.line,
      "fallback",
      null,
    ),
  });
}

const readExactTargetCount = (targets) =>
  targets.filter((target) => target.execution.kind === "exactResidentLine").length;

function readIdentityMigrationFallbackKinds(targets) {
  return Object.freeze(
    targets
      .filter((target) => target.outcome === "fallback")
      .map((target) => target.fallback),
  );
}

function createIdentityMigrationExecutionDigest(targets) {
  if (targets.length === 0) {
    return "mutation-response-identity-execution|none";
  }
  return `mutation-response-identity-execution|${targets.map((target) => {
    if (target.execution.kind === "exactResidentLine") {
      return [
        target.targetId,
        target.execution.kind,
        target.execution.previousCanonicalKey,
        target.execution.nextCanonicalKey,
        target.execution.outcomeKind ?? "planned",
      ].join(":");
    }
    return [
      target.targetId,
      target.execution.kind,
      target.execution.kind === "fallback" ? target.execution.fallback : "none",
    ].join(":");
  }).join(",")}`;
}

function readIdentityMigrationExecutionLabel(target) {
  const lineLabel = `${target.family.kind} ${target.family.familyId}`;
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
  createIdentityMigrationExecutionDigest,
  executePreparedMutationResponseIdentityMigration,
  readExactTargetCount,
  readIdentityMigrationFallbackKinds,
  RESOURCE_MUTATION_RESPONSE_PREPARED_IDENTITY_MIGRATIONS,
};
