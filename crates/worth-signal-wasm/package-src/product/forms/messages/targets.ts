import { FormDeclarationError } from "../form_errors.js";
import { requireDeclaredField } from "../validation/declarations.js";

export function requireDeclaredValidationMessageTarget(declaredFieldIds, target) {
  if (declaredFieldIds === null || declaredFieldIds === undefined || target === undefined || target === null) {
    return;
  }
  requireDeclaredField(declaredFieldIds, requireNonEmptyTarget(target, "validation message target"));
}

function requireNonEmptyTarget(value, label) {
  if (typeof value !== "string" || value.length === 0) {
    throw new FormDeclarationError(`${label} must be a non-empty string`, { target: value });
  }
  return value;
}
