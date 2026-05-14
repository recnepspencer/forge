import { createSignals } from "./index.js";

const signals = createSignals();
const objectState = signals.input({
  title: "Ship docs",
  auditItems: [] as Array<{ id: string; label: string }>,
  evidence: { digest: "file-1", name: "audit.pdf" },
});
const typedResourceLine = null as unknown as import("./types/resource/resource_lifecycle.js").ResourceLine<
  { id: string },
  { title: string }
>;
const publicTaskInput = signals.publicInput(objectState, { authority: "readOnly" });
const phaseOneForm = signals.form({
  id: "phase-one-type-smoke",
  source: signals.form.source.graphPublicInput(publicTaskInput, { id: "task-public-input" }),
  fields: ({ field, repeated, attachment }) => ({
    title: field<string>("title"),
    auditItems: repeated<Array<{ id: string; label: string }>>("auditItems", {
      itemIdentity: "id",
    }),
    evidence: attachment<{ digest: string; name: string }>("evidence", {
      attachmentIdentity: "digest",
      metadata: { required: true },
    }),
  }),
  presentation: {
    entry: {
      bootstrap: {
        sourceCompatibility: true,
        validation: true,
        readiness: true,
      },
    },
    action: {
      settleOn: ["messages", "layout"],
    },
  },
});

const phaseOneSourceBootstrapForm = signals.form({
  source: {
    value: signals.form.source.graphPublicInput(publicTaskInput, { id: "bootstrap-public-input" }),
    sourceAdmission: {
      status: "ready",
      reason: "descriptor source is admitted",
    },
    draftRestore: {
      status: "ready",
      reason: "descriptor draft restore is settled",
    },
  },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  presentation: {
    entry: {
      bootstrap: {
        sourceAdmission: true,
        draftRestore: true,
      },
    },
  },
});
const phaseEightResourceForm = signals.form({
  source: signals.form.source.resourceLine(typedResourceLine, { id: "task-resource" }),
  fields: ({ field }) => ({
    title: field("title"),
  }),
});
const phaseEightResourceActionForm = signals.form({
  source: signals.form.source.resourceLine(typedResourceLine, { id: "task-resource-action" }),
  fields: ({ field }) => ({
    title: field("title"),
  }),
  actions: ({ submit }) => ({
    submit: submit({
      resourceEffectProfile: signals.resource.effects.branchNative(),
    }),
  }),
});

const phaseOneSourceKind = phaseOneForm.sourceAuthority().kind;
const phaseOneBootstrapSourceKind = phaseOneSourceBootstrapForm.sourceAuthority().kind;
const phaseEightResourceSourceKind = phaseEightResourceForm.resourceSource()?.sourceKind;
const phaseEightResourceSourceDigest = phaseEightResourceForm.verification().digests.resourceSourceDigest;
const phaseEightResourceEffectProfileName = phaseEightResourceForm.resourceSource()?.effectProfile.profile?.name;
const phaseEightResourceVisibleSelectionKind = phaseEightResourceForm.resourceSource()?.visibleSelection.kind;
const phaseEightResourceVerificationPackageDigest = phaseEightResourceForm.resourceSource()?.verification.packageDigest;
const phaseEightResourceEffectCloseoutDigest =
  phaseEightResourceForm.verification().digests.resourceEffectCloseoutMatrixDigest;
const phaseEightResourceVisibleBranchDigest =
  phaseEightResourceForm.verification().digests.resourceVisibleBranchSelectionDigest;
const phaseEightResourceSubmitEffectProfileSource =
  phaseEightResourceActionForm.actionPlan("submit").resourceEffectProfile.source;
