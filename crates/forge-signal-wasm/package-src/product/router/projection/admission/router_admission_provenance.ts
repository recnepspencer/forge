function createRouteAdmissionPlanProvenance(
  projectedCandidate,
  prerequisiteNames,
  recoveryNames,
  consumedSources,
  normalizedFacts,
) {
  return Object.freeze({
    attemptedRouteId: projectedCandidate.routeId,
    attemptedHref: projectedCandidate.href,
    prerequisiteNames,
    recoveryNames,
    consumedSources,
    factsKeys: Object.freeze(Object.keys(normalizedFacts).sort()),
  });
}

function createRouteOutcomeProvenance({
  attemptedRouteId,
  attemptedHref,
  resolvedRouteId,
  resolvedHref,
  terminalSource,
  terminalArtifact,
  prerequisiteDecisions,
  recoveryTrail,
}) {
  return Object.freeze({
    attemptedRouteId,
    attemptedHref,
    resolvedRouteId,
    resolvedHref,
    terminalSource,
    terminalArtifact,
    prerequisiteDecisions: Object.freeze(prerequisiteDecisions.slice()),
    recoveryTrail: Object.freeze(recoveryTrail.slice()),
  });
}

function createRecoveryTrailEntry(recoveryArtifact, terminalArtifact, originalCandidate, recoveredOutcome) {
  return Object.freeze({
    recovery: recoveryArtifact.recovery,
    href: recoveryArtifact.href,
    reason: recoveryArtifact.reason,
    detail: recoveryArtifact.detail,
    fromArtifactKind: terminalArtifact.kind,
    fromRouteId: originalCandidate.routeId,
    fromHref: originalCandidate.href,
    toRouteId: recoveredOutcome.routeId,
    toHref: recoveredOutcome.href,
  });
}

function createAdmissionDecisionProvenance(projectedCandidate, prerequisiteArtifact, consumedSources) {
  return Object.freeze({
    routeId: projectedCandidate.routeId,
    href: projectedCandidate.href,
    kind: prerequisiteArtifact.kind,
    prerequisite: prerequisiteArtifact.prerequisite,
    artifactHref: prerequisiteArtifact.href,
    reason: prerequisiteArtifact.reason,
    detail: prerequisiteArtifact.detail,
    consumedSources,
  });
}

export {
  createAdmissionDecisionProvenance,
  createRecoveryTrailEntry,
  createRouteAdmissionPlanProvenance,
  createRouteOutcomeProvenance,
};
