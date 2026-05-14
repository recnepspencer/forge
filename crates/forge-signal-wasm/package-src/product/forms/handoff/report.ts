import { stableValueDigest } from "../values/value_paths.js";

export function readHandoffReport(store) {
  const current = store.current();
  const history = store.history();
  const summary = Object.freeze({
    status: current?.status ?? "ready",
    scopeKind: current?.scopeKind ?? null,
    surfaceId: current?.surfaceId ?? null,
    activeTarget: current?.target ?? null,
    unsupportedReason: current?.unsupportedReason ?? null,
  });
  const counters = Object.freeze({
    costBasis: "handoffPresentationHistoryScan",
    incrementalStatus: "notIncremental",
    updates: history.length,
    routeScopeUpdates: history.filter((entry) => entry.scopeKind === "route").length,
    modalScopeUpdates: history.filter((entry) => entry.scopeKind === "modal").length,
    externalScopeUpdates: history.filter((entry) => entry.scopeKind === "external").length,
    settlingUpdates: history.filter((entry) => entry.status === "settling").length,
    failedUpdates: history.filter((entry) => entry.status === "failed").length,
    unavailableUpdates: history.filter((entry) => entry.status === "unavailable").length,
  });
  return Object.freeze({
    current,
    history,
    summary,
    counters,
    digest: stableValueDigest({ current, history, summary, counters }),
  });
}
