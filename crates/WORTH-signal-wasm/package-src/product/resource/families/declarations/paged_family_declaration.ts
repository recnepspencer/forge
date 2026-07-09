import { requireResourceDeclarationBase } from "./family_declaration_base.js";
import { requireResourceCollectionShape } from "../../reconciliation/resource_collection_shape.js";

function validatePagedDeclaration(declaration) {
  const paged = requireResourceDeclarationBase("paged", declaration);
  if (typeof paged.itemIdentity !== "function") {
    throw new TypeError("paged resources require itemIdentity(...)");
  }
  if (typeof paged.accumulatePage !== "function") {
    throw new TypeError("paged resources require accumulatePage(...)");
  }
  if (paged.reconcile !== undefined) {
    requireResourceCollectionShape(paged.reconcile, "paged");
  }
  return paged;
}

export { validatePagedDeclaration };
