import { FormDeclarationError } from "./form_errors.js";
import { materializeActionDeclarations } from "./actions/declarations.js";
import { findActionPlan, planActions } from "./actions/planning.js";
import { createActionAttemptStore } from "./actions/results.js";
import { createActionExecutionStore } from "./actions/execution.js";
import { createCanonicalizationStore } from "./canonicalization.js";
import {
  admissionCapabilityBlocker,
  admissionReadinessBlockers,
} from "./admission/artifacts.js";
import { materializeAdmissionDeclarations } from "./admission/declarations.js";
import { evaluateAdmission } from "./admission/execution.js";
import {
  availabilityReadinessBlockers,
  availabilityEditBlocker,
  clearedFieldIds,
  omittedFieldIds,
} from "./availability/artifacts.js";
import { materializeAvailabilityDeclarations } from "./availability/declarations.js";
import { evaluateAvailability } from "./availability/execution.js";
import { materializeFieldDeclarations } from "./fields/declarations.js";
import { createFieldHandle } from "./fields/handles.js";
import { evaluateSteps } from "./steps/artifacts.js";
import { materializeStepDeclarations } from "./steps/declarations.js";
import { readSource } from "./sources/form_sources.js";
import {
  buildPatchPlan,
  dirtyFieldRecords,
  rawInputBlockers,
} from "./patching/patch_planning.js";
import {
  validationReadinessBlockers,
  visibleMessages,
} from "./validation/artifacts.js";
import { createAsyncValidationStore } from "./validation/async_execution.js";
import { materializeValidationDeclarations } from "./validation/declarations.js";
import { validateForm } from "./validation/execution.js";
import { cloneFormValue, mergeDraft } from "./values/value_paths.js";
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
  const canonicalizations = createCanonicalizationStore();

  const form = {
    source() {
      return cloneFormValue(authoritativeSource());
    },
    draft() {
      return cloneFormValue(draft);
    },
    effective() {
      return mergeDraft(authoritativeSource(), draft);
    },
    dirty() {
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
      const availability = form.availability();
      return buildPatchPlan(fieldDeclarations, form, rawInputs, {
        omittedFields: omittedFieldIds(availability),
        clearedFields: clearedFieldIds(availability),
      });
    },
    validation() {
      return validateForm(
        fieldDeclarations,
        validationDeclarations,
        form,
        parseFailures,
        asyncValidations.artifacts(),
      );
    },
    availability() {
      return evaluateAvailability(availabilityDeclarations, form);
    },
    admission() {
      return evaluateAdmission(admissionDeclarations, form, fieldDeclarations);
    },
    visibleMessages() {
      return visibleMessages(form.validation());
    },
    steps() {
      return evaluateSteps(stepDeclarations, form);
    },
    actions() {
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
      return actionExecutions.execute(form.actionPlan(actionId));
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
    actionReadiness(actionId) {
      return form.actionPlan(actionId).readiness;
    },
    verification() {
      return buildFormVerificationPackage(form);
    },
    fieldWritePosture(fieldId, capability = "edit") {
      form.field(fieldId);
      const availabilityBlocker = availabilityEditBlocker(form.availability(), fieldId);
      const admissionBlocker = admissionCapabilityBlocker(form.admission(), fieldId, capability);
      const blockers = [availabilityBlocker, admissionBlocker].filter(Boolean);
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
      blockers.push(...validationReadinessBlockers(form.validation()));
      blockers.push(...availabilityReadinessBlockers(form.availability()));
      blockers.push(...admissionReadinessBlockers(form.admission()));
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
      return Object.freeze({
        kind: "form",
        fieldCount: fieldDeclarations.length,
        dirty: form.dirty(),
        patchPlan: form.patchPlan(),
        validation: form.validation(),
        availability: form.availability(),
        admission: form.admission(),
        steps: form.steps(),
        actions: form.actions(),
        actionHistory: form.actionHistory(),
        actionExecutionHistory: form.actionExecutionHistory(),
        asyncValidationHistory: form.asyncValidationHistory(),
        canonicalizationHistory: form.canonicalizationHistory(),
        verification: form.verification(),
        inputAdapters: Object.freeze(
          fieldDeclarations.map((field) => ({
            field: field.id,
            path: field.path,
            tier: field.inputAdapter.tier,
            capabilities: field.inputAdapter.capabilities,
            unavailable: field.inputAdapter.unavailable,
          })),
        ),
      });
    },
    fields: fieldHandles,
    namespace: signalNamespace,
  };

  for (const declaration of fieldDeclarations) {
    const handle = createFieldHandle(declaration, form, {
      writeDraft(nextDraft) {
        draft = nextDraft;
      },
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
}
