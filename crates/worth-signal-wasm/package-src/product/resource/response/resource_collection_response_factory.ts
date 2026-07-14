import { requireResourceItemAspects } from "../reconciliation/resource_item_aspects.js";
import { requireResourceValueSummaries } from "../reconciliation/resource_value_summaries.js";
import { createResponseLensProof } from "./resource_response_lens_proof.js";

const RESOURCE_COLLECTION_RESPONSE = Symbol("WorthSignal.resourceCollectionResponse");

function createCollectionResponse(kind, options, lensOptions) {
  if (typeof options.itemId !== "function") {
    throw new TypeError(`${kind} requires itemId(item)`);
  }
  if (typeof options.items !== "function") {
    throw new TypeError(`${kind} requires items(value)`);
  }
  if (typeof options.replaceItems !== "function") {
    throw new TypeError(`${kind} requires replaceItems(value, nextItems)`);
  }
  const aspects =
    options.aspects === undefined
      ? null
      : requireResourceItemAspects(options.aspects, kind);
  const summaries =
    options.summaries === undefined
      ? null
      : requireResourceValueSummaries(options.summaries, kind);
  return Object.freeze({
    kind: "collection",
    source: kind,
    lensProof: createResponseLensProof({
      source: kind,
      topology: lensOptions.topology,
      itemField: lensOptions.itemField,
      aspectNames:
        aspects === null
          ? []
          : Object.keys(aspects.definitions),
      jsonAspectNames:
        aspects === null
          ? []
          : readAspectNamesForLocus(aspects, "jsonItemAspect"),
      summaryNames:
        summaries === null
          ? []
          : Object.keys(summaries.definitions),
      summaryPatchScope: summaries?.patchScope ?? null,
    }),
    itemIdentity: options.itemId,
    items: options.items,
    replaceItems: options.replaceItems,
    readItem: options.readItem,
    replaceItem: options.replaceItem,
    topologyHelpers: options.topologyHelpers ?? null,
    aspects,
    summaries,
    [RESOURCE_COLLECTION_RESPONSE]: "resourceCollectionResponse",
  });
}

function readAspectNamesForLocus(aspects, locus) {
  return Object.entries(aspects.definitions)
    .filter(([_aspect, definition]) => definition.locus === locus)
    .map(([aspect]) => aspect);
}

function requireResourceCollectionResponse(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_COLLECTION_RESPONSE] !== "resourceCollectionResponse"
  ) {
    throw new TypeError(
      `${kind} requires a resource.response collection contract`,
    );
  }
  return value;
}

export { createCollectionResponse, requireResourceCollectionResponse };
