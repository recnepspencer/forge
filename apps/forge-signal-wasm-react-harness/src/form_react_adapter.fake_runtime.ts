export function createFakeStore(formFactory?: (declaration: unknown) => unknown) {
  const diagnosticsListeners = new Set<() => void>();
  const signalListeners = new Map<string, Set<() => void>>();
  const signalVersions = new Map<string, number>();
  const signalSnapshots = new Map<string, { version: number; value: unknown }>();
  let version = 0;
  let currentSnapshot = {
    latestObservation: null,
    latestFlow: null,
    performanceSummary: {
      version,
    },
  };

  function notifySignal(signalId: string | undefined): void {
    if (!signalId) {
      return;
    }
    signalListeners.get(signalId)?.forEach((listener) => listener());
  }

  return {
    signals: {
      ...(formFactory ? { form: formFactory } : {}),
    },
    subscribeSignal(signal: { id: string }, listener: () => void) {
      let scopedListeners = signalListeners.get(signal.id);
      if (!scopedListeners) {
        scopedListeners = new Set();
        signalListeners.set(signal.id, scopedListeners);
      }
      scopedListeners.add(listener);
      return () => {
        scopedListeners!.delete(listener);
        if (scopedListeners!.size === 0) {
          signalListeners.delete(signal.id);
        }
      };
    },
    getSignalSnapshot(signal: { get(): unknown }) {
      const currentVersion = signalVersions.get(signal.id) ?? 0;
      const cachedSnapshot = signalSnapshots.get(signal.id);
      if (cachedSnapshot && cachedSnapshot.version === currentVersion) {
        return cachedSnapshot.value;
      }
      const nextValue = signal.get();
      signalSnapshots.set(signal.id, {
        version: currentVersion,
        value: nextValue,
      });
      return nextValue;
    },
    subscribeDiagnostics(listener: () => void) {
      diagnosticsListeners.add(listener);
      return () => {
        diagnosticsListeners.delete(listener);
      };
    },
    getDiagnosticsSnapshot() {
      return currentSnapshot;
    },
    transaction(callback: (tx: unknown) => void) {
      callback({});
    },
    batch(callback: (tx: unknown) => void) {
      callback({});
    },
    refreshDiagnostics() {
      return currentSnapshot;
    },
    performanceSummary() {
      return {
        activeSignalSubscriptionCount: signalListeners.size,
        activeReactSubscriberCount: Array.from(signalListeners.values()).reduce(
          (count, scopedListeners) => count + scopedListeners.size,
          0,
        ),
        activeRuntimeWatchHandleCount: 0,
        diagnosticsSubscriberCount: diagnosticsListeners.size,
        sharedFanoutRatio: 0,
      };
    },
    dispose() {},
    emit(signalId?: string) {
      version += 1;
      currentSnapshot = {
        latestObservation: null,
        latestFlow: null,
        performanceSummary: {
          version,
        },
      };
      if (signalId) {
        signalVersions.set(signalId, (signalVersions.get(signalId) ?? 0) + 1);
      }
      notifySignal(signalId);
      diagnosticsListeners.forEach((listener) => listener());
    },
  };
}

