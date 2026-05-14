import { FormDeclarationError } from "./form_errors.js";
import { materializeActionDeclarations } from "./actions/declarations.js";
import { createActionRuntimeBindings } from "./actions/runtime_bindings.js";
import { findActionPlan, planActions } from "./actions/planning.js";
import { resolveResourceEffectProfileBinding } from "./actions/resource_effect_profile_binding.js";
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
import {
  materializeFormDeclarationRecord,
  readFieldContractDiagnostics,
  readFormDeclarationDiagnostics,
  readInputAdapterDiagnostics,
} from "./controller_declaration_diagnostics.js";
import { readFormDiagnostics } from "./form_diagnostics.js";
import { createDerivedReportBindings } from "./derived_report_bindings.js";
import { readLayoutReport } from "./layout/artifacts.js";
import { materializeMeasurementDeclaration } from "./measurement/declarations.js";
import { readMeasurementSemanticContext } from "./measurement/semantic_context.js";
import { createLayoutMeasurementStore } from "./measurement/store.js";
import { readMediaPresentationReport } from "./media/report.js";
import { createMediaPresentationStore } from "./media/store.js";
import { readMessagePresentationReport } from "./messages/report.js";
import { createMessagePresentationStore } from "./messages/store.js";
import { createPresentationBindings } from "./presentation/controller_bindings.js";
import { materializePresentationDeclaration } from "./presentation/declarations.js";
import { createPresentationRuntimeBindings } from "./presentation/runtime_bindings.js";
import { createDeclaredPresentationScopeRegistry } from "./presentation/scope_registry.js";
import { createPresentationStore } from "./presentation/store.js";
import {
  createResourceMergeProjectionRegistry,
  previewResourceMerge as materializeResourceMergePreview,
} from "./resource_merge/projection.js";
import {
  readResourceMergeReport,
  resourceMergeReadinessBlockers,
} from "./resource_merge/report.js";
import { createResourceMergeStore } from "./resource_merge/store.js";
import { createFormResetStore } from "./reset/store.js";
import { createFieldHandle } from "./fields/handles.js";
import { evaluateSteps } from "./steps/artifacts.js";
import { materializeStepDeclarations } from "./steps/declarations.js";
import { materializeFormSourceAuthority, readSourceBootstrapArtifact } from "./sources/form_sources.js";
import { readResourceSourceReport, resourceSourceReadinessBlockers } from "./sources/resource_source_report.js";
import { createSourceCompatibilityStore, sourceCompatibilityBlockers } from "./sources/source_compatibility.js";
import { rawInputBlockers } from "./patching/patch_planning.js";
import { validationReadinessBlockers } from "./validation/artifacts.js";
import { createAsyncValidationStore } from "./validation/async_execution.js";
import { materializeValidationDeclarations } from "./validation/declarations.js";
import { cloneFormValue, mergeDraft, stableValueDigest } from "./values/value_paths.js";
import { buildFormVerificationPackage } from "./verification.js";

