import { FormDeclarationError } from "../form_errors.js";
import { requireDeclaredField } from "../validation/declarations.js";

export function createDeclaredPresentationScopeRegistry(
  fieldDeclarations,
  stepDeclarations,
  actionDeclarations,
  availabilityDeclarations,
) {
  const sectionIds = new Set(stepDeclarations.map((step) => step.id));
  const controlIds = new Set();
  for (const declaration of availabilityDeclarations) {
    if (declaration.scope === "section") {
      sectionIds.add(declaration.ownerId);
    }
    if (declaration.scope === "control") {
      controlIds.add(declaration.ownerId);
    }
  }
  return Object.freeze({
    fieldIds: new Set(fieldDeclarations.map((field) => field.id)),
    stepIds: new Set(stepDeclarations.map((step) => step.id)),
    actionIds: new Set(actionDeclarations.map((action) => action.id)),
    sectionIds,
    controlIds,
  });
}

export function requireDeclaredPresentationScopeTarget(targetRegistry, scope, target, boundaryLabel = "presentation target") {
  if (scope === "wholeForm") {
    return;
  }
  const declaredTarget = requireNonEmptyTarget(target, `scoped ${scope} ${boundaryLabel}`);
  if (scope === "field") {
    requireDeclaredField(targetRegistry.fieldIds, declaredTarget);
    return;
  }
  if (scope === "step") {
    requireDeclaredSetMember(targetRegistry.stepIds, declaredTarget, "declared step", boundaryLabel);
    return;
  }
  if (scope === "section") {
    requireDeclaredSetMember(targetRegistry.sectionIds, declaredTarget, "declared section", boundaryLabel);
    return;
  }
  if (scope === "action") {
    requireDeclaredSetMember(targetRegistry.actionIds, declaredTarget, "declared action", boundaryLabel);
    return;
  }
  if (scope === "control") {
    requireDeclaredSetMember(targetRegistry.controlIds, declaredTarget, "declared control", boundaryLabel);
  }
}

function requireDeclaredSetMember(ids, target, label, boundaryLabel) {
  if (!ids.has(target)) {
    throw new FormDeclarationError(`${boundaryLabel} must reference a ${label}`, { target });
  }
}

function requireNonEmptyTarget(value, label) {
  if (typeof value !== "string" || value.length === 0) {
    throw new FormDeclarationError(`${label} must be a non-empty string`, { target: value });
  }
  return value;
}
