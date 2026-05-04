import { executeLineReload } from "./line_reload_execution.js";

function refreshLine(materialization) {
  return executeLineReload(materialization, "refresh");
}

export { refreshLine };
