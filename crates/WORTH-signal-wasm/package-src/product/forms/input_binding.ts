export function createFormBoundInput(handle, options = {}) {
  const defaultSource = options.source;
  const parser = options.parse;
  return Object.freeze({
    input(rawValue, inputOptions = {}) {
      handle.input(rawValue, {
        commit: inputOptions.commit === true,
        source: inputOptions.source ?? defaultSource,
      });
    },
    compose(rawValue) {
      handle.compose(rawValue);
    },
    commit(rawValue) {
      if (rawValue !== undefined) {
        handle.input(rawValue, { source: defaultSource });
      }
      handle.commitInput(parser);
    },
    focus() {
      handle.focus();
    },
    blur() {
      handle.blur();
    },
    touch() {
      handle.touch();
    },
    visit() {
      handle.visit();
    },
    set(value) {
      handle.set(value);
    },
    clearDraft() {
      handle.clearDraft();
    },
  });
}
