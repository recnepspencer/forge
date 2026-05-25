import { readMeasurementSemanticContext } from "./semantic_context.js";

export function createMeasurementSemanticContextReader({
  cache,
  authoritativeSource,
  draft,
  rawInputs,
  parseFailures,
  asyncValidations,
  sourceCompatibility,
  formRef,
}) {
  return function currentMeasurementSemanticContext() {
    return readMeasurementSemanticContext({
      cache,
      authoritativeSource: authoritativeSource(),
      draft: draft(),
      rawInputs,
      parseFailures,
      asyncValidationArtifacts: asyncValidations.artifacts(),
      sourceCompatibilityHistoryLength: sourceCompatibility.history().length,
      form: formRef(),
    });
  };
}
