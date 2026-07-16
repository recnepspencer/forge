export function createReactiveFormBindings(signalNamespace, formId, formRef) {
  const formScope = signalNamespace.scope(`${formId}:reactive`);
  const summaryState = formScope.input(readSummary(formRef()), {
    debugName: `${formId}.summaryState`,
  });
  const summarySignal = formScope.computed(
    () => summaryState(),
    {
      debugName: `${formId}.summary`,
    },
  );

  function noteMutation() {
    summaryState.set(readSummary(formRef()));
  }

  function summarySignalHandle() {
    return summarySignal;
  }

  return Object.freeze({
    noteMutation,
    summarySignalHandle,
    wrapFieldHandle(handle) {
      return createWrappedMutationFacade(handle, FIELD_MUTATION_METHODS, noteMutation);
    },
    wrapControllerMutations(form) {
      wrapMutationMethods(form, FORM_MUTATION_METHODS, noteMutation);
      return form;
    },
  });
}

function readSummary(form) {
  return Object.freeze({
    source: form.source(),
    draft: form.draft(),
    effective: form.effective(),
    dirty: form.dirty(),
    patchPlan: form.patchPlan(),
    readiness: form.readiness(),
    visibleMessages: form.visibleMessages(),
  });
}

const FIELD_MUTATION_METHODS = Object.freeze([
  "set",
  "clearDraft",
  "input",
  "compose",
  "commitInput",
  "touch",
  "visit",
  "focus",
  "blur",
  "addItem",
  "removeItem",
  "replaceItem",
  "moveItem",
]);

const FORM_MUTATION_METHODS = Object.freeze([
  "attemptAction",
  "executeAction",
  "fulfillAction",
  "rejectAction",
  "cancelAction",
  "timeoutAction",
  "retryAction",
  "startAsyncValidation",
  "fulfillAsyncValidation",
  "rejectAsyncValidation",
  "cancelAsyncValidation",
  "timeoutAsyncValidation",
  "reportRouteAuthority",
  "bindRouteAuthority",
  "clearRouteAuthority",
  "reportFieldInteraction",
  "reportSubmitIntent",
  "clearSubmitIntent",
  "recordLayoutMeasurement",
  "reset",
  "rollbackLastResourceEffect",
  "replayExactResourceSource",
  "restoreExactResourceSource",
  "acknowledgePresentation",
  "timeoutPresentation",
  "clearPresentationLane",
]);

function wrapMutationMethods(target, methodNames, noteMutation) {
  for (const methodName of methodNames) {
    wrapMutationMethod(target, methodName, noteMutation);
  }
}

function wrapMutationMethod(target, methodName, noteMutation) {
  const original = target[methodName];
  if (typeof original !== "function") {
    return;
  }
  Object.defineProperty(target, methodName, {
    enumerable: false,
    configurable: true,
    writable: true,
    value(...args) {
      const result = original.apply(this, args);
      if (result && typeof result.then === "function") {
        return result.then((resolved) => {
          noteMutation();
          return resolved;
        });
      }
      noteMutation();
      return result;
    },
  });
}

function createWrappedMutationFacade(target, methodNames, noteMutation) {
  const wrapped = {
    ...target,
  };
  wrapMutationMethods(wrapped, methodNames, noteMutation);
  return Object.isFrozen(target) ? Object.freeze(wrapped) : wrapped;
}
