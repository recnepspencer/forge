import {
  createMutationResponseTargetBasisSnapshotsForTargets,
  readMutationResponseTargetStaleness,
} from "../resource_mutation_response_target_basis.js";
import {
  createIdentityDigest,
  createIdentityMigrationFallbackDigest,
  createIdentityMigrationTargetDigest,
  createIdentityMigrationTargetIdentityDigest,
} from "./resource_mutation_response_identity_migration_digests.js";
import {
  createIdentityMigrationExecutionDigest,
  readExactTargetCount,
  readIdentityMigrationFallbackKinds,
  RESOURCE_MUTATION_RESPONSE_PREPARED_IDENTITY_MIGRATIONS,
} from "./resource_mutation_response_identity_migration_execution.js";
import {
  applyIdentityMigrationAtomicityPolicy,
  applyIdentityMigrationSiblingConflictPolicy,
  readIdentityMigrationPartialAdmission,
} from "./resource_mutation_response_identity_migration_admission.js";

function createSubmittedMutationResponseIdentityMigration(declaration, mutationParams) {
  if (declaration === null) {
    return null;
  }
  const submittedIdentity = requireIdentityString(
    declaration.submitted(mutationParams),
    `${declaration.source}.submitted(...)`,
  );
  return Object.freeze({
    submittedIdentity,
    submittedIdentityDigest: createIdentityDigest("submitted", submittedIdentity),
    submittedTargets: createMutationResponseTargetBasisSnapshotsForTargets(
      declaration.targets,
      mutationParams,
    ),
  });
}

function createMutationResponseIdentityMigrationPlan(
  declaration,
  requestParams,
  responseValue,
  submittedIdentityMigration,
) {
  if (declaration === null || submittedIdentityMigration === null) {
    return null;
  }
  const responseIdentity = requireOptionalIdentityString(
    declaration.response === null ? null : declaration.response(responseValue),
    `${declaration.source}.response(...)`,
  );
  const canonicalIdentity = requireIdentityString(
    declaration.canonical(responseValue, responseIdentity),
    `${declaration.source}.canonical(...)`,
  );
  const migrationNeeded =
    submittedIdentityMigration.submittedIdentity !== canonicalIdentity;
  const plannedTargets = declaration.targets.map((target, index) =>
    createIdentityMigrationTargetPlan(
      target,
      requestParams,
      responseValue,
      submittedIdentityMigration.submittedTargets[index] ?? null,
      responseIdentity,
      canonicalIdentity,
      migrationNeeded,
    ));
  const conflictCheckedTargets = applyIdentityMigrationSiblingConflictPolicy(
    plannedTargets,
  );
  const policyAppliedTargets = applyIdentityMigrationAtomicityPolicy(
    declaration,
    conflictCheckedTargets,
  );
  const publicTargets = Object.freeze(
    policyAppliedTargets.map((target) => target.publicTarget),
  );
  const partialAdmission = readIdentityMigrationPartialAdmission(
    declaration,
    conflictCheckedTargets,
  );
  const identityMigration = {
    declarationDigest: declaration.declarationDigest,
    atomicity: declaration.atomicity,
    partialAdmission,
    submittedIdentity: submittedIdentityMigration.submittedIdentity,
    submittedIdentityDigest: submittedIdentityMigration.submittedIdentityDigest,
    responseIdentity,
    responseIdentityDigest: createIdentityDigest("response", responseIdentity),
    canonicalIdentity,
    canonicalIdentityDigest: createIdentityDigest("canonical", canonicalIdentity),
    migrationNeeded,
    exactTargetCount: readExactTargetCount(publicTargets),
    targets: publicTargets,
    targetCount: publicTargets.length,
    targetDigest: createIdentityMigrationTargetDigest(publicTargets),
    fallbackDigest: createIdentityMigrationFallbackDigest(publicTargets, migrationNeeded),
    fallbackKinds: readIdentityMigrationFallbackKinds(publicTargets),
    executionDigest: createIdentityMigrationExecutionDigest(publicTargets),
    counters: Object.freeze({
      responseIdentityExtractionBreadth: declaration.response === null ? 0 : 1,
      canonicalIdentityBreadth: 1,
      targetFanoutBreadth: publicTargets.length,
      targetBasisSnapshotBreadth: submittedIdentityMigration.submittedTargets.length,
      staleTargetDenialBreadth: publicTargets.filter((target) =>
        target.staleness !== null).length,
      exactTargetCount: readExactTargetCount(publicTargets),
      requestDescriptorRewriteBreadth: readExactTargetCount(publicTargets),
      lifecycleProofBreadth: migrationNeeded ? publicTargets.length : 0,
      partialPolicyBreadth: 1,
    }),
    digest: [
      declaration.declarationDigest,
      declaration.atomicity,
      partialAdmission,
      submittedIdentityMigration.submittedIdentityDigest,
      createIdentityDigest("response", responseIdentity),
      createIdentityDigest("canonical", canonicalIdentity),
      createIdentityMigrationTargetDigest(publicTargets),
      createIdentityMigrationFallbackDigest(publicTargets, migrationNeeded),
      createIdentityMigrationExecutionDigest(publicTargets),
    ].join("|"),
  };
  Object.defineProperty(
    identityMigration,
    RESOURCE_MUTATION_RESPONSE_PREPARED_IDENTITY_MIGRATIONS,
    {
      value: Object.freeze(
        policyAppliedTargets.map((target) => target.preparedExecution),
      ),
      enumerable: false,
    },
  );
  return Object.freeze(identityMigration);
}

