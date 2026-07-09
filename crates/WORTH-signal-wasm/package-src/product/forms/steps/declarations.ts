import { requireDeclaredDependencies } from "../dependency_graph.js";
import { FormDeclarationError } from "../form_errors.js";

const STEP_DECLARATION_BRAND = Symbol("WORTH.form.stepDeclaration");

export function materializeStepDeclarations(declaration, fieldDeclarations) {
  if (declaration.steps === undefined) {
    return Object.freeze([]);
  }
  const declaredFieldIds = new Set(fieldDeclarations.map((field) => field.id));
  const factory = createStepFactory(declaredFieldIds);
  const declared =
    typeof declaration.steps === "function"
      ? declaration.steps({ step: factory.step })
      : declaration.steps;
  if (!declared || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form steps must be declared as an object");
  }
  const declarations = Object.entries(declared).map(([name, step]) =>
    normalizeStepDeclaration(name, step),
  );
  denyDuplicateStepIds(declarations);
  return Object.freeze(
    declarations.sort((left, right) => left.order - right.order || left.id.localeCompare(right.id)),
  );
}

function createStepFactory(declaredFieldIds) {
  return {
    step(stepId, fields, options = {}) {
      requireStepId(stepId);
      const stepFields = requireStepFields(declaredFieldIds, stepId, fields);
      const dependencies = options.dependencies === undefined
        ? stepFields
        : requireDeclaredDependencies(declaredFieldIds, stepId, options.dependencies);
      if (options.resolve !== undefined && typeof options.resolve !== "function") {
        throw new FormDeclarationError("step posture resolver must be a function", { stepId });
      }
      if (options.routeCoupled !== undefined && typeof options.routeCoupled !== "boolean") {
        throw new FormDeclarationError("step routeCoupled posture must be a boolean", { stepId });
      }
      return Object.freeze({
        [STEP_DECLARATION_BRAND]: true,
        id: stepId,
        fields: stepFields,
        dependencies,
        routeCoupled: options.routeCoupled === true,
        group: options.group === undefined ? null : requireNonEmptyString(options.group, "step group"),
        order: normalizeStepOrder(options.order),
        orderDeclared: options.order !== undefined,
        layout: normalizeStepLayout(options),
        defaultPosture: options.optional === true ? "optional" : "active",
        resolve: options.resolve ?? null,
      });
    },
  };
}

function normalizeStepDeclaration(name, step) {
  if (!step || step[STEP_DECLARATION_BRAND] !== true) {
    throw new FormDeclarationError("step entries must be declared with steps.step", { name });
  }
  return Object.freeze({
    ...step,
    name,
  });
}

function denyDuplicateStepIds(declarations) {
  const seen = new Set();
  for (const declaration of declarations) {
    if (seen.has(declaration.id)) {
      throw new FormDeclarationError("step declaration ids must be unique", {
        id: declaration.id,
      });
    }
    seen.add(declaration.id);
  }
}

function requireStepFields(declaredFieldIds, stepId, fields) {
  if (!Array.isArray(fields) || fields.length === 0) {
    throw new FormDeclarationError("step declarations require at least one field", { stepId });
  }
  const seen = new Set();
  for (const fieldId of fields) {
    if (!declaredFieldIds.has(fieldId)) {
      throw new FormDeclarationError("step declaration references an undeclared field", {
        stepId,
        fieldId,
      });
    }
    if (seen.has(fieldId)) {
      throw new FormDeclarationError("step declaration fields must be unique", {
        stepId,
        fieldId,
      });
    }
    seen.add(fieldId);
  }
  return Object.freeze([...fields]);
}

function requireStepId(stepId) {
  requireNonEmptyString(stepId, "step id");
}

function requireNonEmptyString(value, name) {
  if (typeof value !== "string" || value.length === 0) {
    throw new FormDeclarationError(`${name} must be a non-empty string`);
  }
  return value;
}

function normalizeStepOrder(order) {
  if (order === undefined) {
    return 0;
  }
  if (!Number.isInteger(order)) {
    throw new FormDeclarationError("step order must be an integer", { order });
  }
  return order;
}

function normalizeStepLayout(options) {
  const responsive = options.responsive ?? [];
  if (!Array.isArray(responsive) || responsive.some((entry) => typeof entry !== "string" || entry.length === 0)) {
    throw new FormDeclarationError("step layout responsive entries must be non-empty strings", {
      responsive,
    });
  }
  return Object.freeze({
    density: normalizeEnum(options.density, ["compact", "comfortable", "spacious"], "comfortable", "step layout density"),
    alignment: normalizeEnum(options.alignment, ["start", "center", "stretch"], "stretch", "step layout alignment"),
    responsive: Object.freeze([...responsive]),
  });
}

function normalizeEnum(value, allowed, fallback, label) {
  if (value === undefined) {
    return fallback;
  }
  if (!allowed.includes(value)) {
    throw new FormDeclarationError(`${label} is not supported`, { value });
  }
  return value;
}
