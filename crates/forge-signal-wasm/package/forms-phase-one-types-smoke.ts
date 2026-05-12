import { createSignals } from "./index.js";

const signals = createSignals();
const objectState = signals.input({
  title: "Ship docs",
  auditItems: [] as Array<{ id: string; label: string }>,
  evidence: { digest: "file-1", name: "audit.pdf" },
});
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
});

const phaseOneSourceKind = phaseOneForm.sourceAuthority().kind;
const phaseOneDeclarationId = phaseOneForm.declaration().formId;
const phaseOneFieldFamily = phaseOneForm.fieldContract()[1]?.family;
const phaseOneAdapterTier = phaseOneForm.inputAdapters()[0]?.tier;
const phaseOneSourceDigest = phaseOneForm.verification().digests.sourceAuthorityDigest;
const phaseOneFieldContractDigest = phaseOneForm.verification().digests.fieldContractDigest;
const phaseOneAdapterDigest = phaseOneForm.verification().digests.inputAdapterCapabilityDigest;
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
// @ts-expect-error repeated fields require explicit stable item identity
signals.form({ source: {}, fields: ({ repeated }) => ({ items: repeated("items") }) });
// @ts-expect-error repeated item identity must be a field name or resolver
signals.form({ source: {}, fields: ({ repeated }) => ({ items: repeated("items", { itemIdentity: 1 }) }) });
// @ts-expect-error attachment fields require explicit attachment identity
signals.form({ source: {}, fields: ({ attachment }) => ({ evidence: attachment("evidence") }) });
// @ts-expect-error attachment identity must be a field name or resolver
signals.form({ source: {}, fields: ({ attachment }) => ({ evidence: attachment("evidence", { attachmentIdentity: 1 }) }) });

void phaseOneForm;
void phaseOneSourceKind;
void phaseOneDeclarationId;
void phaseOneFieldFamily;
void phaseOneAdapterTier;
void phaseOneSourceDigest;
void phaseOneFieldContractDigest;
void phaseOneAdapterDigest;
void phaseOneCollectionIdentity;
void phaseOneAttachmentDigest;
