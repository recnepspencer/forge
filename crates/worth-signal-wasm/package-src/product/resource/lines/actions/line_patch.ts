import { executeLinePatch } from "./line_patch_execution.js";

function patchLine(materialization, patch) {
  return executeLinePatch(materialization, patch);
}

export { patchLine };
