import {
  createAppliedEffectHistoryProof,
  createAwaitingExecutionHistoryProof,
  createFallbackHistoryProof,
  createIdentityMigrationUnavailableHistoryProof,
  createReplayExactDigest,
  createRestoreExactDigest,
} from "./resource_mutation_response_lifecycle_history_proof.js";
import {
  createIdentityMigrationLifecycleDigest,
  createMergeRebaseDigest,
  createRollbackDigest,
  createTargetEffectProofDigest,
  readIdentityMigrationGranularity,
  readMutationResponseMergeRebaseProof,
  readMutationResponseRollbackProof,
} from "./resource_mutation_response_lifecycle_proof_core.js";

function createMutationResponseTargetEffectProof(effect, artifact) {
  if (effect === null) {
    return null;
  }
  const rollback = readMutationResponseRollbackProof(effect);
  const mergeRebase = readMutationResponseMergeRebaseProof(effect);
  return Object.freeze({
    effectId: effect.effectId,
    authorityDigest: effect.authority.envelopeDigest,
    rollback,
    mergeRebase,
    branchLifecycleKind: effect.branchLifecycle.kind,
    optimisticKind: effect.optimistic.kind,
    locusKind: effect.locus.kind,
    locusProofDigest: effect.locusProof?.effectLocusDigest ?? null,
    digest: createTargetEffectProofDigest(effect, artifact, rollback, mergeRebase),
  });
}

function createMutationResponseLifecycleProof(
  executionArtifacts,
  identityMigration = null,
) {
  const entries = Object.freeze(
    [
      ...executionArtifacts.map((artifact) =>
        createMutationResponseLifecycleProofEntry(artifact)),
      ...createIdentityMigrationLifecycleProofEntries(identityMigration),
    ],
  );
  const rollbackDigest = createRollbackDigest(entries);
  const mergeRebaseDigest = createMergeRebaseDigest(entries);
  const replayExactDigest = createReplayExactDigest(entries);
  const restoreExactDigest = createRestoreExactDigest(entries);
  return Object.freeze({
    entries,
    count: entries.length,
    rollbackDigest,
    mergeRebaseDigest,
    replayExactDigest,
    restoreExactDigest,
    digest: [
      "mutation-response-lifecycle",
      rollbackDigest,
      mergeRebaseDigest,
      replayExactDigest,
      restoreExactDigest,
    ].join("|"),
  });
}

function createMutationResponseLifecycleProofEntry(artifact) {
  if (artifact.kind === "fallback") {
    return Object.freeze({
      entryKind: "reconciliation",
      targetId: artifact.targetId,
      effectId: null,
      authorityDigest: null,
      ...createFallbackHistoryProof(artifact.detail),
      rollback: Object.freeze({
        kind: "fallbackUnavailable",
        mode: null,
        branchId: null,
        snapshotId: null,
        inverseKind: null,
        detail: artifact.detail,
      }),
      mergeRebase: Object.freeze({
        kind: "fallbackUnavailable",
        granularity: artifact.fallback,
        detail: artifact.detail,
      }),
      digest: [
        artifact.targetId,
        "fallback",
        artifact.fallback,
        artifact.canonicalKey,
      ].join(":"),
    });
  }
  if (artifact.effectProof === null || artifact.effectProof === undefined) {
    return Object.freeze({
      entryKind: "reconciliation",
      targetId: artifact.targetId,
      effectId: null,
      authorityDigest: null,
      ...createAwaitingExecutionHistoryProof(
        "exact mutation response target has not executed yet",
      ),
      rollback: Object.freeze({
        kind: "awaitingExecution",
        mode: null,
        branchId: null,
        snapshotId: null,
        inverseKind: null,
        detail: "exact mutation response target has not executed yet",
      }),
      mergeRebase: Object.freeze({
        kind: "awaitingExecution",
        granularity: artifact.scope,
        detail: "exact mutation response target has not executed yet",
      }),
      digest: [
        artifact.targetId,
        "awaitingExecution",
        artifact.kind,
        artifact.scope,
        artifact.canonicalKey,
      ].join(":"),
    });
  }
  return Object.freeze({
    entryKind: "reconciliation",
    targetId: artifact.targetId,
    effectId: artifact.effectProof.effectId,
    authorityDigest: artifact.effectProof.authorityDigest,
    ...createAppliedEffectHistoryProof(),
    rollback: artifact.effectProof.rollback,
    mergeRebase: artifact.effectProof.mergeRebase,
    digest: artifact.effectProof.digest,
  });
}

