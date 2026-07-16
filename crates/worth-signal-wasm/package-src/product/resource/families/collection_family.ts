import { validateCollectionDeclaration } from "./declarations/collection_family_declaration.js";
import { RESOURCE_FAMILY_KINDS } from "./family_kind.js";
import { createMaterializedFamily } from "./materialization/materialized_family_factory.js";

function createCollectionFamily(
  signalNamespace,
  resourceLineEpoch,
  familyId,
  declaration,
  compatibility,
  effectProjectionCoordinator,
) {
  return createMaterializedFamily(
    RESOURCE_FAMILY_KINDS.collection,
    signalNamespace,
    resourceLineEpoch,
    familyId,
    validateCollectionDeclaration(declaration),
    compatibility,
    effectProjectionCoordinator,
  );
}

export { createCollectionFamily };
