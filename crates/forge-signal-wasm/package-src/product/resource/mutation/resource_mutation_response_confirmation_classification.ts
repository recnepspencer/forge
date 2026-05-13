const MUTATION_RESPONSE_EXACT_EXECUTION_KINDS = Object.freeze([
  "exactDetail",
  "exactCollectionItem",
  "exactSummary",
]);

function createMutationResponseConfirmationClassification(
  executionArtifacts,
  diagnosticFacts,
) {
  const exactTargetCount = executionArtifacts.filter((artifact) =>
    MUTATION_RESPONSE_EXACT_EXECUTION_KINDS.includes(artifact.kind)).length;
  const fallbackArtifacts = executionArtifacts.filter((artifact) =>
    artifact.kind === "fallback");
  const kind = classifyMutationResponseConfirmation(
    exactTargetCount,
    fallbackArtifacts,
  );
  const fallbackKinds = Object.freeze(
    fallbackArtifacts.map((artifact) => artifact.fallback),
  );
  const detail = createMutationResponseConfirmationDetail(
    kind,
    exactTargetCount,
    fallbackArtifacts.length,
    diagnosticFacts.count,
  );
  return Object.freeze({
    kind,
    detail,
    exactTargetCount,
    fallbackTargetCount: fallbackArtifacts.length,
    diagnosticCount: diagnosticFacts.count,
    fallbackKinds,
    digest: createMutationResponseConfirmationDigest(
      kind,
      exactTargetCount,
      fallbackKinds,
      diagnosticFacts.digest,
    ),
  });
}

function classifyMutationResponseConfirmation(exactTargetCount, fallbackArtifacts) {
  if (exactTargetCount > 0 && fallbackArtifacts.length === 0) {
    return "consumedCanonicalTruth";
  }
  if (exactTargetCount > 0 || hasPartialFallback(fallbackArtifacts)) {
    return "partialCanonicalTruth";
  }
  if (fallbackArtifacts.length === 0) {
    return "preservedOptimisticTruth";
  }
  if (fallbackArtifacts.every((artifact) => artifact.fallback === "deliveryAwaited")) {
    return "deliveryAwaited";
  }
  if (fallbackArtifacts.every((artifact) => artifact.fallback === "refetchRequired")) {
    return "refetchRequired";
  }
  return "partialCanonicalTruth";
}

function hasPartialFallback(fallbackArtifacts) {
  return fallbackArtifacts.some((artifact) =>
    artifact.fallback === "partialReconciliation"
    || artifact.fallback === "unsupportedTarget");
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
