import { BUILT_IN_ACTION_IDS, createActionRuntimeMap, isPlainObject, normalizeCustomActions, normalizeInitialState, requireDialogOptions } from "./local_dialog_declaration_support.js";
import { createStateSnapshot, changedDialogStateKeys, cloneValue, createMessage, dialogStateChanged, stableDigest, stepProgressFromForm } from "./local_dialog_state_projection_support.js";
import { COLLABORATION_POSTURE_RANK, createNativeCollaborationState, dialogCollaborationConflicts, dialogCollaborationEventKind } from "./local_dialog_collaboration_support.js";
import { executeLocalDialogAction } from "./local_dialog_action_runtime.js";
import { RAW_SIGNALS } from "../symbols.js";

export function createLocalDialogState(namespace, options) {
  const normalized = requireDialogOptions(options);
  const initialState = normalizeInitialState(normalized);
  const scope = namespace.scope(normalized.identity);
  const rawSignals = namespace[RAW_SIGNALS];
  const handles = Object.freeze({
    isOpen: scope.spec.input("isOpen", initialState.isOpen, { debugName: `${scope.scopeId}.isOpen` }),
    mode: scope.spec.input("mode", initialState.mode, { debugName: `${scope.scopeId}.mode` }),
    data: scope.spec.input("data", initialState.data, { debugName: `${scope.scopeId}.data` }),
    context: scope.spec.input("context", initialState.context, { debugName: `${scope.scopeId}.context` }),
    loading: scope.spec.input("loading", initialState.loading, { debugName: `${scope.scopeId}.loading` }),
    bindingRevision: scope.spec.input("bindingRevision", 0, { debugName: `${scope.scopeId}.bindingRevision` }),
    collaborationRevision: scope.spec.input("collaborationRevision", 0, { debugName: `${scope.scopeId}.collaborationRevision` }),
    actionRevision: scope.spec.input("actionRevision", 0, { debugName: `${scope.scopeId}.actionRevision` }),
    historyRevision: scope.spec.input("historyRevision", 0, { debugName: `${scope.scopeId}.historyRevision` }),
  });
  const customActions = normalizeCustomActions(normalized, scope.scopeId);
  const actionIds = Object.freeze([...BUILT_IN_ACTION_IDS, ...Object.keys(customActions)]);
  const actionRuntime = createActionRuntimeMap(actionIds);
  const stateHistory = [];
  const actionHistory = [];
  const collaborationHistory = [];
  const collaborationEvents = [];
  let sourceState = cloneValue(initialState);
  let boundForm = null;
  let boundFormSummaryWatch = null;
  let bindingRevisionVersion = 0;
  let bindingRevisionFlushQueued = false;
  let nativeCollaboration = createNativeCollaborationState(normalized.collaboration);
  let currentNativeArtifact = null;
  let nextArtifactId = 1;

  function disposeBoundFormSummaryWatch() {
    if (!boundFormSummaryWatch) {
      return;
    }
    if (typeof boundFormSummaryWatch[Symbol.dispose] === "function") {
      boundFormSummaryWatch[Symbol.dispose]();
    } else if (typeof boundFormSummaryWatch.free === "function") {
      boundFormSummaryWatch.free();
    } else if (typeof boundFormSummaryWatch.dispose === "function") {
      boundFormSummaryWatch.dispose();
    }
    boundFormSummaryWatch = null;
  }
  function scheduleBindingRevisionRefresh() {
    if (bindingRevisionFlushQueued) {
      return;
    }
    bindingRevisionFlushQueued = true;
    queueMicrotask(() => {
      bindingRevisionFlushQueued = false;
      bindingRevisionVersion += 1;
      void handles.bindingRevision.set(bindingRevisionVersion);
    });
  }

  function recordHistory(action, reason, previous, next) {
    stateHistory.push(Object.freeze({ kind: "dialogState", action, reason: reason ?? null, previous, next, timestampMs: Date.now() }));
    handles.historyRevision.set(handles.historyRevision() + 1);
  }

  function readFormState() {
    if (!boundForm) {
      return Object.freeze({
        dirty: false,
        readiness: { blockers: [] },
        collaboration: null,
        visibleMessages: [],
        currentStepId: null,
        stepProgress: "none",
        routeBlocked: null,
      });
    }
    const { form } = boundForm;
    const dirty = form.dirty();
    const step = stepProgressFromForm(form);
    return Object.freeze({
      dirty: typeof dirty === "object" && dirty !== null ? Boolean(dirty.isDirty) : Boolean(dirty),
      readiness: form.readiness(),
      collaboration: form.collaboration(),
      visibleMessages: form.visibleMessages(),
      currentStepId: step.currentStepId,
      stepProgress: step.progress,
      routeBlocked: step.routeBlocked,
    });
  }

  function readCollaboration() {
    const formState = readFormState();
    const formCollaboration = formState.collaboration;
    const conflicts = dialogCollaborationConflicts(nativeCollaboration, formCollaboration);
    const top = [nativeCollaboration, formCollaboration]
      .filter(Boolean)
      .reduce((current, candidate) => {
        if (!candidate) {
          return current;
        }
        if (!current) {
          return candidate;
        }
        return COLLABORATION_POSTURE_RANK[candidate.posture] >= COLLABORATION_POSTURE_RANK[current.posture]
          ? candidate
          : current;
      }, null);
    return Object.freeze({
      declared: nativeCollaboration.declared || Boolean(formCollaboration?.declared),
      mode: top?.mode ?? "notDeclared",
      actorId: top?.actorId ?? null,
      posture: top?.posture ?? "notDeclared",
      reason: top?.reason ?? "dialog collaboration is not declared",
      lockOwnerId: nativeCollaboration.lockOwnerId ?? formCollaboration?.lockOwnerId ?? null,
      leasedModes: nativeCollaboration.leasedModes,
      branchId: nativeCollaboration.branchId ?? formCollaboration?.branchId ?? null,
      readOnly: nativeCollaboration.readOnly || Boolean(formCollaboration?.readOnly),
      remoteUpdateDigest: nativeCollaboration.remoteUpdateDigest ?? formCollaboration?.remoteUpdateDigest ?? null,
      presence: Object.freeze([...(nativeCollaboration.presence ?? []), ...(formCollaboration?.presence ?? [])]),
      comments: Object.freeze([...(nativeCollaboration.comments ?? []), ...(formCollaboration?.comments ?? [])]),
      history: Object.freeze([...collaborationHistory]),
      events: Object.freeze([...collaborationEvents]),
      conflicts,
      sources: Object.freeze({ native: currentNativeArtifact, boundForm: formCollaboration ?? null }),
      digest: stableDigest({
        native: nativeCollaboration.digest,
        form: formCollaboration?.digest ?? null,
        conflicts,
      }),
    });
  }

  function readCloseBlockers() {
    const formState = readFormState();
    const blockers = [];
    if (handles.loading()) {
      blockers.push(Object.freeze({ kind: "dialog:loading", source: "dialog", action: "close", reason: "dialog is still loading" }));
    }
    if ((boundForm?.options.blockCloseWhenDirty ?? true) && formState.dirty) {
      blockers.push(Object.freeze({ kind: "dialog:dirty", source: "form", action: "close", reason: "bound form has unsaved changes" }));
    }
    if (formState.routeBlocked) {
      blockers.push(Object.freeze({ kind: "dialog:step", source: "form", action: "close", reason: formState.routeBlocked.reason ?? "route-coupled step is unavailable" }));
    }
    return Object.freeze(blockers);
  }

  function readWriteBlockers(actionId) {
    const collaboration = readCollaboration();
    const blockers = [];
    for (const conflict of collaboration.conflicts) {
      blockers.push(Object.freeze({ kind: `collaboration:${conflict.kind}`, source: "collaboration", action: actionId, reason: conflict.reason }));
    }
    if (collaboration.posture === "blocked" || collaboration.posture === "unavailable") {
      blockers.push(Object.freeze({ kind: `collaboration:${collaboration.posture}`, source: "collaboration", action: actionId, reason: collaboration.reason }));
    }
    if (collaboration.readOnly) {
      blockers.push(Object.freeze({ kind: "collaboration:readOnly", source: "collaboration", action: actionId, reason: "dialog is currently read-only" }));
    }
    if (handles.loading()) {
      blockers.push(Object.freeze({ kind: "dialog:loading", source: "dialog", action: actionId, reason: "dialog is still loading" }));
    }
    return Object.freeze(blockers);
  }

  function readActionPlan(actionId) {
    if (actionId === "close") {
      const blockers = readCloseBlockers();
      return Object.freeze({ actionId, status: blockers.length === 0 ? "accepted" : "blocked", readiness: Object.freeze({ canRun: blockers.length === 0, blockers }) });
    }
    if (actionId === "discard") {
      return Object.freeze({ actionId, status: "accepted", readiness: Object.freeze({ canRun: true, blockers: Object.freeze([]) }) });
    }
    if (actionId === "confirm") {
      const form = boundForm?.form ?? null;
      const formPlan = form && boundForm.options.confirmActionId ? form.actionPlan(boundForm.options.confirmActionId) : null;
      const blockers = Object.freeze([...(formPlan?.readiness?.blockers ?? []), ...readWriteBlockers(actionId)]);
      return Object.freeze({ actionId, status: blockers.length === 0 ? "accepted" : "blocked", readiness: Object.freeze({ canRun: blockers.length === 0 && (formPlan?.readiness?.canRun ?? true), blockers }) });
    }
    const custom = customActions[actionId];
    const blockers = [...readWriteBlockers(actionId)];
    if (typeof custom.readiness === "function") {
      const result = custom.readiness(readActionContext());
      if (result === false) {
        blockers.push(Object.freeze({ kind: "dialog:customReadiness", source: "dialog", action: actionId, reason: "custom dialog action is blocked" }));
      } else if (isPlainObject(result) && result.canRun === false) {
        blockers.push(...(result.blockers?.length ? result.blockers : [Object.freeze({ kind: "dialog:customReadiness", source: "dialog", action: actionId, reason: result.reason ?? "custom dialog action is blocked" })]));
      }
    }
    return Object.freeze({ actionId, status: blockers.length === 0 ? "accepted" : "blocked", readiness: Object.freeze({ canRun: blockers.length === 0, blockers: Object.freeze(blockers) }) });
  }

  function readReadiness() {
    const actions = Object.freeze(Object.fromEntries(actionIds.map((actionId) => [actionId, readActionPlan(actionId)])));
    const formState = readFormState();
    const currentState = createStateSnapshot(handles);
    return Object.freeze({
      dirty: dialogStateChanged(sourceState, currentState),
      blockers: Object.freeze(Object.values(actions).flatMap((entry) => entry.readiness.blockers)),
      actions,
      currentStepId: formState.currentStepId,
      stepProgress: formState.stepProgress,
    });
  }

  function readVisibleMessages() {
    const readiness = readReadiness();
    const formState = readFormState();
    const messages = readiness.blockers.map((blocker) =>
      createMessage(`dialog.${blocker.kind}`, blocker.source, blocker.action === "close" ? "warning" : "error", blocker.action === "close" ? "visible" : "blocked", blocker.reason),
    );
    for (const message of formState.visibleMessages) {
      messages.push(createMessage("dialog.form.visibleMessage", "form", "info", "summary", message?.message?.text ?? message?.code ?? "bound form reported a visible message", message?.target ?? null));
    }
    const collaboration = readCollaboration();
    for (const conflict of collaboration.conflicts) {
      messages.push(createMessage("dialog.collaboration.conflict", "collaboration", "error", "blocked", conflict.reason));
    }
    if (collaboration.posture !== "notDeclared" && collaboration.posture !== "active") {
      messages.push(createMessage("dialog.collaboration.posture", "collaboration", collaboration.posture === "settling" ? "warning" : "error", "summary", collaboration.reason));
    }
    return Object.freeze(messages);
  }

  function readActionContext() {
    return Object.freeze({
      dialog: dialogState,
      state: createStateSnapshot(handles),
      collaboration: readCollaboration(),
      form: boundForm?.form ?? null,
    });
  }

  function readActionBinding(actionId) {
    if (!actionRuntime.has(actionId)) {
      throw new TypeError(`signals.local.dialogState(...) action("${actionId}") is not declared on "${scope.scopeId}"`);
    }
    const runtime = actionRuntime.get(actionId);
    const plan = readActionPlan(actionId);
    return Object.freeze({
      plan,
      disabled: plan.status !== "accepted" || !plan.readiness.canRun || runtime.pending,
      pending: runtime.pending,
      latestExecution: runtime.latestExecution,
      resultKind: runtime.latestExecution?.resultKind ?? null,
      execute() {
        return executeAction(actionId);
      },
    });
  }

  async function applyState(next, historyAction, reason, options = {}) {
    const previous = createStateSnapshot(handles);
    let result = null;
    for (const [key, value] of Object.entries(next)) {
      if (value === undefined) {
        continue;
      }
      result = await handles[key].set(value);
    }
    const current = createStateSnapshot(handles);
    if (options.updateSource === true) {
      sourceState = cloneValue(current);
    }
    recordHistory(historyAction, reason, previous, current);
    return result;
  }

  function updateActionRuntime(actionId, next) {
    actionRuntime.set(actionId, next);
    handles.actionRevision.set(handles.actionRevision() + 1);
  }

  function recordActionExecution(entry) {
    actionHistory.push(Object.freeze(entry));
    handles.historyRevision.set(handles.historyRevision() + 1);
  }

  function executeAction(actionId) {
    return executeLocalDialogAction({
      actionId,
      binding: readActionBinding(actionId),
      boundForm,
      customActions,
      readActionContext,
      applyState,
      updateActionRuntime,
      recordActionExecution,
      loadingHandle: handles.loading,
    });
  }

    const summarySignalHandle = scope.computed(() => {
      handles.bindingRevision();
      handles.collaborationRevision();
      handles.actionRevision();
      handles.historyRevision();
    return Object.freeze({
        state: createStateSnapshot(handles),
        source: sourceState,
      dirty: dialogStateChanged(sourceState, createStateSnapshot(handles)),
      readiness: readReadiness(),
      collaborationDigest: readCollaboration().digest,
      actionHistoryLength: actionHistory.length,
      stateHistoryLength: stateHistory.length,
    });
  }, {
    debugName: normalized.debugName ?? `${scope.scopeId}.summary`,
  });

  const dialogState = Object.freeze({
    scope,
    scopeId: scope.scopeId,
    isOpen: handles.isOpen,
    mode: handles.mode,
    data: handles.data,
    context: handles.context,
    loading: handles.loading,
    source() { return cloneValue(sourceState); },
    draft() { return cloneValue(createStateSnapshot(handles)); },
    effective() { return cloneValue(createStateSnapshot(handles)); },
    dirty() { return dialogStateChanged(sourceState, createStateSnapshot(handles)); },
    patchPlan() { const current = createStateSnapshot(handles); return Object.freeze({ changed: dialogStateChanged(sourceState, current), changedKeys: changedDialogStateKeys(sourceState, current) }); },
    readiness() { return readReadiness(); },
    visibleMessages() { return readVisibleMessages(); },
    summarySignal() { return summarySignalHandle; },
    stateHistory() { return Object.freeze([...stateHistory]); },
    actionHistory() { return Object.freeze([...actionHistory]); },
    diagnostics() { return Object.freeze({ state: createStateSnapshot(handles), source: cloneValue(sourceState), readiness: readReadiness(), collaboration: readCollaboration(), actions: dialogState.actions() }); },
    open(mode, extra = {}) { return applyState({ isOpen: true, mode, data: extra.data, context: extra.context, loading: Boolean(extra.loading) }, "open", extra.reason ?? "dialog open", { updateSource: true }); },
    close(options = {}) { return applyState(options.clear ? { isOpen: false, mode: null, data: null, context: null, loading: false } : { isOpen: false, loading: false }, "close", options.reason ?? "dialog close", { updateSource: true }); },
    toggle(options = {}) { return handles.isOpen() ? dialogState.close(options) : dialogState.open(handles.mode() ?? normalized.modes?.[0] ?? null, options); },
    patch(next) { return applyState(next, "patch", "dialog patch"); },
    setLoading(next, options = {}) { return applyState({ loading: Boolean(next) }, "setLoading", options.reason ?? "dialog loading change"); },
    reset(options = {}) { return applyState(sourceState, "reset", options.reason ?? "dialog reset"); },
    async requestClose(options = {}) {
      const plan = readActionPlan("close");
      const current = createStateSnapshot(handles);
      stateHistory.push(Object.freeze({ kind: "dialogState", action: "requestClose", reason: options.reason ?? "dialog request close", previous: current, next: current, timestampMs: Date.now() }));
      handles.historyRevision.set(handles.historyRevision() + 1);
      if (!plan.readiness.canRun) {
        return Object.freeze({ status: "blocked", blockers: plan.readiness.blockers, closed: false });
      }
      await dialogState.close(options);
      return Object.freeze({ status: "accepted", blockers: Object.freeze([]), closed: true });
    },
    action(actionId) { return readActionBinding(actionId); },
    actions() { return Object.freeze(Object.fromEntries(actionIds.map((actionId) => [actionId, readActionBinding(actionId)]))); },
    bindForm(form, bindOptions = {}) {
      disposeBoundFormSummaryWatch();
      boundForm = { form, options: { blockCloseWhenDirty: bindOptions.blockCloseWhenDirty ?? true, ...bindOptions } };
      const summarySignal = form.summarySignal?.();
      if (summarySignal?.id) {
        boundFormSummaryWatch = rawSignals.watch(summarySignal.id, () => {
          scheduleBindingRevisionRefresh();
        });
      }
      bindingRevisionVersion += 1;
      handles.bindingRevision.set(bindingRevisionVersion);
    },
    collaboration() { return readCollaboration(); },
    reportCollaboration(artifact) {
      const previous = currentNativeArtifact;
      currentNativeArtifact = Object.freeze({
        kind: "dialogCollaboration",
        artifactId: nextArtifactId++,
        source: "report",
        mode: normalized.collaboration?.mode ?? null,
        actorId: normalized.collaboration?.actorId ?? null,
        posture: artifact.posture,
        reason: artifact.reason,
        lockOwnerId: artifact.lockOwnerId ?? null,
        leasedModes: Object.freeze([...(artifact.leasedModes ?? [])]),
        branchId: artifact.branchId ?? null,
        readOnly: Boolean(artifact.readOnly),
        remoteUpdateDigest: artifact.remoteUpdateDigest ?? null,
        presence: Object.freeze([...(artifact.presence ?? [])]),
        comments: Object.freeze([...(artifact.comments ?? [])]),
        digest: stableDigest({ artifact }),
      });
      nativeCollaboration = Object.freeze({ declared: Boolean(normalized.collaboration), mode: normalized.collaboration?.mode ?? "notDeclared", actorId: normalized.collaboration?.actorId ?? null, posture: artifact.posture, reason: artifact.reason, lockOwnerId: artifact.lockOwnerId ?? null, leasedModes: Object.freeze([...(artifact.leasedModes ?? [])]), branchId: artifact.branchId ?? null, readOnly: Boolean(artifact.readOnly), remoteUpdateDigest: artifact.remoteUpdateDigest ?? null, presence: Object.freeze([...(artifact.presence ?? [])]), comments: Object.freeze([...(artifact.comments ?? [])]), digest: currentNativeArtifact.digest });
      collaborationHistory.push(currentNativeArtifact);
      collaborationEvents.push(Object.freeze({ kind: dialogCollaborationEventKind(previous, currentNativeArtifact), source: "report", artifactId: currentNativeArtifact.artifactId, previousArtifactId: previous?.artifactId ?? null, mode: currentNativeArtifact.mode, posture: currentNativeArtifact.posture, reason: currentNativeArtifact.reason, lockOwnerId: currentNativeArtifact.lockOwnerId, leasedModes: currentNativeArtifact.leasedModes, branchId: currentNativeArtifact.branchId, readOnly: currentNativeArtifact.readOnly, remoteUpdateDigest: currentNativeArtifact.remoteUpdateDigest, presence: currentNativeArtifact.presence, comments: currentNativeArtifact.comments, digest: currentNativeArtifact.digest }));
      handles.collaborationRevision.set(handles.collaborationRevision() + 1);
      return currentNativeArtifact;
    },
    clearCollaboration(options = {}) {
      const cleared = dialogState.reportCollaboration({ posture: normalized.collaboration ? "active" : "unavailable", reason: options.reason ?? "dialog collaboration cleared" });
      currentNativeArtifact = Object.freeze({ ...cleared, source: "clear" });
      collaborationHistory[collaborationHistory.length - 1] = currentNativeArtifact;
      return currentNativeArtifact;
    },
    free() { disposeBoundFormSummaryWatch(); summarySignalHandle.free(); Object.values(handles).forEach((handle) => handle.free()); },
    [Symbol.dispose]() { disposeBoundFormSummaryWatch(); summarySignalHandle[Symbol.dispose](); Object.values(handles).forEach((handle) => handle[Symbol.dispose]()); },
  });

  return dialogState;
}
