import { cloneFormValue, isPlainObject, stableValueDigest } from "./values/value_paths.js";

export function createCanonicalizationStore() {
  let nextCanonicalizationId = 1;
  let canonicalSource = null;
  const history = [];
  return Object.freeze({
    sourceFor(rawSource) {
      if (!isCanonicalSourceCurrent(rawSource)) {
        return cloneFormValue(rawSource);
      }
      return cloneFormValue(canonicalSource.value);
    },
    applyFulfilledAction(execution, previousSource, previousDraft, nextDraft, draftClearedFields, rawSource) {
      if (execution.resultKind !== "fulfilled" || execution.canonicalValue === undefined) {
        return null;
      }
      const artifact = canonicalizationArtifact({
        canonicalizationId: nextCanonicalizationId++,
        operationId: execution.operationId,
        action: execution.action,
        planDigest: execution.planDigest,
        previousSource,
        previousDraft,
        nextDraft,
        draftClearedFields,
        rawSource,
        canonicalValue: execution.canonicalValue,
        resourceSubmission: execution.resourceSubmission,
        reason: execution.reason,
      });
      canonicalSource = Object.freeze({
        value: cloneCanonicalValue(execution.canonicalValue),
        sourceBasisDigest: artifact.sourceBasisDigest,
      });
      history.push(artifact);
      return artifact;
    },
    history() {
      return Object.freeze([...history]);
    },
  });

  function isCanonicalSourceCurrent(rawSource) {
    return (
      canonicalSource !== null &&
      stableValueDigest(rawSource) === canonicalSource.sourceBasisDigest
    );
  }
}

function canonicalizationArtifact(options) {
  const sourceBasisDigest = stableValueDigest(options.rawSource);
  const canonicalValue = cloneCanonicalValue(options.canonicalValue);
  const artifact = {
    kind: "canonicalization",
    canonicalizationId: options.canonicalizationId,
    observedAtMs: Date.now(),
    operationId: options.operationId,
    action: options.action,
    planDigest: options.planDigest,
    previousSourceDigest: stableValueDigest(options.previousSource),
    previousDraftDigest: stableValueDigest(options.previousDraft),
    previousDraftValue: cloneCanonicalValue(options.previousDraft),
    nextDraftDigest: stableValueDigest(options.nextDraft),
    nextDraftValue: cloneCanonicalValue(options.nextDraft),
    sourceBasisDigest,
    canonicalSourceDigest: stableValueDigest(canonicalValue),
    canonicalValue,
    resourceLine: options.resourceSubmission === null || options.resourceSubmission === undefined
      ? null
      : Object.freeze({
        sourceKind: options.resourceSubmission.sourceKind,
        effectProfile: options.resourceSubmission.effectProfile,
        rollback: options.resourceSubmission.rollback,
        visibleSelection: options.resourceSubmission.visibleSelection,
        mutationResponse: options.resourceSubmission.mutationResponse,
        verification: options.resourceSubmission.verification,
        resourceSubmissionDigest: options.resourceSubmission.digest,
      }),
    draftReset: Object.keys(options.nextDraft).length === 0,
    draftClearedFields: Object.freeze([...(options.draftClearedFields ?? [])]),
    sourceProjection: resolveSourceProjection(options.resourceSubmission),
    reason: options.reason,
  };
  return Object.freeze({
    ...artifact,
    canonicalizationDigest: stableValueDigest(artifact),
  });
}

function resolveSourceProjection(resourceSubmission) {
  const confirmationKind = resourceSubmission?.mutationResponse?.confirmationKind ?? null;
  switch (confirmationKind) {
    case null:
    case "consumedCanonicalTruth":
      return "serverCanonicalUntilAuthoritativeSourceDrift";
    case "preservedOptimisticTruth":
      return "resourceMutationResponsePreservedOptimisticTruth";
    case "partialCanonicalTruth":
      return "resourceMutationResponsePartialCanonicalTruth";
    case "refetchRequired":
      return "resourceMutationResponseRefetchRequired";
    case "deliveryAwaited":
      return "resourceMutationResponseDeliveryAwaited";
    default:
      throw new TypeError(`unsupported resource mutation-response confirmation kind "${confirmationKind}"`);
  }
}

function cloneCanonicalValue(canonicalValue) {
  return freezeCanonicalValue(cloneFormValue(canonicalValue));
}

function freezeCanonicalValue(value) {
  if (Array.isArray(value)) {
    for (const entry of value) {
      freezeCanonicalValue(entry);
    }
    return Object.freeze(value);
  }
  if (isPlainObject(value)) {
    for (const entry of Object.values(value)) {
      freezeCanonicalValue(entry);
    }
    return Object.freeze(value);
  }
  return value;
}
