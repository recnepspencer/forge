import { resourceItemAspects } from "../reconciliation/resource_item_aspects.js";

function objectAspects() {
  return function defineObjectAspects(fields) {
    return createObjectAspects(
      fields,
      "resource.response.objectAspects<T>()(...)",
      "itemAspect",
    );
  };
}

function jsonObjectAspects() {
  return function defineJsonObjectAspects(fields) {
    return createObjectAspects(
      fields,
      "resource.response.jsonObjectAspects<T>()(...)",
      "jsonItemAspect",
    );
  };
}

function createObjectAspects(fields, kind, locus) {
  if (!fields || typeof fields !== "object" || Array.isArray(fields)) {
    throw new TypeError(`${kind} requires an aspect field object`);
  }
  const definitions = {};
  for (const [aspect, field] of Object.entries(fields)) {
    const objectAspectField = requireObjectAspectField(kind, aspect, field);
    definitions[aspect] = Object.freeze({
      read(item) {
        return requireObjectAspectItem(kind, item, aspect)[objectAspectField];
      },
      write(item, value) {
        return {
          ...requireObjectAspectItem(kind, item, aspect),
          [objectAspectField]: value,
        };
      },
      locus,
    });
  }
  return resourceItemAspects(definitions);
}

function requireObjectAspectField(kind, aspect, field) {
  if (typeof aspect !== "string" || aspect.length === 0) {
    throw new TypeError(
      `${kind} aspect names must be non-empty strings`,
    );
  }
  if (typeof field !== "string" || field.length === 0) {
    throw new TypeError(
      `${kind} aspect "${aspect}" requires a non-empty field name`,
    );
  }
  return field;
}

function requireObjectAspectItem(kind, item, aspect) {
  if (!item || typeof item !== "object" || Array.isArray(item)) {
    throw new TypeError(
      `${kind} aspect "${aspect}" requires object items`,
    );
  }
  return item;
}

export { jsonObjectAspects, objectAspects };
