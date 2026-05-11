import { requireResourceItemAspects } from "../reconciliation/resource_item_aspects.js";
import { requireResourceValueSummaries } from "../reconciliation/resource_value_summaries.js";
import { createResponseLensProof } from "./resource_response_lens_proof.js";

const RESOURCE_COLLECTION_RESPONSE = Symbol("forgeSignal.resourceCollectionResponse");

function collection(options) {
  if (options === undefined) {
    return function defineCollectionResponse(collectionOptions) {
      return collection(collectionOptions);
    };
  }
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError("resource.response.collection(...) requires an options object");
  }
  return createCollectionResponse(
    "resource.response.collection(...)",
    options,
    { topology: "customCollection", itemField: null },
  );
}

function array(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError("resource.response.array(...) requires an options object");
  }
  return createCollectionResponse(
    "resource.response.array(...)",
    {
      ...options,
      items(value) {
        if (!Array.isArray(value)) {
          throw new TypeError(
            "resource.response.array(...) requires list/paged values to stay direct arrays",
          );
        }
        return value;
      },
      replaceItems(_value, nextItems) {
        return [...nextItems];
      },
    },
    { topology: "directArray", itemField: null },
  );
}

function objectItems() {
  return function defineObjectItems(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resource.response.objectItems<T>()(...) requires an options object",
      );
    }
    const field = requireObjectItemsField(options.field);
    return createCollectionResponse(
      "resource.response.objectItems<T>()(...)",
      {
        ...options,
        items(value) {
          return requireObjectItemsArray(value, field);
        },
        replaceItems(value, nextItems) {
          requireObjectItemsArray(value, field);
          return {
            ...value,
            [field]: [...nextItems],
          };
        },
      },
      { topology: "objectItems", itemField: field },
    );
  };
}

function entityStore() {
  return function defineEntityStore(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resource.response.entityStore<T>()(...) requires an options object",
      );
    }
    if (typeof options.entities !== "function") {
      throw new TypeError(
        "resource.response.entityStore<T>()(...) requires entities(value)",
      );
    }
    if (typeof options.replaceEntities !== "function") {
      throw new TypeError(
        "resource.response.entityStore<T>()(...) requires replaceEntities(value, nextEntities)",
      );
    }
    return createCollectionResponse(
      "resource.response.entityStore<T>()(...)",
      {
        ...options,
        items(value) {
          return Object.values(
            requireEntityStoreRecord(options.entities(value)),
          );
        },
        replaceItems(value, nextItems) {
          requireEntityStoreRecord(options.entities(value));
          return options.replaceEntities(
            value,
            createEntityStoreRecord(options.itemId, nextItems),
          );
        },
      },
      { topology: "entityStore", itemField: null },
    );
  };
}

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

function requireObjectItemsField(field) {
  if (typeof field !== "string" || field.length === 0) {
    throw new TypeError(
      "resource.response.objectItems<T>()(...) requires a non-empty field name",
    );
  }
  return field;
}

function requireObjectItemsArray(value, field) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(
      `resource.response.objectItems<T>() field "${field}" requires object response values`,
    );
  }
  const items = value[field];
  if (!Array.isArray(items)) {
    throw new TypeError(
      `resource.response.objectItems<T>() field "${field}" requires an array`,
    );
  }
  return items;
}

function requireEntityStoreRecord(entities) {
  if (!entities || typeof entities !== "object" || Array.isArray(entities)) {
    throw new TypeError(
      "resource.response.entityStore<T>()(...) requires entities(value) to return an object record",
    );
  }
  return entities;
}

function createEntityStoreRecord(itemId, nextItems) {
  const entities = {};
  for (const item of nextItems) {
    const key = itemId(item);
    if (key in entities) {
      throw new TypeError(
        `resource.response.entityStore<T>()(...) cannot replace duplicated entity id "${key}"`,
      );
    }
    entities[key] = item;
  }
  return entities;
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

export {
  array,
  collection,
  entityStore,
  objectItems,
  requireResourceCollectionResponse,
};
