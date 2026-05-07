import { requireResourceDeclarationBase } from "./family_declaration_base.js";

function validateDetailDeclaration(declaration) {
  const detail = requireResourceDeclarationBase("detail", declaration);
  if ("itemIdentity" in detail) {
    throw new TypeError("detail resources must not declare itemIdentity");
  }
  if ("accumulatePage" in detail) {
    throw new TypeError("detail resources must not declare accumulatePage");
  }
  if ("reconcile" in detail) {
    throw new TypeError("detail resources must not declare reconcile");
  }
  return detail;
}

export { validateDetailDeclaration };
