import { createSignals } from "./index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

type DialogMode = "create" | "edit" | "delete";
type ProductRecord = {
  readonly id: string;
  readonly title: string;
};
type DialogContext = {
  readonly source: "catalog" | "sales";
};

const dialog = signals.local.dialogState({
  identity: "invite-user-dialog",
  modes: ["create", "edit", "delete"] as const,
  initial: {
    isOpen: true,
    mode: "create" as DialogMode,
    data: { id: "p-1", title: "Northstar Jacket" } as ProductRecord,
    context: { source: "catalog" } as DialogContext,
    loading: false,
  },
  collaboration: {
    mode: "singleWriterLock",
    actorId: "alex",
  },
  actions: ({ custom }) => ({
    saveDraft: custom({
      writes: true,
      closeOnSuccess: false,
      execute() {
        return { accepted: true };
      },
    }),
  }),
});

const fakeForm = {
  summarySignal: () => signals.computed(() => "summary"),
  dirty: () => false,
  visibleMessages: () => [],
  steps: () => ({ artifacts: [] }),
  navigation: () => ({ currentStepId: null }),
  actionPlan: () => ({ readiness: { canRun: true, blockers: [] } }),
  executeAction: () => ({ ok: true }),
  collaboration: () => ({
    declared: false,
    mode: "notDeclared",
    posture: "notDeclared",
    reason: "not declared",
    lockOwnerId: null,
    leasedFields: [],
    branchId: null,
    readOnly: false,
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
    actorId: null,
  }),
  readiness: () => ({ blockers: [] }),
  reset: () => ({ ok: true }),
};

dialog.bindForm(fakeForm, {
  confirmActionId: "submit",
  closeOnSuccess: true,
});

const draft = dialog.draft();
const source = dialog.source();
const diagnostics = dialog.diagnostics();
const summarySignal = dialog.summarySignal();
const action = dialog.action("saveDraft");
const actions = dialog.actions();
const collaboration = dialog.collaboration();
const modeSignal = dialog.mode;
const dataSignal = dialog.data;
const contextSignal = dialog.context;
const loadingSignal = dialog.loading;
const draftMode: DialogMode | null = draft.mode;
const sourceContext: DialogContext | null = source.context;
const collaborationDigest: string = collaboration.digest;
const resultKind: string | null = action.resultKind;
const saveDraftAction = actions.saveDraft;

dialog.open("edit", {
  data: { id: "p-2", title: "Orbit Parka" },
  context: { source: "sales" },
});
dialog.patch({
  loading: true,
});
dialog.setLoading(false);
dialog.reportCollaboration({
  posture: "blocked",
  reason: "alex owns the modal lock",
  lockOwnerId: "alex",
});
await dialog.requestClose({ reason: "escape" });

void draftMode;
void sourceContext;
void diagnostics;
void summarySignal;
void collaborationDigest;
void resultKind;
void saveDraftAction;
void modeSignal;
void dataSignal;
void contextSignal;
void loadingSignal;

const list = signals.local.listState<string>({
  identity: "candidate-users",
  initial: ["a", "b"],
});
const formSource = signals.local.formSource({
  identity: "invite-user-form",
  initial: {
    email: "",
  },
});
const scopedDialog = signals.scope("admin").local.dialogState({
  identity: "delete-product-dialog",
  initial: {
    isOpen: false,
    mode: "delete" as DialogMode,
  },
});

const listItems = list.items;
const formSignal = formSource.signal;
const formDeclaration = formSource.source;
const scopedDialogScopeId: string = scopedDialog.scopeId;

list.reset();
formSource.reset();

void listItems;
void formSignal;
void formDeclaration;
void scopedDialogScopeId;

await signals.terminate();