function createIdentityMigrationTargetPlan(
  target,
  requestParams,
  responseValue,
  submittedTarget,
  responseIdentity,
  canonicalIdentity,
  migrationNeeded,
) {
  const targetParams = target.params(requestParams);
  const lineIdentity = target.readTargetLineIdentity(targetParams);
  const targetMaterialization =
    lineIdentity.residency === "resident"
      ? target.lookupResidentTargetMaterialization(targetParams)
      : null;
  const staleness =
    !migrationNeeded
      || submittedTarget === null
      || targetMaterialization === null
      ? null
      : readMutationResponseTargetStaleness(
          lineIdentity,
          targetMaterialization,
          submittedTarget,
        );
  if (!migrationNeeded) {
    const targetLabel = readIdentityMigrationTargetLabel(target, lineIdentity);
    return createIdentityMigrationTargetRecord(
      target,
      lineIdentity,
      submittedTarget,
      staleness,
      "noMigrationRequired",
      `${targetLabel} already matches canonical identity`,
      Object.freeze({
        kind: "noMigrationRequired",
        detail: `${targetLabel} did not require identity migration`,
      }),
      Object.freeze({ kind: "noMigrationRequired" }),
    );
  }
  if (staleness !== null || targetMaterialization === null) {
    return createIdentityMigrationFallbackTarget(
      target,
      lineIdentity,
      submittedTarget,
      staleness,
      staleness?.detail
        ?? createIdentityMigrationFallbackDetail(target, lineIdentity),
    );
  }
  if (target.scope.kind === "detailChild") {
    return createIdentityMigrationFallbackTarget(
      target,
      lineIdentity,
      submittedTarget,
      null,
      createIdentityMigrationFallbackDetail(target, lineIdentity),
    );
  }
  if (target.canonicalParams === null) {
    return createIdentityMigrationFallbackTarget(
      target,
      lineIdentity,
      submittedTarget,
      null,
      createIdentityMigrationFallbackDetail(target, lineIdentity),
    );
  }
  const canonicalTargetParams = target.canonicalParams(
    requestParams,
    responseValue,
    canonicalIdentity,
    responseIdentity,
  );
  const canonicalParamIdentity =
    target.canonicalizeTargetParams(canonicalTargetParams);
  if (canonicalParamIdentity.canonicalKey === lineIdentity.canonicalKey) {
    const targetLabel = readIdentityMigrationTargetLabel(target, lineIdentity);
    return createIdentityMigrationTargetRecord(
      target,
      lineIdentity,
      submittedTarget,
      null,
      "noMigrationRequired",
      `${targetLabel} already resolves to the canonical target params`,
      Object.freeze({
        kind: "noMigrationRequired",
        detail: `${targetLabel} already resolves to the canonical target params`,
      }),
      Object.freeze({ kind: "noMigrationRequired" }),
    );
  }
  if (target.readTargetLineIdentity(canonicalTargetParams).residency === "resident") {
    return createIdentityMigrationFallbackTarget(
      target,
      lineIdentity,
      submittedTarget,
      null,
      `${target.family.kind} ${target.family.familyId} cannot migrate ${lineIdentity.canonicalKey} to ${canonicalParamIdentity.canonicalKey} because the canonical destination is already resident`,
    );
  }
  const detail =
    `${readIdentityMigrationTargetLabel(target, lineIdentity)} will migrate to ${canonicalParamIdentity.canonicalKey} using the canonical response identity`;
  return createIdentityMigrationTargetRecord(
    target,
    lineIdentity,
    submittedTarget,
    null,
    "exactResidentLine",
    detail,
    Object.freeze({
      kind: "exactResidentLine",
      previousCanonicalKey: lineIdentity.canonicalKey,
      nextCanonicalKey: canonicalParamIdentity.canonicalKey,
      previousRuntimeLineId: lineIdentity.runtimeLineId,
      nextRuntimeLineId: null,
      basisId: targetMaterialization.requestState.currentBasisId(),
      requestPath: null,
      outcomeKind: null,
      detail,
    }),
    Object.freeze({
      kind: "exactResidentLine",
      targetMaterialization,
      nextCanonicalParamIdentity: canonicalParamIdentity,
    }),
  );
}

