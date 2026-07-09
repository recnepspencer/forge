import {
  buildRecursiveTreeDeletePatchValue,
} from "./line_collection_recursive_tree_delete_patch_execution.js";

function buildTopologySpecificDeletePatchValue({
  patchRecord,
  currentValue,
  patch,
  matchingItems,
  locatedItem = undefined,
}) {
  const topology = patchRecord.responseLensProof?.topology ?? null;
  if (topology === "groupedCollection") {
    return buildGroupedCollectionDeletePatchValue({
      patchRecord,
      currentValue,
      patch,
      matchingItems,
      locatedItem,
    });
  }
  if (topology === "namedCollection") {
    return buildNamedCollectionDeletePatchValue({
      patchRecord,
      currentValue,
      patch,
      matchingItems,
      locatedItem,
    });
  }
  if (topology === "recursiveTree") {
    return buildRecursiveTreeDeletePatchValue({
      patchRecord,
      currentValue,
      patch,
      locatedItem,
    });
  }
  if (topology === "sparsePage") {
    return buildSparsePageDeletePatchValue({
      patchRecord,
      currentValue,
      patch,
      matchingItems,
      locatedItem,
    });
  }
  return null;
}

function buildGroupedCollectionDeletePatchValue({
  patchRecord,
  currentValue,
  patch,
  matchingItems,
  locatedItem,
}) {
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "groupedCollection") {
    return null;
  }
  const matchedItem = requireSingleMatchingTopologyDeleteItem(
    resolveGroupedDeleteMatchingItems(
      patchRecord,
      currentValue,
      patch,
      matchingItems,
      locatedItem,
    ),
    patch,
    patchRecord.familyKind,
  );
  const groupedLookup = requireTopologyDeleteLookup(
    patchRecord,
    currentValue,
    patch.itemId,
    locatedItem,
    "groupedCollection",
  );
  const actualGroupId = requireNonEmptyTopologyBucketId(
    topologyHelpers.groupId(matchedItem),
    "group id",
    patchRecord.familyKind,
    "delete",
  );
  if (groupedLookup.groupId !== actualGroupId) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.delete(...) grouped lookup group id "${groupedLookup.groupId}" to match actual item group id "${actualGroupId}" for itemId "${patch.itemId}"`,
    );
  }
  patchRecord.reconcile.items(currentValue);
  const currentGroups = topologyHelpers.groups(currentValue);
  const currentGroupItems = Array.isArray(currentGroups[actualGroupId])
    ? currentGroups[actualGroupId]
    : [];
  const nextGroupItems = currentGroupItems.filter(
    (item) => patchRecord.itemIdentity(item) !== patch.itemId,
  );
  return topologyHelpers.replaceGroups(currentValue, {
    ...currentGroups,
    [actualGroupId]: nextGroupItems,
  });
}

function buildNamedCollectionDeletePatchValue({
  patchRecord,
  currentValue,
  patch,
  matchingItems,
  locatedItem,
}) {
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "namedCollection") {
    return null;
  }
  const matchedItem = requireSingleMatchingTopologyDeleteItem(
    resolveNamedDeleteMatchingItems(
      patchRecord,
      currentValue,
      patch,
      matchingItems,
      locatedItem,
    ),
    patch,
    patchRecord.familyKind,
  );
  const namedLookup = requireTopologyDeleteLookup(
    patchRecord,
    currentValue,
    patch.itemId,
    locatedItem,
    "namedCollection",
  );
  const actualCollectionId = requireNonEmptyTopologyBucketId(
    topologyHelpers.collectionId(matchedItem),
    "collection id",
    patchRecord.familyKind,
    "delete",
  );
  if (namedLookup.collectionId !== actualCollectionId) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.delete(...) named lookup collection id "${namedLookup.collectionId}" to match actual item collection id "${actualCollectionId}" for itemId "${patch.itemId}"`,
    );
  }
  patchRecord.reconcile.items(currentValue);
  const currentCollections = topologyHelpers.collections(currentValue);
  const currentCollectionItems = Array.isArray(currentCollections[actualCollectionId])
    ? currentCollections[actualCollectionId]
    : [];
  const nextCollectionItems = currentCollectionItems.filter(
    (item) => patchRecord.itemIdentity(item) !== patch.itemId,
  );
  return topologyHelpers.replaceCollections(currentValue, {
    ...currentCollections,
    [actualCollectionId]: nextCollectionItems,
  });
}

function requireSingleMatchingTopologyDeleteItem(matchingItems, patch, familyKind) {
  if (matchingItems.length === 0) {
    throw new RangeError(
      `${familyKind} resource lines could not find itemId "${patch.itemId}" for patch(...)`,
    );
  }
  if (matchingItems.length > 1) {
    throw new TypeError(
      `${familyKind} resource lines cannot admit narrow patch(...) for duplicated visible itemId "${patch.itemId}"; use resourcePatch.replace(...) when item identity is ambiguous`,
    );
  }
  return matchingItems[0];
}

