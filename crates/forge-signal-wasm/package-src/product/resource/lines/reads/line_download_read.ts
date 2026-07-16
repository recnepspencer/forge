import { readLineBindingState } from "../state/line_binding_state.js";

function readLineDownload(materialization) {
  return readLineBindingState(materialization.binding).download;
}

export { readLineDownload };
