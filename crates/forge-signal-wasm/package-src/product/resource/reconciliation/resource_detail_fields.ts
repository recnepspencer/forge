import {
  createResourceDetailFieldProof,
  requireResourceDetailFieldProof,
} from "../response/detail_field_proof.js";

const RESOURCE_DETAIL_FIELDS = Symbol("forgeSignal.resourceDetailFields");

function resourceDetailFields(definitions) {
  if (!definitions || typeof definitions !== "object" || Array.isArray(definitions)) {
    throw new TypeError("resourceDetailFields(...) requires a definition object");
  }
  const normalized = Object.create(null);
  for (const [field, definition] of readDetailFieldDeclarations(
    definitions,
    "resourceDetailFields(...)",
  )) {
    requireSafeDetailFieldName(field, "resourceDetailFields(...)");
    if (!definition || typeof definition !== "object" || Array.isArray(definition)) {
      throw new TypeError(`resourceDetailFields(...) field "${field}" must be an object`);
    }
    if (typeof definition.read !== "function") {
      throw new TypeError(`resourceDetailFields(...) field "${field}" requires read(...)`);
    }
    if (typeof definition.write !== "function") {
      throw new TypeError(`resourceDetailFields(...) field "${field}" requires write(...)`);
    }
    normalized[field] = Object.freeze({
      read: definition.read,
      write: definition.write,
      fieldProof: createResourceDetailFieldProof(field),
    });
  }
  return Object.freeze({
    definitions: Object.freeze(normalized),
    [RESOURCE_DETAIL_FIELDS]: "resourceDetailFields",
  });
}

function requireResourceDetailFields(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_DETAIL_FIELDS] !== "resourceDetailFields"
  ) {
    const label =
      kind === undefined
        ? "resourceDetailFields(...)"
        : `${kind} requires detail fields created with resourceDetailFields(...)`;
    throw new TypeError(label);
  }
  const normalized = Object.create(null);
  for (const [field, definition] of readDetailFieldDeclarations(
    value.definitions ?? {},
    kind ?? "resourceDetailFields(...)",
  )) {
    requireSafeDetailFieldName(field, kind ?? "resourceDetailFields(...)");
    if (!definition || typeof definition.read !== "function" || typeof definition.write !== "function") {
      throw new TypeError(
        `${kind ?? "resourceDetailFields(...)"} requires valid detail field definitions`,
      );
    }
    normalized[field] = Object.freeze({
      read: definition.read,
      write: definition.write,
      fieldProof: requireResourceDetailFieldProof(definition.fieldProof, field),
    });
  }
  return Object.freeze({
    definitions: Object.freeze(normalized),
    [RESOURCE_DETAIL_FIELDS]: "resourceDetailFields",
  });
}

function readDetailFieldDeclarations(definitions, source) {
  const declarations = [];
  for (const key of Reflect.ownKeys(definitions)) {
    if (typeof key !== "string") {
      continue;
    }
    const descriptor = Object.getOwnPropertyDescriptor(definitions, key);
    if (descriptor === undefined || !descriptor.enumerable) {
      continue;
    }
    if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
      throw new TypeError(
        `${source} rejects accessor detail field declaration "${key}"`,
      );
    }
    declarations.push(Object.freeze([key, descriptor.value]));
  }
  return declarations;
}

function requireSafeDetailFieldName(field, source) {
  if (typeof field !== "string" || field.length === 0) {
    throw new TypeError(`${source} field names must be non-empty strings`);
  }
  if (field === "__proto__" || field === "constructor" || field === "prototype") {
    throw new TypeError(`${source} rejects unsafe detail field "${field}"`);
  }
  return field;
}

export { requireResourceDetailFields, resourceDetailFields };