function createIdentityMigrationLifecycleProofEntries(identityMigration) {
  if (identityMigration === null || identityMigration.migrationNeeded !== true) {
    return [];
  }
  return identityMigration.targets.map((target) =>
    createIdentityMigrationLifecycleProofEntry(
      target,
      identityMigration.declarationDigest,
    ));
}

function createIdentityMigrationLifecycleProofEntry(target, authorityDigest) {
  if (target.execution.kind === "fallback") {
    return Object.freeze({
      entryKind: "identityMigration",
      targetId: target.targetId,
      effectId: null,
      authorityDigest: null,
      ...createFallbackHistoryProof(target.execution.detail),
      rollback: Object.freeze({
        kind: "fallbackUnavailable",
        mode: null,
        branchId: null,
        snapshotId: null,
        inverseKind: null,
        detail: target.execution.detail,
      }),
      mergeRebase: Object.freeze({
        kind: "fallbackUnavailable",
        granularity: target.execution.fallback,
        detail: target.execution.detail,
      }),
      digest: [
        "identityMigration",
        target.targetId,
        "fallback",
        target.execution.fallback,
        target.line.canonicalKey,
      ].join(":"),
    });
  }
  if (target.execution.kind === "exactDetailChildRegion") {
    if (target.execution.effectProof === null || target.execution.effectProof === undefined) {
      return Object.freeze({
        entryKind: "identityMigration",
        targetId: target.targetId,
        effectId: null,
        authorityDigest,
        ...createAwaitingExecutionHistoryProof(
          "exact detail-child identity migration target has not executed yet",
        ),
        rollback: Object.freeze({
          kind: "awaitingExecution",
          mode: null,
          branchId: null,
          snapshotId: null,
          inverseKind: null,
          detail: "exact detail-child identity migration target has not executed yet",
        }),
        mergeRebase: Object.freeze({
          kind: "awaitingExecution",
          granularity: `region:${target.execution.region}`,
          detail: "exact detail-child identity migration target has not executed yet",
        }),
        digest: createIdentityMigrationLifecycleDigest(
          target,
          authorityDigest,
          "awaitingExecution",
          "awaitingExecution",
        ),
      });
    }
    return Object.freeze({
      entryKind: "identityMigration",
      targetId: target.targetId,
      effectId: target.execution.effectProof.effectId,
      authorityDigest: target.execution.effectProof.authorityDigest,
      ...createAppliedEffectHistoryProof(),
      rollback: target.execution.effectProof.rollback,
      mergeRebase: target.execution.effectProof.mergeRebase,
      digest: target.execution.effectProof.digest,
    });
  }
  if (target.execution.outcomeKind !== "applied") {
    return Object.freeze({
      entryKind: "identityMigration",
      targetId: target.targetId,
      effectId: null,
      authorityDigest,
      ...createAwaitingExecutionHistoryProof(
        "exact identity migration target has not executed yet",
      ),
      rollback: Object.freeze({
        kind: "awaitingExecution",
        mode: null,
        branchId: null,
        snapshotId: null,
        inverseKind: null,
        detail: "exact identity migration target has not executed yet",
      }),
      mergeRebase: Object.freeze({
        kind: "awaitingExecution",
        granularity: readIdentityMigrationGranularity(target),
        detail: "exact identity migration target has not executed yet",
      }),
      digest: createIdentityMigrationLifecycleDigest(
        target,
        authorityDigest,
        "awaitingExecution",
        "awaitingExecution",
      ),
    });
  }
  return Object.freeze({
    entryKind: "identityMigration",
    targetId: target.targetId,
    effectId: null,
    authorityDigest,
    ...createIdentityMigrationUnavailableHistoryProof(
      "identity migration preserved lifecycle continuity through resident line rematerialization, so exact replay and exact branch restore stay unavailable on the migrated resident line",
    ),
    rollback: Object.freeze({
      kind: "identityMigrationUnavailable",
      mode: null,
      branchId: null,
      snapshotId: null,
      inverseKind: null,
      detail:
        "identity migration preserved lifecycle continuity through resident line rematerialization, but no resource-effect rollback envelope exists for canonical-key rewrite",
    }),
    mergeRebase: Object.freeze({
      kind: "identityMigrationUnavailable",
      granularity: readIdentityMigrationGranularity(target),
      detail:
        "identity migration rewrote resident line identity without issuing a resource effect locus, so merge and rebase stay unavailable",
    }),
    digest: createIdentityMigrationLifecycleDigest(
      target,
      authorityDigest,
      "identityMigrationUnavailable",
      "identityMigrationUnavailable",
    ),
  });
}

export {
  createMutationResponseLifecycleProof,
  createMutationResponseTargetEffectProof,
};
