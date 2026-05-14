import { FormDeclarationError } from "./form_errors.js";
import { materializeActionDeclarations } from "./actions/declarations.js";
import { findActionPlan, planActions } from "./actions/planning.js";
import { createActionAttemptStore } from "./actions/results.js";
import { createActionExecutionStore } from "./actions/execution.js";
import { readAttachmentPresentationReport } from "./attachments/report.js";
import { createAttachmentPresentationStore } from "./attachments/store.js";
import { createCanonicalizationStore } from "./canonicalization.js";
import { collaborationFieldWriteBlocker, collaborationReadinessBlockers, normalizeCollaborationUpdate, readCollaborationReport } from "./collaboration/artifacts.js";
import { materializeCollaborationDeclaration } from "./collaboration/declarations.js";
import { createCollaborationStore } from "./collaboration/store.js";
import { admissionCapabilityBlocker, admissionReadinessBlockers } from "./admission/artifacts.js";
import { materializeAdmissionDeclarations } from "./admission/declarations.js";
import { evaluateAdmission } from "./admission/execution.js";
import { deriveExitPresentationBasis, readExitPresentationReport } from "./exit/report.js";
import { createExitPresentationStore } from "./exit/store.js";
import { availabilityReadinessBlockers, availabilityEditBlocker, clearedFieldIds, omittedFieldIds } from "./availability/artifacts.js";
import { materializeAvailabilityDeclarations } from "./availability/declarations.js";
import { evaluateAvailability } from "./availability/execution.js";
import { materializeFieldDeclarations } from "./fields/declarations.js";
import { hostRequirementBlockers, readHostReport } from "./host/artifacts.js";
import { materializeHostBindings } from "./host/declarations.js";
import { readHandoffReport } from "./handoff/report.js";
import { createHandoffStore } from "./handoff/store.js";
import { readInputCapabilitiesReport } from "./input_capabilities/report.js";
import { createInteractionBindings } from "./interaction/controller_bindings.js";
import { readInteractionReport } from "./interaction/report.js";
import { createInteractionStore } from "./interaction/store.js";
import { readNavigationReport } from "./navigation/report.js";
import { createNavigationStore } from "./navigation/store.js";
import { readAccessibilityReport } from "./accessibility/artifacts.js";
import { readFormDiagnostics } from "./form_diagnostics.js";
import { readLayoutReport } from "./layout/artifacts.js";
import { materializeMeasurementDeclaration } from "./measurement/declarations.js";
import { readMeasurementSemanticContext } from "./measurement/semantic_context.js";
import { createLayoutMeasurementStore } from "./measurement/store.js";
import { readMediaPresentationReport } from "./media/report.js";
import { createMediaPresentationStore } from "./media/store.js";
import { createPresentationBindings } from "./presentation/controller_bindings.js";
import { materializePresentationDeclaration } from "./presentation/declarations.js";
import { createPresentationStore } from "./presentation/store.js";
import { createFieldHandle } from "./fields/handles.js";
import { evaluateSteps } from "./steps/artifacts.js";
import { materializeStepDeclarations } from "./steps/declarations.js";
import { readSource } from "./sources/form_sources.js";
import { createSourceCompatibilityStore, sourceCompatibilityBlockers } from "./sources/source_compatibility.js";
import { buildPatchPlan, dirtyFieldRecords, rawInputBlockers } from "./patching/patch_planning.js";
import { validationReadinessBlockers, visibleMessages } from "./validation/artifacts.js";
import { createAsyncValidationStore } from "./validation/async_execution.js";
import { materializeValidationDeclarations } from "./validation/declarations.js";
import { validateForm } from "./validation/execution.js";
import { cloneFormValue, mergeDraft, stableValueDigest } from "./values/value_paths.js";
import { buildFormVerificationPackage } from "./verification.js";

