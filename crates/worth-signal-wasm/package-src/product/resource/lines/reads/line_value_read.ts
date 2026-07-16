import { readLineBindingState } from "../state/line_binding_state.js";

function readLineValue(materialization) {
  return readLineBindingState(materialization.binding).value;
}

export { readLineValue };