const phaseEightResourceReset = phaseEightResourceForm.reset();
const phaseEightResourceRollback = phaseEightResourceForm.rollbackLastResourceEffect();
const phaseEightResourceResetHistory = phaseEightResourceForm.resetHistory();
const phaseOneSourceAdmission = phaseOneSourceBootstrapForm.sourceAdmission()?.status;
const phaseOneDraftRestore = phaseOneSourceBootstrapForm.draftRestore()?.status;
const phaseOneDeclarationId = phaseOneForm.declaration().formId;
const phaseOneFieldFamily = phaseOneForm.fieldContract()[1]?.family;
const phaseOneAdapterTier = phaseOneForm.inputAdapters()[0]?.tier;
const phaseOneSourceDigest = phaseOneForm.verification().digests.sourceAuthorityDigest;
const phaseOneFieldContractDigest = phaseOneForm.verification().digests.fieldContractDigest;
const phaseOneAdapterDigest = phaseOneForm.verification().digests.inputAdapterCapabilityDigest;
const phaseOneAttachmentReport = phaseOneForm.reportAttachments({
  status: "busy",
  reason: "uploading evidence",
  section: "evidence",
});
const phaseOneMediaReport = phaseOneForm.reportMedia({
  status: "busy",
  reason: "cropping evidence",
  surfaceId: "cropper-modal",
});
const phaseOneHandoffReport = phaseOneForm.reportHandoff({
  status: "pending",
  reason: "opening external handoff",
  scopeKind: "modal",
  surfaceId: "share-modal",
});
const phaseOneExitReport = phaseOneForm.reportExit({
  status: "pending",
  reason: "confirming route exit",
  scopeKind: "route",
  surfaceId: "browser-history",
});
const phaseOneMessageReport = phaseOneForm.reportMessages({
  status: "settling",
  reason: "save toast visible",
  channel: "toast",
  scope: "wholeForm",
});
const phaseOneFieldMessageReport = phaseOneForm.reportMessages({
  status: "busy",
  reason: "title message visible",
  scope: "field",
  target: "title",
});
const phaseOneAttachmentLane = phaseOneForm.reportPresentationLane("attachments", {
  status: "busy",
  reason: "uploading evidence",
  section: "evidence",
});
const phaseOneMediaLane = phaseOneForm.reportPresentationLane("media", {
  status: "busy",
  reason: "opening cropper",
  surfaceId: "cropper-modal",
});
const phaseOneHandoffLane = phaseOneForm.reportPresentationLane("handoff", {
  status: "pending",
  reason: "opening share handoff",
  scopeKind: "modal",
  surfaceId: "share-modal",
});
const phaseOneExitLane = phaseOneForm.reportPresentationLane("exit", {
  status: "pending",
  reason: "confirming route exit",
  scopeKind: "route",
  surfaceId: "browser-history",
});
// @ts-expect-error attachment scope requires an explicit section
phaseOneForm.reportAttachments({ status: "busy", reason: "uploading evidence" });
// @ts-expect-error media scope requires an explicit surface id
phaseOneForm.reportMedia({ status: "busy", reason: "opening cropper" });
// @ts-expect-error handoff scope requires an explicit scope kind and surface id
phaseOneForm.reportHandoff({ status: "pending", reason: "opening share handoff" });
// @ts-expect-error exit scope requires an explicit scope kind and surface id
phaseOneForm.reportExit({ status: "pending", reason: "confirming route exit" });
// @ts-expect-error generic attachment lane updates require an explicit section
phaseOneForm.reportPresentationLane("attachments", { status: "busy", reason: "uploading evidence" });
// @ts-expect-error generic media lane updates require an explicit surface id
phaseOneForm.reportPresentationLane("media", { status: "busy", reason: "opening cropper" });
// @ts-expect-error generic handoff lane updates require explicit scope metadata
phaseOneForm.reportPresentationLane("handoff", { status: "pending", reason: "opening share handoff" });
// @ts-expect-error generic exit lane updates require explicit scope metadata
phaseOneForm.reportPresentationLane("exit", { status: "pending", reason: "confirming route exit" });
// @ts-expect-error message channel must be supported
phaseOneForm.reportMessages({ status: "busy", reason: "bad", channel: "snackbar" });
// @ts-expect-error scoped message updates require an explicit target
phaseOneForm.reportMessages({ status: "busy", reason: "missing target", scope: "field" });
signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  presentation: {
    messages: {
      // @ts-expect-error settleOn is only admitted on the action presentation lane
      settleOn: ["layout"],
    },
  },
});
signals.form({
  source: {
    value: { title: "Ship docs" },
    // @ts-expect-error sourceAdmission requires a structured bootstrap artifact
    sourceAdmission: "pending",
  },
  fields: ({ field }) => ({
    title: field("title"),
  }),
});
signals.form({
  source: {
    value: { title: "Ship docs" },
    // @ts-expect-error draftRestore status must be supported
    draftRestore: { status: "failed", reason: "bad status" },
  },
  fields: ({ field }) => ({
    title: field("title"),
  }),
});
signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  presentation: {
    entry: {
      bootstrap: {
        // @ts-expect-error bootstrap validation flags must be booleans
        validation: "yes",
      },
    },
  },
});
signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  presentation: {
    layout: {
      // @ts-expect-error bootstrap is only admitted on the entry presentation lane
      bootstrap: {
        layoutMeasurement: true,
      },
    },
  },
});
// @ts-expect-error scalar field handles cannot perform repeated-field operations
phaseOneForm.fields.title.addItem({ id: "bad" });
// @ts-expect-error repeated field handles cannot perform attachment identity operations
phaseOneForm.fields.auditItems.attachmentIdentity();
phaseOneForm.fields.auditItems.addItem({ id: "audit-1", label: "Audit" });
const phaseOneCollectionIdentity = phaseOneForm.fields.auditItems.collectionIdentity().items[0]?.itemId;
const phaseOneAttachmentDigest =
  phaseOneForm.fields.evidence.attachmentIdentity({ digest: "file-1", name: "audit.pdf" }).attachmentDigest;
