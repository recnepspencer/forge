import { stableValueDigest } from "../values/value_paths.js";

export function readExitPresentationReport(store, basis) {
  const current = store.current();
  const history = store.history();
  const summary = Object.freeze({
    status: current?.status ?? basis.status,
    scopeKind: current?.scopeKind ?? null,
    surfaceId: current?.surfaceId ?? null,
    activeTarget: current?.target ?? basis.activeTarget,
    unavailableReason: current?.unavailableReason ?? basis.unavailableReason,
    guardKind: basis.guardKind,
    pendingActions: basis.pendingActions,
    requiresConfirmation: basis.requiresConfirmation,
  });
  const counters = Object.freeze({
    costBasis: "exitPresentationDerivedAndHistoryScan",
    incrementalStatus: "notIncremental",
    updates: history.length,
    routeScopeUpdates: history.filter((entry) => entry.scopeKind === "route").length,
    modalScopeUpdates: history.filter((entry) => entry.scopeKind === "modal").length,
    externalScopeUpdates: history.filter((entry) => entry.scopeKind === "external").length,
    settlingUpdates: history.filter((entry) => entry.status === "settling").length,
    failedUpdates: history.filter((entry) => entry.status === "failed").length,
    unavailableUpdates: history.filter((entry) => entry.status === "unavailable").length,
    pendingActions: basis.pendingActions,
    dirtyGuardActivations: basis.guardKind === "dirty" ? 1 : 0,
    sourceUnavailableGuards: basis.guardKind === "sourceUnavailable" ? 1 : 0,
  });
  return Object.freeze({
    current,
    history,
    summary,
    counters,
    digest: stableValueDigest({ current, history, summary, counters, basis }),
  });
}

export function deriveExitPresentationBasis(form) {
  const sourceCompatibility = form.sourceCompatibility();
  if (sourceCompatibility.posture === "unavailable") {
    return Object.freeze({
      status: "unavailable",
      guardKind: "sourceUnavailable",
      activeTarget: null,
      unavailableReason: sourceCompatibility.reason ?? "exit presentation is unavailable because form truth is unresolved",
      pendingActions: 0,
      requiresConfirmation: false,
    });
  }
  const pendingActions = form.actionExecutionHistory().filter((entry) => entry.resultKind === "pending").length;
  if (pendingActions > 0) {
    return Object.freeze({
      status: "busy",
      guardKind: "pendingAction",
      activeTarget: "pending-actions",
      unavailableReason: null,
      pendingActions,
      requiresConfirmation: false,
    });
  }
  const dirty = form.dirty();
  if (dirty.isDirty) {
    return Object.freeze({
      status: "busy",
      guardKind: "dirty",
      activeTarget: dirty.fields[0]?.field ?? "dirty-draft",
      unavailableReason: null,
      pendingActions: 0,
      requiresConfirmation: true,
    });
  }
  return Object.freeze({
    status: "ready",
    guardKind: "clean",
    activeTarget: null,
    unavailableReason: null,
    pendingActions: 0,
    requiresConfirmation: false,
  });
}
