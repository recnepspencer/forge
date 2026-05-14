import { stableValueDigest } from "../values/value_paths.js";

export function readMessagePresentationReport(store, semanticVisibleMessages) {
  const current = store.current();
  const history = store.history();
  const semanticSummary = {
    total: semanticVisibleMessages.length,
    visible: semanticVisibleMessages.filter((message) => message.visibility === "visible").length,
    summary: semanticVisibleMessages.filter((message) => message.visibility === "summary").length,
    blocked: semanticVisibleMessages.filter((message) => message.visibility === "blocked").length,
  };
  const summary = Object.freeze({
    status: current?.status ?? "ready",
    activeChannel: current?.channel ?? null,
    activeAudience: current?.audience ?? null,
    activeTarget: current?.target ?? null,
    semanticVisibleCount: semanticSummary.total,
    externalVisibleCount: current?.visibleCount ?? null,
  });
  const counters = Object.freeze({
    costBasis: "messagePresentationHistoryScan",
    incrementalStatus: "notIncremental",
    semanticVisibleMessages: semanticSummary.total,
    updates: history.length,
    settlingUpdates: history.filter((entry) => entry.status === "settling").length,
    failedUpdates: history.filter((entry) => entry.status === "failed").length,
    unavailableUpdates: history.filter((entry) => entry.status === "unavailable").length,
  });
  return Object.freeze({
    current,
    history,
    semantic: Object.freeze(semanticSummary),
    summary,
    counters,
    digest: stableValueDigest({
      current,
      history,
      semanticSummary,
      summary,
      counters,
    }),
  });
}
