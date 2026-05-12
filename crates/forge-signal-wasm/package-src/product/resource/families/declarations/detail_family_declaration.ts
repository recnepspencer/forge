import { requireResourceDeclarationBase } from "./family_declaration_base.js";
import { requireResourceDetailFields } from "../../reconciliation/resource_detail_fields.js";
import { requireResourceDetailJsonPaths } from "../../reconciliation/resource_detail_json_paths.js";
import { requireResourceDetailRegions } from "../../reconciliation/resource_detail_regions.js";
import { requireResponseLensProof } from "../../response/resource_response_lens_proof.js";

function validateDetailDeclaration(declaration) {
  const detail = requireResourceDeclarationBase("detail", declaration);
  if ("itemIdentity" in detail) {
    throw new TypeError("detail resources must not declare itemIdentity");
  }
  if ("accumulatePage" in detail) {
    throw new TypeError("detail resources must not declare accumulatePage");
  }
  if (detail.reconcile !== undefined) {
    requireResourceDetailReconcile(detail.reconcile, "detail resource");
  }
  if (detail.responseLensProof !== undefined) {
    requireResponseLensProof(detail.responseLensProof, "detail resource");
  }
  return detail;
}

function requireResourceDetailReconcile(value, kind) {
  try {
    return requireResourceDetailFields(value, kind);
  } catch (fieldError) {
    try {
      return requireResourceDetailJsonPaths(value, kind);
    } catch (jsonPathError) {
      try {
        return requireResourceDetailRegions(value, kind);
      } catch {
        throw jsonPathError ?? fieldError;
      }
    }
  }
}

export { validateDetailDeclaration };
