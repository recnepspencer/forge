import {
  createCollectionResponse,
} from "./resource_collection_response_factory.js";

function sparse() {
  return function defineSparsePageResponse(options) {
    requireSparsePageResponseOptions(options);
    return createCollectionResponse(
      "resource.response.sparse<T>()(...)",
      createSparsePageAdapter(options),
      { topology: "sparsePage", itemField: null },
    );
  };
}

function requireSparsePageResponseOptions(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      "resource.response.sparse<T>()(...) requires an options object",
    );
  }
  for (const field of [
    "pageId",
    "pageForItem",
    "pages",
    "replacePages",
    "replacePageItem",
  ]) {
    if (typeof options[field] !== "function") {
      throw new TypeError(
        `resource.response.sparse<T>()(...) requires ${field}(...)`,
      );
    }
  }
}

function createSparsePageAdapter(options) {
  return {
    ...options,
    topologyHelpers: Object.freeze({
      kind: "sparsePage",
      pageId: options.pageId,
      pages: options.pages,
      replacePages: options.replacePages,
    }),
    items(value) {
      return readSparsePageItems(
        options.pages(value),
        options.itemId,
        options.pageId,
        "pages(value)",
      );
    },
    replaceItems(value, nextItems) {
      requireSparsePageIdentityRecord(
        options.pages(value),
        options.itemId,
        options.pageId,
        "pages(value)",
      );
      return options.replacePages(
        value,
        createSparsePageRecord(options.itemId, options.pageId, nextItems),
      );
    },
    readItem(value, itemIdValue) {
      return readSparsePageItem(
        options.pages(value),
        options.itemId,
        options.pageId,
        options.pageForItem,
        itemIdValue,
        "pages(value)",
      );
    },
    replaceItem(value, itemIdValue, nextItem) {
      return replaceSingleSparsePageItem(value, itemIdValue, nextItem, options);
    },
  };
}

function replaceSingleSparsePageItem(value, itemIdValue, nextItem, options) {
  const currentItem = readSparsePageItem(
    options.pages(value),
    options.itemId,
    options.pageId,
    options.pageForItem,
    itemIdValue,
    "pages(value)",
  );
  if (!currentItem.found) {
    throw new RangeError(
      `resource.response.sparse<T>()(...) could not find loaded sparse page item id "${itemIdValue}"`,
    );
  }
  requireSparsePageItemIdentity(currentItem.pageId, nextItem, options.itemId, options.pageId, "replacePageItem(value, pageId, itemId, nextItem)");
  const nextValue = options.replacePageItem(
    value,
    currentItem.pageId,
    itemIdValue,
    nextItem,
  );
  assertReplacePageItemPreservedLookup(nextValue, itemIdValue, currentItem.pageId, options);
  return nextValue;
}

function assertReplacePageItemPreservedLookup(nextValue, itemIdValue, pageKey, options) {
  const replacedItem = readSparsePageItem(
    options.pages(nextValue),
    options.itemId,
    options.pageId,
    options.pageForItem,
    itemIdValue,
    "replacePageItem(value, pageId, itemId, nextItem)",
  );
  if (!replacedItem.found || replacedItem.pageId !== pageKey) {
    throw new TypeError(
      `resource.response.sparse<T>()(...) requires replacePageItem(value, pageId, itemId, nextItem) to preserve loaded page item "${itemIdValue}" in page "${pageKey}"`,
    );
  }
}

function readSparsePageItems(rawPages, itemId, pageId, source) {
  return Object.values(
    requireSparsePageIdentityRecord(rawPages, itemId, pageId, source),
  ).flat();
}

function requireSparsePageRecord(rawPages, source) {
  if (!rawPages || typeof rawPages !== "object" || Array.isArray(rawPages)) {
    throw new TypeError(
      "resource.response.sparse<T>()(...) requires pages(value) to return an object record of loaded page arrays",
    );
  }
  for (const [pageKey, items] of Object.entries(rawPages)) {
    if (!Array.isArray(items)) {
      throw new TypeError(
        `resource.response.sparse<T>()(...) requires ${source} loaded page "${pageKey}" to be an array`,
      );
    }
  }
  return rawPages;
}

function requireSparsePageIdentityRecord(rawPages, itemId, pageId, source) {
  const pages = requireSparsePageRecord(rawPages, source);
  const seen = new Set();
  for (const [pageKey, items] of Object.entries(pages)) {
    for (const item of items) {
      const key = itemId(item);
      if (seen.has(key)) {
        throw new TypeError(
          `resource.response.sparse<T>()(...) cannot expose duplicated sparse page item id "${key}"`,
        );
      }
      seen.add(key);
      requireSparsePageItemIdentity(pageKey, item, itemId, pageId, source);
    }
  }
  return pages;
}

function readSparsePageItem(rawPages, itemId, pageId, pageForItem, itemIdValue, source) {
  const pages = requireSparsePageRecord(rawPages, source);
  const pageKey = requireSparsePageLookup(pageForItem, itemIdValue, source);
  const items = pages[pageKey];
  if (!Array.isArray(items)) {
    return Object.freeze({ found: false, pageId: pageKey, item: null });
  }
  let foundItem = null;
  for (const item of items) {
    requireSparsePageItemIdentity(pageKey, item, itemId, pageId, source);
    if (itemId(item) === itemIdValue) {
      if (foundItem !== null) {
        throw new TypeError(
          `resource.response.sparse<T>()(...) cannot expose duplicated sparse page item id "${itemIdValue}" in loaded page "${pageKey}"`,
        );
      }
      foundItem = item;
    }
  }
  return foundItem === null
    ? Object.freeze({ found: false, pageId: pageKey, item: null })
    : Object.freeze({ found: true, pageId: pageKey, item: foundItem });
}

function requireSparsePageLookup(pageForItem, itemIdValue, source) {
  const pageKey = pageForItem(itemIdValue);
  if (typeof pageKey !== "string" || pageKey.length === 0) {
    throw new TypeError(
      `resource.response.sparse<T>()(...) requires pageForItem(itemId) during ${source} to return a non-empty page id`,
    );
  }
  return pageKey;
}

function requireSparsePageItemIdentity(pageKey, item, itemId, pageId, source) {
  const actualPageId = pageId(item);
  if (actualPageId !== pageKey) {
    throw new TypeError(
      `resource.response.sparse<T>()(...) requires ${source} page key "${pageKey}" to match pageId(item) "${actualPageId}"`,
    );
  }
  if (typeof itemId(item) !== "string") {
    throw new TypeError(
      "resource.response.sparse<T>()(...) requires itemId(item) to return a string",
    );
  }
}

function createSparsePageRecord(itemId, pageId, nextItems) {
  const pages = {};
  const seen = new Set();
  for (const item of nextItems) {
    const itemKey = itemId(item);
    if (seen.has(itemKey)) {
      throw new TypeError(
        `resource.response.sparse<T>()(...) cannot replace duplicated sparse page item id "${itemKey}"`,
      );
    }
    seen.add(itemKey);
    const pageKey = pageId(item);
    pages[pageKey] ??= [];
    pages[pageKey].push(item);
  }
  return pages;
}

export { sparse };
