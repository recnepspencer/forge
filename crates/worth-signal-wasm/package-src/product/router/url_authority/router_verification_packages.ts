function createRouteReferenceVerification(declaration, descriptor) {
  const routeSchemaDigest = createDigest("route-schema", {
    route: descriptor.route,
    pathParamNames: descriptor.pathParamNames,
    search: Object.entries(declaration.search).map(([key, field]) => ({
      key,
      valueKind: field.valueKind,
      required: field.required,
    })),
    hash: declaration.hash === null ? null : { valueKind: declaration.hash.valueKind },
  });
  const routeDeclarationDigest = createDigest("route-declaration", {
    routeId: descriptor.routeId,
    declarationPath: descriptor.declarationPath,
    routeSchemaDigest,
  });
  const routeReferenceDigest = createDigest("route-reference", {
    routeId: descriptor.routeId,
    scopeId: descriptor.scopeId,
    routeDeclarationDigest,
  });
  return Object.freeze({
    routeId: descriptor.routeId,
    routeSchemaDigest,
    routeDeclarationDigest,
    routeReferenceDigest,
  });
}

function createRawLocationVerification(rawLocationAuthority, canonicalAuthority) {
  return Object.freeze({
    rawLocationDigest: createDigest("raw-location", {
      href: rawLocationAuthority.href,
      pathname: rawLocationAuthority.pathname,
      searchParams: rawLocationAuthority.searchParams,
      hashFragment: rawLocationAuthority.hashFragment ?? null,
      navigationType: rawLocationAuthority.navigationType,
    }),
    canonicalUrlDigest: canonicalAuthority.canonicalUrlDigest,
    equivalenceDigest: canonicalAuthority.equivalenceDigest,
  });
}

function createCanonicalVerification(canonicalAuthority) {
  return Object.freeze({
    canonicalUrlDigest: canonicalAuthority.canonicalUrlDigest,
    equivalenceDigest: canonicalAuthority.equivalenceDigest,
    searchDigest: canonicalAuthority.searchDigest,
    hashDigest: canonicalAuthority.hashDigest,
  });
}

function createRouteCanonicalVerification(referenceVerification, canonicalArtifact) {
  return Object.freeze({
    routeId: canonicalArtifact.routeId,
    routeSchemaDigest: referenceVerification.routeSchemaDigest,
    routeDeclarationDigest: referenceVerification.routeDeclarationDigest,
    routeReferenceDigest: referenceVerification.routeReferenceDigest,
    canonicalUrlDigest: canonicalArtifact.canonicalUrlDigest,
    equivalenceDigest: canonicalArtifact.equivalenceDigest,
    searchDigest: canonicalArtifact.searchDigest,
    hashDigest: canonicalArtifact.hashDigest,
  });
}

function createNavigationIntentVerification(
  referenceVerification,
  canonicalArtifact,
  intentKind,
  policy,
) {
  const transitionPolicy = {
    commit: policy.commit,
    redirect: policy.redirect,
    continuity: policy.continuity,
    deployment: policy.deployment,
  };
  const freshnessPolicy = {
    projectionRefresh: policy.projectionRefresh,
    continuity: policy.continuity,
    commit: policy.commit,
    redirect: policy.redirect,
    deployment: policy.deployment,
  };
  const historyEffect = deriveNavigationHistoryEffect(intentKind);
  return Object.freeze({
    routeId: canonicalArtifact.routeId,
    routeSchemaDigest: referenceVerification.routeSchemaDigest,
    routeDeclarationDigest: referenceVerification.routeDeclarationDigest,
    routeReferenceDigest: referenceVerification.routeReferenceDigest,
    canonicalUrlDigest: canonicalArtifact.canonicalUrlDigest,
    equivalenceDigest: canonicalArtifact.equivalenceDigest,
    navigationIntentDigest: createDigest("navigation-intent", {
      routeId: canonicalArtifact.routeId,
      intentKind,
      href: canonicalArtifact.href,
    }),
    navigationPolicyDigest: createDigest("navigation-policy", policy),
    navigationTransitionPolicyDigest: createDigest(
      "navigation-transition-policy",
      transitionPolicy,
    ),
    navigationFreshnessPolicyDigest: createDigest(
      "navigation-freshness-policy",
      freshnessPolicy,
    ),
    navigationHistoryEffectDigest: createDigest("navigation-history-effect", {
      intentKind,
      historyEffect,
    }),
    navigationExecutionContractDigest: createDigest("navigation-execution-contract", {
      intentKind,
      historyEffect,
      routeTruthEffect: explanationlessRouteTruthEffect(intentKind),
      visibleProjectionEffect: explanationlessVisibleProjectionEffect(policy),
      artifactEffect:
        policy.artifactPolicy === "diagnostics"
          ? "materialize-diagnostic-navigation-artifacts"
          : "materialize-minimal-navigation-artifacts",
      commitBoundary: policy.commit,
      redirectBoundary: policy.redirect,
      deployment: policy.deployment,
    }),
  });
}

function createNavigationPlanVerification(
  referenceVerification,
  canonicalArtifact,
  intentKind,
  policy,
  explanation,
) {
  const intentVerification = createNavigationIntentVerification(
    referenceVerification,
    canonicalArtifact,
    intentKind,
    policy,
  );
  return Object.freeze({
    ...intentVerification,
    navigationPlanDigest: createDigest("navigation-plan", {
      routeId: canonicalArtifact.routeId,
      intentKind,
      href: canonicalArtifact.href,
      policy,
    }),
    navigationExplainabilityDigest: createDigest("navigation-explainability", explanation),
    navigationFreshnessDigest: createDigest(
      "navigation-freshness",
      explanation.freshness,
    ),
    navigationContinuityAttributionDigest: createDigest(
      "navigation-continuity-attribution",
      {
        continuityAttribution: explanation.freshness.continuityAttribution,
        staleVisibilityReason: explanation.freshness.staleVisibilityReason,
        visibleFreshness: explanation.freshness.visibleFreshness,
      },
    ),
  });
}

function deriveNavigationHistoryEffect(intentKind) {
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

function explanationlessRouteTruthEffect(intentKind) {
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

function explanationlessVisibleProjectionEffect(policy) {
  switch (policy.projectionRefresh) {
    case "after-admission":
      return "refresh-visible-projection-after-admission";
    case "explicit":
      return "preserve-visible-projection-until-explicit-refresh";
    default:
      return "refresh-visible-projection-immediately";
  }
}

function createDigest(label, value) {
  return `worth-router:${label}:${JSON.stringify(value)}`;
}

function createCanonicalDigest(label, value) {
  return createDigest(label, value);
}

export {
  createCanonicalDigest,
  createCanonicalVerification,
  createNavigationIntentVerification,
  createNavigationPlanVerification,
  createRawLocationVerification,
  createRouteCanonicalVerification,
  createRouteReferenceVerification,
};
