import { executeResourceBackedSubmit } from "./resource_submit_execution.js";

export function createActionRuntimeBindings({
  formRef,
  actionAttempts,
  actionExecutions,
  asyncValidations,
  canonicalizations,
  sourceAuthority,
  source,
  fieldDeclarations,
  setDraft,
  applyControllerLocalNavigation,
}) {
  return Object.freeze({
    attemptAction(actionId) {
      return actionAttempts.attempt(formRef().actionPlan(actionId));
    },
    actionHistory() {
      return actionAttempts.history();
    },
    executeAction(actionId) {
      const form = formRef();
      const plan = form.actionPlan(actionId);
      const previousSource = form.source();
      const previousDraft = form.draft();
      if (plan.status !== "accepted") {
        const execution = actionExecutions.execute(plan);
        applyControllerLocalNavigation(execution);
        return execution;
      }
      const resourceExecution = executeResourceBackedSubmit(source, fieldDeclarations, plan);
      const execution = resourceExecution === null
        ? actionExecutions.execute(plan)
        : actionExecutions.executeResolved(plan, resourceExecution);
      if (resourceExecution?.resultKind === "fulfilled") {
        const canonicalization = canonicalizations.applyFulfilledAction(
          execution,
          previousSource,
          previousDraft,
          sourceAuthority.read(),
        );
        if (canonicalization) {
          setDraft({});
        }
      }
      applyControllerLocalNavigation(execution);
      return execution;
    },
    fulfillAction(operationId, payload = {}) {
      const form = formRef();
      const previousSource = form.source();
      const previousDraft = form.draft();
      const settled = actionExecutions.fulfill(operationId, payload, (actionId) => form.actionPlan(actionId));
      const canonicalization = canonicalizations.applyFulfilledAction(
        settled,
        previousSource,
        previousDraft,
        sourceAuthority.read(),
      );
      if (canonicalization) {
        setDraft({});
      }
      applyControllerLocalNavigation(settled);
      return settled;
    },
    rejectAction(operationId, payload = {}) {
      return actionExecutions.reject(operationId, payload, (actionId) => formRef().actionPlan(actionId));
    },
    cancelAction(operationId, payload = {}) {
      return actionExecutions.cancel(operationId, payload);
    },
    timeoutAction(operationId, payload = {}) {
      return actionExecutions.timeout(operationId, payload);
    },
    retryAction(operationId) {
      return actionExecutions.retry(operationId, (actionId) => formRef().actionPlan(actionId));
    },
    actionExecutionHistory() {
      return actionExecutions.history();
    },
    startAsyncValidation(validationId) {
      return asyncValidations.start(validationId, formRef());
    },
    fulfillAsyncValidation(operationId, payload = {}) {
      return asyncValidations.fulfill(operationId, payload, formRef());
    },
    rejectAsyncValidation(operationId, payload = {}) {
      return asyncValidations.reject(operationId, payload, formRef());
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
  });
}
