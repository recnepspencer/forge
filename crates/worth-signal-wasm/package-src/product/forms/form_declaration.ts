import { FormDeclarationError } from "./form_errors.js";

export function defineFormDeclaration(declaration) {
  if (declaration === null || typeof declaration !== "object" || Array.isArray(declaration)) {
    throw new FormDeclarationError("signals.form.define(...) expects a form declaration object");
  }
  return Object.freeze(declaration);
}
