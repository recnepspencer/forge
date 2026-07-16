import { readLineBindingState } from "../state/line_binding_state.js";

function readLineUpload(materialization) {
  return readLineBindingState(materialization.binding).upload;
}

export { readLineUpload };