export function createFakeForm(store: ReturnType<typeof createFakeStore>) {
  const summarySignalId = "fake-form.summary";
  const sourceState = {
    title: "",
    published: false,
    role: "editor",
    appIds: [] as string[],
  };
  const state = {
    title: sourceState.title,
    published: sourceState.published,
    role: sourceState.role,
    appIds: [...sourceState.appIds] as string[],
    messages: [] as Array<{ target?: string; visibility?: string }>,
    pending: false,
    latestExecution: null as null | {
      action: string;
      resultKind: string;
    },
  };

  function isDirty(): boolean {
    return state.title.length > 0 || state.published || state.role !== "editor" || state.appIds.length > 0;
  }

  function emitSummary(): void {
    store.emit(summarySignalId);
  }

  return {
    summarySignal() {
      return {
        id: summarySignalId,
        get() {
          return {
            source: {
              title: sourceState.title,
              published: sourceState.published,
              role: sourceState.role,
              appIds: [...sourceState.appIds],
            },
            draft: {
              title: state.title,
              published: state.published,
              role: state.role,
              appIds: [...state.appIds],
            },
            effective: {
              title: state.title,
              published: state.published,
              role: state.role,
              appIds: [...state.appIds],
            },
            dirty: {
              isDirty: isDirty(),
            },
            patchPlan: {
              empty: !isDirty(),
            },
            readiness: {
              canSubmit: !state.pending && isDirty(),
              blockers: isDirty() ? [] : [{ kind: "unchanged" }],
            },
            visibleMessages: state.messages,
          };
        },
      };
    },
    source() {
      return { ...sourceState, appIds: [...sourceState.appIds] };
    },
    draft() {
      return {
        title: state.title,
        published: state.published,
        role: state.role,
        appIds: [...state.appIds],
      };
    },
    effective() {
      return {
        title: state.title,
        published: state.published,
        role: state.role,
        appIds: [...state.appIds],
      };
    },
    patchPlan() {
      return {
        empty: !isDirty(),
      };
    },
    bindInput(fieldId: string) {
      return {
        input(rawValue: string | string[]) {
          if (fieldId === "title") {
            state.title = String(rawValue);
          } else if (fieldId === "role") {
            state.role = String(rawValue);
          } else if (fieldId === "appIds") {
            state.appIds = Array.isArray(rawValue) ? rawValue : [String(rawValue)];
          }
          emitSummary();
        },
        focus() {},
        blur() {
          state.messages = [{ target: fieldId, visibility: "visible" }];
          emitSummary();
        },
        touch() {},
        visit() {},
        set(value: boolean | string | string[]) {
          if (fieldId === "published") {
            state.published = Boolean(value);
          }
          emitSummary();
        },
        clearDraft() {
          if (fieldId === "title") {
            state.title = "";
          } else if (fieldId === "role") {
            state.role = "editor";
          } else if (fieldId === "appIds") {
            state.appIds = [];
          } else if (fieldId === "published") {
            state.published = false;
          }
          emitSummary();
        },
      };
    },
    field(fieldId: string) {
      return {
        id: fieldId,
        path: fieldId,
        value() {
          if (fieldId === "title") {
            return state.title;
          }
          if (fieldId === "published") {
            return state.published;
          }
          if (fieldId === "role") {
            return state.role;
          }
          if (fieldId === "appIds") {
            return state.appIds;
          }
          return null;
        },
        dirty() {
          const currentValue = this.value();
          return {
            isDirty: Array.isArray(currentValue)
              ? currentValue.length > 0
              : typeof currentValue === "string"
                ? currentValue.length > 0
                : Boolean(currentValue),
          };
        },
        diagnostics() {
          return {
            field: fieldId,
          };
        },
      };
    },
    visibleMessages() {
      return state.messages;
    },
    interaction() {
      return {
        fields: [{ field: "title" }],
      };
    },
    fieldWritePosture(fieldId: string) {
      return {
        field: fieldId,
        canWrite: true,
      };
    },
    actionPlan() {
      return {
        status: "accepted",
        readiness: {
          canRun: !state.pending && isDirty(),
          blockers: isDirty() ? [] : [{ kind: "unchanged" }],
        },
      };
    },
    debugAction() {
      return {
        pending: state.pending,
        latestExecution: state.latestExecution,
      };
    },
    executeAction(actionId: string) {
      state.pending = true;
      state.latestExecution = {
        action: actionId,
        resultKind: "pending",
      };
      emitSummary();
      return state.latestExecution;
    },
    actions() {
      return {
        catalog: [{ id: "submit" }],
      };
    },
    dirty() {
      return {
        isDirty: isDirty(),
      };
    },
    readiness() {
      return {
        canSubmit: !state.pending && isDirty(),
        blockers: isDirty() ? [] : [{ kind: "unchanged" }],
        patchPlan: this.patchPlan(),
      };
    },
    reset() {
      state.title = sourceState.title;
      state.published = sourceState.published;
      state.role = sourceState.role;
      state.appIds = [...sourceState.appIds];
      state.messages = [];
      state.pending = false;
      state.latestExecution = null;
      emitSummary();
      return {
        kind: "reset",
      };
    },
  };
}
