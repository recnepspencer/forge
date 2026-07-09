import { stableValueDigest } from "../values/value_paths.js";

export function readMeasurementSemanticContext({
  cache,
  authoritativeSource,
  draft,
  rawInputs,
  parseFailures,
  asyncValidationArtifacts,
  sourceCompatibilityHistoryLength,
  form,
}) {
  const basisKey = measurementBasisKey({
    authoritativeSource,
    draft,
    rawInputs,
    parseFailures,
    asyncValidationArtifacts,
    sourceCompatibilityHistoryLength,
    hostDigest: form.host().digest,
  });
  if (cache.value && cache.value.basisKey === basisKey) {
    return cache.value.snapshot;
  }
  const snapshot = Object.freeze({
    host: form.host(),
    accessibility: form.accessibility(),
    layout: form.layout(),
    semanticDigests: Object.freeze({
      validationDigest: stableValueDigest(form.validation()),
      readinessDigest: stableValueDigest(form.readiness().blockers),
      actionPlanDigestSetDigest: form.actions().digests.planDigestSetDigest,
    }),
  });
  cache.value = Object.freeze({ basisKey, snapshot });
  return snapshot;
}

function measurementBasisKey({
  authoritativeSource,
  draft,
  rawInputs,
  parseFailures,
  asyncValidationArtifacts,
  sourceCompatibilityHistoryLength,
  hostDigest,
}) {
  return stableValueDigest({
    source: authoritativeSource,
    draft,
    rawInputs: [...rawInputs.entries()],
    parseFailureFields: [...parseFailures.keys()],
    asyncValidationArtifacts,
    sourceCompatibilityHistoryLength,
    hostDigest,
  });
}
