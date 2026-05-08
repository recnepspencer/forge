import { resourceItemAspects } from "../reconciliation/resource_item_aspects.js";

function objectAspects() {
  return function defineObjectAspects(fields) {
    if (!fields || typeof fields !== "object" || Array.isArray(fields)) {
      throw new TypeError(
        "resource.response.objectAspects<T>()(...) requires an aspect field object",
      );
    }
    const definitions = {};
    for (const [aspect, field] of Object.entries(fields)) {
      const objectAspectField = requireObjectAspectField(aspect, field);
      definitions[aspect] = Object.freeze({
        read(item) {
          return requireObjectAspectItem(item, aspect)[objectAspectField];
        },
        write(item, value) {
          return {
            ...requireObjectAspectItem(item, aspect),
            [objectAspectField]: value,
          };
        },
      });
    }
    return resourceItemAspects(definitions);
  };
}

function requireObjectAspectField(aspect, field) {
  if (typeof aspect !== "string" || aspect.length === 0) {
    throw new TypeError(
      "resource.response.objectAspects<T>()(...) aspect names must be non-empty strings",
    );
  }
  if (typeof field !== "string" || field.length === 0) {
    throw new TypeError(
      `resource.response.objectAspects<T>()(...) aspect "${aspect}" requires a non-empty field name`,
    );
  }
  return field;
}

function requireObjectAspectItem(item, aspect) {
  if (!item || typeof item !== "object" || Array.isArray(item)) {
    throw new TypeError(
      `resource.response.objectAspects<T>() aspect "${aspect}" requires object items`,
    );
  }
  return item;
}

export { objectAspects };
