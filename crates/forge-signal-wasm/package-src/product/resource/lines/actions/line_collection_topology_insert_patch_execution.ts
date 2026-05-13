import {
  buildRecursiveTreeInsertPatchValue,
} from "./line_collection_recursive_tree_insert_patch_execution.js";

function buildTopologySpecificInsertPatchValue(options) {
  const topology = options.patchRecord.responseLensProof?.topology ?? null;
  if (topology === "groupedCollection") {
    return buildGroupedCollectionInsertPatchValue(options);
  }
  if (topology === "namedCollection") {
    return buildNamedCollectionInsertPatchValue(options);
  }
  if (topology === "recursiveTree") {
    return buildRecursiveTreeInsertPatchValue(options);
  }
  if (topology === "sparsePage") {
    return buildSparsePageInsertPatchValue(options);
  }
  return null;
}

function buildGroupedCollectionInsertPatchValue({
  patchRecord,
  currentValue,
  patch,
  locatedItem,
}) {
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "groupedCollection") {
    return null;
  }
  const groupedLookup = requireGroupedCollectionInsertLookup(
    patchRecord,
    currentValue,
    patch.itemId,
    locatedItem,
  );
  const groupId = requireNonEmptyTopologyBucketId(
    topologyHelpers.groupId(patch.nextItem),
    "group id",
    patchRecord.familyKind,
  );
  if (groupedLookup.groupId !== groupId) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.insert(...) nextItem group id "${groupId}" to match grouped lookup group id "${groupedLookup.groupId}" for itemId "${patch.itemId}"`,
    );
  }
  patchRecord.reconcile.items(currentValue);
  const currentGroups = topologyHelpers.groups(currentValue);
  const currentGroupItems = Array.isArray(currentGroups[groupId])
    ? currentGroups[groupId]
    : [];
  const nextGroupItems = patch.placement === "prepend"
    ? [patch.nextItem, ...currentGroupItems]
    : [...currentGroupItems, patch.nextItem];
  return topologyHelpers.replaceGroups(currentValue, {
    ...currentGroups,
    [groupId]: nextGroupItems,
  });
}

function buildNamedCollectionInsertPatchValue({
  patchRecord,
  currentValue,
  patch,
  locatedItem,
}) {
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "namedCollection") {
    return null;
  }
  const namedLookup = requireNamedCollectionInsertLookup(
    patchRecord,
    currentValue,
    patch.itemId,
    locatedItem,
  );
  const collectionId = requireNonEmptyTopologyBucketId(
    topologyHelpers.collectionId(patch.nextItem),
    "collection id",
    patchRecord.familyKind,
  );
  if (namedLookup.collectionId !== collectionId) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.insert(...) nextItem collection id "${collectionId}" to match named lookup collection id "${namedLookup.collectionId}" for itemId "${patch.itemId}"`,
    );
  }
  patchRecord.reconcile.items(currentValue);
  const currentCollections = topologyHelpers.collections(currentValue);
  const currentCollectionItems = Array.isArray(currentCollections[collectionId])
    ? currentCollections[collectionId]
    : [];
  const nextCollectionItems = patch.placement === "prepend"
    ? [patch.nextItem, ...currentCollectionItems]
    : [...currentCollectionItems, patch.nextItem];
  return topologyHelpers.replaceCollections(currentValue, {
    ...currentCollections,
    [collectionId]: nextCollectionItems,
  });
}

function buildSparsePageInsertPatchValue({
  patchRecord,
  currentValue,
  patch,
  locatedItem,
}) {
  const topologyHelpers = patchRecord.reconcile.topologyHelpers;
  if (topologyHelpers?.kind !== "sparsePage") {
    return null;
  }
  const sparseLookup = requireSparsePageInsertLookup(
    patchRecord,
    currentValue,
    patch.itemId,
    locatedItem,
  );
  const pageId = requireNonEmptyTopologyBucketId(
    topologyHelpers.pageId(patch.nextItem),
    "page id",
    patchRecord.familyKind,
  );
  if (sparseLookup.pageId !== pageId) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.insert(...) nextItem page id "${pageId}" to match sparse page lookup page id "${sparseLookup.pageId}" for itemId "${patch.itemId}"`,
    );
  }
  const currentPages = requireSparsePageRecord(
    topologyHelpers.pages(currentValue),
    patchRecord.familyKind,
  );
  const currentPageItems = Array.isArray(currentPages[pageId])
    ? currentPages[pageId]
    : [];
  const nextPageItems = patch.placement === "prepend"
    ? [patch.nextItem, ...currentPageItems]
    : [...currentPageItems, patch.nextItem];
  return topologyHelpers.replacePages(currentValue, {
    ...currentPages,
    [pageId]: nextPageItems,
  });
}

function requireGroupedCollectionInsertLookup(
  patchRecord,
  currentValue,
  itemId,
  locatedItem,
) {
  return requireTopologyInsertLookup(
    patchRecord,
    currentValue,
    itemId,
    locatedItem,
    "groupedCollection",
  );
}

function requireNamedCollectionInsertLookup(
  patchRecord,
  currentValue,
  itemId,
  locatedItem,
) {
  return requireTopologyInsertLookup(
    patchRecord,
    currentValue,
    itemId,
    locatedItem,
    "namedCollection",
  );
}

function requireSparsePageInsertLookup(
  patchRecord,
  currentValue,
  itemId,
  locatedItem,
) {
  return requireTopologyInsertLookup(
    patchRecord,
    currentValue,
    itemId,
    locatedItem,
    "sparsePage",
  );
}

function requireTopologyInsertLookup(
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
      `${patchRecord.familyKind} resource lines require readItem(...) proof for exact ${topology} resourcePatch.insert(...)`,
    );
  }
  return patchRecord.reconcile.readItem(currentValue, itemId);
}

function requireNonEmptyTopologyBucketId(value, bucketLabel, familyKind) {
  if (typeof value !== "string" || value.length === 0) {
    throw new TypeError(
      `${familyKind} resource lines require resourcePatch.insert(...) nextItem to carry a non-empty ${bucketLabel}`,
    );
  }
  return value;
}

function requireSparsePageRecord(rawPages, familyKind) {
  if (!rawPages || typeof rawPages !== "object" || Array.isArray(rawPages)) {
    throw new TypeError(
      `${familyKind} resource lines require sparse-page insert topology helpers to expose a record of loaded page arrays`,
    );
  }
  for (const [pageId, items] of Object.entries(rawPages)) {
    if (!Array.isArray(items)) {
      throw new TypeError(
        `${familyKind} resource lines require sparse-page loaded page "${pageId}" to be an array during resourcePatch.insert(...)`,
      );
    }
  }
  return rawPages;
}

export { buildTopologySpecificInsertPatchValue };
