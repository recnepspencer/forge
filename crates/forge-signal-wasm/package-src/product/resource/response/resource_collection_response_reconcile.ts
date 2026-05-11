import {
  createResponseLensResourceCollectionShape,
} from "../reconciliation/resource_collection_shape.js";

function createResourceCollectionResponseReconcile(response) {
  return createResponseLensResourceCollectionShape(
    {
      items(value) {
        return requireResponseItems(
          response,
          response.items(value),
          "items(value)",
        );
      },
      replaceItems(value, nextItems) {
        const nextValue = response.replaceItems(value, nextItems);
        requireResponseItems(
          response,
          response.items(nextValue),
          "replaceItems(value, nextItems)",
        );
        return nextValue;
      },
      aspects: response.aspects ?? undefined,
      summaries: response.summaries ?? undefined,
    },
    response.lensProof,
    response.source,
  );
}

function requireResponseItems(response, items, label) {
  if (!Array.isArray(items)) {
    throw new TypeError(`${response.source} requires ${label} to produce an array`);
  }
  return items;
}

export { createResourceCollectionResponseReconcile };
