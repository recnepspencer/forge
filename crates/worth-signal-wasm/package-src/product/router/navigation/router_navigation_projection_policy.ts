function createNavigationCost(location, intentKind, policy) {
  const routeDelta =
    Object.keys(location.params).length +
    Object.keys(location.search).length +
    (location.hash === undefined ? 0 : 1);
  const costClass = createNavigationCostClass(intentKind, policy);
  return Object.freeze({
    costClass,
    intentKind,
    routeDelta,
    projectionRefresh: policy.projectionRefresh,
    continuity: policy.continuity,
    commit: policy.commit,
    redirect: policy.redirect,
    artifactPolicy: policy.artifactPolicy,
    looksExpensive: costClass !== "url-only-navigation",
  });
}

function createNavigationCostClass(intentKind, policy) {
  if (policy.commit === "speculativeBranch") {
    return "speculative-transition";
  }
  if (intentKind === "restoreBack" || intentKind === "breadcrumbReturn") {
    return "restore-navigation";
  }
  if (
    policy.projectionRefresh === "explicit"
    || policy.continuity === "preserve-visible-until-explicit-refresh"
  ) {
    return "explicit-visible-staleness";
  }
  if (
    policy.projectionRefresh === "after-admission"
    || policy.continuity === "preserve-visible-while-pending"
  ) {
    return "deferred-visible-refresh";
  }
  return "url-only-navigation";
}

function createTransitionPolicy(policy) {
  return Object.freeze({
    navigationFamily: deriveNavigationFamily(policy.intentKind),
    historyEffect: deriveHistoryEffect(policy.intentKind),
    artifactPolicy: policy.artifactPolicy,
    commit: policy.commit,
    redirect: policy.redirect,
    continuity: policy.continuity,
    deployment: policy.deployment,
  });
}

function createExecutionContract(policy) {
  return Object.freeze({
    navigationFamily: deriveNavigationFamily(policy.intentKind),
    historyEffect: deriveHistoryEffect(policy.intentKind),
    routeTruthEffect: deriveRouteTruthEffect(policy.intentKind),
    visibleProjectionEffect: deriveVisibleProjectionEffect(policy),
    artifactEffect: deriveArtifactEffect(policy),
    commitBoundary: policy.commit,
    redirectBoundary: policy.redirect,
    deployment: policy.deployment,
  });
}

function createProjectionPolicy(policy) {
  const visibleFreshness = deriveVisibleFreshness(policy);
  return Object.freeze({
    projectionRefresh: policy.projectionRefresh,
    continuity: policy.continuity,
    deployment: policy.deployment,
    visibleFreshness,
    admittedRouteTruth:
      visibleFreshness === "freshly-refreshed"
        ? "converges-with-visible-refresh"
        : "may-advance-before-visible-refresh",
    refreshAttribution: deriveRefreshAttribution(policy),
    continuityAttribution: deriveContinuityAttribution(policy),
  });
}

function createFreshnessDiagnostics(policy) {
  const visibleFreshness = deriveVisibleFreshness(policy);
  return Object.freeze({
    visibleFreshness,
    admittedRouteTruth:
      visibleFreshness === "freshly-refreshed"
        ? "converges-with-visible-refresh"
        : "may-advance-before-visible-refresh",
    projectionRefresh: policy.projectionRefresh,
    continuity: policy.continuity,
    commit: policy.commit,
    redirect: policy.redirect,
    deployment: policy.deployment,
    refreshAttribution: deriveRefreshAttribution(policy),
    continuityAttribution: deriveContinuityAttribution(policy),
    staleVisibilityReason: deriveStaleVisibilityReason(policy),
  });
}

function deriveVisibleFreshness(policy) {
  if (
    policy.projectionRefresh === "explicit"
    || policy.continuity === "preserve-visible-until-explicit-refresh"
  ) {
    return "intentionally-stale";
  }
  if (
    policy.projectionRefresh === "after-admission"
    || policy.continuity === "preserve-visible-while-pending"
  ) {
    return "continuity-preserved";
  }
  return "freshly-refreshed";
}

function deriveRefreshAttribution(policy) {
  switch (policy.projectionRefresh) {
    case "after-admission":
      return "refreshes-visible-projection-after-admission";
    case "explicit":
      return "requires-explicit-visible-refresh";
    default:
      return "refreshes-visible-projection-immediately";
  }
}

function deriveContinuityAttribution(policy) {
  switch (policy.continuity) {
    case "preserve-visible-while-pending":
      return "preserve-visible-while-pending";
    case "preserve-visible-until-explicit-refresh":
      return "preserve-visible-until-explicit-refresh";
    default:
      return "no-visible-continuity-preservation";
  }
}

function deriveStaleVisibilityReason(policy) {
  if (policy.projectionRefresh === "explicit") {
    return "waiting-for-explicit-refresh";
  }
  if (policy.projectionRefresh === "after-admission") {
    return "waiting-for-admission-refresh";
  }
  return null;
}

function deriveNavigationFamily(intentKind) {
  switch (intentKind) {
    case "canonicalize":
      return "canonicalization";
    case "softRefresh":
      return "soft-refresh";
    case "sameRouteMutation":
      return "same-route-mutation";
    case "breadcrumbReturn":
    case "restoreBack":
      return "restore-navigation";
    default:
      return "direct-route";
  }
}

function deriveHistoryEffect(intentKind) {
  switch (intentKind) {
    case "replace":
    case "canonicalize":
      return "replacestate";
    case "softRefresh":
    case "sameRouteMutation":
      return "none";
    default:
      return "pushstate";
  }
}

function deriveRouteTruthEffect(intentKind) {
  switch (intentKind) {
    case "canonicalize":
      return "canonicalize-admitted-route-truth";
    case "softRefresh":
      return "re-admit-current-route-truth";
    case "sameRouteMutation":
      return "re-admit-current-route-with-mutation";
    case "breadcrumbReturn":
    case "restoreBack":
      return "restore-admitted-route-truth";
    default:
      return "advance-admitted-route-truth";
  }
}

function deriveVisibleProjectionEffect(policy) {
  switch (policy.projectionRefresh) {
    case "after-admission":
      return "refresh-visible-projection-after-admission";
    case "explicit":
      return "preserve-visible-projection-until-explicit-refresh";
    default:
      return "refresh-visible-projection-immediately";
  }
}

function deriveArtifactEffect(policy) {
  return policy.artifactPolicy === "diagnostics"
    ? "materialize-diagnostic-navigation-artifacts"
    : "materialize-minimal-navigation-artifacts";
}

function createNavigationExplanation(
  location,
  intentKind,
  policy,
  transitionPolicy,
  executionContract,
  cost,
  projectionPolicy,
  freshness,
) {
  return Object.freeze({
    kind: intentKind,
    routeId: location.routeId,
    href: location.href,
    deployment: policy.deployment,
    artifactPolicy: policy.artifactPolicy,
    canonical: location.canonical(),
    transitionPolicy,
    executionContract,
    projectionPolicy,
    freshness,
    cost,
  });
}

export {
  createFreshnessDiagnostics,
  createNavigationCost,
  createExecutionContract,
  createNavigationExplanation,
  createProjectionPolicy,
  createTransitionPolicy,
};
