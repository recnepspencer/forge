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

function createMutationResponseLifecycleProof(executionArtifacts) {
  const entries = Object.freeze(
    executionArtifacts.map((artifact) =>
      createMutationResponseLifecycleProofEntry(artifact)),
  );
  const rollbackDigest = createRollbackDigest(entries);
  const mergeRebaseDigest = createMergeRebaseDigest(entries);
  return Object.freeze({
    entries,
    count: entries.length,
    rollbackDigest,
    mergeRebaseDigest,
    digest: [
      "mutation-response-lifecycle",
      rollbackDigest,
      mergeRebaseDigest,
    ].join("|"),
  });
}

function createMutationResponseLifecycleProofEntry(artifact) {
  if (artifact.kind === "fallback") {
    return Object.freeze({
      targetId: artifact.targetId,
      effectId: null,
      authorityDigest: null,
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
      targetId: artifact.targetId,
      effectId: null,
      authorityDigest: null,
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
    targetId: artifact.targetId,
    effectId: artifact.effectProof.effectId,
    authorityDigest: artifact.effectProof.authorityDigest,
    rollback: artifact.effectProof.rollback,
    mergeRebase: artifact.effectProof.mergeRebase,
    digest: artifact.effectProof.digest,
  });
}

function readMutationResponseRollbackProof(effect) {
  const rollback = effect.optimistic.rollback;
  return Object.freeze({
    kind: rollback.kind,
    mode: "mode" in rollback ? rollback.mode : null,
    branchId: "branchId" in rollback ? rollback.branchId : null,
    snapshotId: "snapshotId" in rollback ? rollback.snapshotId : null,
    inverseKind:
      "inverse" in rollback && rollback.inverse !== null
        ? rollback.inverse.kind
        : null,
    detail: rollback.detail,
  });
}

function readMutationResponseMergeRebaseProof(effect) {
  const profileRebase = effect.profile?.rebase ?? "unavailable";
  return Object.freeze({
    kind: profileRebase === "nativeMergePlan"
      ? "nativeMergePlan"
      : "unavailable",
    granularity: readMergeRebaseGranularity(effect),
    locusKind: effect.locus.kind,
    locusProofDigest: effect.locusProof?.effectLocusDigest ?? null,
    detail: profileRebase === "nativeMergePlan"
      ? "mutation response target inherits native merge/rebase planning from the resource effect locus"
      : "mutation response target has no native merge/rebase profile",
  });
}

function readMergeRebaseGranularity(effect) {
  switch (effect.locus.kind) {
    case "detailField":
      return `field:${effect.locus.field}`;
    case "detailRegion":
      return `region:${effect.locus.region}:${readRegionMergeGranularity(effect)}`;
    case "detailJsonPath":
      return `jsonPath:${effect.locus.path}`;
    case "membership":
    case "entityStore":
    case "connection":
    case "discriminatedTuple":
    case "groupedCollection":
    case "mapCollection":
    case "namedCollection":
    case "recursiveTree":
    case "sparsePage":
      return `item:${effect.locus.itemId}`;
    case "summary":
      return `summary:${effect.locus.summary}`;
    default:
      return effect.locus.kind;
  }
}

function readRegionMergeGranularity(effect) {
  const mergeGranularity = effect.patch.region?.mergeGranularity;
  if (typeof mergeGranularity !== "string" || mergeGranularity.length === 0) {
    throw new TypeError(
      "mutation response lifecycle proof requires detail region mergeGranularity",
    );
  }
  return mergeGranularity;
}

function createTargetEffectProofDigest(effect, artifact, rollback, mergeRebase) {
  return [
    artifact.targetId,
    effect.effectId,
    rollback.kind,
    rollback.mode ?? "none",
    mergeRebase.kind,
    mergeRebase.granularity,
    effect.authority.envelopeDigest,
  ].join("|");
}

function createRollbackDigest(entries) {
  if (entries.length === 0) {
    return "mutation-response-rollback|none";
  }
  return `mutation-response-rollback|${entries.map((entry) =>
    [
      entry.targetId,
      entry.effectId ?? "none",
      entry.rollback.kind,
      entry.rollback.mode ?? "none",
      entry.rollback.branchId ?? "none",
      entry.rollback.snapshotId ?? "none",
      entry.rollback.inverseKind ?? "none",
    ].join(":")).join(",")}`;
}

function createMergeRebaseDigest(entries) {
  if (entries.length === 0) {
    return "mutation-response-merge-rebase|none";
  }
  return `mutation-response-merge-rebase|${entries.map((entry) =>
    [
      entry.targetId,
      entry.effectId ?? "none",
      entry.mergeRebase.kind,
      entry.mergeRebase.granularity,
      entry.mergeRebase.locusKind ?? "none",
      entry.mergeRebase.locusProofDigest ?? "none",
    ].join(":")).join(",")}`;
}

export {
  createMutationResponseLifecycleProof,
  createMutationResponseTargetEffectProof,
};
