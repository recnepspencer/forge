import { readLineBindingState } from "../state/line_binding_state.js";

function readLineProcessing(materialization) {
  return readLineBindingState(materialization.binding).processing;
}

export { readLineProcessing };
