const MUTATION_RESPONSE_EXACT_EXECUTION_KINDS = Object.freeze([
  "exactDetail",
  "exactCollectionItem",
  "exactSummary",
]);

function createMutationResponseConfirmationClassification(
  executionArtifacts,
  diagnosticFacts,
  additionalFallbackKinds = [],
  additionalExactTargetCount = 0,
) {
  const exactTargetCount = executionArtifacts.filter((artifact) =>
    MUTATION_RESPONSE_EXACT_EXECUTION_KINDS.includes(artifact.kind)).length
    + additionalExactTargetCount;
  const fallbackArtifacts = executionArtifacts.filter((artifact) =>
    artifact.kind === "fallback");
  const allFallbackKinds = Object.freeze([
    ...fallbackArtifacts.map((artifact) => artifact.fallback),
    ...additionalFallbackKinds,
  ]);
  const kind = classifyMutationResponseConfirmation(
    exactTargetCount,
    allFallbackKinds,
  );
  const detail = createMutationResponseConfirmationDetail(
    kind,
    exactTargetCount,
    allFallbackKinds.length,
    diagnosticFacts.count,
  );
  return Object.freeze({
    kind,
    detail,
    exactTargetCount,
    fallbackTargetCount: allFallbackKinds.length,
    diagnosticCount: diagnosticFacts.count,
    fallbackKinds: allFallbackKinds,
    digest: createMutationResponseConfirmationDigest(
      kind,
      exactTargetCount,
      allFallbackKinds,
      diagnosticFacts.digest,
    ),
  });
}

function classifyMutationResponseConfirmation(exactTargetCount, fallbackKinds) {
  if (exactTargetCount > 0 && fallbackKinds.length === 0) {
    return "consumedCanonicalTruth";
  }
  if (exactTargetCount > 0 || hasPartialFallback(fallbackKinds)) {
    return "partialCanonicalTruth";
  }
  if (fallbackKinds.length === 0) {
    return "preservedOptimisticTruth";
  }
  if (fallbackKinds.every((fallback) => fallback === "deliveryAwaited")) {
    return "deliveryAwaited";
  }
  if (fallbackKinds.every((fallback) => fallback === "refetchRequired")) {
    return "refetchRequired";
  }
  return "partialCanonicalTruth";
}

function hasPartialFallback(fallbackKinds) {
  return fallbackKinds.some((fallback) =>
    fallback === "partialReconciliation"
    || fallback === "unsupportedTarget"
    || fallback === "identityMigrationUnavailable");
}

function createMutationResponseConfirmationDetail(
  kind,
  exactTargetCount,
  fallbackTargetCount,
  diagnosticCount,
) {
  return [
    `mutation response classified as ${kind}`,
    `with ${exactTargetCount} exact target(s)`,
    `${fallbackTargetCount} fallback target(s)`,
    `and ${diagnosticCount} diagnostic fact(s)`,
  ].join(" ");
}

function createMutationResponseConfirmationDigest(
  kind,
  exactTargetCount,
  fallbackKinds,
  diagnosticDigest,
) {
  return [
    "mutation-response-confirmation",
    kind,
    `exact:${exactTargetCount}`,
    `fallbacks:${fallbackKinds.join(",") || "none"}`,
    `diagnostics:${diagnosticDigest}`,
  ].join("|");
}

export { createMutationResponseConfirmationClassification };