// @ts-expect-error attachment field handles cannot perform repeated-field operations
phaseOneForm.fields.evidence.addItem({ id: "bad" });
// @ts-expect-error graph public input source requires signals.publicInput(...) output
signals.form.source.graphPublicInput(objectState);
// @ts-expect-error signal source authority requires a product signal handle
signals.form.source.signal(() => ({ title: "not a signal handle" }));
signals.form({
  source: signals.form.source.resourceLine(typedResourceLine, { id: "typed-resource-profile" }),
  fields: ({ field }) => ({
    title: field("title"),
  }),
  actions: ({ submit }) => ({
    submit: submit({
      resourceEffectProfile: signals.resource.effects.branchNative(),
    }),
  }),
});
signals.form({
  source: { title: "Plain source" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  actions: ({ submit }) => ({
    submit: submit({
      // @ts-expect-error resourceEffectProfile must be a validated resource effect profile
      resourceEffectProfile: { name: "forged-profile" },
    }),
  }),
});
// @ts-expect-error repeated fields require explicit stable item identity
signals.form({ source: {}, fields: ({ repeated }) => ({ items: repeated("items") }) });
// @ts-expect-error repeated item identity must be a field name or resolver
signals.form({ source: {}, fields: ({ repeated }) => ({ items: repeated("items", { itemIdentity: 1 }) }) });
// @ts-expect-error attachment fields require explicit attachment identity
signals.form({ source: {}, fields: ({ attachment }) => ({ evidence: attachment("evidence") }) });
// @ts-expect-error attachment identity must be a field name or resolver
signals.form({ source: {}, fields: ({ attachment }) => ({ evidence: attachment("evidence", { attachmentIdentity: 1 }) }) });

void phaseOneForm;
void phaseOneSourceBootstrapForm;
void phaseEightResourceForm;
void phaseEightResourceActionForm;
void phaseOneSourceKind;
void phaseOneBootstrapSourceKind;
void phaseEightResourceSourceKind;
void phaseEightResourceSourceDigest;
void phaseEightResourceEffectProfileName;
void phaseEightResourceVisibleSelectionKind;
void phaseEightResourceVerificationPackageDigest;
void phaseEightResourceEffectCloseoutDigest;
void phaseEightResourceVisibleBranchDigest;
void phaseEightResourceSubmitEffectProfileSource;
void phaseEightResourceReset;
void phaseEightResourceRollback;
void phaseEightResourceResetHistory;
void phaseOneSourceAdmission;
void phaseOneDraftRestore;
void phaseOneDeclarationId;
void phaseOneFieldFamily;
void phaseOneAdapterTier;
void phaseOneSourceDigest;
void phaseOneFieldContractDigest;
void phaseOneAdapterDigest;
void phaseOneAttachmentReport;
void phaseOneMediaReport;
void phaseOneHandoffReport;
void phaseOneExitReport;
void phaseOneMessageReport;
void phaseOneFieldMessageReport;
void phaseOneAttachmentLane;
void phaseOneMediaLane;
void phaseOneHandoffLane;
void phaseOneExitLane;
void phaseOneCollectionIdentity;
void phaseOneAttachmentDigest;
