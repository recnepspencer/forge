import { requireResourceJsonPathAspectProof } from "../response/resource_json_path_aspect_proof.js";

const RESOURCE_ITEM_ASPECTS = Symbol("forgeSignal.resourceItemAspects");

function resourceItemAspects(definitions) {
  if (!definitions || typeof definitions !== "object" || Array.isArray(definitions)) {
    throw new TypeError("resourceItemAspects(...) requires a definition object");
  }
  const normalizedDefinitions = {};
  for (const [aspect, definition] of Object.entries(definitions)) {
    if (!definition || typeof definition !== "object" || Array.isArray(definition)) {
      throw new TypeError(`resourceItemAspects(...) aspect "${aspect}" must be an object`);
    }
    if (typeof definition.read !== "function") {
      throw new TypeError(`resourceItemAspects(...) aspect "${aspect}" requires read(...)`);
    }
    if (typeof definition.write !== "function") {
      throw new TypeError(`resourceItemAspects(...) aspect "${aspect}" requires write(...)`);
    }
    const locus = requireAspectLocus(definition.locus, aspect);
    const normalizedDefinition = {
      read: definition.read,
      write: definition.write,
      locus,
    };
    const jsonPathProof = requireResourceJsonPathAspectProof(
      definition.jsonPathProof,
      aspect,
    );
    if (jsonPathProof !== undefined) {
      normalizedDefinition.jsonPathProof = jsonPathProof;
    }
    normalizedDefinitions[aspect] = Object.freeze(normalizedDefinition);
  }
  return Object.freeze({
    definitions: Object.freeze(normalizedDefinitions),
    [RESOURCE_ITEM_ASPECTS]: "resourceItemAspects",
  });
}

function requireResourceItemAspects(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_ITEM_ASPECTS] !== "resourceItemAspects"
  ) {
    const label = kind.includes("(")
      ? `${kind} requires aspects created with resourceItemAspects(...)`
      : `${kind} resources require aspects created with resourceItemAspects(...)`;
    throw new TypeError(
      label,
    );
  }
  return value;
}

function requireAspectLocus(locus, aspect) {
  if (locus === undefined) {
    return "itemAspect";
  }
  if (locus === "itemAspect" || locus === "jsonItemAspect") {
    return locus;
  }
  throw new TypeError(
    `resourceItemAspects(...) aspect "${aspect}" has unsupported effect locus "${locus}"`,
  );
}

export { requireResourceItemAspects, resourceItemAspects };
