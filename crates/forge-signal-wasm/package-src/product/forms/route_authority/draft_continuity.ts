export function createRouteAuthorityDraftContinuityArtifact({
  routeId,
  href,
  surfaceId,
  continuityApplied,
  transitionKind,
  previousAuthorityDigest,
  previousDraftDigest,
  nextDraftDigest,
  reason,
}) {
  const draftChanged = previousDraftDigest !== nextDraftDigest;
  return Object.freeze({
    kind: "routeAuthorityDraftContinuity",
    routeId,
    href,
    surfaceId,
    posture: continuityApplied,
    authorityChange: transitionKind,
    draftChanged,
    draftResolution: classifyDraftResolution(continuityApplied, draftChanged),
    previousAuthorityDigest,
    previousDraftDigest,
    nextDraftDigest,
    reason,
  });
}

function classifyDraftResolution(continuityApplied, draftChanged) {
  if (continuityApplied === "discardedDraft") {
    return "replacedFromSource";
  }
  if (continuityApplied === "deferredDraft") {
    return "awaitingAdmittedTruth";
  }
  if (continuityApplied === "frozeDraft") {
    return "preservedFrozenValue";
  }
  if (continuityApplied === "clearedAuthority") {
    return "authorityCleared";
  }
  return "preservedValue";
}

export function routeAuthorityDraftContinuityReason(authority, continuityApplied) {
  if (continuityApplied === "discardedDraft") {
    return authority.reason ?? "route authority replaced route-scoped draft truth from admitted source truth";
  }
  if (continuityApplied === "deferredDraft") {
    return authority.reason
      ?? "route authority deferred route-scoped draft continuity until later admitted truth is present";
  }
  if (continuityApplied === "frozeDraft") {
    return authority.reason ?? "route authority froze route-scoped draft continuity";
  }
  if (continuityApplied === "maintainedAuthority") {
    return authority.reason ?? "route authority refreshed without changing route-scoped draft continuity";
  }
  return authority.reason ?? "route authority preserved route-scoped draft continuity";
}
