function createLinePatchInverseDescriptor(materialization, patch, currentValue) {
  if (patch.kind === "replace") {
    return null;
  }
  if (patch.kind === "summary") {
    return createSummaryInverseDescriptor(materialization, patch, currentValue);
  }
  return createItemScopedInverseDescriptor(materialization, patch, currentValue);
}

function createSummaryInverseDescriptor(materialization, patch, currentValue) {
  const patchRecord = materialization.patch;
  const summaryDefinitions = patchRecord.reconcile?.summaries?.definitions ?? null;
  const summaryPatchScope = patchRecord.reconcile?.summaries?.patchScope ?? null;
  if (patchRecord.familyKind === "paged" && summaryPatchScope !== "pageWindow") {
    throw new TypeError(
      'paged resource lines require resourceValueSummaries.pageWindow(...) for narrow summary patch(...) admission',
    );
  }
  if (patchRecord.familyKind !== "paged" && summaryPatchScope === "pageWindow") {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit resourceValueSummaries.pageWindow(...) summary patch(...)`,
    );
  }
  if (summaryDefinitions === null || !(patch.summary in summaryDefinitions)) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit summary patch(...) for undeclared summary "${patch.summary}"`,
    );
  }
  const previousSummaryValue = snapshotCompactInversePreimage(() =>
    summaryDefinitions[patch.summary].read(currentValue),
  );
  if (previousSummaryValue.kind === "unavailable") {
    return null;
  }
  return Object.freeze({
    kind: "compactPatchInverse",
    mode: "CompactInversePatch",
    preimage: "summaryValue",
    scope: "summary",
    itemId: null,
    aspect: null,
    summary: patch.summary,
    patch: Object.freeze({
      kind: "summary",
      summary: patch.summary,
      value: previousSummaryValue.value,
    }),
    cost: Object.freeze({
      retainedValueCount: 1,
      retainedResponsePreimage: false,
    }),
  });
}

