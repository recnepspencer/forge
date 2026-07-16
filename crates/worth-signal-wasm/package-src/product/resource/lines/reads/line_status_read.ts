import { readLineBindingState } from "../state/line_binding_state.js";

function readLineStatus(materialization) {
  return readLineBindingState(materialization.binding).status;
}

export { readLineStatus };
