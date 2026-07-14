import { requireResourceDeclarationBase } from "./family_declaration_base.js";
import { requireResourceCollectionShape } from "../../reconciliation/resource_collection_shape.js";

function validateCollectionDeclaration(declaration) {
  const collection = requireResourceDeclarationBase("collection", declaration);
  if (typeof collection.itemIdentity !== "function") {
    throw new TypeError("collection resources require itemIdentity(...)");
  }
  if ("accumulatePage" in collection) {
    throw new TypeError("collection resources must not declare accumulatePage");
  }
  if (collection.reconcile !== undefined) {
    requireResourceCollectionShape(collection.reconcile, "collection");
  }
  return collection;
}

export { validateCollectionDeclaration };
