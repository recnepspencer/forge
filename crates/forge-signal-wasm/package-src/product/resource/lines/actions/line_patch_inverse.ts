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
  const currentItems = [...patchRecord.reconcile.items(currentValue)];
  const currentItem = readSingleMatchingItem(patchRecord, currentItems, patch);
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
  const previousAspectValue = snapshotCompactInversePreimage(() =>
    aspectDefinitions[patch.aspect].read(currentItem),
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
