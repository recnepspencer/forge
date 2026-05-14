import { FormDeclarationError } from "./form_errors.js";

export function materializeFormDeclarationRecord(declaration, sourceAuthority) {
  if (declaration.id !== undefined && typeof declaration.id !== "string") {
    throw new FormDeclarationError("form declaration id must be a string when provided");
  }
  if (declaration.contract !== undefined && typeof declaration.contract !== "string") {
    throw new FormDeclarationError("form declaration contract must be a string when provided");
  }
  const formId = declaration.id ?? `form:${sourceAuthority.kind}:${sourceAuthority.sourceId}`;
  return Object.freeze({
    formId,
    contract: declaration.contract ?? "phase1-form-declaration-v1",
  });
}

export function readFormDeclarationDiagnostics(formDeclaration, sourceAuthority, fieldDeclarations) {
  const families = {
    scalar: 0,
    repeated: 0,
    attachment: 0,
  };
  for (const field of fieldDeclarations) {
    families[field.family] += 1;
  }
  return Object.freeze({
    formId: formDeclaration.formId,
    contract: formDeclaration.contract,
    source: Object.freeze({
      kind: sourceAuthority.kind,
      sourceId: sourceAuthority.sourceId,
    }),
    fieldFamilies: Object.freeze(families),
    fieldCount: fieldDeclarations.length,
  });
}

export function readFieldContractDiagnostics(fieldDeclarations) {
  return Object.freeze(
    fieldDeclarations.map((field) => ({
      id: field.id,
      name: field.name,
      family: field.family,
      path: field.path,
      collectionIdentity: field.collectionIdentity === null
        ? null
        : {
            kind: field.collectionIdentity.kind,
            field: field.collectionIdentity.field,
            posture: field.collectionIdentity.posture,
          },
      attachment: field.attachment === null
        ? null
        : {
            identityKind: field.attachment.identityKind,
            identityField: field.attachment.identityField,
            metadata: field.attachment.metadata,
            posture: field.attachment.posture,
          },
    })),
  );
}

export function readInputAdapterDiagnostics(fieldDeclarations) {
  return Object.freeze(
    fieldDeclarations.map((field) => ({
      field: field.id,
      path: field.path,
      family: field.family,
      tier: field.inputAdapter.tier,
      capabilities: field.inputAdapter.capabilities,
      unavailable: field.inputAdapter.unavailable,
    })),
  );
}
