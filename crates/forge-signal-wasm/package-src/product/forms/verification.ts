import { stableValueDigest } from "./values/value_paths.js";

export function buildFormVerificationPackage(form) {
  const source = form.source();
  const sourceAuthority = form.sourceAuthority();
  const sourceAdmission = form.sourceAdmission();
  const draftRestore = form.draftRestore();
  const resourceSource = form.resourceSource();
  const resourceMerge = form.resourceMerge();
  const formDeclaration = form.declaration();
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
  const messages = form.messages();
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
  const fieldContract = form.fieldContract();
  const inputAdapters = form.inputAdapters();
  const actionHistory = form.actionHistory();
  const actionExecutionHistory = form.actionExecutionHistory();
  const asyncValidationHistory = form.asyncValidationHistory();
  const canonicalizationHistory = form.canonicalizationHistory();
  const resetHistory = form.resetHistory();
  const sourceCompatibilityHistory = form.sourceCompatibilityHistory();
  const digests = Object.freeze({
    sourceAuthorityDigest: sourceAuthority.sourceAuthorityDigest,
    sourceAuthorityContractDigest: stableValueDigest({
      kind: sourceAuthority.kind,
      sourceId: sourceAuthority.sourceId,
      explicit: sourceAuthority.explicit,
      contract: sourceAuthority.contract,
      identity: sourceAuthority.identity,
    }),
    sourceAdmissionDigest: stableValueDigest(sourceAdmission),
    draftRestoreDigest: stableValueDigest(draftRestore),
    resourceSourceDigest: resourceSource?.digest ?? null,
    resourceMergeDigest: resourceMerge.digest,
    resourceEffectProfileDigest: stableValueDigest(resourceSource?.effectProfile.profile ?? null),
    resourceVisibleBranchSelectionDigest: stableValueDigest(resourceSource?.visibleSelection ?? null),
    resourceVerificationPackageDigest: resourceSource?.verification.packageDigest ?? null,
    resourceEffectCloseoutMatrixDigest: resourceSource?.effectProfile.closeoutMatrixDigest ?? null,
    resourceMutationResponseDigest: resourceSource?.mutationResponse?.digest ?? null,
    resourceMutationResponseConfirmationDigest:
      resourceSource?.mutationResponse?.confirmationDigest ?? null,
    resourceMutationResponseTargetOutcomeDigest:
      resourceSource?.mutationResponse?.targetOutcomeDigest ?? null,
    resourceMutationResponseCloseoutMatrixDigest:
      resourceSource?.verification.mutationResponseCloseoutMatrixDigest ?? null,
    sourceValueDigest: stableValueDigest(source),
    formDeclarationDigest: stableValueDigest(formDeclaration),
    fieldContractDigest: stableValueDigest(fieldContract),
    inputAdapterCapabilityDigest: stableValueDigest(inputAdapters),
    hostFactDigest: host.digest,
    inputCapabilityDigest: inputCapabilities.digest,
    exitDigest: exit.digest,
    handoffDigest: handoff.digest,
    attachmentDigest: attachments.digest,
    mediaDigest: media.digest,
    messageDigest: messages.digest,
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
    resetRollbackDigest: stableValueDigest(
      canonicalizationHistory.map((artifact) => artifact.resourceBacked?.rollback ?? null),
    ),
    resetHistoryDigest: stableValueDigest(resetHistory),
    resourceMergeHistoryDigest: stableValueDigest(resourceMerge.history),
    mutationResponseReconciliationDigest: stableValueDigest(
      canonicalizationHistory.map((artifact) => artifact.resourceBacked?.mutationResponse ?? null),
    ),
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
      sourceAuthority,
      sourceAdmission,
      draftRestore,
      resourceSource,
      resourceMerge,
      formDeclaration,
      fieldContract,
      inputAdapters,
      dirty,
      patchPlan,
      readiness,
      actionHistory,
      actionExecutionHistory,
      asyncValidationHistory,
      canonicalizationHistory,
      resetHistory,
      host,
      inputCapabilities,
      exit,
      handoff,
      attachments,
      media,
      messages,
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
    sourceAuthority,
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
    resetHistory: Object.freeze({
      operations: resetHistory.length,
      digest: digests.resetHistoryDigest,
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
      resourceSource,
      resourceMerge,
      host,
      inputCapabilities,
      exit,
      handoff,
      attachments,
      media,
      messages,
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
      resetHistory,
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
    resetOperations: reports.resetHistory.length,
    interactionOperations: reports.interactionHistory.length,
    navigationOperations: reports.navigationHistory.length,
    sourceCompatibilityOperations: reports.sourceCompatibilityHistory.length,
    resourceSource: reports.resourceSource?.counters ?? null,
    resourceMerge: reports.resourceMerge.counters,
    hostFacts: reports.host.counters,
    inputCapabilities: reports.inputCapabilities.counters,
    exit: reports.exit.counters,
    handoff: reports.handoff.counters,
    attachments: reports.attachments.counters,
    media: reports.media.counters,
    messages: reports.messages.counters,
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
