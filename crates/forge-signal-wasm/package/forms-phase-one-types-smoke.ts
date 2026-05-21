import { createSignals } from "./index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });
const objectState = signals.input({
  title: "Ship docs",
  auditItems: [] as Array<{ id: string; label: string }>,
  evidence: { digest: "file-1", name: "audit.pdf" },
});
const publicTaskInput = signals.publicInput(objectState, { authority: "readOnly" });
const phaseOneForm = signals.form({
  id: "phase-one-type-smoke",
  source: signals.form.source.graphPublicInput(publicTaskInput, { id: "task-public-input" }),
  fields: ({ field, repeated, evidence }) => ({
    title: field<string>("title"),
    auditItems: repeated<Array<{ id: string; label: string }>>("auditItems", {
      itemIdentity: "id",
      resourceLocus: { kind: "collectionItems", placement: "append" },
    }),
    evidence: evidence<{ digest: string; name: string }>("evidence", {
      attachmentIdentity: "digest",
      metadata: { required: true },
      resourceLocus: { kind: "region", region: "evidenceRegion" },
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
const phaseOneSourceKind = phaseOneForm.sourceAuthority().kind;
const phaseOneBootstrapSourceKind = phaseOneSourceBootstrapForm.sourceAuthority().kind;
const phaseOneSourceAdmission = phaseOneSourceBootstrapForm.sourceAdmission()?.status;
const phaseOneDraftRestore = phaseOneSourceBootstrapForm.draftRestore()?.status;
const phaseOneDeclarationId = phaseOneForm.declaration().formId;
const phaseOneFieldFamily = phaseOneForm.fieldContract()[1]?.family;
const phaseOneFieldResourceLocus = phaseOneForm.fieldContract()[1]?.resourceLocus?.kind;
const phaseOneAttachmentResourceLocus = phaseOneForm.fieldContract()[2]?.resourceLocus?.kind;
const phaseOneAdapterTier = phaseOneForm.inputAdapters()[0]?.tier;
const phaseOneSourceDigest = phaseOneForm.verification().digests.sourceAuthorityDigest;
const phaseOneDiagnosticsSummaryDigest = phaseOneForm.diagnosticsSummary().digest;
const phaseOneDiagnosticsHistoryLength = phaseOneForm.diagnosticsHistory().length;
const phaseOneStateHistoryLength = phaseOneForm.stateHistory().length;
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
const phaseOneCollaborationNumericBranch = phaseOneForm.reportCollaboration({
  branchId: 7,
  reason: "numeric branch ids stay admitted at the public collaboration boundary",
});
// @ts-expect-error collaboration branch ids must remain string-or-number identity, not booleans
phaseOneForm.reportCollaboration({ branchId: true });
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
const phaseOneAttachmentIdentity =
  phaseOneForm.fields.evidence.attachmentIdentity({ digest: "file-1", name: "audit.pdf" });
if (phaseOneAttachmentIdentity === null) {
  throw new Error("attachment identity should be present for explicit attachment values");
}
const phaseOneAttachmentDigest = phaseOneAttachmentIdentity.attachmentDigest;
// @ts-expect-error evidence field handles cannot perform repeated-field operations
phaseOneForm.fields.evidence.addItem({ id: "bad" });
// @ts-expect-error graph public input source requires signals.publicInput(...) output
signals.form.source.graphPublicInput(objectState);
// @ts-expect-error signal source authority requires a product signal handle
signals.form.source.signal(() => ({ title: "not a signal handle" }));
// @ts-expect-error repeated fields require explicit stable item identity
signals.form({ source: {}, fields: ({ repeated }) => ({ items: repeated("items") }) });
// @ts-expect-error repeated item identity must be a field name or resolver
signals.form({ source: {}, fields: ({ repeated }) => ({ items: repeated("items", { itemIdentity: 1 }) }) });
// @ts-expect-error repeated resource locus kind must be supported
signals.form({ source: {}, fields: ({ repeated }) => ({ items: repeated("items", { itemIdentity: "id", resourceLocus: { kind: "rows" } }) }) });
// @ts-expect-error scalar resource locus field must be a non-empty string
signals.form({ source: {}, fields: ({ field }) => ({ title: field("title", { resourceLocus: { kind: "field", field: 1 } }) }) });
// @ts-expect-error evidence fields require explicit attachment identity
signals.form({ source: {}, fields: ({ evidence }) => ({ evidence: evidence("evidence") }) });
// @ts-expect-error evidence identity must be a field name or resolver
signals.form({ source: {}, fields: ({ evidence }) => ({ evidence: evidence("evidence", { attachmentIdentity: 1 }) }) });
// @ts-expect-error evidence resource locus region must be a string
signals.form({ source: {}, fields: ({ evidence }) => ({ evidence: evidence("evidence", { attachmentIdentity: "digest", resourceLocus: { kind: "region", region: 1 } }) }) });

void phaseOneForm;
void phaseOneSourceBootstrapForm;
void phaseOneSourceKind;
void phaseOneBootstrapSourceKind;
void phaseOneSourceAdmission;
void phaseOneDraftRestore;
void phaseOneDeclarationId;
void phaseOneFieldFamily;
void phaseOneFieldResourceLocus;
void phaseOneAttachmentResourceLocus;
void phaseOneAdapterTier;
void phaseOneSourceDigest;
void phaseOneDiagnosticsSummaryDigest;
void phaseOneDiagnosticsHistoryLength;
void phaseOneStateHistoryLength;
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
