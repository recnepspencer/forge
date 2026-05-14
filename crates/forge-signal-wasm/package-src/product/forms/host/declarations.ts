import { FormDeclarationError } from "../form_errors.js";

const HOST_FACT_KEYS = Object.freeze([
  "focus",
  "visibility",
  "viewport",
  "online",
  "persistence",
  "credentials",
  "autofill",
]);

export function materializeHostBindings(declaration) {
  const declared = declaration.host;
  if (declared === undefined) {
    return emptyHostBindings();
  }
  if (!declared || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form host bindings must be declared as an object");
  }
  const unknownKeys = Object.keys(declared).filter((key) => !HOST_FACT_KEYS.includes(key));
  if (unknownKeys.length > 0) {
    throw new FormDeclarationError("form host bindings use unsupported fact families", {
      hostKeys: unknownKeys,
    });
  }
  return Object.freeze({
    focus: declared.focus ?? null,
    visibility: declared.visibility ?? null,
    viewport: declared.viewport ?? null,
    online: declared.online ?? null,
    persistence: declared.persistence ?? null,
    credentials: declared.credentials ?? null,
    autofill: declared.autofill ?? null,
  });
}

export function emptyHostBindings() {
  return Object.freeze({
    focus: null,
    visibility: null,
    viewport: null,
    online: null,
    persistence: null,
    credentials: null,
    autofill: null,
  });
}
