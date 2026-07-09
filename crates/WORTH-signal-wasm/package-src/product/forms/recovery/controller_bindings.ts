export function createRecoveryBindings({
  formRef,
  source,
  writeDraft,
  resets,
  replayRestores,
}) {
  return Object.freeze({
    reset(options = {}) {
      return resets.acceptCanonicalValue({ form: formRef(), writeDraft }, options);
    },
    rollbackLastResourceEffect(options = {}) {
      return resets.rollbackLastResourceEffect(
        { form: formRef(), source, writeDraft },
        options,
      );
    },
    resetHistory() {
      return resets.history();
    },
    replayExactResourceSource(options = {}) {
      return replayRestores.replayExactResourceSource(
        { form: formRef(), source },
        options,
      );
    },
    restoreExactResourceSource(options = {}) {
      return replayRestores.restoreExactResourceSource(
        { form: formRef(), source },
        options,
      );
    },
    replayRestoreHistory() {
      return replayRestores.history();
    },
  });
}
