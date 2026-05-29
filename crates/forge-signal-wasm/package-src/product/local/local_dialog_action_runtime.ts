import { isSuccessfulBoundFormExecution } from "./local_dialog_collaboration_support.js";

export async function executeLocalDialogAction(options) {
  const {
    actionId,
    binding,
    boundForm,
    customActions,
    readActionContext,
    applyState,
    updateActionRuntime,
    recordActionExecution,
    loadingHandle,
  } = options;
  if (binding.plan.status !== "accepted") {
    const blocked = Object.freeze({
      actionId,
      source: "dialog",
      resultKind: "blocked",
      reason: binding.plan.readiness.blockers[0]?.reason ?? "dialog action is blocked",
      startedAtMs: Date.now(),
      finishedAtMs: Date.now(),
      error: null,
      delegatedResultKind: null,
    });
    updateActionRuntime(actionId, { pending: false, latestExecution: blocked });
    recordActionExecution(blocked);
    return blocked;
  }
  const pending = Object.freeze({
    actionId,
    source: actionId === "confirm" && boundForm?.options.confirmActionId ? "form" : "dialog",
    resultKind: "pending",
    reason: null,
    startedAtMs: Date.now(),
    finishedAtMs: null,
    error: null,
    delegatedResultKind: null,
  });
  updateActionRuntime(actionId, { pending: true, latestExecution: pending });
  try {
    if (actionId === "close") {
      await applyState({ isOpen: false, loading: false }, "close", "close action", { updateSource: true });
    } else if (actionId === "discard") {
      await applyState({ isOpen: false, loading: false }, "close", "discard action", { updateSource: true });
      if (boundForm?.options.resetOnClose) {
        boundForm.form.reset?.({ reason: "dialog discard action" });
      }
    } else if (actionId === "confirm") {
      if (boundForm?.options.confirmActionId) {
        const execution = await boundForm.form.executeAction(boundForm.options.confirmActionId);
        if (!isSuccessfulBoundFormExecution(execution)) {
          const nonFulfilled = Object.freeze({
            actionId,
            source: "form",
            resultKind: execution?.resultKind ?? "rejected",
            reason: execution?.reason ?? "bound form confirm action did not fulfill",
            startedAtMs: pending.startedAtMs,
            finishedAtMs: Date.now(),
            error: execution?.error ?? null,
            delegatedResultKind: execution?.resultKind ?? null,
          });
          updateActionRuntime(actionId, { pending: false, latestExecution: nonFulfilled });
          recordActionExecution(nonFulfilled);
          return nonFulfilled;
        }
      }
      if (boundForm?.options.closeOnSuccess) {
        await applyState({ isOpen: false, loading: false }, "close", "confirm action", { updateSource: true });
        if (boundForm.options.resetOnClose) {
          boundForm.form.reset?.({ reason: "dialog confirm close" });
        }
      }
    } else {
      const custom = customActions[actionId];
      await custom.execute(readActionContext());
      if (custom.closeOnSuccess) {
        await applyState({ isOpen: false, loading: false }, "close", `${actionId} action`, { updateSource: true });
      }
    }
    const fulfilled = Object.freeze({
      actionId,
      source: pending.source,
      resultKind: "fulfilled",
      reason: null,
      startedAtMs: pending.startedAtMs,
      finishedAtMs: Date.now(),
      error: null,
      delegatedResultKind: null,
    });
    updateActionRuntime(actionId, { pending: false, latestExecution: fulfilled });
    recordActionExecution(fulfilled);
    return fulfilled;
  } catch (error) {
    const rejected = Object.freeze({
      actionId,
      source: pending.source,
      resultKind: "rejected",
      reason: error instanceof Error ? error.message : "dialog action failed",
      startedAtMs: pending.startedAtMs,
      finishedAtMs: Date.now(),
      error,
      delegatedResultKind: null,
    });
    updateActionRuntime(actionId, { pending: false, latestExecution: rejected });
    recordActionExecution(rejected);
    if (!boundForm?.options.stayOpenOnError) {
      await loadingHandle.set(false);
    }
    return rejected;
  }
}
