import {
  createCollectionResponse,
} from "./resource_collection_response_factory.js";

function grouped() {
  return function defineGroupedResponse(options) {
    requireGroupedResponseOptions(options);
    return createCollectionResponse(
      "resource.response.grouped<T>()(...)",
      createGroupedCollectionAdapter(options),
      { topology: "groupedCollection", itemField: null },
    );
  };
}

function requireGroupedResponseOptions(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      "resource.response.grouped<T>()(...) requires an options object",
    );
  }
  for (const field of [
    "groupId",
    "groups",
    "groupForItem",
    "replaceGroups",
    "replaceGroupItem",
  ]) {
    if (typeof options[field] !== "function") {
      throw new TypeError(
        `resource.response.grouped<T>()(...) requires ${field}(...)`,
      );
    }
  }
}

function createGroupedCollectionAdapter(options) {
  return {
    ...options,
    items(value) {
      return readGroupedItems(
        options.groups(value),
        options.itemId,
        options.groupId,
        "groups(value)",
      );
    },
    replaceItems(value, nextItems) {
      requireGroupedIdentityRecord(
        options.groups(value),
        options.itemId,
        options.groupId,
        "groups(value)",
      );
      return options.replaceGroups(
        value,
        createGroupedRecord(options.itemId, options.groupId, nextItems),
      );
    },
    readItem(value, itemIdValue) {
      return readGroupedItem(
        options.groups(value),
        options.itemId,
        options.groupId,
        options.groupForItem,
        itemIdValue,
        "groups(value)",
      );
    },
    replaceItem(value, itemIdValue, nextItem) {
      return replaceSingleGroupedItem(value, itemIdValue, nextItem, options);
    },
  };
}

function replaceSingleGroupedItem(value, itemIdValue, nextItem, options) {
  const currentItem = readGroupedItem(
    options.groups(value),
    options.itemId,
    options.groupId,
    options.groupForItem,
    itemIdValue,
    "groups(value)",
  );
  requireFoundGroupedItem(currentItem, itemIdValue);
  requireNextGroupedItemStaysInCurrentGroup(
    options.groupId(nextItem),
    currentItem.groupId,
  );
  const nextValue = options.replaceGroupItem(
    value,
    currentItem.groupId,
    itemIdValue,
    nextItem,
  );
  assertReplaceGroupItemPreservedLookup(
    nextValue,
    itemIdValue,
    currentItem.groupId,
    options,
  );
  return nextValue;
}

function requireFoundGroupedItem(currentItem, itemIdValue) {
  if (!currentItem.found) {
    throw new RangeError(
      `resource.response.grouped<T>()(...) could not find grouped item id "${itemIdValue}"`,
    );
  }
}

function requireNextGroupedItemStaysInCurrentGroup(nextItemGroup, currentGroup) {
  if (nextItemGroup !== currentGroup) {
    throw new TypeError(
      `resource.response.grouped<T>()(...) requires replaceGroupItem(value, groupId, itemId, nextItem) to preserve group id "${currentGroup}"`,
    );
  }
}

function assertReplaceGroupItemPreservedLookup(
  nextValue,
  itemIdValue,
  currentGroup,
  options,
) {
  const replacedItem = readGroupedItem(
    options.groups(nextValue),
    options.itemId,
    options.groupId,
    options.groupForItem,
    itemIdValue,
    "replaceGroupItem(value, groupId, itemId, nextItem)",
  );
  if (!replacedItem.found || replacedItem.groupId !== currentGroup) {
    throw new TypeError(
      `resource.response.grouped<T>()(...) requires replaceGroupItem(value, groupId, itemId, nextItem) to preserve grouped item "${itemIdValue}" in group "${currentGroup}"`,
    );
  }
}

