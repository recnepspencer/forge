import { digestFormDiagnosticsProof } from "./diagnostics/digests.js";
import { stableValueDigest } from "./values/value_paths.js";
import { digestFormDiagnosticsHistory } from "./diagnostics/history.js";
import { digestFormStateHistory } from "./state_history.js";

export function buildFormVerificationPackage(form, diagnosticsSnapshot) {
  const state = diagnosticsSnapshot.state;
  const source = form.source();
  const sourceAuthority = state.sourceAuthority;
  const sourceAdmission = form.sourceAdmission();
  const draftRestore = form.draftRestore();
  const resourceSource = state.resourceSource;
  const resourceMerge = state.resourceMerge;
  const resourceDrift = state.resourceDrift;
  const attachmentTransfers = state.attachmentTransfers;
  const formDeclaration = state.declaration;
  const draft = form.draft();
  const effective = form.effective();
  const dirty = state.dirty;
  const patchPlan = state.patchPlan;
  const readiness = state.readiness;
  const validation = state.validation;
  const availability = state.availability;
  const admission = state.admission;
  const host = state.host;
  const inputCapabilities = state.inputCapabilities;
  const exit = state.exit;
  const handoff = state.handoff;
  const attachments = state.attachments;
  const media = state.media;
  const messages = state.messages;
  const collaboration = state.collaboration;
  const interaction = state.interaction;
  const interactionHistory = interaction.history;
  const navigation = state.navigation;
  const navigationHistory = navigation.history;
  const accessibility = state.accessibility;
  const layout = state.layout;
  const layoutMeasurement = state.layoutMeasurement;
  const presentation = state.presentation;
  const presentationHistory = state.presentationHistory;
  const sourceCompatibility = state.sourceCompatibility;
  const steps = state.steps;
  const actions = state.actions;
  const fieldContract = state.fieldContract;
  const inputAdapters = state.inputAdapters;
  const actionHistory = state.actionHistory;
  const actionExecutionHistory = state.actionExecutionHistory;
  const asyncValidationHistory = state.asyncValidationHistory;
  const canonicalizationHistory = state.canonicalizationHistory;
  const resetHistory = state.resetHistory;
  const stateHistory = state.stateHistory;
  const replayRestoreHistory = state.replayRestoreHistory;
  const sourceCompatibilityHistory = state.sourceCompatibilityHistory;
  const diagnosticsSummary = diagnosticsSnapshot.summary;
  const diagnosticsHistory = diagnosticsSnapshot.history;
  const diagnosticsDigest = digestFormDiagnosticsProof(
    diagnosticsSummary,
    diagnosticsSnapshot.diagnosticsStateDigest,
    diagnosticsHistory,
  );
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
    resourceDriftDigest: resourceDrift.digest,
    resourceShapeDigest: resourceSource?.shape.digest ?? null,
    resourceLifecycleDigest: resourceSource?.lifecycle.digest ?? null,
    resourceSettlementDigest: resourceSource?.settlement.digest ?? null,
    resourceEffectProfileDigest: stableValueDigest(resourceSource?.effectProfile.profile ?? null),
    resourceExternalCompatibilityDigest: resourceSource?.externalCompatibility.digest ?? null,
    resourceTransferDigest: resourceSource?.transfer.digest ?? null,
    resourceVisibleBranchSelectionDigest: resourceSource?.visibleSelection.digest ?? stableValueDigest(null),
    resourceVerificationPackageDigest: resourceSource?.verification.packageDigest ?? null,
    resourceEffectCloseoutMatrixDigest: resourceSource?.effectProfile.closeoutMatrixDigest ?? null,
    resourceMutationResponseDigest: resourceSource?.mutationResponse?.digest ?? null,
    resourceMutationResponseConfirmationDigest:
      resourceSource?.mutationResponse?.confirmationDigest ?? null,
    resourceMutationResponseContractDigest:
      resourceSource?.mutationResponse?.contract.digest ?? null,
    resourceMutationResponseCompletionDigest:
      resourceSource?.mutationResponse?.completion.digest ?? null,
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
    attachmentTransferDigest: attachmentTransfers.digest,
    mediaDigest: media.digest,
    messageDigest: messages.digest,
    collaborationDigest: collaboration.digest,
    collaborationEventDigest: collaboration.eventsDigest,
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
    replayRestoreDigest: stableValueDigest(replayRestoreHistory.at(-1) ?? null),
    resetRollbackDigest: stableValueDigest(
      canonicalizationHistory.map((artifact) => artifact.resourceLine?.rollback ?? null),
    ),
    resetHistoryDigest: stableValueDigest(resetHistory),
    replayRestoreHistoryDigest: stableValueDigest(replayRestoreHistory),
    stateHistoryDigest: digestFormStateHistory(stateHistory),
    resourceMergeHistoryDigest: stableValueDigest(resourceMerge.history),
    resourceDriftHistoryDigest: stableValueDigest(resourceDrift.history),
    mutationResponseReconciliationDigest: stableValueDigest(
      canonicalizationHistory.map((artifact) => artifact.resourceLine?.mutationResponse ?? null),
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
    diagnosticsHistoryDigest: digestFormDiagnosticsHistory(diagnosticsHistory),
    diagnosticsSummaryDigest: diagnosticsSummary.digest,
    diagnosticsDigest,
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
    replayRestoreHistory: Object.freeze({
      operations: replayRestoreHistory.length,
      digest: digests.replayRestoreHistoryDigest,
    }),
    stateHistory: Object.freeze({
      operations: stateHistory.length,
      digest: digests.stateHistoryDigest,
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
    diagnosticsHistory: Object.freeze({
      operations: diagnosticsHistory.length,
      digest: digests.diagnosticsHistoryDigest,
    }),
    sourceCompatibilityHistory: Object.freeze({
      operations: sourceCompatibilityHistory.length,
      digest: digests.sourceCompatibilityHistoryDigest,
    }),
    performanceEnvelope: formPerformanceEnvelope({
      sourceCompatibility,
      resourceSource,
      resourceMerge,
      resourceDrift,
      attachmentTransfers,
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
      replayRestoreHistory,
      stateHistory,
      diagnosticsHistory,
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
    diagnosticsHistoryOperations: reports.diagnosticsHistory.length,
    actionHistoryAttempts: reports.actionHistory.length,
    actionExecutionOperations: reports.actionExecutionHistory.length,
    asyncValidationOperations: reports.asyncValidationHistory.length,
    canonicalizationOperations: reports.canonicalizationHistory.length,
    resetOperations: reports.resetHistory.length,
    replayRestoreOperations: reports.replayRestoreHistory.length,
    fieldWriteOperations: reports.stateHistory.filter((entry) => entry.entryKind === "draftWrite").length,
    rawInputOperations: reports.stateHistory.filter((entry) => entry.entryKind === "rawInput").length,
    interactionOperations: reports.interactionHistory.length,
    navigationOperations: reports.navigationHistory.length,
    sourceCompatibilityOperations: reports.sourceCompatibilityHistory.length,
    resourceSource: reports.resourceSource?.counters ?? null,
    resourceMerge: reports.resourceMerge.counters,
    resourceDrift: reports.resourceDrift.counters,
    hostFacts: reports.host.counters,
    inputCapabilities: reports.inputCapabilities.counters,
    exit: reports.exit.counters,
    handoff: reports.handoff.counters,
    attachments: reports.attachments.counters,
    attachmentTransfers: reports.attachmentTransfers.counters,
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
