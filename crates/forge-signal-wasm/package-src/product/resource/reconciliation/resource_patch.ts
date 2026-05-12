const RESOURCE_PATCH = Symbol("forgeSignal.resourcePatch");

const resourcePatch = Object.freeze({
  replace(nextValue) {
    return Object.freeze({
      kind: "replace",
      nextValue,
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
