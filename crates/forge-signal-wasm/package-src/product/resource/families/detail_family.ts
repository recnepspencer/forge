import { createMaterializedFamily } from "./materialization/materialized_family_factory.js";
import { RESOURCE_FAMILY_KINDS } from "./family_kind.js";
import { validateDetailDeclaration } from "./declarations/detail_family_declaration.js";

function createDetailFamily(signalNamespace, familyId, declaration) {
  return createMaterializedFamily(
    RESOURCE_FAMILY_KINDS.detail,
    signalNamespace,
    familyId,
    validateDetailDeclaration(declaration),
  );
}

export { createDetailFamily };
