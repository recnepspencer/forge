import { FormDeclarationError } from "../form_errors.js";
import {
  denyDependencyCycles,
  requireDeclaredDependencies,
} from "../dependency_graph.js";

const AVAILABILITY_DECLARATION_BRAND = Symbol("forge.form.availabilityDeclaration");

export function materializeAvailabilityDeclarations(declaration, fieldDeclarations) {
  if (declaration.availability === undefined) {
    return Object.freeze([]);
  }
  const declaredFieldIds = new Set(fieldDeclarations.map((field) => field.id));
  const factory = createAvailabilityFactory(declaredFieldIds);
  const declared =
    typeof declaration.availability === "function"
      ? declaration.availability({
          field: factory.field,
          action: factory.action,
          control: factory.control,
          group: factory.group,
          section: factory.section,
        })
      : declaration.availability;
  if (!declared || typeof declared !== "object" || Array.isArray(declared)) {
    throw new FormDeclarationError("form availability must be declared as an object");
  }
  const declarations = Object.entries(declared).map(([name, availability]) =>
    normalizeAvailabilityDeclaration(name, availability),
  );
  denyDuplicateAvailabilityIds(declarations);
  denyDependencyCycles(
    declarations.filter((entry) => entry.scope === "field"),
    "availability",
  );
  return Object.freeze(declarations);
}

function createAvailabilityFactory(declaredFieldIds) {
  return {
    field(fieldId, dependencies, resolver, options = {}) {
      requireDeclaredField(declaredFieldIds, fieldId);
      requireAvailabilityResolver(resolver);
      return Object.freeze({
        [AVAILABILITY_DECLARATION_BRAND]: true,
        id: options.id ?? `field:${fieldId}`,
        scope: "field",
        ownerId: fieldId,
        dependencies: requireDeclaredDependencies(declaredFieldIds, fieldId, dependencies),
        resolver,
      });
    },
    action(actionId, dependencies, resolver, options = {}) {
      requireActionId(actionId);
      requireAvailabilityResolver(resolver);
      return Object.freeze({
        [AVAILABILITY_DECLARATION_BRAND]: true,
        id: options.id ?? `action:${actionId}`,
        scope: "action",
        ownerId: actionId,
        dependencies: requireDeclaredDependencies(declaredFieldIds, actionId, dependencies),
        resolver,
      });
    },
    control(controlId, dependencies, resolver, options = {}) {
      requireRegionId(controlId, "control");
      requireAvailabilityResolver(resolver);
      return Object.freeze({
        [AVAILABILITY_DECLARATION_BRAND]: true,
        id: options.id ?? `control:${controlId}`,
        scope: "control",
        ownerId: controlId,
        fields: Object.freeze([]),
        dependencies: requireDeclaredDependencies(declaredFieldIds, controlId, dependencies),
        resolver,
      });
    },
    group(groupId, fields, dependencies, resolver, options = {}) {
      requireRegionId(groupId, "group");
      requireAvailabilityResolver(resolver);
      return Object.freeze({
        [AVAILABILITY_DECLARATION_BRAND]: true,
        id: options.id ?? `group:${groupId}`,
        scope: "group",
        ownerId: groupId,
        fields: requireDeclaredRegionFields(declaredFieldIds, groupId, fields),
        dependencies: requireDeclaredDependencies(declaredFieldIds, groupId, dependencies),
        resolver,
      });
    },
    section(sectionId, fields, dependencies, resolver, options = {}) {
      requireRegionId(sectionId, "section");
      requireAvailabilityResolver(resolver);
      return Object.freeze({
        [AVAILABILITY_DECLARATION_BRAND]: true,
        id: options.id ?? `section:${sectionId}`,
        scope: "section",
        ownerId: sectionId,
        fields: requireDeclaredRegionFields(declaredFieldIds, sectionId, fields),
        dependencies: requireDeclaredDependencies(declaredFieldIds, sectionId, dependencies),
        resolver,
      });
    },
  };
}

function normalizeAvailabilityDeclaration(name, availability) {
  if (!availability || availability[AVAILABILITY_DECLARATION_BRAND] !== true) {
    throw new FormDeclarationError("availability entries must be declared with availability.field/action", {
      name,
    });
  }
  return Object.freeze({
    ...availability,
    name,
  });
}

function denyDuplicateAvailabilityIds(declarations) {
  const seen = new Set();
  for (const declaration of declarations) {
    if (seen.has(declaration.id)) {
      throw new FormDeclarationError("availability declaration ids must be unique", {
        id: declaration.id,
      });
    }
    seen.add(declaration.id);
  }
}

function requireDeclaredField(declaredFieldIds, fieldId) {
  if (!declaredFieldIds.has(fieldId)) {
    throw new FormDeclarationError("availability declaration references an undeclared field", {
      fieldId,
    });
  }
}

function requireActionId(actionId) {
  if (typeof actionId !== "string" || actionId.length === 0) {
    throw new FormDeclarationError("availability action id must be a non-empty string");
  }
}

function requireRegionId(regionId, regionKind) {
  if (typeof regionId !== "string" || regionId.length === 0) {
    throw new FormDeclarationError(`availability ${regionKind} id must be a non-empty string`);
  }
}

function requireDeclaredRegionFields(declaredFieldIds, ownerId, fields) {
  if (!Array.isArray(fields) || fields.length === 0) {
    throw new FormDeclarationError("availability region declarations require at least one field", {
      ownerId,
    });
  }
  const seen = new Set();
  for (const fieldId of fields) {
    if (!declaredFieldIds.has(fieldId)) {
      throw new FormDeclarationError("availability region references an undeclared field", {
        ownerId,
        fieldId,
      });
    }
    if (seen.has(fieldId)) {
      throw new FormDeclarationError("availability region fields must be unique", {
        ownerId,
        fieldId,
      });
    }
    seen.add(fieldId);
  }
  return Object.freeze([...fields]);
}

function requireAvailabilityResolver(resolver) {
  if (typeof resolver !== "function") {
    throw new FormDeclarationError("availability resolver must be a function");
  }
}