export function createFormController(signalNamespace, declaration) {
  if (!declaration || typeof declaration !== "object") {
    throw new FormDeclarationError("signals.form(...) expects a declaration object");
  }
  if (!("source" in declaration)) {
    throw new FormDeclarationError("signals.form(...) requires a source value or signal");
  }
  let draft = {};
  const fieldDeclarations = materializeFieldDeclarations(declaration);
  const hostBindings = materializeHostBindings(declaration);
  const validationDeclarations = materializeValidationDeclarations(declaration, fieldDeclarations);
  const availabilityDeclarations = materializeAvailabilityDeclarations(declaration, fieldDeclarations);
  const admissionDeclarations = materializeAdmissionDeclarations(declaration, fieldDeclarations);
  const stepDeclarations = materializeStepDeclarations(declaration, fieldDeclarations);
  const actionDeclarations = materializeActionDeclarations(declaration, stepDeclarations);
  const fieldHandles = {};
  const fieldsById = new Map();
  const rawInputs = new Map();
  const parseFailures = new Map();
  const actionAttempts = createActionAttemptStore();
  const actionExecutions = createActionExecutionStore(actionAttempts);
  const asyncValidations = createAsyncValidationStore(validationDeclarations, fieldDeclarations);
  const exits = createExitPresentationStore();
  const handoffs = createHandoffStore();
  const attachments = createAttachmentPresentationStore();
  const canonicalizations = createCanonicalizationStore();
  const interactions = createInteractionStore();
  const navigation = createNavigationStore();
  const sourceCompatibility = createSourceCompatibilityStore(declaration.source);
  const collaborationDeclaration = materializeCollaborationDeclaration(declaration, fieldDeclarations);
  const collaborations = createCollaborationStore();
  const measurementPolicy = materializeMeasurementDeclaration(declaration);
  const layoutMeasurements = createLayoutMeasurementStore(measurementPolicy);
  const media = createMediaPresentationStore();
  const presentationPolicy = materializePresentationDeclaration(declaration);
  const presentationSettlements = createPresentationStore();
  const interactionBindings = createInteractionBindings(interactions, fieldDeclarations);
  const measurementSemanticCache = { value: null };
  let form;
  const presentationBindings = createPresentationBindings(
    () => form,
    () => syncSourceCompatibility(authoritativeSource()),
    presentationPolicy,
    actionDeclarations,
    stepDeclarations,
    presentationSettlements,
    handoffs,
    exits,
    attachments,
    media,
  );

  form = {
    source() {
      const currentSource = authoritativeSource();
      syncSourceCompatibility(currentSource);
      return cloneFormValue(currentSource);
    },
    draft() {
      syncSourceCompatibility(authoritativeSource());
      return cloneFormValue(draft);
    },
    effective() {
      const currentSource = authoritativeSource();
      syncSourceCompatibility(currentSource);
      return mergeDraft(currentSource, draft);
    },
    host() { return readHostReport(hostBindings); },
    inputCapabilities() { return readInputCapabilitiesReport(fieldDeclarations); },
    exit() { return readExitPresentationReport(exits, deriveExitPresentationBasis(form)); },
    handoff() { return readHandoffReport(handoffs); },
    attachments() { return readAttachmentPresentationReport(attachments); },
    media() { return readMediaPresentationReport(media); },
    collaboration() { return readCollaborationReport(collaborationDeclaration, collaborations); },
    interaction() { return readInteractionReport(fieldDeclarations, form.host(), interactions); },
    ...interactionBindings,
    navigation() { syncSourceCompatibility(authoritativeSource()); return readNavigationReport(navigation, form.steps().artifacts); },
    accessibility() { syncSourceCompatibility(authoritativeSource()); return readAccessibilityReport(fieldDeclarations, form); },
    layout() { syncSourceCompatibility(authoritativeSource()); return readLayoutReport(fieldDeclarations, form); },
    layoutMeasurement() { syncSourceCompatibility(authoritativeSource()); return layoutMeasurements.report(); },
    presentation: presentationBindings.presentation,
    presentationLifecycle: presentationBindings.presentationLifecycle,
    reportPresentationLane: presentationBindings.reportPresentationLane,
    clearPresentationLane: presentationBindings.clearPresentationLane,
    acknowledgePresentation: presentationBindings.acknowledgePresentation,
    timeoutPresentation: presentationBindings.timeoutPresentation,
    presentationHistory: presentationBindings.presentationHistory,
    reportExit(update) { const artifact = exits.report(update); presentationBindings.reportPresentationLane("exit", { ...update, __alreadyTracked: true }); return artifact; },
    clearExit(options = {}) { const artifact = exits.clear(options.reason ?? null); presentationBindings.clearPresentationLane("exit", { ...options, __alreadyTracked: true }); return artifact; },
    reportHandoff(update) { const artifact = handoffs.report(update); presentationBindings.reportPresentationLane("handoff", { ...update, __alreadyTracked: true }); return artifact; },
    clearHandoff(options = {}) { const artifact = handoffs.clear(options.reason ?? null); presentationBindings.clearPresentationLane("handoff", { ...options, __alreadyTracked: true }); return artifact; },
    reportAttachments(update) { const artifact = attachments.report(update); presentationBindings.reportPresentationLane("attachments", { ...update, __alreadyTracked: true }); return artifact; },
    clearAttachments(options = {}) { const artifact = attachments.clear(options.reason ?? null); presentationBindings.clearPresentationLane("attachments", { ...options, __alreadyTracked: true }); return artifact; },
    reportMedia(update) { const artifact = media.report(update); presentationBindings.reportPresentationLane("media", { ...update, __alreadyTracked: true }); return artifact; },
    clearMedia(options = {}) { const artifact = media.clear(options.reason ?? null); presentationBindings.clearPresentationLane("media", { ...options, __alreadyTracked: true }); return artifact; },
    reportCollaboration(update) { return collaborations.report(normalizeCollaborationUpdate(collaborationDeclaration, update)); },
    clearCollaboration(options = {}) { return collaborations.clear(options.reason ?? undefined); },
    recordLayoutMeasurement(rows, options = {}) { syncSourceCompatibility(authoritativeSource()); return layoutMeasurements.record(currentMeasurementSemanticContext(), rows, options); },
    dirty() {
      syncSourceCompatibility(authoritativeSource());
      const availability = form.availability();
      const dirtyFields = dirtyFieldRecords(fieldDeclarations, form, {
        omittedFields: omittedFieldIds(availability),
        clearedFields: clearedFieldIds(availability),
      });
      return Object.freeze({
        isDirty: dirtyFields.fields.length > 0,
        semanticDirty: dirtyFields.fields.length > 0,
        fields: dirtyFields.fields,
        equality: dirtyFields.equality,
        breadth: dirtyFields.breadth,
      });
    },
    patchPlan() {
      syncSourceCompatibility(authoritativeSource());
      const availability = form.availability();
      return buildPatchPlan(fieldDeclarations, form, rawInputs, {
        omittedFields: omittedFieldIds(availability),
        clearedFields: clearedFieldIds(availability),
      });
    },
    validation() {
      syncSourceCompatibility(authoritativeSource());
      return validateForm(
        fieldDeclarations,
        validationDeclarations,
        form,
        parseFailures,
        asyncValidations.artifacts(),
      );
    },
    availability() {
      syncSourceCompatibility(authoritativeSource());
      return evaluateAvailability(availabilityDeclarations, form);
    },
    admission() {
      syncSourceCompatibility(authoritativeSource());
      return evaluateAdmission(admissionDeclarations, form, fieldDeclarations);
    },
    visibleMessages() {
      return visibleMessages(form.validation());
    },
    steps() {
      syncSourceCompatibility(authoritativeSource());
      return evaluateSteps(stepDeclarations, form);
    },
    actions() {
      syncSourceCompatibility(authoritativeSource());
      return planActions(actionDeclarations, form, fieldDeclarations);
    },
    actionPlan(actionId) {
      return findActionPlan(actionDeclarations, form, fieldDeclarations, actionId);
    },
    attemptAction(actionId) {
      return actionAttempts.attempt(form.actionPlan(actionId));
    },
    actionHistory() {
      return actionAttempts.history();
    },
    executeAction(actionId) {
      const execution = actionExecutions.execute(form.actionPlan(actionId));
      applyControllerLocalNavigation(execution);
      return execution;
    },
    fulfillAction(operationId, payload = {}) {
      const previousSource = form.source();
      const previousDraft = form.draft();
      const settled = actionExecutions.fulfill(operationId, payload, (actionId) => form.actionPlan(actionId));
      const canonicalization = canonicalizations.applyFulfilledAction(
        settled,
        previousSource,
        previousDraft,
        readSource(declaration.source),
      );
      if (canonicalization) {
        draft = {};
      }
      applyControllerLocalNavigation(settled);
      return settled;
    },
    rejectAction(operationId, payload = {}) {
      return actionExecutions.reject(operationId, payload, (actionId) => form.actionPlan(actionId));
    },
    cancelAction(operationId, payload = {}) {
      return actionExecutions.cancel(operationId, payload);
    },
    timeoutAction(operationId, payload = {}) {
      return actionExecutions.timeout(operationId, payload);
    },
    retryAction(operationId) {
      return actionExecutions.retry(operationId, (actionId) => form.actionPlan(actionId));
    },
    actionExecutionHistory() {
      return actionExecutions.history();
    },
    startAsyncValidation(validationId) {
      return asyncValidations.start(validationId, form);
    },
    fulfillAsyncValidation(operationId, payload = {}) {
      return asyncValidations.fulfill(operationId, payload, form);
    },
    rejectAsyncValidation(operationId, payload = {}) {
      return asyncValidations.reject(operationId, payload, form);
    },
    cancelAsyncValidation(operationId, payload = {}) {
      return asyncValidations.cancel(operationId, payload);
    },
    timeoutAsyncValidation(operationId, payload = {}) {
      return asyncValidations.timeout(operationId, payload);
    },
    asyncValidationHistory() {
      return asyncValidations.history();
    },
    canonicalizationHistory() {
      return canonicalizations.history();
    },
    sourceCompatibility() {
      return syncSourceCompatibility(authoritativeSource());
    },
    sourceCompatibilityHistory() {
      syncSourceCompatibility(authoritativeSource());
      return sourceCompatibility.history();
    },
    actionReadiness(actionId) {
      return form.actionPlan(actionId).readiness;
    },
    verification() {
      currentMeasurementSemanticContext();
      return buildFormVerificationPackage(form);
    },
    fieldWritePosture(fieldId, capability = "edit") {
      form.field(fieldId);
      const availabilityBlocker = availabilityEditBlocker(form.availability(), fieldId);
      const admissionBlocker = admissionCapabilityBlocker(form.admission(), fieldId, capability);
      const collaborationBlocker = collaborationFieldWriteBlocker(form.collaboration(), fieldId, capability);
      const blockers = [
        ...sourceCompatibilityBlockers(form.sourceCompatibility()),
        availabilityBlocker,
        admissionBlocker,
        collaborationBlocker,
      ].filter(Boolean);
      return Object.freeze({
        field: fieldId,
        capability,
        canWrite: blockers.length === 0,
        blockers: Object.freeze(blockers),
        reason: blockers[0]?.reason ?? "field write admitted",
      });
    },
    readiness() {
      const patchPlan = form.patchPlan();
      const blockers = rawInputBlockers(rawInputs);
      blockers.push(...sourceCompatibilityBlockers(form.sourceCompatibility()));
      blockers.push(...validationReadinessBlockers(form.validation()));
      blockers.push(...availabilityReadinessBlockers(form.availability()));
      blockers.push(...admissionReadinessBlockers(form.admission()));
      blockers.push(...collaborationReadinessBlockers(form.collaboration(), patchPlan));
      const submitAction = actionDeclarations.find((entry) => entry.id === "submit");
      if (submitAction) {
        blockers.push(...hostRequirementBlockers(form.host(), submitAction.hostRequirements, "submit"));
      }
      if (patchPlan.empty) {
        blockers.push({
          kind: "unchanged",
          reason: "form has no semantic changes to submit",
        });
      }
      return Object.freeze({
        canSubmit: blockers.length === 0,
        blockers: Object.freeze(blockers),
        patchPlan,
      });
    },
    diagnostics() {
      return readFormDiagnostics(form, fieldDeclarations);
    },
    fields: fieldHandles,
    namespace: signalNamespace,
  };

  for (const declaration of fieldDeclarations) {
    const handle = createFieldHandle(declaration, form, {
      writeDraft(nextDraft) {
        sourceCompatibility.noteDraft(nextDraft, authoritativeSource());
        draft = nextDraft;
      },
      interactions,
      rawInputs,
      parseFailures,
    });
    fieldHandles[declaration.name] = handle;
    fieldsById.set(declaration.id, handle);
  }
  Object.freeze(fieldHandles);
  Object.defineProperty(form, "field", {
    enumerable: false,
    value(fieldId) {
      const handle = fieldsById.get(fieldId);
      if (!handle) {
        throw new FormDeclarationError("form field is not declared", { fieldId });
      }
      return handle;
    },
  });
  return Object.freeze(form);

  function authoritativeSource() {
    return canonicalizations.sourceFor(readSource(declaration.source));
  }

  function syncSourceCompatibility(rawSource) {
    const resolution = sourceCompatibility.reconcile(rawSource, draft);
    if (resolution.draft !== draft) {
      draft = resolution.draft;
    }
    return resolution.report;
  }

  function currentMeasurementSemanticContext() {
    return readMeasurementSemanticContext({
      cache: measurementSemanticCache,
      authoritativeSource: authoritativeSource(),
      draft,
      rawInputs,
      parseFailures,
      asyncValidationArtifacts: asyncValidations.artifacts(),
      sourceCompatibilityHistoryLength: sourceCompatibility.history().length,
      form,
    });
  }

  function applyControllerLocalNavigation(execution) {
    const plan = execution.planSnapshot;
    if (
      execution.resultKind !== "fulfilled" ||
      plan?.kind !== "step" ||
      plan.step?.routeCoupled === true
    ) {
      return;
    }
    navigation.applyStepAction(plan, form.steps().artifacts);
  }
}
