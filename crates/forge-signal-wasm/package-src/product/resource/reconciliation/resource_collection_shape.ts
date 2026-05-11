import { requireResourceItemAspects } from "./resource_item_aspects.js";
import { requireResourceValueSummaries } from "./resource_value_summaries.js";
import { requireResponseLensProof } from "../response/resource_response_lens_proof.js";

const RESOURCE_COLLECTION_SHAPE = Symbol("forgeSignal.resourceCollectionShape");

function resourceCollectionShape(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError("resourceCollectionShape(...) requires an options object");
  }
  if (typeof options.items !== "function") {
    throw new TypeError("resourceCollectionShape(...) requires items(...)");
  }
  if (typeof options.replaceItems !== "function") {
    throw new TypeError("resourceCollectionShape(...) requires replaceItems(...)");
  }
  const aspects =
    options.aspects === undefined
      ? null
      : requireResourceItemAspects(
          options.aspects,
          "resourceCollectionShape(...)",
        );
  const summaries =
    options.summaries === undefined
      ? null
      : requireResourceValueSummaries(
          options.summaries,
          "resourceCollectionShape(...)",
        );
  const responseLensProof =
    options.responseLensProof === undefined
      ? null
      : requireResponseLensProof(
          options.responseLensProof,
          "resourceCollectionShape(...)",
        );
  return Object.freeze({
    items: options.items,
    replaceItems: options.replaceItems,
    aspects,
    summaries,
    responseLensProof,
    [RESOURCE_COLLECTION_SHAPE]: "resourceCollectionShape",
  });
}

function normalizeResourceCollectionShape(kind, shape) {
  const aspects =
    shape.aspects === null
      ? null
      : requireResourceItemAspects(shape.aspects, kind);
  const summaries =
    shape.summaries === null || shape.summaries === undefined
      ? null
      : requireResourceValueSummaries(shape.summaries, kind);
  const responseLensProof =
    shape.responseLensProof === null || shape.responseLensProof === undefined
      ? null
      : requireResponseLensProof(shape.responseLensProof, kind);
  return Object.freeze({
    items: shape.items,
    replaceItems: shape.replaceItems,
    aspects,
    summaries,
    responseLensProof,
    [RESOURCE_COLLECTION_SHAPE]: "resourceCollectionShape",
  });
}

function requireResourceCollectionShape(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_COLLECTION_SHAPE] !== "resourceCollectionShape"
  ) {
    throw new TypeError(
      `${kind} resources require reconcile created with resourceCollectionShape(...)`,
    );
  }
  return normalizeResourceCollectionShape(kind, value);
}

export { requireResourceCollectionShape, resourceCollectionShape };
