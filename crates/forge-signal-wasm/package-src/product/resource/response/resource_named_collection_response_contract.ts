import {
  createCollectionResponse,
} from "./resource_collection_response_factory.js";

const NAMED_COLLECTION_SOURCE = "resource.response.named<T>()(...)";
const MULTIPLE_COLLECTION_SOURCE = "resource.response.multiple<T>()(...)";

function named() {
  return createNamedCollectionResponseFactory(NAMED_COLLECTION_SOURCE);
}

function multiple() {
  return createNamedCollectionResponseFactory(MULTIPLE_COLLECTION_SOURCE);
}

function createNamedCollectionResponseFactory(source) {
  return function defineNamedCollectionResponse(options) {
    requireNamedCollectionResponseOptions(options, source);
    return createCollectionResponse(
      source,
      createNamedCollectionAdapter(options, source),
      { topology: "namedCollection", itemField: null },
    );
  };
}

function requireNamedCollectionResponseOptions(options, source) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      `${source} requires an options object`,
    );
  }
  for (const field of [
    "collectionId",
    "collectionForItem",
    "collections",
    "replaceCollections",
    "replaceCollectionItem",
  ]) {
    if (typeof options[field] !== "function") {
      throw new TypeError(
        `${source} requires ${field}(...)`,
      );
    }
  }
}

function createNamedCollectionAdapter(options, source) {
  return {
    ...options,
    items(value) {
      return readNamedCollectionItems(
        options.collections(value),
        options.itemId,
        options.collectionId,
        "collections(value)",
        source,
      );
    },
    replaceItems(value, nextItems) {
      requireNamedCollectionIdentityRecord(
        options.collections(value),
        options.itemId,
        options.collectionId,
        "collections(value)",
        source,
      );
      return options.replaceCollections(
        value,
        createNamedCollectionRecord(options.itemId, options.collectionId, nextItems, source),
      );
    },
    readItem(value, itemIdValue) {
      return readNamedCollectionItem(
        options.collections(value),
        options.itemId,
        options.collectionId,
        options.collectionForItem,
        itemIdValue,
        "collections(value)",
        source,
      );
    },
    replaceItem(value, itemIdValue, nextItem) {
      return replaceSingleNamedCollectionItem(value, itemIdValue, nextItem, options, source);
    },
  };
}

function replaceSingleNamedCollectionItem(value, itemIdValue, nextItem, options, source) {
  const currentItem = readNamedCollectionItem(
    options.collections(value),
    options.itemId,
    options.collectionId,
    options.collectionForItem,
    itemIdValue,
    "collections(value)",
    source,
  );
  if (!currentItem.found) {
    throw new RangeError(
      `${source} could not find named collection item id "${itemIdValue}"`,
    );
  }
  requireNamedCollectionItemIdentity(currentItem.collectionId, nextItem, options.itemId, options.collectionId, "replaceCollectionItem(value, collectionId, itemId, nextItem)", source);
  const nextValue = options.replaceCollectionItem(
    value,
    currentItem.collectionId,
    itemIdValue,
    nextItem,
  );
  assertReplaceCollectionItemPreservedLookup(nextValue, itemIdValue, currentItem.collectionId, options, source);
  return nextValue;
}

function assertReplaceCollectionItemPreservedLookup(nextValue, itemIdValue, collectionKey, options, source) {
  const replacedItem = readNamedCollectionItem(
    options.collections(nextValue),
    options.itemId,
    options.collectionId,
    options.collectionForItem,
    itemIdValue,
    "replaceCollectionItem(value, collectionId, itemId, nextItem)",
    source,
  );
  if (!replacedItem.found || replacedItem.collectionId !== collectionKey) {
    throw new TypeError(
      `${source} requires replaceCollectionItem(value, collectionId, itemId, nextItem) to preserve named collection item "${itemIdValue}" in collection "${collectionKey}"`,
    );
  }
}

