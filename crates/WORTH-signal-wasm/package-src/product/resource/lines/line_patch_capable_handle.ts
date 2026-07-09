import { deliverLine } from "./actions/line_delivery.js";
import { requireActiveLine } from "./actions/line_activity_guard.js";
import { patchLine } from "./actions/line_patch.js";
import { readLineReconciliation } from "./reconciliation/line_reconciliation_read.js";
import { requireCurrentMaterialization } from "./state/line_handle_helpers.js";

function createPatchCapableLineHandle(handle, lineBacking) {
  return Object.freeze({
    ...handle,
    patch(patch) {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "patch");
      return patchLine(materialization, patch);
    },
    deliver(packet) {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "deliver");
      return deliverLine(materialization, packet);
    },
    reconciliation() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "reconciliation");
      return readLineReconciliation(materialization);
    },
  });
}

export { createPatchCapableLineHandle };
