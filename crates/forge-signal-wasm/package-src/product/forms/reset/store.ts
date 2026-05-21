import { readResourceLineHandle } from "../sources/form_sources.js";
import { stableValueDigest } from "../values/value_paths.js";
import { readFormStateSnapshot } from "../recovery/form_state_snapshot.js";

export function createFormResetStore() {
  let nextResetId = 1;
  const history = [];
  return Object.freeze({
    acceptCanonicalValue(context, options = {}) {
      const before = readFormStateSnapshot(context.form);
      if (before.draftDigest === "{}") {
        return recordArtifact({
          mode: "acceptCanonicalValue",
          resultKind: "noOp",
          reason: options.reason ?? "form reset did not change truth because no draft edits were present",
          before,
          after: before,
          resourceRollback: null,
        });
      }
      context.writeDraft({});
      const after = readFormStateSnapshot(context.form);
      return recordArtifact({
        mode: "acceptCanonicalValue",
        resultKind: "reset",
        reason: options.reason ?? "form accepted canonical source truth and cleared local draft edits",
        before,
        after,
        resourceRollback: null,
      });
    },
    rollbackLastResourceEffect(context, options = {}) {
      const line = readResourceLineHandle(context.source);
      const before = readFormStateSnapshot(context.form);
      if (line === null || typeof line.history !== "function") {
        return recordArtifact({
          mode: "resourceRollback",
          resultKind: "unavailable",
          reason: options.reason ?? "resource rollback is unavailable because the form source is not a resource line",
          before,
          after: before,
          resourceRollback: unavailableRollback("resourceSourceUnavailable", "form source is not a resource line"),
        });
      }
      const historyRead = line.history();
      if (!historyRead || typeof historyRead.rollbackLastEffect !== "function") {
        return recordArtifact({
          mode: "resourceRollback",
          resultKind: "unavailable",
          reason: options.reason ?? "resource rollback is unavailable because the resource line does not expose rollback history",
          before,
          after: before,
          resourceRollback: unavailableRollback("rollbackHistoryUnavailable", "resource line history does not expose rollbackLastEffect()"),
        });
      }
      const canonicalization = latestResourceCanonicalization(context.form);
      if (canonicalization === null) {
        return recordArtifact({
          mode: "resourceRollback",
          resultKind: "unavailable",
          reason: options.reason ?? "resource rollback is unavailable because the form has no recorded resource canonicalization proof",
          before,
          after: before,
          resourceRollback: unavailableRollback(
            "runtimeRejected",
            "form rollback requires recorded resource canonicalization proof for exact draft restoration",
          ),
        });
      }
      const rollback = historyRead.rollbackLastEffect();
      if (isPromiseLike(rollback)) {
        return rollback.then((settledRollback) => {
          if (settledRollback.kind === "unavailable") {
            return recordArtifact({
              mode: "resourceRollback",
              resultKind: "unavailable",
              reason: options.reason ?? settledRollback.detail,
              before,
              after: before,
              resourceRollback: normalizeRollbackResult(settledRollback),
            });
          }
          context.writeDraft(canonicalization.previousDraftValue);
          const after = readFormStateSnapshot(context.form);
          return recordArtifact({
            mode: "resourceRollback",
            resultKind: "rolledBack",
            reason: options.reason ?? "resource rollback restored visible source truth and cleared local draft edits",
            before,
            after,
            resourceRollback: normalizeRollbackResult(settledRollback),
          });
        });
      }
      if (rollback.kind === "unavailable") {
        return recordArtifact({
          mode: "resourceRollback",
          resultKind: "unavailable",
          reason: options.reason ?? rollback.detail,
          before,
          after: before,
          resourceRollback: normalizeRollbackResult(rollback),
        });
      }
      context.writeDraft(canonicalization.previousDraftValue);
      const after = readFormStateSnapshot(context.form);
      return recordArtifact({
        mode: "resourceRollback",
        resultKind: "rolledBack",
        reason: options.reason ?? "resource rollback restored visible source truth and cleared local draft edits",
        before,
        after,
        resourceRollback: normalizeRollbackResult(rollback),
      });
    },
    history() {
      return Object.freeze([...history]);
    },
  });

  function recordArtifact(options) {
    const artifact = {
      kind: "formReset",
      resetId: nextResetId++,
      observedAtMs: Date.now(),
      mode: options.mode,
      resultKind: options.resultKind,
      reason: options.reason,
      previousSourceDigest: options.before.sourceDigest,
      previousDraftDigest: options.before.draftDigest,
      previousEffectiveDigest: options.before.effectiveDigest,
      nextSourceDigest: options.after.sourceDigest,
      nextDraftDigest: options.after.draftDigest,
      nextEffectiveDigest: options.after.effectiveDigest,
      resourceRollback: options.resourceRollback,
    };
    const frozen = Object.freeze({
      ...artifact,
      resetDigest: stableValueDigest(artifact),
    });
    history.push(frozen);
    return frozen;
  }
}

function isPromiseLike(value) {
  return value !== null && typeof value === "object" && typeof value.then === "function";
}

function latestResourceCanonicalization(form) {
  return [...form.canonicalizationHistory()]
    .reverse()
    .find((artifact) => artifact.resourceLine !== null) ?? null;
}

function normalizeRollbackResult(rollback) {
  if (rollback.kind === "unavailable") {
    return unavailableRollback(rollback.reason, rollback.detail);
  }
  return Object.freeze({
    kind: "rolledBack",
    mode: rollback.mode,
    effectId: rollback.effectId,
    branchId: rollback.branchId,
    snapshotId: rollback.snapshotId,
    basisCurrentId: rollback.basisCurrentId,
    basisAdvanceCount: rollback.basisAdvanceCount,
    rollback: rollback.rollback,
    reloadStatus: rollback.reloadStatus,
    digest: stableValueDigest({
      kind: rollback.kind,
      mode: rollback.mode,
      effectId: rollback.effectId,
      branchId: rollback.branchId,
      snapshotId: rollback.snapshotId,
      basisCurrentId: rollback.basisCurrentId,
      basisAdvanceCount: rollback.basisAdvanceCount,
      rollback: rollback.rollback,
      reloadStatus: rollback.reloadStatus,
    }),
  });
}

function unavailableRollback(reason, detail) {
  return Object.freeze({
    kind: "unavailable",
    reason,
    detail,
    digest: stableValueDigest({
      kind: "unavailable",
      reason,
      detail,
    }),
  });
}
