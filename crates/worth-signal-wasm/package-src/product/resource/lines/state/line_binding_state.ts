function freezeLineState(state) {
  return Object.freeze({
    value: state.value,
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
    current: freezeLineState(initialState),
  };
}

function readLineBindingState(binding) {
  return binding.state.current;
}

function replaceLineBindingState(binding, nextState) {
  const frozenNextState = freezeLineState(nextState);
  const previousState = binding.state.current;
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
  if (previousState === null || previousState.value !== nextState.value) {
    binding.valueSignal.set(nextState.value);
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
  if (previousState === null || previousState.status !== nextState.status) {
    binding.statusSignal.set(nextState.status);
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
}

export {
  createLineBindingState,
  patchLineBindingState,
  publishLineBindingState,
  readLineBindingState,
  replaceLineBindingState,
};
