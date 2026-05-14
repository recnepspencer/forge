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
  const host = form.host();
  const inputCapabilities = form.inputCapabilities();
  const exit = form.exit();
  const handoff = form.handoff();
  const attachments = form.attachments();
  const media = form.media();
  const collaboration = form.collaboration();
  const interaction = form.interaction();
  const interactionHistory = interaction.history;
  const navigation = form.navigation();
  const navigationHistory = navigation.history;
  const accessibility = form.accessibility();
  const layout = form.layout();
  const layoutMeasurement = form.layoutMeasurement();
  const presentation = form.presentation();
  const presentationHistory = form.presentationHistory();
  const sourceCompatibility = form.sourceCompatibility();
  const steps = form.steps();
  const actions = form.actions();
  const actionHistory = form.actionHistory();
  const actionExecutionHistory = form.actionExecutionHistory();
  const asyncValidationHistory = form.asyncValidationHistory();
  const canonicalizationHistory = form.canonicalizationHistory();
  const sourceCompatibilityHistory = form.sourceCompatibilityHistory();
  const digests = Object.freeze({
    sourceAuthorityDigest: stableValueDigest(source),
    hostFactDigest: host.digest,
    inputCapabilityDigest: inputCapabilities.digest,
    exitDigest: exit.digest,
    handoffDigest: handoff.digest,
    attachmentDigest: attachments.digest,
    mediaDigest: media.digest,
    collaborationDigest: collaboration.digest,
    interactionDigest: interaction.digest,
    interactionHistoryDigest: stableValueDigest(interactionHistory),
    navigationDigest: navigation.digest,
    navigationHistoryDigest: stableValueDigest(navigationHistory),
    accessibilityDigest: accessibility.digest,
    presentationOrderHintDigest: accessibility.orderDigest,
    layoutDigest: layout.digest,
    layoutMeasurementDigest: layoutMeasurement.digest,
    presentationDigest: presentation.digest,
    presentationSettlementAcknowledgementDigest: presentation.acknowledgements.digest,
    sourceCompatibilityDigest: stableValueDigest(sourceCompatibility),
    draftDigest: stableValueDigest(draft),
    effectiveValueDigest: stableValueDigest(effective),
    semanticEqualityDigest: stableValueDigest(dirty),
    patchPlanDigest: patchPlan.equivalenceDigest,
    readinessDigest: stableValueDigest(readiness.blockers),
    validationDigest: stableValueDigest(validation),
    asyncValidationLifecycleDigest: stableValueDigest(asyncValidationHistory),
    canonicalizationDigest: stableValueDigest(canonicalizationHistory),
    sourceCompatibilityHistoryDigest: stableValueDigest(sourceCompatibilityHistory),
    presentationHistoryDigest: stableValueDigest(presentationHistory),
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
      host,
      inputCapabilities,
      exit,
      handoff,
      attachments,
      media,
      collaboration,
      interaction,
      interactionHistory,
      navigation,
      navigationHistory,
      accessibility,
      layout,
      layoutMeasurement,
      presentation,
      presentationHistory,
      sourceCompatibility,
      sourceCompatibilityHistory,
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
    interactionHistory: Object.freeze({
      operations: interactionHistory.length,
      digest: digests.interactionHistoryDigest,
    }),
    navigationHistory: Object.freeze({
      operations: navigationHistory.length,
      digest: digests.navigationHistoryDigest,
    }),
    presentationHistory: Object.freeze({
      operations: presentationHistory.length,
      digest: digests.presentationHistoryDigest,
    }),
    sourceCompatibilityHistory: Object.freeze({
      operations: sourceCompatibilityHistory.length,
      digest: digests.sourceCompatibilityHistoryDigest,
    }),
    performanceEnvelope: formPerformanceEnvelope({
      sourceCompatibility,
      host,
      inputCapabilities,
      exit,
      handoff,
      attachments,
      media,
      collaboration,
      interaction,
      navigation,
      accessibility,
      layout,
      layoutMeasurement,
      presentation,
      validation,
      availability,
      admission,
      steps,
      actions,
      actionHistory,
      actionExecutionHistory,
      asyncValidationHistory,
      canonicalizationHistory,
      interactionHistory,
      navigationHistory,
      sourceCompatibilityHistory,
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
    interactionOperations: reports.interactionHistory.length,
    navigationOperations: reports.navigationHistory.length,
    sourceCompatibilityOperations: reports.sourceCompatibilityHistory.length,
    hostFacts: reports.host.counters,
    inputCapabilities: reports.inputCapabilities.counters,
    exit: reports.exit.counters,
    handoff: reports.handoff.counters,
    attachments: reports.attachments.counters,
    media: reports.media.counters,
    collaboration: reports.collaboration.counters,
    interaction: reports.interaction.counters,
    navigation: reports.navigation.counters,
    accessibility: reports.accessibility.counters,
    layout: reports.layout.counters,
    layoutMeasurement: reports.layoutMeasurement.counters,
    presentation: reports.presentation.counters,
    sourceCompatibility: reports.sourceCompatibility.counters,
    validation: reports.validation.counters,
    availability: reports.availability.counters,
    admission: reports.admission.counters,
    steps: reports.steps.counters,
    actions: reports.actions.counters,
  });
}