function createItemScopedInverseDescriptor(materialization, patch, currentValue) {
  const patchRecord = materialization.patch;
  if (typeof patchRecord.itemIdentity !== "function") {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit item patch(...)`,
    );
  }
  if (patchRecord.reconcile === null) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require reconcile: resourceCollectionShape(...) for narrow patch(...) admission`,
    );
  }
  const currentItem = readCurrentItemForInverse(patchRecord, currentValue, patch);
  if (patch.kind === "item") {
    const previousItem = snapshotCompactInversePreimage(() => currentItem);
    if (previousItem.kind === "unavailable") {
      return null;
    }
    return Object.freeze({
      kind: "compactPatchInverse",
      mode: "CompactInversePatch",
      preimage: "itemFragment",
      scope: "item",
      itemId: patch.itemId,
      aspect: null,
      summary: null,
      patch: Object.freeze({
        kind: "item",
        itemId: patch.itemId,
        nextItem: previousItem.value,
      }),
      cost: Object.freeze({
        retainedValueCount: 1,
        retainedResponsePreimage: false,
      }),
    });
  }
  const aspectDefinitions = patchRecord.reconcile.aspects?.definitions ?? null;
  if (aspectDefinitions === null || !(patch.aspect in aspectDefinitions)) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit itemAspect patch(...) for undeclared aspect "${patch.aspect}"`,
    );
  }
  const aspectDefinition = aspectDefinitions[patch.aspect];
  if (optionalJsonPathTerminalWasAbsent(aspectDefinition, currentItem)) {
    return null;
  }
  const previousAspectValue = snapshotCompactInversePreimage(() =>
    aspectDefinition.read(currentItem),
  );
  if (previousAspectValue.kind === "unavailable") {
    return null;
  }
  return Object.freeze({
    kind: "compactPatchInverse",
    mode: "CompactInversePatch",
    preimage: "aspectValue",
    scope: "aspect",
    itemId: patch.itemId,
    aspect: patch.aspect,
    summary: null,
    patch: Object.freeze({
      kind: "itemAspect",
      itemId: patch.itemId,
      aspect: patch.aspect,
      value: previousAspectValue.value,
    }),
    cost: Object.freeze({
      retainedValueCount: 1,
      retainedResponsePreimage: false,
    }),
  });
}

function readCurrentItemForInverse(patchRecord, currentValue, patch) {
  if (typeof patchRecord.reconcile.readItem === "function") {
    if (!inverseShouldUseDirectItemRead(patchRecord)) {
      return readSingleMatchingItem(
        patchRecord,
        [...patchRecord.reconcile.items(currentValue)],
        patch,
      );
    }
    const locatedItem = patchRecord.reconcile.readItem(currentValue, patch.itemId);
    if (locatedItem?.found !== true) {
      throw new RangeError(
        createDirectReadMissingItemMessage(patchRecord, patch.itemId),
      );
    }
    return locatedItem.item;
  }
  return readSingleMatchingItem(
    patchRecord,
    [...patchRecord.reconcile.items(currentValue)],
    patch,
  );
}

function inverseShouldUseDirectItemRead(patchRecord) {
  const topology = patchRecord.responseLensProof?.topology;
  return topology === "sparsePage" || topology === "recursiveTree";
}

function createDirectReadMissingItemMessage(patchRecord, itemId) {
  if (patchRecord.responseLensProof?.topology === "sparsePage") {
    return `${patchRecord.familyKind} resource lines could not find loaded sparse page itemId "${itemId}" for patch(...)`;
  }
  return `${patchRecord.familyKind} resource lines could not find itemId "${itemId}" for patch(...)`;
}

function optionalJsonPathTerminalWasAbsent(aspectDefinition, currentItem) {
  const jsonPathProof = aspectDefinition.jsonPathProof;
  if (jsonPathProof?.policy.absence !== "readAsNull") {
    return false;
  }
  const terminal = jsonPathProof.path.at(-1);
  if (typeof terminal === "number") {
    return false;
  }
  const parentContainer = readJsonPathTerminalParentForInverse(
    currentItem,
    jsonPathProof,
  );
  if (parentContainer === null) {
    return false;
  }
  return Object.getOwnPropertyDescriptor(parentContainer, terminal) === undefined;
}

function readJsonPathTerminalParentForInverse(currentItem, jsonPathProof) {
  const rootDescriptor = Object.getOwnPropertyDescriptor(
    currentItem,
    jsonPathProof.field,
  );
  if (!isJsonPathInverseDataDescriptor(rootDescriptor)) {
    return null;
  }
  let current = rootDescriptor.value;
  for (let index = 0; index < jsonPathProof.path.length - 1; index += 1) {
    const segment = jsonPathProof.path[index];
    if (!isJsonPathInverseContainer(current, segment)) {
      return null;
    }
    const segmentDescriptor = Object.getOwnPropertyDescriptor(current, segment);
    if (!isJsonPathInverseDataDescriptor(segmentDescriptor)) {
      return null;
    }
    current = segmentDescriptor.value;
  }
  const terminal = jsonPathProof.path.at(-1);
  return isJsonPathInverseContainer(current, terminal) ? current : null;
}

function isJsonPathInverseContainer(value, segment) {
  if (typeof segment === "number") {
    return Array.isArray(value);
  }
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function isJsonPathInverseDataDescriptor(descriptor) {
  return descriptor !== undefined
    && descriptor.enumerable
    && Object.prototype.hasOwnProperty.call(descriptor, "value");
}

function readSingleMatchingItem(patchRecord, currentItems, patch) {
  const matchingItems = currentItems.filter(
    (item) => patchRecord.itemIdentity(item) === patch.itemId,
  );
  if (matchingItems.length === 0) {
    throw new RangeError(
      `${patchRecord.familyKind} resource lines could not find itemId "${patch.itemId}" for patch(...)`,
    );
  }
  if (matchingItems.length > 1) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines cannot admit narrow patch(...) for duplicated visible itemId "${patch.itemId}"; use resourcePatch.replace(...) when item identity is ambiguous`,
    );
  }
  return matchingItems[0];
}

function snapshotCompactInversePreimage(readPreimage) {
  let preimage;
  try {
    preimage = readPreimage();
  } catch {
    return Object.freeze({ kind: "unavailable" });
  }
  try {
    const retainedPreimage = structuredClone(preimage);
    deepFreezeRetainedPreimage(retainedPreimage, new WeakSet());
    return Object.freeze({
      kind: "available",
      value: retainedPreimage,
    });
  } catch {
    return Object.freeze({ kind: "unavailable" });
  }
}

function deepFreezeRetainedPreimage(value, seen) {
  if (value === null || typeof value !== "object" || seen.has(value)) {
    return value;
  }
  seen.add(value);
  for (const nestedValue of Object.values(value)) {
    deepFreezeRetainedPreimage(nestedValue, seen);
  }
  return Object.freeze(value);
}

export { createLinePatchInverseDescriptor };
