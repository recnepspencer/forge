import { deliverLine } from "./actions/line_delivery.js";
import { requireActiveLine } from "./actions/line_activity_guard.js";
import { patchLine } from "./actions/line_patch.js";
import { readLineReconciliation } from "./reconciliation/line_reconciliation_read.js";

function createPatchCapableLineHandle(handle, materialization) {
  return Object.freeze({
    ...handle,
    patch(patch) {
      requireActiveLine(materialization, "patch");
      return patchLine(materialization, patch);
    },
    deliver(packet) {
      requireActiveLine(materialization, "deliver");
      return deliverLine(materialization, packet);
    },
    reconciliation() {
      requireActiveLine(materialization, "reconciliation");
      return readLineReconciliation(materialization);
    },
  });
}

export { createPatchCapableLineHandle };
