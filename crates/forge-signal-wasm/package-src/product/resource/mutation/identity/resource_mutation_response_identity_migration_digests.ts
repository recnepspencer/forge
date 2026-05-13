function createIdentityMigrationTargetDigest(plannedTargets) {
  if (plannedTargets.length === 0) {
    return "mutation-response-identity-targets|none";
  }
  return `mutation-response-identity-targets|${plannedTargets.map((target) =>
    `${target.targetId}:${target.family.kind}:${target.family.familyId}:${createIdentityMigrationScopeDigest(target.scope)}:${target.line.canonicalKey}:${target.outcome}`).join(",")}`;
}

function createIdentityMigrationFallbackDigest(plannedTargets, migrationNeeded) {
  if (!migrationNeeded) {
    return "mutation-response-identity-fallbacks|none";
  }
  const fallbackTargets = plannedTargets.filter((target) => target.outcome === "fallback");
  if (plannedTargets.length === 0) {
    return "mutation-response-identity-fallbacks|route:identityMigrationUnavailable";
  }
  if (fallbackTargets.length === 0) {
    return "mutation-response-identity-fallbacks|none";
  }
  return `mutation-response-identity-fallbacks|${fallbackTargets.map((target) =>
    `${target.targetId}:${target.fallback}:${target.line.canonicalKey}`).join(",")}`;
}

function createIdentityMigrationDeclarationDigest(route, method, atomicity, targets) {
  if (targets.length === 0) {
    return `mutation-response-identity|${route}|${method}|${atomicity}|targets:none`;
  }
  return `mutation-response-identity|${route}|${method}|${atomicity}|targets:${targets.map((target) =>
    `${target.targetId}:${target.family.kind}:${target.family.familyId}:${createIdentityMigrationScopeDigest(target.scope)}:${target.fallback}`).join(",")}`;
}

function createIdentityMigrationTargetIdentityDigest(
  target,
  lineIdentity,
  outcome,
  staleness,
) {
  return [
    target.targetId,
    target.family.kind,
    target.family.familyId,
    createIdentityMigrationScopeDigest(target.scope),
    lineIdentity.canonicalKey,
    outcome === "fallback"
      ? staleness === null
        ? target.fallback
        : `stale:${staleness.reason}`
      : outcome,
  ].join("|");
}

function createIdentityMigrationScopeDigest(scope) {
  if (scope.kind === "residentLine" || scope.kind === "visibleSelection") {
    return scope.kind;
  }
  if (scope.kind === "summary") {
    return `${scope.kind}:${scope.summary}`;
  }
  return `${scope.kind}:${scope.region}`;
}

function createIdentityDigest(kind, value) {
  return `mutation-response-identity-${kind}|${value ?? "none"}`;
}

export {
  createIdentityDigest,
  createIdentityMigrationDeclarationDigest,
  createIdentityMigrationFallbackDigest,
  createIdentityMigrationTargetDigest,
  createIdentityMigrationTargetIdentityDigest,
};
