function freezeLineState(state, canonicalRevision) {
  return Object.freeze({
    value: state.value,
    canonicalValue: state.canonicalValue,
    canonicalRevision,
    processing: state.processing,
    upload: state.upload,
    download: state.download,
    status: state.status,
    freshness: state.freshness,
    diagnostics: state.diagnostics,
  });
}

function createLineBindingState(initialState) {
  return {
    current: freezeLineState(initialState, 0),
  };
}

function readLineBindingState(binding) {
  return binding.state.current;
}

function replaceLineBindingState(binding, nextState) {
  const previousState = binding.state.current;
  const canonicalRevision = previousState.canonicalValue === nextState.canonicalValue
    ? previousState.canonicalRevision
    : previousState.canonicalRevision + 1;
  const frozenNextState = freezeLineState(nextState, canonicalRevision);
  binding.state.current = frozenNextState;
  publishLineBindingState(binding, frozenNextState, previousState);
  return frozenNextState;
}

function patchLineBindingState(binding, patch) {
  return replaceLineBindingState(binding, {
    ...binding.state.current,
    ...patch,
  });
}

function publishLineBindingState(binding, nextState, previousState = null) {
  const statusChanged = previousState === null
    || previousState.status !== nextState.status;
  const enteringPending = statusChanged && nextState.status.kind === "pending";
  if (enteringPending) {
    binding.statusSignal.set(nextState.status);
  }
  if (previousState === null || previousState.value !== nextState.value) {
    binding.valueSignal.set(nextState.value);
  }
  if (
    previousState === null
    || previousState.canonicalValue !== nextState.canonicalValue
  ) {
    binding.canonicalValueSignal.set(nextState.canonicalValue);
  }
  if (
    previousState === null
    || previousState.processing !== nextState.processing
  ) {
    binding.processingSignal.set(nextState.processing);
  }
  if (previousState === null || previousState.upload !== nextState.upload) {
    binding.uploadSignal.set(nextState.upload);
  }
  if (
    previousState === null
    || previousState.download !== nextState.download
  ) {
    binding.downloadSignal.set(nextState.download);
  }
  if (
    previousState === null
    || previousState.freshness !== nextState.freshness
  ) {
    binding.freshnessSignal.set(nextState.freshness);
  }
  if (
    previousState === null
    || previousState.diagnostics !== nextState.diagnostics
  ) {
    binding.diagnosticsSignal.set(nextState.diagnostics);
  }
  if (statusChanged && !enteringPending) {
    binding.statusSignal.set(nextState.status);
  }
}

export {
  createLineBindingState,
  patchLineBindingState,
  publishLineBindingState,
  readLineBindingState,
  replaceLineBindingState,
};
