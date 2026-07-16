import { readLineBindingState } from "../state/line_binding_state.js";

function readLineFreshness(materialization) {
  return readLineBindingState(materialization.binding).freshness;
}

export { readLineFreshness };
