import { resourceDelivery } from "../../delivery/resource_delivery.js";
import { executeLineDelivery } from "../../lines/actions/line_delivery_execution.js";
import { createMutationResponseTargetEffectProof } from "../resource_mutation_response_lifecycle_proof.js";
import { resourcePatch } from "../../reconciliation/resource_patch.js";
import { readLineBindingState } from "../../lines/state/line_binding_state.js";
import {
  createIdentityMigrationFallbackDigest,
  createIdentityMigrationTargetDigest,
  createIdentityMigrationTargetIdentityDigest,
} from "./resource_mutation_response_identity_migration_digests.js";

const RESOURCE_MUTATION_RESPONSE_PREPARED_IDENTITY_MIGRATIONS = Symbol(
  "WORTHSignal.resourceMutationResponsePreparedIdentityMigrations",
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
      if (preparedExecution.kind === "exactResidentLine") {
        const result = preparedExecution.targetMaterialization.migrateIdentity(
          preparedExecution.nextCanonicalParamIdentity,
        );
        if (result.kind !== "migrated") {
          return createExecutedFallbackIdentityMigrationTarget(target, result);
        }
        return createExecutedExactIdentityMigrationTarget(target, result);
      }
      if (preparedExecution.kind === "exactDetailChildRegion") {
        return executePreparedDetailChildRegionMigration(target, preparedExecution);
      }
      if (preparedExecution.kind !== "exactResidentLine") {
        return target;
      }
      return target;
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
      requestDescriptorRewriteBreadth:
        readRequestDescriptorRewriteBreadth(executedTargets),
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

function executePreparedDetailChildRegionMigration(target, preparedExecution) {
  const diagnostics = readLineBindingState(
    preparedExecution.targetMaterialization.binding,
  ).diagnostics;
  const delivery = resourceDelivery.patch({
    packetId: preparedExecution.packetId,
    basisId: null,
    nextBasisId: preparedExecution.targetMaterialization.requestState.currentBasisId(),
    patch: resourcePatch.region({
      region: preparedExecution.region,
      value: preparedExecution.nextRegionValue,
    }),
  });
  const result = executeLineDelivery(preparedExecution.targetMaterialization, delivery);
  if (
    result.kind !== "applied"
    && !(
      result.kind === "duplicateIgnored"
      && diagnostics.lastDeliveryPacketId === delivery.packetId
    )
  ) {
    return createExecutedFallbackIdentityMigrationTarget(
      target,
      Object.freeze({
        detail: [
          readIdentityMigrationExecutionLabel(target),
          "could not complete exact detail-child region migration",
          `because delivery ${delivery.packetId} ended in ${result.kind}`,
        ].join(" "),
      }),
    );
  }
  const nextDiagnostics = readLineBindingState(
    preparedExecution.targetMaterialization.binding,
  ).diagnostics;
  const effect = nextDiagnostics.lastEffect ?? null;
  const detail = [
    readIdentityMigrationExecutionLabel(target),
    `rewrote region "${preparedExecution.region}" from canonical response truth`,
  ].join(" ");
  return Object.freeze({
    ...target,
    outcome: "exactDetailChildRegion",
    detail,
    execution: Object.freeze({
      ...target.execution,
      effectId: effect?.effectId ?? null,
      effectProof:
        effect === null
          ? null
          : createMutationResponseTargetEffectProof(effect, {
              targetId: target.targetId,
              kind: "exactDetail",
              scope: "region",
              familyKind: target.family.kind,
              familyId: target.family.familyId,
              canonicalKey: target.line.canonicalKey,
              runtimeLineId: target.line.runtimeLineId,
              residency: target.line.residency,
              packetId: preparedExecution.packetId,
              submittedTarget: target.submittedTarget,
              staleness: null,
              itemId: null,
              placement: null,
              field: null,
              region: preparedExecution.region,
              path: null,
              summary: null,
              summaryScope: null,
            }),
      outcomeKind: "applied",
      targetVisibleValueVersion: nextDiagnostics.visibleValueVersion,
      detail,
    }),
    targetDigest: createIdentityMigrationTargetIdentityDigest(
      target,
      target.line,
      "exactDetailChildRegion",
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
  targets.filter((target) =>
    target.execution.kind === "exactResidentLine"
    || target.execution.kind === "exactDetailChildRegion").length;

function readIdentityMigrationFallbackKinds(targets) {
  return Object.freeze(
    targets
      .filter((target) => target.outcome === "fallback")
      .map((target) => target.fallback),
  );
}

function readRequestDescriptorRewriteBreadth(targets) {
  return targets.filter((target) => target.execution.kind === "exactResidentLine").length;
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
    if (target.execution.kind === "exactDetailChildRegion") {
      return [
        target.targetId,
        target.execution.kind,
        target.execution.region,
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
