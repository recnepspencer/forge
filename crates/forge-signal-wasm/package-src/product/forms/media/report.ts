import { stableValueDigest } from "../values/value_paths.js";

export function readMediaPresentationReport(store) {
  const current = store.current();
  const history = store.history();
  const summary = Object.freeze({
    status: current?.status ?? "ready",
    mode: current?.mode ?? null,
    surfaceId: current?.surfaceId ?? null,
    activeTarget: current?.target ?? null,
  });
  const counters = Object.freeze({
    costBasis: "mediaPresentationHistoryScan",
    incrementalStatus: "notIncremental",
    updates: history.length,
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
