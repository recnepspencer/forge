import { describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";

async function flushMicrotasks() {
  await Promise.resolve();
  await Promise.resolve();
}

function createBindableForm(signals: Awaited<ReturnType<typeof createSignals>>, options?: {
  dirty?: boolean;
  currentStepId?: string | null;
  routeBlockedReason?: string | null;
  collaborationPosture?: "notDeclared" | "active" | "blocked";
  collaborationMode?: "notDeclared" | "singleWriterLock" | "branchPerActor";
  readOnly?: boolean;
  executeResultKind?: "fulfilled" | "blocked" | "rejected";
}) {
  const summary = signals.input(0);
  const stepArtifacts = options?.routeBlockedReason
    ? [{ id: "review", routeCoupled: true, posture: "blocked", reason: options.routeBlockedReason }]
    : [];
  return {
    summarySignal: () => summary,
    dirty: () => Boolean(options?.dirty),
    visibleMessages: () => (options?.dirty ? [{ code: "form.unsaved", target: "title" }] : []),
    steps: () => ({ artifacts: stepArtifacts }),
    navigation: () => ({ currentStepId: options?.currentStepId ?? null }),
    actionPlan: () => ({ readiness: { canRun: true, blockers: [] } }),
    executeAction: async () => ({
      resultKind: options?.executeResultKind ?? "fulfilled",
      reason: options?.executeResultKind === "blocked" ? "form action is blocked" : null,
      error: options?.executeResultKind === "rejected" ? new Error("form action rejected") : null,
    }),
    collaboration: () => ({
      declared: options?.collaborationPosture !== undefined && options.collaborationPosture !== "notDeclared",
      mode: options?.collaborationMode ?? (options?.collaborationPosture === "notDeclared" || options?.collaborationPosture === undefined ? "notDeclared" : "singleWriterLock"),
      actorId: "maya",
      posture: options?.collaborationPosture ?? "notDeclared",
      reason: options?.readOnly ? "review-only session" : "collaboration clear",
      lockOwnerId: options?.readOnly ? "maya" : null,
      leasedFields: [],
      branchId: null,
      readOnly: Boolean(options?.readOnly),
      remoteUpdateDigest: null,
      presence: [],
      comments: [],
      resourceProof: {
        required: false,
        admitted: false,
        sourceKind: null,
        visibleSelectionKind: null,
        branchId: null,
        reason: null,
        digest: "none",
      },
      history: [],
      events: [],
      eventsDigest: "none",
      counters: {
        costBasis: "derivedCollaborationPostureScan",
        incrementalStatus: "notIncremental",
        blockingFields: 0,
        presenceActors: 0,
        commentArtifacts: 0,
        historyArtifacts: 0,
        eventArtifacts: 0,
        postureChanges: 0,
        lockChanges: 0,
        leaseChanges: 0,
        branchChanges: 0,
        presenceChanges: 0,
        commentChanges: 0,
        blocked: 0,
        settling: 0,
        unavailable: 0,
        resourceProofRequired: 0,
        resourceProofUnavailable: 0,
      },
      digest: "none",
    }),
    readiness: () => ({ blockers: [] }),
    reset: () => ({ ok: true }),
  };
}

function createMutableBindableForm(signals: Awaited<ReturnType<typeof createSignals>>) {
  const form = signals.form({
    source: {
      title: "",
      status: "draft",
    },
    fields: ({ field }) => ({
      title: field("title"),
      status: field("status"),
    }),
  });
  return {
    ...form,
    async setDirty(next: boolean) {
      if (next) {
        await form.fields.title.set("changed");
        return;
      }
      form.fields.title.clearDraft();
    },
  };
}

describe("local namespace authoring", () => {
  it("creates a rich dialog controller without raw scope plumbing", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const dialog = signals.local.dialogState({
        identity: "invite-user-dialog",
        modes: ["create", "edit", "delete"] as const,
        initial: {
          isOpen: true,
          mode: "create" as const,
          data: { id: "p-1", title: "Northstar Jacket" },
          context: { source: "catalog" as const },
          loading: false,
        },
        actions: ({ custom }) => ({
          saveDraft: custom({
            writes: true,
            execute() {
              return { accepted: true };
            },
          }),
        }),
      });

      expect(dialog.scopeId).toBe("invite-user-dialog");
      expect(dialog.isOpen.signalIdentity?.().localId).toBe("isOpen");
      expect(dialog.mode()).toBe("create");
      expect(dialog.data().title).toBe("Northstar Jacket");
      expect(dialog.context().source).toBe("catalog");
      expect(dialog.dirty()).toBe(false);

      await dialog.patch({
        data: { id: "p-1", title: "Northstar Jacket v2" },
      });

      expect(dialog.data().title).toBe("Northstar Jacket v2");
      expect(dialog.dirty()).toBe(true);
      expect(dialog.patchPlan().changed).toBe(true);
      expect(dialog.actions().saveDraft.disabled).toBe(false);
      expect(typeof dialog.summarySignal().get).toBe("function");

      await dialog.open("edit", {
        data: { id: "p-2", title: "Orbit Parka" },
        context: { source: "sales" },
      });

      expect(dialog.mode()).toBe("edit");
      expect(dialog.data().id).toBe("p-2");
      expect(dialog.context().source).toBe("sales");
      expect(dialog.source().mode).toBe("edit");
      expect(dialog.dirty()).toBe(false);

      await dialog.action("saveDraft").execute();
      expect(dialog.actionHistory().at(-1)?.actionId).toBe("saveDraft");
      expect(dialog.actionHistory().at(-1)?.resultKind).toBe("fulfilled");
    } finally {
      signals.free();
    }
  });

  it("supports collaboration reports and composes bound form blockers into close readiness", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const dialog = signals.local.dialogState({
        identity: "delete-product-dialog",
        initial: {
          isOpen: true,
          mode: "delete" as const,
        },
        collaboration: {
          mode: "singleWriterLock",
          actorId: "alex",
        },
      });

      dialog.bindForm(createBindableForm(signals, {
        dirty: true,
        currentStepId: "review",
        routeBlockedReason: "route review step is still unavailable",
      }), {
        closeOnSuccess: true,
      });

      const closeAttempt = await dialog.requestClose({ reason: "escape" });
      expect(closeAttempt.status).toBe("blocked");
      expect(closeAttempt.blockers.some((blocker) => blocker.kind === "dialog:dirty")).toBe(true);
      expect(closeAttempt.blockers.some((blocker) => blocker.kind === "dialog:step")).toBe(true);

      const nativeCollaboration = dialog.reportCollaboration({
        posture: "blocked",
        reason: "alex currently owns the modal lock",
        lockOwnerId: "alex",
        readOnly: true,
      });

      expect(nativeCollaboration.posture).toBe("blocked");
      expect(dialog.collaboration().lockOwnerId).toBe("alex");
      expect(dialog.collaboration().readOnly).toBe(true);
      expect(dialog.visibleMessages().some((message) => message.code === "dialog.collaboration.posture")).toBe(true);
    } finally {
      signals.free();
    }
  });

  it("reflects bound form state on demand and preserves non-fulfilled confirm truth", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const dialog = signals.local.dialogState({
        identity: "edit-product-dialog",
        initial: {
          isOpen: true,
          mode: "edit" as const,
        },
      });
      const mutableForm = createMutableBindableForm(signals);
      dialog.bindForm(mutableForm, {
        closeOnSuccess: true,
      });
      await mutableForm.setDirty(true);
      await flushMicrotasks();
      expect(dialog.summarySignal().get().readiness.blockers.some((blocker) => blocker.kind === "dialog:dirty")).toBe(true);
      expect((await dialog.requestClose({ reason: "escape" })).status).toBe("blocked");

      await mutableForm.setDirty(false);
      await flushMicrotasks();
      expect(dialog.summarySignal().get().readiness.blockers.some((blocker) => blocker.kind === "dialog:dirty")).toBe(false);
      expect((await dialog.requestClose({ reason: "escape", clear: true })).status).toBe("accepted");

      const rejectingDialog = signals.local.dialogState({
        identity: "review-product-dialog",
        initial: {
          isOpen: true,
          mode: "edit" as const,
        },
      });
      rejectingDialog.bindForm(createBindableForm(signals, {
        executeResultKind: "blocked",
      }), {
        confirmActionId: "submit",
        closeOnSuccess: true,
      });

      const execution = await rejectingDialog.action("confirm").execute();
      expect(execution.resultKind).toBe("blocked");
      expect(execution.delegatedResultKind).toBe("blocked");
      expect(rejectingDialog.isOpen()).toBe(true);
    } finally {
      signals.free();
    }
  });

  it("surfaces collaboration mode conflicts explicitly", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const dialog = signals.local.dialogState({
        identity: "approve-product-dialog",
        initial: {
          isOpen: true,
          mode: "edit" as const,
        },
        collaboration: {
          mode: "singleWriterLock",
          actorId: "alex",
        },
      });

      dialog.bindForm(createBindableForm(signals, {
        collaborationPosture: "active",
        collaborationMode: "singleWriterLock",
      }), {});

      expect(dialog.collaboration().conflicts).toHaveLength(0);

      dialog.bindForm(createBindableForm(signals, {
        collaborationPosture: "active",
        collaborationMode: "branchPerActor",
      }), {});

      expect(dialog.collaboration().conflicts).toHaveLength(1);
      expect(dialog.visibleMessages().some((message) => message.code === "dialog.collaboration.conflict")).toBe(true);
      expect(dialog.action("confirm").plan.readiness.blockers.some((blocker) => blocker.kind === "collaboration:modeConflict")).toBe(true);
    } finally {
      signals.free();
    }
  });

  it("keeps local helpers available on scoped namespaces", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const scoped = signals.scope("admin");
      const dialog = scoped.local.dialogState({
        identity: "delete-product-dialog",
        initial: {
          isOpen: true,
          mode: "delete" as const,
        },
      });

      expect(dialog.scopeId).toBe("admin.delete-product-dialog");
      expect(dialog.isOpen.signalIdentity?.().canonicalId).toBe(
        "admin.delete-product-dialog.isOpen",
      );
      expect(dialog.isOpen()).toBe(true);
    } finally {
      signals.free();
    }
  });
});