function readNamedCollectionItems(rawCollections, itemId, collectionId, valueSource, source) {
  return Object.values(
    requireNamedCollectionIdentityRecord(rawCollections, itemId, collectionId, valueSource, source),
  ).flat();
}

function requireNamedCollectionRecord(rawCollections, valueSource, source) {
  if (!rawCollections || typeof rawCollections !== "object" || Array.isArray(rawCollections)) {
    throw new TypeError(
      `${source} requires collections(value) to return an object record of item arrays`,
    );
  }
  for (const [collectionKey, items] of Object.entries(rawCollections)) {
    if (!Array.isArray(items)) {
      throw new TypeError(
        `${source} requires ${valueSource} collection "${collectionKey}" to be an array`,
      );
    }
  }
  return rawCollections;
}

function requireNamedCollectionIdentityRecord(rawCollections, itemId, collectionId, valueSource, source) {
  const collections = requireNamedCollectionRecord(rawCollections, valueSource, source);
  const seen = new Set();
  for (const [collectionKey, items] of Object.entries(collections)) {
    for (const item of items) {
      const key = itemId(item);
      if (seen.has(key)) {
        throw new TypeError(
          `${source} cannot expose duplicated named collection item id "${key}"`,
        );
      }
      seen.add(key);
      requireNamedCollectionItemIdentity(collectionKey, item, itemId, collectionId, valueSource, source);
    }
  }
  return collections;
}

function readNamedCollectionItem(rawCollections, itemId, collectionId, collectionForItem, itemIdValue, valueSource, source) {
  const collections = requireNamedCollectionRecord(rawCollections, valueSource, source);
  const collectionKey = requireNamedCollectionLookup(collectionForItem, itemIdValue, valueSource, source);
  const items = collections[collectionKey];
  if (!Array.isArray(items)) {
    return Object.freeze({ found: false, collectionId: collectionKey, item: null });
  }
  let foundItem = null;
  for (const item of items) {
    requireNamedCollectionItemIdentity(collectionKey, item, itemId, collectionId, valueSource, source);
    if (itemId(item) === itemIdValue) {
      if (foundItem !== null) {
        throw new TypeError(
          `${source} cannot expose duplicated named collection item id "${itemIdValue}" in collection "${collectionKey}"`,
        );
      }
      foundItem = item;
    }
  }
  return foundItem === null
    ? Object.freeze({ found: false, collectionId: collectionKey, item: null })
    : Object.freeze({ found: true, collectionId: collectionKey, item: foundItem });
}

function requireNamedCollectionLookup(collectionForItem, itemIdValue, valueSource, source) {
  const collectionKey = collectionForItem(itemIdValue);
  if (typeof collectionKey !== "string" || collectionKey.length === 0) {
    throw new TypeError(
      `${source} requires collectionForItem(itemId) during ${valueSource} to return a non-empty collection id`,
    );
  }
  return collectionKey;
}

function requireNamedCollectionItemIdentity(collectionKey, item, itemId, collectionId, valueSource, source) {
  const actualCollectionId = collectionId(item);
  if (actualCollectionId !== collectionKey) {
    throw new TypeError(
      `${source} requires ${valueSource} collection key "${collectionKey}" to match collectionId(item) "${actualCollectionId}"`,
    );
  }
  if (typeof itemId(item) !== "string") {
    throw new TypeError(
      `${source} requires itemId(item) to return a string`,
    );
  }
}

function createNamedCollectionRecord(itemId, collectionId, nextItems, source) {
  const collections = {};
  const seen = new Set();
  for (const item of nextItems) {
    const itemKey = itemId(item);
    if (seen.has(itemKey)) {
      throw new TypeError(
        `${source} cannot replace duplicated named collection item id "${itemKey}"`,
      );
    }
    seen.add(itemKey);
    const collectionKey = collectionId(item);
    collections[collectionKey] ??= [];
    collections[collectionKey].push(item);
  }
  return collections;
}

export { multiple, named };
