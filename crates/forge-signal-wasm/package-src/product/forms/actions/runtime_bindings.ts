import { executeResourceBackedAction } from "./resource_action_execution.js";
import { consumeDraftFields } from "./action_patch_scope.js";
import { readActionDebug } from "./debug.js";

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
  reportRouteHandoff,
  resets,
  replayRestores,
}) {
  return Object.freeze({
    attemptAction(actionId) {
      return actionAttempts.attempt(formRef().actionPlan(actionId));
    },
    actionHistory() {
      return actionAttempts.history();
    },
    debugAction(actionId) {
      return readActionDebug(formRef(), actionId);
    },
    executeAction(actionId) {
      const form = formRef();
      const plan = form.actionPlan(actionId);
      const previousSource = form.source();
      const previousDraft = form.draft();
      if (plan.status !== "accepted") {
        const execution = actionExecutions.execute(plan, {
          resourceSettlement: currentResourceSettlementForPlan(form, plan),
        });
        applyControllerLocalNavigation(execution);
        reportRouteHandoff?.(plan, execution);
        return execution;
      }
      const resourceExecution = executeResourceBackedAction(source, fieldDeclarations, plan, {
        form,
        writeDraft: setDraft,
        rollbackLastResourceEffect: resets.rollbackLastResourceEffect,
        replayExactResourceSource: replayRestores.replayExactResourceSource,
        restoreExactResourceSource: replayRestores.restoreExactResourceSource,
      });
      const execution = resourceExecution === null
        ? actionExecutions.execute(plan)
        : actionExecutions.executeResolved(plan, resourceExecution);
      if (
        resourceExecution?.resultKind === "fulfilled"
        && resourceExecution.resourceSubmission !== null
        && resourceExecution.resourceSubmission !== undefined
      ) {
        const nextDraft = nextDraftAfterCanonicalization(plan, previousDraft, fieldDeclarations);
        const canonicalization = canonicalizations.applyFulfilledAction(
          execution,
          previousSource,
          previousDraft,
          nextDraft.nextDraft,
          nextDraft.clearedFields,
          sourceAuthority.read(),
        );
        if (canonicalization) {
          setDraft(nextDraft.nextDraft);
        }
      }
      applyControllerLocalNavigation(execution);
      reportRouteHandoff?.(plan, execution);
      return execution;
    },
    fulfillAction(operationId, payload = {}) {
      const form = formRef();
      const previousSource = form.source();
      const previousDraft = form.draft();
      const settled = actionExecutions.fulfill(operationId, payload, (actionId) => form.actionPlan(actionId));
      const nextDraft = settled.planSnapshot === undefined || settled.planSnapshot === null
        ? { nextDraft: {}, clearedFields: Object.freeze([]), draftReset: true }
        : nextDraftAfterCanonicalization(settled.planSnapshot, previousDraft, fieldDeclarations);
      const canonicalization = canonicalizations.applyFulfilledAction(
        settled,
        previousSource,
        previousDraft,
        nextDraft.nextDraft,
        nextDraft.clearedFields,
        sourceAuthority.read(),
      );
      if (canonicalization) {
        setDraft(nextDraft.nextDraft);
      }
      applyControllerLocalNavigation(settled);
      reportRouteHandoff?.(settled.planSnapshot ?? null, settled);
      return settled;
    },
    rejectAction(operationId, payload = {}) {
      return actionExecutions.reject(operationId, payload, (actionId) => formRef().actionPlan(actionId));
    },
    cancelAction(operationId, payload = {}) {
      return actionExecutions.cancel(operationId, payload, (actionId) => formRef().actionPlan(actionId));
    },
    timeoutAction(operationId, payload = {}) {
      return actionExecutions.timeout(operationId, payload, (actionId) => formRef().actionPlan(actionId));
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

function currentResourceSettlementForPlan(form, plan) {
  const resourceSource = form.resourceSource();
  if (resourceSource === null) {
    return null;
  }
  if (plan.id !== "submit" && plan.resourceAction.declared !== true) {
    return null;
  }
  if (!plan.readiness.blockers.some(blockerUsesResourceSettlement)) {
    return null;
  }
  return resourceSource.settlement.kind === "none" ? null : resourceSource.settlement;
}

function blockerUsesResourceSettlement(blocker) {
  return (
    blocker.kind === "resource:pending"
    || blocker.kind === "resource:rejected"
    || blocker.kind === "resource:timedOut"
  );
}

function nextDraftAfterCanonicalization(plan, previousDraft, fieldDeclarations) {
  if (
    plan.resourceAction.action?.kind === "patchPlan"
    && Array.isArray(plan.resourceAction.action.fields)
    && plan.resourceAction.action.fields.length > 0
  ) {
    return consumeDraftFields(previousDraft, plan.patch, fieldDeclarations);
  }
  return Object.freeze({
    nextDraft: {},
    clearedFields: Object.freeze([]),
    draftReset: true,
  });
}
