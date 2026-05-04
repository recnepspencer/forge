function createFreshFreshness() {
  return Object.freeze({
    kind: "fresh",
  });
}

function createPolicyStaleFreshness() {
  return Object.freeze({
    kind: "stale",
    reason: "policyProfile",
  });
}

function createRejectedFreshness(operation) {
  return Object.freeze({
    kind: "stale",
    reason:
      operation === "initialLoad"
        ? "initialLoadRejected"
        : operation === "refresh"
          ? "refreshRejected"
          : "revalidateRejected",
  });
}

function createPendingFreshness(operation) {
  return Object.freeze({
    kind: "stale",
    reason:
      operation === "initialLoad"
        ? "initialLoadPending"
        : operation === "refresh"
          ? "refreshPending"
          : "revalidatePending",
  });
}

function createTimedOutFreshness(operation) {
  return Object.freeze({
    kind: "stale",
    reason:
      operation === "initialLoad"
        ? "initialLoadTimedOut"
        : operation === "refresh"
          ? "refreshTimedOut"
          : "revalidateTimedOut",
  });
}

function createInvalidatedFreshness(cause) {
  return Object.freeze({
    kind: "stale",
    reason: cause,
  });
}

function createFreshnessFromPolicy(policy) {
  if (policy.staleAfterSettle) {
    return createPolicyStaleFreshness();
  }
  return createFreshFreshness();
}

export {
  createFreshFreshness,
  createFreshnessFromPolicy,
  createInvalidatedFreshness,
  createPendingFreshness,
  createPolicyStaleFreshness,
  createRejectedFreshness,
  createTimedOutFreshness,
};
