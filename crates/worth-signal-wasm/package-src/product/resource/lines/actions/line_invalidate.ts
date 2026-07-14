import { invalidateLine } from "./line_invalidation_execution.js";

function invalidateSingleLine(materialization) {
  return invalidateLine(materialization, "manualLineInvalidate", "line");
}

export { invalidateSingleLine };
