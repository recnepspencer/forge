import { executeLinePatch } from "./line_patch_execution.js";

function patchLine(materialization, patch, options) {
  return executeLinePatch(materialization, patch, options);
}

export { patchLine };
