const RESOURCE_PATCH = Symbol("WorthSignal.resourcePatch");

const resourcePatch = Object.freeze({
  replace(nextValue) {
    return Object.freeze({
      kind: "replace",
      nextValue,
      [RESOURCE_PATCH]: "resourcePatch",
    });
  },
  field(options) {
    requirePatchObject(options, "resourcePatch.field(...)");
    if (typeof options.field !== "string" || options.field.length === 0) {
      throw new TypeError("resourcePatch.field(...) requires field");
    }
    return Object.freeze({
      kind: "field",
      field: options.field,
      value: options.value,
      [RESOURCE_PATCH]: "resourcePatch",
    });
  },
  region(options) {
    requirePatchObject(options, "resourcePatch.region(...)");
    if (typeof options.region !== "string" || options.region.length === 0) {
      throw new TypeError("resourcePatch.region(...) requires region");
    }
    return Object.freeze({
      kind: "region",
      region: options.region,
      value: options.value,
      [RESOURCE_PATCH]: "resourcePatch",
    });
  },
  jsonPath(options) {
    requirePatchObject(options, "resourcePatch.jsonPath(...)");
    if (typeof options.path !== "string" || options.path.length === 0) {
      throw new TypeError("resourcePatch.jsonPath(...) requires path");
    }
    return Object.freeze({
      kind: "jsonPath",
      path: options.path,
      value: options.value,
      [RESOURCE_PATCH]: "resourcePatch",
    });
  },
  item(options) {
    requirePatchObject(options, "resourcePatch.item(...)");
    if (typeof options.itemId !== "string" || options.itemId.length === 0) {
      throw new TypeError("resourcePatch.item(...) requires itemId");
    }
    return Object.freeze({
      kind: "item",
      itemId: options.itemId,
      nextItem: options.nextItem,
      [RESOURCE_PATCH]: "resourcePatch",
    });
  },
  delete(options) {
    requirePatchObject(options, "resourcePatch.delete(...)");
    if (typeof options.itemId !== "string" || options.itemId.length === 0) {
      throw new TypeError("resourcePatch.delete(...) requires itemId");
    }
    return Object.freeze({
      kind: "delete",
      itemId: options.itemId,
      [RESOURCE_PATCH]: "resourcePatch",
    });
  },
  insert(options) {
    requirePatchObject(options, "resourcePatch.insert(...)");
    if (typeof options.itemId !== "string" || options.itemId.length === 0) {
      throw new TypeError("resourcePatch.insert(...) requires itemId");
    }
    if (options.placement !== "append" && options.placement !== "prepend") {
      throw new TypeError(
        "resourcePatch.insert(...) placement must be append or prepend",
      );
    }
    return Object.freeze({
      kind: "insert",
      itemId: options.itemId,
      placement: options.placement,
      nextItem: options.nextItem,
      [RESOURCE_PATCH]: "resourcePatch",
    });
  },
  itemAspect(options) {
    requirePatchObject(options, "resourcePatch.itemAspect(...)");
    if (typeof options.itemId !== "string" || options.itemId.length === 0) {
      throw new TypeError("resourcePatch.itemAspect(...) requires itemId");
    }
    if (typeof options.aspect !== "string" || options.aspect.length === 0) {
      throw new TypeError("resourcePatch.itemAspect(...) requires aspect");
    }
    return Object.freeze({
      kind: "itemAspect",
      itemId: options.itemId,
      aspect: options.aspect,
      value: options.value,
      [RESOURCE_PATCH]: "resourcePatch",
    });
  },
  summary(options) {
    requirePatchObject(options, "resourcePatch.summary(...)");
    if (typeof options.summary !== "string" || options.summary.length === 0) {
      throw new TypeError("resourcePatch.summary(...) requires summary");
    }
    return Object.freeze({
      kind: "summary",
      summary: options.summary,
      value: options.value,
      [RESOURCE_PATCH]: "resourcePatch",
    });
  },
});

function requireResourcePatch(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_PATCH] !== "resourcePatch"
  ) {
    throw new TypeError(
      `${kind} resource lines require patch(...) values created with resourcePatch.*()`,
    );
  }
  return value;
}

function requirePatchObject(value, label) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`${label} requires an options object`);
  }
}

export { requireResourcePatch, resourcePatch };
