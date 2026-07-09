import { FormDeclarationError } from "../form_errors.js";
import { requireDeclaredDependencies } from "../dependency_graph.js";

const ADMISSION_DECLARATION_BRAND = Symbol("worth.form.admissionDeclaration");
const ADMISSION_CAPABILITIES = new Set([
  "edit",
  "patch",
  "submit",
  "action",
  "approval",
  "signature",
  "review",
  "reason",
]);

export function materializeAdmissionDeclarations(declaration, fieldDeclarations) {
  if (declaration.admission === undefined) {
    return Object.freeze([]);
  }
  const declaredFieldIds = new Set(fieldDeclarations.map((field) => field.id));
  const factory = createAdmissionFactory(declaredFieldIds);
  const declared =
    typeof declaration.admission === "function"
      ? declaration.admission({
          field: factory.field,
          action: factory.action,
        })
      : declaration.admission;
  if (!declared || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form admission must be declared as an object");
  }
  const declarations = Object.entries(declared).map(([name, admission]) =>
    normalizeAdmissionDeclaration(name, admission),
  );
  denyDuplicateAdmissionIds(declarations);
  return Object.freeze(declarations);
}

function createAdmissionFactory(declaredFieldIds) {
  return {
    field(fieldId, capability, dependencies, resolver, options = {}) {
      requireDeclaredField(declaredFieldIds, fieldId);
      requireAdmissionCapability(capability);
      requireAdmissionResolver(resolver);
      return Object.freeze({
        [ADMISSION_DECLARATION_BRAND]: true,
        id: options.id ?? `field:${fieldId}:${capability}`,
        scope: "field",
        ownerId: fieldId,
        capability,
        dependencies: requireDeclaredDependencies(declaredFieldIds, fieldId, dependencies),
        resolver,
      });
    },
    action(actionId, capability, dependencies, resolver, options = {}) {
      requireActionId(actionId);
      requireAdmissionCapability(capability);
      requireAdmissionResolver(resolver);
      return Object.freeze({
        [ADMISSION_DECLARATION_BRAND]: true,
        id: options.id ?? `action:${actionId}:${capability}`,
        scope: "action",
        ownerId: actionId,
        capability,
        dependencies: requireDeclaredDependencies(declaredFieldIds, actionId, dependencies),
        resolver,
      });
    },
  };
}

function normalizeAdmissionDeclaration(name, admission) {
  if (!admission || admission[ADMISSION_DECLARATION_BRAND] !== true) {
    throw new FormDeclarationError("admission entries must be declared with admission.field/action", {
      name,
    });
  }
  return Object.freeze({
    ...admission,
    name,
  });
}

function denyDuplicateAdmissionIds(declarations) {
  const seen = new Set();
  for (const declaration of declarations) {
    if (seen.has(declaration.id)) {
      throw new FormDeclarationError("admission declaration ids must be unique", {
        id: declaration.id,
      });
    }
    seen.add(declaration.id);
  }
}

function requireDeclaredField(declaredFieldIds, fieldId) {
  if (!declaredFieldIds.has(fieldId)) {
    throw new FormDeclarationError("admission declaration references an undeclared field", {
      fieldId,
    });
  }
}

function requireActionId(actionId) {
  if (typeof actionId !== "string" || actionId.length === 0) {
    throw new FormDeclarationError("admission action id must be a non-empty string");
  }
}

function requireAdmissionCapability(capability) {
  if (!ADMISSION_CAPABILITIES.has(capability)) {
    throw new FormDeclarationError("admission capability is not supported", {
      capability,
    });
  }
}

function requireAdmissionResolver(resolver) {
  if (typeof resolver !== "function") {
    throw new FormDeclarationError("admission resolver must be a function");
  }
}

