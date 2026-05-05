import { createMaterializedFamily } from "./materialization/materialized_family_factory.js";
import { RESOURCE_FAMILY_KINDS } from "./family_kind.js";
import { validatePagedDeclaration } from "./declarations/paged_family_declaration.js";

function createPagedFamily(
  signalNamespace,
  familyId,
  declaration,
  compatibility,
) {
  return createMaterializedFamily(
    RESOURCE_FAMILY_KINDS.paged,
    signalNamespace,
    familyId,
    validatePagedDeclaration(declaration),
    compatibility,
  );
}

export { createPagedFamily };
