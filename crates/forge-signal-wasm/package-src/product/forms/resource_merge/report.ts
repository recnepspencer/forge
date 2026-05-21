import { stableValueDigest } from "../values/value_paths.js";
import { currentResourceMergeEffectDigest } from "./projection.js";

export function readResourceMergeReport(store, source) {
  const history = store.history().map((artifact) => normalizeStaleness(artifact, source));
  const current = normalizeStaleness(store.current(), source);
  const active = current && current.stale !== true ? current : null;
  const summary = Object.freeze({
    status: active?.status ?? "ready",
    stale: current?.stale ?? false,
    conflictCount: active?.conflictCount ?? 0,
    blockerCount: active?.blockers.length ?? 0,
    messageCount: active?.messages.length ?? 0,
    fieldCount: active?.projectedFields.length ?? 0,
    sectionCount: active?.projectedSections.length ?? 0,
  });
  const counters = Object.freeze({
    costBasis: "resourceMergePreviewHistoryScan",
    incrementalStatus: "notIncremental",
    previews: history.filter((entry) => entry.source === "preview").length,
    conflictPreviews: history.filter((entry) => entry.status === "conflict").length,
    unavailablePreviews: history.filter((entry) => entry.status === "unavailable").length,
    stalePreviews: history.filter((entry) => entry.stale === true).length,
    projectedFields: active?.projectedFields.length ?? 0,
    projectedSections: active?.projectedSections.length ?? 0,
    blockers: active?.blockers.length ?? 0,
    messages: active?.messages.length ?? 0,
  });
  return Object.freeze({
    current,
    history: Object.freeze(history),
    summary,
    counters,
    digest: stableValueDigest({
      currentDigest: current?.resultDigest ?? null,
      historyDigests: history.map((artifact) => artifact?.resultDigest ?? null),
      summary,
      counters,
    }),
  });
}

export function resourceMergeVisibleMessages(report) {
  const current = activeCurrentPreview(report);
  return current === null ? Object.freeze([]) : current.messages;
}

export function resourceMergeReadinessBlockers(report) {
  const current = activeCurrentPreview(report);
  return current === null ? Object.freeze([]) : current.blockers;
}

function activeCurrentPreview(report) {
  const current = report.current;
  if (current === null || current.stale === true) {
    return null;
  }
  return current;
}

function normalizeStaleness(artifact, source) {
  if (artifact === null) {
    return null;
  }
  const currentEffectDigest = currentResourceMergeEffectDigest(source);
  const stale = artifact.effectDigest !== null && artifact.effectDigest !== currentEffectDigest;
  return stale === artifact.stale
    ? artifact
    : Object.freeze({
      ...artifact,
      stale,
    });
}
