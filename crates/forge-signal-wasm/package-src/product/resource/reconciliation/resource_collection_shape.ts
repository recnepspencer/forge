import { requireResourceItemAspects } from "./resource_item_aspects.js";
import { requireResourceValueSummaries } from "./resource_value_summaries.js";
import { requireResponseLensProof } from "../response/resource_response_lens_proof.js";

const RESOURCE_COLLECTION_SHAPE = Symbol("forgeSignal.resourceCollectionShape");

function resourceCollectionShape(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError("resourceCollectionShape(...) requires an options object");
  }
  if (options.responseLensProof !== undefined) {
    throw new TypeError(
      "resourceCollectionShape(...) does not accept responseLensProof; use resource.response.* declarations to attach compiled response lens proof",
    );
  }
  return createResourceCollectionShape(options, null, "resourceCollectionShape(...)");
}

function createResponseLensResourceCollectionShape(
  options,
  responseLensProof,
  kind = "resource.response reconciliation",
) {
  return createResourceCollectionShape(
    options,
    requireResponseLensProof(
      responseLensProof,
      kind,
    ),
    kind,
  );
}

function createResourceCollectionShape(options, responseLensProof, kind) {
  if (typeof options.items !== "function") {
    throw new TypeError(`${kind} requires items(...)`);
  }
  if (typeof options.replaceItems !== "function") {
    throw new TypeError(`${kind} requires replaceItems(...)`);
  }
  const aspects =
    options.aspects === undefined
      ? null
      : requireResourceItemAspects(
          options.aspects,
          kind,
        );
  const summaries =
    options.summaries === undefined
      ? null
      : requireResourceValueSummaries(
          options.summaries,
          kind,
        );
  assertResponseLensProofMatchesReconciliationShape(
    responseLensProof,
    aspects,
    summaries,
    kind,
  );
  return Object.freeze({
    items: options.items,
    replaceItems: options.replaceItems,
    readItem: typeof options.readItem === "function" ? options.readItem : null,
    replaceItem:
      typeof options.replaceItem === "function" ? options.replaceItem : null,
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
  assertResponseLensProofMatchesReconciliationShape(
    responseLensProof,
    aspects,
    summaries,
    kind,
  );
  return Object.freeze({
    items: shape.items,
    replaceItems: shape.replaceItems,
    readItem: typeof shape.readItem === "function" ? shape.readItem : null,
    replaceItem: typeof shape.replaceItem === "function" ? shape.replaceItem : null,
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

function assertResponseLensProofMatchesReconciliationShape(
  responseLensProof,
  aspects,
  summaries,
  kind,
) {
  if (responseLensProof === null) {
    return;
  }
  assertDeclaredNamesMatchResponseLensProof(
    responseLensProof.aspectNames,
    aspects === null ? [] : Object.keys(aspects.definitions),
    "aspect",
    kind,
  );
  assertDeclaredNamesMatchResponseLensProof(
    responseLensProof.summaryNames,
    summaries === null ? [] : Object.keys(summaries.definitions),
    "summary",
    kind,
  );
  const summaryPatchScope = summaries?.patchScope ?? null;
  if (responseLensProof.summaryPatchScope !== summaryPatchScope) {
    throw new TypeError(
      `${kind} response lens proof summary patch scope "${responseLensProof.summaryPatchScope}" does not match reconciliation summary patch scope "${summaryPatchScope}"`,
    );
  }
}

function assertDeclaredNamesMatchResponseLensProof(
  proofNames,
  declarationNames,
  label,
  kind,
) {
  const sortedProofNames = [...proofNames].sort();
  const sortedDeclarationNames = [...declarationNames].sort();
  if (sortedProofNames.length !== sortedDeclarationNames.length) {
    throw new TypeError(
      `${kind} response lens proof ${label} declarations do not match reconciliation ${label} declarations`,
    );
  }
  for (let index = 0; index < sortedProofNames.length; index += 1) {
    if (sortedProofNames[index] !== sortedDeclarationNames[index]) {
      throw new TypeError(
        `${kind} response lens proof ${label} "${sortedProofNames[index]}" does not match reconciliation ${label} "${sortedDeclarationNames[index]}"`,
      );
    }
  }
}

export {
  createResponseLensResourceCollectionShape,
  requireResourceCollectionShape,
  resourceCollectionShape,
};