function requireTopologyDeleteLookup(
  patchRecord,
  currentValue,
  itemId,
  locatedItem,
  topology,
) {
  if (locatedItem !== undefined) {
    return locatedItem;
  }
  if (typeof patchRecord.reconcile.readItem !== "function") {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require readItem(...) proof for exact ${topology} resourcePatch.delete(...)`,
    );
  }
  return patchRecord.reconcile.readItem(currentValue, itemId);
}

function buildSparsePageDeletePatchValue({
  patchRecord,
  currentValue,
  patch,
  matchingItems,
  locatedItem,
}) {
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "sparsePage") {
    return null;
  }
  const matchedItem = requireSingleMatchingTopologyDeleteItem(
    resolveSparseDeleteMatchingItems(
      patchRecord,
      currentValue,
      patch,
      matchingItems,
      locatedItem,
    ),
    patch,
    patchRecord.familyKind,
  );
  const sparseLookup = requireTopologyDeleteLookup(
    patchRecord,
    currentValue,
    patch.itemId,
    locatedItem,
    "sparsePage",
  );
  const actualPageId = requireNonEmptyTopologyBucketId(
    topologyHelpers.pageId(matchedItem),
    "page id",
    patchRecord.familyKind,
    "delete",
  );
  if (sparseLookup.pageId !== actualPageId) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.delete(...) sparse page lookup page id "${sparseLookup.pageId}" to match actual item page id "${actualPageId}" for itemId "${patch.itemId}"`,
    );
  }
  const currentPages = requireSparsePageRecord(
    topologyHelpers.pages(currentValue),
    patchRecord.familyKind,
  );
  const currentPageItems = Array.isArray(currentPages[actualPageId])
    ? currentPages[actualPageId]
    : [];
  const nextPageItems = currentPageItems.filter(
    (item) => patchRecord.itemIdentity(item) !== patch.itemId,
  );
  return topologyHelpers.replacePages(currentValue, {
    ...currentPages,
    [actualPageId]: nextPageItems,
  });
}

function resolveSparseDeleteMatchingItems(
  patchRecord,
  currentValue,
  patch,
  matchingItems,
  locatedItem,
) {
  if (locatedItem?.found === true) {
    return matchingItems;
  }
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "sparsePage") {
    return matchingItems;
  }
  const pages = topologyHelpers.pages(currentValue);
  const resolved = [];
  for (const items of Object.values(pages)) {
    if (!Array.isArray(items)) {
      continue;
    }
    for (const item of items) {
      if (patchRecord.itemIdentity(item) === patch.itemId) {
        resolved.push(item);
      }
    }
  }
  return resolved;
}

function requireSparsePageRecord(rawPages, familyKind) {
  if (!rawPages || typeof rawPages !== "object" || Array.isArray(rawPages)) {
    throw new TypeError(
      `${familyKind} resource lines require sparse-page delete topology helpers to expose a record of loaded page arrays`,
    );
  }
  for (const [pageId, items] of Object.entries(rawPages)) {
    if (!Array.isArray(items)) {
      throw new TypeError(
        `${familyKind} resource lines require sparse-page loaded page "${pageId}" to be an array during resourcePatch.delete(...)`,
      );
    }
  }
  return rawPages;
}

function resolveGroupedDeleteMatchingItems(
  patchRecord,
  currentValue,
  patch,
  matchingItems,
  locatedItem,
) {
  if (locatedItem?.found === true) {
    return matchingItems;
  }
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "groupedCollection") {
    return matchingItems;
  }
  const groups = topologyHelpers.groups(currentValue);
  const resolved = [];
  for (const items of Object.values(groups)) {
    if (!Array.isArray(items)) {
      continue;
    }
    for (const item of items) {
      if (patchRecord.itemIdentity(item) === patch.itemId) {
        resolved.push(item);
      }
    }
  }
  return resolved;
}

function resolveNamedDeleteMatchingItems(
  patchRecord,
  currentValue,
  patch,
  matchingItems,
  locatedItem,
) {
  if (locatedItem?.found === true) {
    return matchingItems;
  }
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "namedCollection") {
    return matchingItems;
  }
  const collections = topologyHelpers.collections(currentValue);
  const resolved = [];
  for (const items of Object.values(collections)) {
    if (!Array.isArray(items)) {
      continue;
    }
    for (const item of items) {
      if (patchRecord.itemIdentity(item) === patch.itemId) {
        resolved.push(item);
      }
    }
  }
  return resolved;
}

function requireNonEmptyTopologyBucketId(value, bucketLabel, familyKind, patchKind) {
  if (typeof value !== "string" || value.length === 0) {
    throw new TypeError(
      `${familyKind} resource lines require resourcePatch.${patchKind}(...) target items to carry a non-empty ${bucketLabel}`,
    );
  }
  return value;
}

export { buildTopologySpecificDeletePatchValue };
