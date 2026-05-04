import { executeLineReload } from "./line_reload_execution.js";

function revalidateLine(materialization) {
  return executeLineReload(materialization, "revalidate");
}

export { revalidateLine };