function requireGroupedRecord(rawGroups, itemId, groupId, source) {
  if (!rawGroups || typeof rawGroups !== "object" || Array.isArray(rawGroups)) {
    throw new TypeError(
      "resource.response.grouped<T>()(...) requires groups(value) to return an object record of arrays",
    );
  }
  for (const [groupKey, items] of Object.entries(rawGroups)) {
    if (!Array.isArray(items)) {
      throw new TypeError(
        `resource.response.grouped<T>()(...) requires ${source} group "${groupKey}" to be an array`,
      );
    }
    for (const item of items) {
      requireGroupedItemIdentity(groupKey, item, itemId, groupId, source);
    }
  }
  return rawGroups;
}

function readGroupedItems(rawGroups, itemId, groupId, source) {
  return Object.values(
    requireGroupedIdentityRecord(rawGroups, itemId, groupId, source),
  ).flat();
}

function requireGroupedIdentityRecord(rawGroups, itemId, groupId, source) {
  const groups = requireGroupedRecord(rawGroups, itemId, groupId, source);
  const seen = new Set();
  for (const [groupKey, items] of Object.entries(groups)) {
    for (const item of items) {
      const key = itemId(item);
      if (seen.has(key)) {
        throw new TypeError(
          `resource.response.grouped<T>()(...) cannot expose duplicated grouped item id "${key}"`,
        );
      }
      seen.add(key);
      requireGroupedItemIdentity(groupKey, item, itemId, groupId, source);
    }
  }
  return groups;
}

function readGroupedItem(
  rawGroups,
  itemId,
  groupId,
  groupForItem,
  itemIdValue,
  source,
) {
  const groups = requireGroupedRecord(rawGroups, itemId, groupId, source);
  const groupKey = requireGroupedItemLookupGroup(
    groupForItem,
    itemIdValue,
    source,
  );
  const items = groups[groupKey];
  if (!Array.isArray(items)) {
    return Object.freeze({ found: false, groupId: groupKey, item: null });
  }
  let foundItem = null;
  for (const item of items) {
    requireGroupedItemIdentity(groupKey, item, itemId, groupId, source);
    if (itemId(item) === itemIdValue) {
      if (foundItem !== null) {
        throw new TypeError(
          `resource.response.grouped<T>()(...) cannot expose duplicated grouped item id "${itemIdValue}" in group "${groupKey}"`,
        );
      }
      foundItem = item;
    }
  }
  if (foundItem !== null) {
    return Object.freeze({ found: true, groupId: groupKey, item: foundItem });
  }
  return Object.freeze({ found: false, groupId: groupKey, item: null });
}

function requireGroupedItemLookupGroup(groupForItem, itemIdValue, source) {
  const groupKey = groupForItem(itemIdValue);
  if (typeof groupKey !== "string" || groupKey.length === 0) {
    throw new TypeError(
      `resource.response.grouped<T>()(...) requires groupForItem(itemId) during ${source} to return a non-empty group id`,
    );
  }
  return groupKey;
}

function requireGroupedItemIdentity(groupKey, item, itemId, groupId, source) {
  const actualGroupId = groupId(item);
  if (actualGroupId !== groupKey) {
    throw new TypeError(
      `resource.response.grouped<T>()(...) requires ${source} group key "${groupKey}" to match groupId(item) "${actualGroupId}"`,
    );
  }
  if (typeof itemId(item) !== "string") {
    throw new TypeError(
      "resource.response.grouped<T>()(...) requires itemId(item) to return a string",
    );
  }
}

function createGroupedRecord(itemId, groupId, nextItems) {
  const groups = {};
  const seen = new Set();
  for (const item of nextItems) {
    const itemKey = itemId(item);
    if (seen.has(itemKey)) {
      throw new TypeError(
        `resource.response.grouped<T>()(...) cannot replace duplicated grouped item id "${itemKey}"`,
      );
    }
    seen.add(itemKey);
    const groupKey = groupId(item);
    groups[groupKey] ??= [];
    groups[groupKey].push(item);
  }
  return groups;
}

export { grouped };
