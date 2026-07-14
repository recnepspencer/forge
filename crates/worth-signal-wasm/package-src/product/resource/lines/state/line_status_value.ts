function createFulfilledLineStatus(operation) {
  return Object.freeze({
    kind: "fulfilled",
    operation,
  });
}

function createPendingLineStatus(operation, hasVisibleValue) {
  return Object.freeze({
    kind: "pending",
    operation,
    continuity: hasVisibleValue ? "preservedVisibleValue" : "noVisibleValueYet",
  });
}

function createTimedOutLineStatus(operation, hasVisibleValue) {
  return Object.freeze({
    kind: "timedOut",
    operation,
    continuity: hasVisibleValue ? "preservedVisibleValue" : "noVisibleValueYet",
  });
}

function createRejectedLineStatus(operation, message, hasVisibleValue) {
  return Object.freeze({
    kind: "rejected",
    operation,
    message,
    continuity: hasVisibleValue ? "preservedVisibleValue" : "noVisibleValueYet",
  });
}

export {
  createFulfilledLineStatus,
  createPendingLineStatus,
  createTimedOutLineStatus,
  createRejectedLineStatus,
};
