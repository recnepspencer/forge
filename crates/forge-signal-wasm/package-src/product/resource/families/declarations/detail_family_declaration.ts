import { requireResourceDeclarationBase } from "./family_declaration_base.js";
import { requireResponseLensProof } from "../../response/resource_response_lens_proof.js";

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
  if (detail.responseLensProof !== undefined) {
    requireResponseLensProof(detail.responseLensProof, "detail resource");
  }
  return detail;
}

export { validateDetailDeclaration };
