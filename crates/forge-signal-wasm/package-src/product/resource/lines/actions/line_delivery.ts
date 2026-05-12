import { executeLineDelivery } from "./line_delivery_execution.js";

function deliverLine(materialization, packet) {
  return executeLineDelivery(materialization, packet);
}

export { deliverLine };
