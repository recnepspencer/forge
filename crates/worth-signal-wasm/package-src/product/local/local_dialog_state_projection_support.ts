export function cloneValue(value) {
  if (value === null || value === undefined) {
    return value;
  }
  if (typeof globalThis.structuredClone === "function") {
    return globalThis.structuredClone(value);
  }
  return JSON.parse(JSON.stringify(value));
}

export function stableDigest(value) {
  return JSON.stringify(value);
}

export function createStateSnapshot(handles) {
  return Object.freeze({
    isOpen: handles.isOpen(),
    mode: handles.mode(),
    data: handles.data(),
    context: handles.context(),
    loading: handles.loading(),
  });
}

export function dialogStateDigest(state) {
  return stableDigest(state);
}

export function dialogStateChanged(sourceState, currentState) {
  return dialogStateDigest(sourceState) !== dialogStateDigest(currentState);
}

export function changedDialogStateKeys(sourceState, currentState) {
  return Object.freeze(
    Object.keys(currentState).filter((key) => dialogStateDigest(sourceState[key]) !== dialogStateDigest(currentState[key])),
  );
}

export function createMessage(code, source, severity, visibility, text, target = null) {
  return Object.freeze({ code, source, severity, visibility, text, target });
}

export function currentStepIdFromNavigation(navigation) {
  return navigation?.currentStepId ?? navigation?.current?.stepId ?? null;
}

export function stepProgressFromForm(form) {
  const stepReport = typeof form?.steps === "function" ? form.steps() : null;
  const currentStepId = currentStepIdFromNavigation(typeof form?.navigation === "function" ? form.navigation() : null);
  const dirty = typeof form?.dirty === "function" ? Boolean(form.dirty()) : false;
  const routeBlocked = Array.isArray(stepReport?.artifacts)
    ? stepReport.artifacts.find((step) => step.routeCoupled === true && (step.posture === "blocked" || step.posture === "unavailable"))
    : null;
  return Object.freeze({
    currentStepId,
    progress: dirty || currentStepId !== null ? "started" : "none",
    routeBlocked,
  });
}
