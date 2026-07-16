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
      const settlement = historyRead.rollbackLastEffect();
      const complete = (result) => completeTargetedEffectRejection({
        result,
        canonicalization,
        context,
        options,
        before,
        recordArtifact,
      });
      return isPromiseLike(settlement) ? settlement.then(complete) : complete(settlement);
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

function completeTargetedEffectRejection({
  result,
  canonicalization,
  context,
  options,
  before,
  recordArtifact,
}) {
  if (result.kind === "unavailable") {
    return recordArtifact({
      mode: "resourceRollback",
      resultKind: "unavailable",
      reason: options.reason ?? result.detail,
      before,
      after: before,
      resourceRollback: unavailableRollback(result.reason, result.detail),
    });
  }
  if (result.kind !== "rejectedAndRetired") {
    throw new TypeError(
      `form resource effect rejection expected rejectedAndRetired, received ${result.kind}`,
    );
  }
  if (canonicalization !== null) {
    context.writeDraft(canonicalization.previousDraftValue);
  }
  const after = readFormStateSnapshot(context.form);
  return recordArtifact({
    mode: "resourceRollback",
    resultKind: "effectRejected",
    reason: options.reason ?? (
      canonicalization === null
        ? "the last open resource effect was rejected; no form canonicalization draft required restoration"
        : "the last open resource effect was rejected and the submitted form draft was restored"
    ),
    before,
    after,
    resourceRollback: targetedRejectionArtifact(result),
  });
}

function targetedRejectionArtifact(result) {
  const retiredEffectIds = Object.freeze(
    (result.retired ?? []).map((entry) => entry.effectId),
  );
  const digestSource = {
    kind: "effectRejected",
    effectId: result.effectId,
    terminalKind: result.kind,
    retiredEffectIds,
    projectionKind: result.projection.kind,
    projectionDigest: result.projection.projectionDigest,
    retirement: result.retired ?? Object.freeze([]),
  };
  return Object.freeze({
    ...digestSource,
    retirementDigest: stableValueDigest(digestSource.retirement),
    digest: stableValueDigest(digestSource),
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
