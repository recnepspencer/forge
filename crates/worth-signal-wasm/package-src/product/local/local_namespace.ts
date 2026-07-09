import { createLocalDialogState } from "./local_dialog_state.js";
import { createLocalFormSourceState } from "./local_form_source_state.js";
import { createLocalListState } from "./local_list_state.js";

export function createLocalNamespace(namespace) {
  return Object.freeze({
    dialogState(options) {
      return createLocalDialogState(namespace, options);
    },
    listState(options) {
      return createLocalListState(namespace, options);
    },
    formSource(options) {
      return createLocalFormSourceState(namespace, options);
    },
  });
}
