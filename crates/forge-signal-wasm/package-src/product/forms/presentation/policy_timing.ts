export function busyVisibilityStatus(policy, startedAtMs, nowMs) {
  if (policy.delayedBusyRevealMs <= 0) {
    return "busy";
  }
  return nowMs - startedAtMs < policy.delayedBusyRevealMs ? "pending" : "busy";
}

export function minimumBusyPending(policy, startedAtMs, nowMs) {
  return policy.minimumBusyMs > 0 && nowMs - startedAtMs < policy.minimumBusyMs;
}

export function settlementTimedOut(policy, settlingStartedAtMs, nowMs) {
  return policy.settlementTimeoutMs > 0 && nowMs - settlingStartedAtMs >= policy.settlementTimeoutMs;
}
