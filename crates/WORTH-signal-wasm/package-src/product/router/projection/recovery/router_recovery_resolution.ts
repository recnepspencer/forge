function flattenRouteRecoveryDeclarations(matchedDeclarations) {
  return Object.freeze(matchedDeclarations.flatMap((declaration) => declaration.recovery));
}

async function resolveNearestValidRouteRecovery(
  projectedCandidate,
  terminalArtifact,
  recoveryDeclarations,
  normalizedFacts,
  projectRouteCandidate,
) {
  if (terminalArtifact.kind === "redirect" || recoveryDeclarations.length === 0) {
    return null;
  }
  for (const recoveryDeclaration of recoveryDeclarations) {
    const recoveryArtifact = normalizeRouteRecoveryArtifact(
      recoveryDeclaration.name,
      await recoveryDeclaration.evaluate(
        createRouteRecoveryEvaluationContext(projectedCandidate, terminalArtifact, normalizedFacts),
      ),
    );
    if (recoveryArtifact === null) {
      continue;
    }
    const recoveredCandidate = projectRouteCandidate(recoveryArtifact.href);
    if (recoveredCandidate === null) {
      throw new TypeError(
        `route recovery "${recoveryDeclaration.name}" returned fallback href "${recoveryArtifact.href}" that does not project a declared route candidate`,
      );
    }
    return {
      recoveryArtifact,
      recoveredCandidate,
    };
  }
  return null;
}

function createRouteRecoveryEvaluationContext(projectedCandidate, terminalArtifact, normalizedFacts) {
  return Object.freeze({
    routeId: projectedCandidate.routeId,
    href: projectedCandidate.href,
    params: projectedCandidate.route().params,
    search: projectedCandidate.route().search,
    hash: projectedCandidate.route().hash,
    facts: normalizedFacts,
    terminalArtifact,
    fallback(options) {
      return {
        kind: "fallback",
        href: requireRecoveryHref(options?.href),
        reason: normalizeRecoveryText("reason", options?.reason),
        detail: options?.detail === undefined || options.detail === null
          ? null
          : normalizeRecoveryText("detail", options.detail),
      };
    },
  });
}

function normalizeRouteRecoveryArtifact(recoveryName, value) {
  if (value === null || value === undefined) {
    return null;
  }
  if (!value || typeof value !== "object" || Array.isArray(value) || value.kind !== "fallback") {
    throw new TypeError(
      `route recovery "${recoveryName}" must return null or a fallback artifact created from the evaluation context`,
    );
  }
  return Object.freeze({
    kind: "fallback",
    recovery: recoveryName,
    href: requireRecoveryHref(value.href),
    reason: normalizeRecoveryText("reason", value.reason),
    detail: value.detail === undefined || value.detail === null
      ? null
      : normalizeRecoveryText("detail", value.detail),
  });
}

function requireRecoveryHref(href) {
  if (typeof href !== "string" || href.length === 0 || !href.startsWith("/")) {
    throw new TypeError("route recovery fallback artifacts require a local href starting with /");
  }
  return href;
}

function normalizeRecoveryText(field, value) {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new TypeError(`route recovery ${field} must be a non-empty string`);
  }
  return value;
}

export {
  flattenRouteRecoveryDeclarations,
  resolveNearestValidRouteRecovery,
};
