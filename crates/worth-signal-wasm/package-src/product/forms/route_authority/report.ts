import { stableValueDigest } from "../values/value_paths.js";

export function readRouteAuthorityReport(routeAuthority) {
  const current = routeAuthority.current();
  const history = routeAuthority.history();
  const latest = history.at(-1) ?? null;
  const summary = Object.freeze({
    routeId: current?.routeId ?? null,
    href: current?.href ?? null,
    surfaceId: current?.surfaceId ?? null,
    continuity: latest?.continuity ?? null,
    handoff: latest?.handoff ?? null,
    draftContinuity: latest?.draftContinuity ?? null,
    authorityAvailable: latest?.handoff?.routeCoupledBehavior === "admitted",
    continuityApplied: latest?.continuityApplied ?? null,
    transitionKind: latest?.transitionKind ?? null,
    previousAuthorityDigest: latest?.previousAuthorityDigest ?? null,
    previousDraftDigest: latest?.previousDraftDigest ?? null,
    nextDraftDigest: latest?.nextDraftDigest ?? null,
  });
  const counters = Object.freeze({
    costBasis: "routeAuthorityHistoryScan",
    incrementalStatus: "notIncremental",
    updates: history.length,
    clearUpdates: history.filter((entry) => entry.source === "clear").length,
    initialReports: history.filter((entry) => entry.transitionKind === "initialAuthority").length,
    changedReports: history.filter((entry) => entry.transitionKind === "authorityChanged").length,
    refreshedReports: history.filter((entry) => entry.transitionKind === "authorityRefreshed").length,
    clearedTransitions: history.filter((entry) => entry.transitionKind === "authorityCleared").length,
    redundantClears: history.filter((entry) => entry.transitionKind === "alreadyCleared").length,
    preservedDraftUpdates: history.filter((entry) => entry.draftContinuity.posture === "preservedDraft").length,
    frozenDraftUpdates: history.filter((entry) => entry.draftContinuity.posture === "frozeDraft").length,
    discardedDraftUpdates: history.filter((entry) => entry.draftContinuity.posture === "discardedDraft").length,
    deferredDraftUpdates: history.filter((entry) => entry.draftContinuity.posture === "deferredDraft").length,
    preserveUpdates: history.filter((entry) => entry.continuity === "preserve").length,
    freezeUpdates: history.filter((entry) => entry.continuity === "freeze").length,
    discardUpdates: history.filter((entry) => entry.continuity === "discard").length,
    deferUpdates: history.filter((entry) => entry.continuity === "defer").length,
  });
  return Object.freeze({
    current,
    history,
    summary,
    counters,
    digest: stableValueDigest({
      currentDigest: current?.routeAuthorityDigest ?? null,
      historyDigests: history.map((entry) => entry.routeAuthorityDigest),
      summary,
      counters,
    }),
  });
}
