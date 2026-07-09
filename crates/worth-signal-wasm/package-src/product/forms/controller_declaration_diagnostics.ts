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
    evidence: 0,
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
      resourceLocus: field.resourceLocus === null
        ? null
        : field.resourceLocus.kind === "collectionItems"
          ? {
              kind: field.resourceLocus.kind,
              placement: field.resourceLocus.placement,
              posture: field.resourceLocus.posture,
            }
          : field.resourceLocus.kind === "field"
            ? {
                kind: field.resourceLocus.kind,
                field: field.resourceLocus.field,
                posture: field.resourceLocus.posture,
              }
            : field.resourceLocus.kind === "jsonPath"
              ? {
                  kind: field.resourceLocus.kind,
                  path: field.resourceLocus.path,
                  posture: field.resourceLocus.posture,
                }
              : field.resourceLocus.kind === "region"
                ? {
                  kind: field.resourceLocus.kind,
                  region: field.resourceLocus.region,
                  posture: field.resourceLocus.posture,
                }
                : field.resourceLocus.kind === "itemAspect"
                  ? {
                      kind: field.resourceLocus.kind,
                      itemId: field.resourceLocus.itemId,
                      aspect: field.resourceLocus.aspect,
                      posture: field.resourceLocus.posture,
                    }
                  : {
                      kind: field.resourceLocus.kind,
                      summary: field.resourceLocus.summary,
                      posture: field.resourceLocus.posture,
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
