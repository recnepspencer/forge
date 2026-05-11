import {
  createCollectionResponse,
} from "./resource_collection_response_contract.js";

function map() {
  return function defineMapResponse(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resource.response.map<T>()(...) requires an options object",
      );
    }
    if (typeof options.entries !== "function") {
      throw new TypeError("resource.response.map<T>()(...) requires entries(value)");
    }
    if (typeof options.replaceEntries !== "function") {
      throw new TypeError(
        "resource.response.map<T>()(...) requires replaceEntries(value, nextEntries)",
      );
    }
    if (typeof options.replaceEntry !== "function") {
      throw new TypeError(
        "resource.response.map<T>()(...) requires replaceEntry(value, itemId, nextItem)",
      );
    }
    return createCollectionResponse(
      "resource.response.map<T>()(...)",
      {
        ...options,
        items(value) {
          return [
            ...requireMapIdentityEntries(
              options.entries(value),
              options.itemId,
              "entries(value)",
            ).values(),
          ];
        },
        replaceItems(value, nextItems) {
          requireMapIdentityEntries(
            options.entries(value),
            options.itemId,
            "entries(value)",
          );
          return options.replaceEntries(
            value,
            createMapFromItems(options.itemId, nextItems),
          );
        },
        readItem(value, itemIdValue) {
          return readMapItem(
            options.entries(value),
            options.itemId,
            itemIdValue,
            "entries(value)",
          );
        },
        replaceItem(value, itemIdValue, nextItem) {
          const currentEntry = readMapItem(
            options.entries(value),
            options.itemId,
            itemIdValue,
            "entries(value)",
          );
          if (!currentEntry.found) {
            throw new RangeError(
              `resource.response.map<T>()(...) could not find map key "${itemIdValue}"`,
            );
          }
          const nextValue = options.replaceEntry(value, itemIdValue, nextItem);
          const replacedEntry = readMapItem(
            options.entries(nextValue),
            options.itemId,
            itemIdValue,
            "replaceEntry(value, itemId, nextItem)",
          );
          if (!replacedEntry.found) {
            throw new TypeError(
              `resource.response.map<T>()(...) requires replaceEntry(value, itemId, nextItem) to preserve map key "${itemIdValue}"`,
            );
          }
          return nextValue;
        },
      },
      { topology: "mapCollection", itemField: null },
    );
  };
}

function requireMapEntries(rawEntries, source) {
  if (!(rawEntries instanceof Map)) {
    throw new TypeError(
      `resource.response.map<T>()(...) requires ${source} to return a Map`,
    );
  }
  return rawEntries;
}

function requireMapIdentityEntries(rawEntries, itemId, source) {
  const entries = requireMapEntries(rawEntries, source);
  for (const [entryKey, item] of entries) {
    requireMapItemIdentity(entryKey, item, itemId, source);
  }
  return entries;
}

function readMapItem(rawEntries, itemId, itemIdValue, source) {
  const entries = requireMapEntries(rawEntries, source);
  if (!entries.has(itemIdValue)) {
    return Object.freeze({ found: false, item: null });
  }
  const item = entries.get(itemIdValue);
  requireMapItemIdentity(itemIdValue, item, itemId, source);
  return Object.freeze({ found: true, item });
}

function requireMapItemIdentity(entryKey, item, itemId, source) {
  const actualItemId = itemId(item);
  if (actualItemId !== entryKey) {
    throw new TypeError(
      `resource.response.map<T>()(...) requires ${source} map key "${entryKey}" to match itemId(item) "${actualItemId}"`,
    );
  }
}

function createMapFromItems(itemId, nextItems) {
  const entries = new Map();
  for (const item of nextItems) {
    const key = itemId(item);
    if (entries.has(key)) {
      throw new TypeError(
        `resource.response.map<T>()(...) cannot replace duplicated map key "${key}"`,
      );
    }
    entries.set(key, item);
  }
  return entries;
}

export { map };