function createIdentityMigrationFallbackTarget(
  target,
  lineIdentity,
  submittedTarget,
  staleness,
  detail,
) {
  return createIdentityMigrationTargetRecord(
    target,
    lineIdentity,
    submittedTarget,
    staleness,
    "fallback",
    detail,
    Object.freeze({
      kind: "fallback",
      fallback: target.fallback,
      detail,
    }),
    Object.freeze({ kind: "fallback" }),
  );
}

function createIdentityMigrationTargetRecord(
  target,
  lineIdentity,
  submittedTarget,
  staleness,
  outcome,
  detail,
  execution,
  preparedExecution,
) {
  return Object.freeze({
    publicTarget: Object.freeze({
      targetId: target.targetId,
      family: target.family,
      scope: target.scope,
      line: Object.freeze({
        familyKind: target.family.kind,
        familyId: target.family.familyId,
        canonicalKey: lineIdentity.canonicalKey,
        runtimeLineId: lineIdentity.runtimeLineId,
        residency: lineIdentity.residency,
      }),
      fallback: target.fallback,
      submittedTarget,
      staleness,
      outcome,
      detail,
      execution,
      targetDigest: createIdentityMigrationTargetIdentityDigest(
        target,
        lineIdentity,
        outcome,
        staleness,
      ),
    }),
    preparedExecution,
  });
}

function createIdentityMigrationFallbackDetail(target, lineIdentity) {
  const targetLabel = readIdentityMigrationTargetLabel(target, lineIdentity);
  if (target.scope.kind === "detailChild") {
    return [
      targetLabel,
      `stays in ${target.fallback} posture until detail-child identity rewrite support lands`,
    ].join(" ");
  }
  return [
    targetLabel,
    `stays in ${target.fallback} posture until canonicalParams(...) declares its canonical migration target`,
  ].join(" ");
}

function readIdentityMigrationTargetLabel(target, lineIdentity) {
  const lineLabel =
    `${target.family.kind} ${target.family.familyId} line ${lineIdentity.canonicalKey}`;
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

function requireIdentityString(value, source) {
  if (typeof value !== "string" || value.length === 0) {
    throw new TypeError(`${source} must return a non-empty identity string`);
  }
  return value;
}

function requireOptionalIdentityString(value, source) {
  if (value === null) {
    return null;
  }
  return requireIdentityString(value, source);
}

export {
  createMutationResponseIdentityMigrationPlan,
  createSubmittedMutationResponseIdentityMigration,
};
