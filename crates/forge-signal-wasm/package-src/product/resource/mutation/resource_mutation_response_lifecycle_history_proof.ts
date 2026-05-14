function createFallbackHistoryProof(detail) {
  return Object.freeze({
    replayExact: Object.freeze({
      kind: "fallbackUnavailable",
      mode: null,
      detail,
    }),
    restoreExact: Object.freeze({
      kind: "fallbackUnavailable",
      mode: null,
      detail,
    }),
  });
}

function createAwaitingExecutionHistoryProof(detail) {
  return Object.freeze({
    replayExact: Object.freeze({
      kind: "awaitingExecution",
      mode: null,
      detail,
    }),
    restoreExact: Object.freeze({
      kind: "awaitingExecution",
      mode: null,
      detail,
    }),
  });
}

function createAppliedEffectHistoryProof() {
  return Object.freeze({
    replayExact: Object.freeze({
      kind: "available",
      mode: "SameRuntimeSignalExact",
      detail:
        "effect-backed mutation response reconciliation remains exactly replayable through runtime signal history",
    }),
    restoreExact: Object.freeze({
      kind: "available",
      mode: "SameRuntimeBranchExact",
      detail:
        "effect-backed mutation response reconciliation remains exactly restorable through runtime branch history",
    }),
  });
}

function createIdentityMigrationUnavailableHistoryProof(detail) {
  return Object.freeze({
    replayExact: Object.freeze({
      kind: "identityMigrationUnavailable",
      mode: null,
      detail,
    }),
    restoreExact: Object.freeze({
      kind: "identityMigrationUnavailable",
      mode: null,
      detail,
    }),
  });
}

function createReplayExactDigest(entries) {
  if (entries.length === 0) {
    return "mutation-response-replay-exact|none";
  }
  return `mutation-response-replay-exact|${entries.map((entry) =>
    [
      entry.entryKind,
      entry.targetId,
      entry.effectId ?? "none",
      entry.replayExact.kind,
      entry.replayExact.mode ?? "none",
    ].join(":")).join(",")}`;
}

function createRestoreExactDigest(entries) {
  if (entries.length === 0) {
    return "mutation-response-restore-exact|none";
  }
  return `mutation-response-restore-exact|${entries.map((entry) =>
    [
      entry.entryKind,
      entry.targetId,
      entry.effectId ?? "none",
      entry.restoreExact.kind,
      entry.restoreExact.mode ?? "none",
    ].join(":")).join(",")}`;
}

export {
  createAppliedEffectHistoryProof,
  createAwaitingExecutionHistoryProof,
  createFallbackHistoryProof,
  createIdentityMigrationUnavailableHistoryProof,
  createReplayExactDigest,
  createRestoreExactDigest,
};
