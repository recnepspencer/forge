import { stableValueDigest } from "./values/value_paths.js";

export function buildFormVerificationPackage(form) {
  const source = form.source();
  const draft = form.draft();
  const effective = form.effective();
  const dirty = form.dirty();
  const patchPlan = form.patchPlan();
  const readiness = form.readiness();
  const validation = form.validation();
  const availability = form.availability();
  const admission = form.admission();
  const steps = form.steps();
  const actions = form.actions();
  const actionHistory = form.actionHistory();
  const actionExecutionHistory = form.actionExecutionHistory();
  const asyncValidationHistory = form.asyncValidationHistory();
  const canonicalizationHistory = form.canonicalizationHistory();
  const digests = Object.freeze({
    sourceAuthorityDigest: stableValueDigest(source),
    draftDigest: stableValueDigest(draft),
    effectiveValueDigest: stableValueDigest(effective),
    semanticEqualityDigest: stableValueDigest(dirty),
    patchPlanDigest: patchPlan.equivalenceDigest,
    readinessDigest: stableValueDigest(readiness.blockers),
    validationDigest: stableValueDigest(validation),
    asyncValidationLifecycleDigest: stableValueDigest(asyncValidationHistory),
    canonicalizationDigest: stableValueDigest(canonicalizationHistory),
    availabilityDependencyDigest: stableValueDigest(availability.dependencyBreadth),
    stepDeclarationProgressDigest: stableValueDigest(steps.artifacts),
    admissionPolicyDigest: stableValueDigest(admission.dependencyBreadth),
    regulatedBindingDigest: stableValueDigest(
      admission.artifacts
        .filter((artifact) => artifact.binding !== undefined)
        .map((artifact) => artifact.binding.bindingDigest),
    ),
    actionCatalogDigest: actions.digests.catalogDigest,
    actionReadinessAdmissionDigest: actions.digests.readinessAdmissionDigest,
    actionPlanDigestSetDigest: actions.digests.planDigestSetDigest,
    submitPlanDigest: actions.digests.submitPlanDigest,
    actionLifecycleDigest: stableValueDigest(actionHistory),
    actionExecutionLifecycleDigest: stableValueDigest(actionExecutionHistory),
    diagnosticsHistoryDigest: stableValueDigest({
      dirty,
      patchPlan,
      readiness,
      actionHistory,
      actionExecutionHistory,
      asyncValidationHistory,
      canonicalizationHistory,
      actionPlanDigests: actions.digests.planDigests,
    }),
  });
  return Object.freeze({
    kind: "formVerification",
    digests,
    actionHistory: Object.freeze({
      attempts: actionHistory.length,
      digest: digests.actionLifecycleDigest,
    }),
    actionExecutionHistory: Object.freeze({
      operations: actionExecutionHistory.length,
      digest: digests.actionExecutionLifecycleDigest,
    }),
    asyncValidationHistory: Object.freeze({
      operations: asyncValidationHistory.length,
      digest: digests.asyncValidationLifecycleDigest,
    }),
    canonicalizationHistory: Object.freeze({
      operations: canonicalizationHistory.length,
      digest: digests.canonicalizationDigest,
    }),
    performanceEnvelope: formPerformanceEnvelope({
      validation,
      availability,
      admission,
      steps,
      actions,
      actionHistory,
      actionExecutionHistory,
      asyncValidationHistory,
      canonicalizationHistory,
    }),
    packageDigest: stableValueDigest(digests),
  });
}

function formPerformanceEnvelope(reports) {
  return Object.freeze({
    costBasis: "derivedFullReportScan",
    diagnosticsSummaryBreadth: "summaryShapedNotFullHistoryMaterialization",
    actionHistoryAttempts: reports.actionHistory.length,
    actionExecutionOperations: reports.actionExecutionHistory.length,
    asyncValidationOperations: reports.asyncValidationHistory.length,
    canonicalizationOperations: reports.canonicalizationHistory.length,
    validation: reports.validation.counters,
    availability: reports.availability.counters,
    admission: reports.admission.counters,
    steps: reports.steps.counters,
    actions: reports.actions.counters,
  });
}
