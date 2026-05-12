import { FormDeclarationError } from "../form_errors.js";

const VALIDATION_DECLARATION_BRAND = Symbol("forge.form.validationDeclaration");

export function materializeValidationDeclarations(declaration, fieldDeclarations) {
  if (declaration.validation === undefined) {
    return Object.freeze([]);
  }
  const declaredFieldIds = new Set(fieldDeclarations.map((field) => field.id));
  const factory = createValidationFactory(declaredFieldIds);
  const declared =
    typeof declaration.validation === "function"
      ? declaration.validation({
          field: factory.field,
          form: factory.form,
          asyncField: factory.asyncField,
        })
      : declaration.validation;
  if (!declared || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form validation must be declared as an object");
  }
  const seenIds = new Set();
  return Object.freeze(
    Object.entries(declared).map(([name, validation]) => {
      if (!validation || validation[VALIDATION_DECLARATION_BRAND] !== true) {
        throw new FormDeclarationError("form validators must be declared with validation.field/form", {
          name,
        });
      }
      if (seenIds.has(validation.id)) {
        throw new FormDeclarationError("form validator ids must be unique", {
          id: validation.id,
        });
      }
      seenIds.add(validation.id);
      return Object.freeze({
        ...validation,
        name,
      });
    }),
  );
}

function createValidationFactory(declaredFieldIds) {
  return {
    field(fieldId, validator, options = {}) {
      requireDeclaredField(declaredFieldIds, fieldId);
      requireValidator(validator);
      return Object.freeze({
        [VALIDATION_DECLARATION_BRAND]: true,
        kind: "sync",
        id: options.id ?? `field:${fieldId}`,
        breadth: "field",
        field: fieldId,
        dependencies: Object.freeze([fieldId]),
        validator,
      });
    },
    form(id, dependencies, validator) {
      if (typeof id !== "string" || id.length === 0) {
        throw new FormDeclarationError("cross-field validator id must be a non-empty string");
      }
      if (!Array.isArray(dependencies) || dependencies.length === 0) {
        throw new FormDeclarationError("cross-field validator dependencies must be a non-empty array", {
          id,
        });
      }
      for (const dependency of dependencies) {
        requireDeclaredField(declaredFieldIds, dependency);
      }
      const uniqueDependencies = validateUniqueDependencies(dependencies);
      requireValidator(validator);
      return Object.freeze({
        [VALIDATION_DECLARATION_BRAND]: true,
        kind: "sync",
        id,
        breadth: uniqueDependencies.length === declaredFieldIds.size ? "wholeForm" : "dependencyRegion",
        field: null,
        dependencies: Object.freeze(uniqueDependencies),
        validator,
      });
    },
    asyncField(fieldId, options = {}) {
      requireDeclaredField(declaredFieldIds, fieldId);
      return Object.freeze({
        [VALIDATION_DECLARATION_BRAND]: true,
        kind: "async",
        id: options.id ?? `async:${fieldId}`,
        breadth: "field",
        field: fieldId,
        dependencies: Object.freeze([fieldId]),
        triggerPolicy: normalizeAsyncTriggerPolicy(options),
      });
    },
  };
}

export function requireDeclaredField(declaredFieldIds, fieldId) {
  if (!declaredFieldIds.has(fieldId)) {
    throw new FormDeclarationError("validator references an undeclared form field", { fieldId });
  }
}

function validateUniqueDependencies(dependencies) {
  const seen = new Set();
  for (const dependency of dependencies) {
    if (seen.has(dependency)) {
      throw new FormDeclarationError("cross-field validator dependencies must be unique", {
        dependency,
      });
    }
    seen.add(dependency);
  }
  return [...dependencies];
}

function requireValidator(validator) {
  if (typeof validator !== "function") {
    throw new FormDeclarationError("form validator must be a function");
  }
}

function normalizeAsyncTriggerPolicy(options) {
  const triggers = options.triggers ?? ["explicit"];
  if (!Array.isArray(triggers) || triggers.length === 0) {
    throw new FormDeclarationError("async validation triggers must be a non-empty array");
  }
  return Object.freeze({
    triggers: Object.freeze(triggers.map((trigger) => String(trigger))),
    debounceMs: options.debounceMs === undefined ? null : Number(options.debounceMs),
  });
}