export function createFormController(signalNamespace, declaration) {
  if (!declaration || typeof declaration !== "object") {
    throw new FormDeclarationError("signals.form(...) expects a declaration object");
  }
  if (!("source" in declaration)) {
    throw new FormDeclarationError("signals.form(...) requires a source value or signal");
  }
  const sourceAuthority = materializeFormSourceAuthority(declaration.source);
  const formDeclaration = materializeFormDeclarationRecord(declaration, sourceAuthority);
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
  const messages = createMessagePresentationStore();
  const resourceMerges = createResourceMergeStore();
  const resets = createFormResetStore();
  const presentationPolicy = materializePresentationDeclaration(declaration);
  const presentationSettlements = createPresentationStore();
  const presentationScopeRegistry = createDeclaredPresentationScopeRegistry(
    fieldDeclarations,
    stepDeclarations,
    actionDeclarations,
    availabilityDeclarations,
  );
  const resourceMergeRegistry = createResourceMergeProjectionRegistry(
    fieldDeclarations,
    stepDeclarations,
    availabilityDeclarations,
  );
  const interactionBindings = createInteractionBindings(interactions, fieldDeclarations);
  const measurementSemanticCache = { value: null }; let form;
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
    presentationScopeRegistry,
  );
  const presentationRuntimeBindings = createPresentationRuntimeBindings({
    presentationPolicy,
    presentationBindings,
    exits,
    handoffs,
    attachments,
    media,
    messages,
    scopeRegistry: presentationScopeRegistry,
    collaborationDeclaration,
    collaborations,
  });
  const actionRuntimeBindings = createActionRuntimeBindings({
    formRef: () => form,
    actionAttempts,
    actionExecutions,
    asyncValidations,
    canonicalizations,
    sourceAuthority,
    source: declaration.source,
    fieldDeclarations,
    setDraft: updateDraft,
    applyControllerLocalNavigation,
  });
  const derivedReportBindings = createDerivedReportBindings({
    formRef: () => form,
    syncSourceCompatibility,
    authoritativeSource,
    fieldDeclarations,
    rawInputs,
    parseFailures,
    asyncValidations,
    validationDeclarations, availabilityDeclarations, admissionDeclarations, stepDeclarations, actionDeclarations,
  });

  form = {
    source() {
      const currentSource = authoritativeSource();
      syncSourceCompatibility(currentSource);
      return cloneFormValue(currentSource);
    },
    sourceAuthority() {
      return sourceAuthority.diagnostics();
    },
    declaration() {
      return readFormDeclarationDiagnostics(formDeclaration, sourceAuthority, fieldDeclarations);
    },
    fieldContract() {
      return readFieldContractDiagnostics(fieldDeclarations);
    },
    inputAdapters() {
      return readInputAdapterDiagnostics(fieldDeclarations);
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
    sourceAdmission() { return readSourceBootstrapArtifact(declaration.source, "sourceAdmission"); },
    draftRestore() { return readSourceBootstrapArtifact(declaration.source, "draftRestore"); },
    resourceSource() { return readResourceSourceReport(declaration.source); },
    resourceMerge() { return readResourceMergeReport(resourceMerges, declaration.source); },
    previewResourceMerge(request) {
      return materializeResourceMergePreview(
        signalNamespace,
        declaration.source,
        resourceMerges,
        resourceMergeRegistry,
        request,
      );
    },
    clearResourceMerge(reason = undefined) { return resourceMerges.clear(reason); },
    host() { return readHostReport(hostBindings); },
    inputCapabilities() { return readInputCapabilitiesReport(fieldDeclarations); },
    exit() { return readExitPresentationReport(exits, deriveExitPresentationBasis(form)); },
    handoff() { return readHandoffReport(handoffs); },
    attachments() { return readAttachmentPresentationReport(attachments); },
    media() { return readMediaPresentationReport(media); },
    messages() { return readMessagePresentationReport(messages, form.visibleMessages()); },
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
    ...presentationRuntimeBindings,
    recordLayoutMeasurement(rows, options = {}) { syncSourceCompatibility(authoritativeSource()); return layoutMeasurements.record(currentMeasurementSemanticContext(), rows, options); },
    ...derivedReportBindings,
    actionPlan(actionId) {
      return findActionPlan(actionDeclarations, form, fieldDeclarations, actionId);
    },
    ...actionRuntimeBindings,
    sourceCompatibility() { return syncSourceCompatibility(authoritativeSource()); },
    sourceCompatibilityHistory() {
      syncSourceCompatibility(authoritativeSource());
      return sourceCompatibility.history();
    },
    reset(options = {}) {
      return resets.acceptCanonicalValue({ form, writeDraft: updateDraft }, options);
    },
    rollbackLastResourceEffect(options = {}) {
      return resets.rollbackLastResourceEffect(
        { form, source: declaration.source, writeDraft: updateDraft },
        options,
      );
    },
    resetHistory() {
      return resets.history();
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
      blockers.push(...resourceSourceReadinessBlockers(form.resourceSource()));
      blockers.push(...resourceMergeReadinessBlockers(form.resourceMerge()));
      blockers.push(...validationReadinessBlockers(form.validation()));
      blockers.push(...availabilityReadinessBlockers(form.availability()));
      blockers.push(...admissionReadinessBlockers(form.admission()));
      blockers.push(...collaborationReadinessBlockers(form.collaboration(), patchPlan));
      const submitAction = actionDeclarations.find((entry) => entry.id === "submit");
      if (submitAction) {
        blockers.push(...hostRequirementBlockers(form.host(), submitAction.hostRequirements, "submit"));
        blockers.push(
          ...resolveResourceEffectProfileBinding(
            submitAction,
            form.resourceSource(),
            "submit",
          ).blockers,
        );
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
      writeDraft: updateDraft,
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
    return canonicalizations.sourceFor(sourceAuthority.read());
  }

  function updateDraft(nextDraft) {
    sourceCompatibility.noteDraft(nextDraft, authoritativeSource());
    draft = nextDraft;
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
