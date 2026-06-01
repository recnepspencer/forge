import { areLineValuesSemanticallyEqual } from "../state/line_value_semantic_equality.js";
import {
  buildTopologySpecificInsertPatchValue,
} from "./line_collection_topology_insert_patch_execution.js";
import {
  applyCollectionDeletePatch,
  applyDirectCollectionDeletePatch,
  assertCollectionDeleteTopologyAdmitted,
} from "./line_collection_delete_patch_execution.js";

const INSERT_ADMITTED_COLLECTION_TOPOLOGIES = Object.freeze([
  null,
  "connection",
  "directArray",
  "discriminatedTuple",
  "groupedCollection",
  "objectItems",
  "customCollection",
  "entityStore",
  "mapCollection",
  "namedCollection",
  "recursiveTree",
  "sparsePage",
]);

function applyItemScopedPatch(materialization, patch, currentValue) {
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
  if (
    typeof patchRecord.reconcile.readItem === "function"
    && typeof patchRecord.reconcile.replaceItem === "function"
  ) {
    return applyDirectItemScopedPatch(
      patchRecord,
      patch,
      currentValue,
      materialization,
    );
  }
  const currentItems = [...patchRecord.reconcile.items(currentValue)];
  const matchingIndexes = [];
  for (let index = 0; index < currentItems.length; index += 1) {
    if (patchRecord.itemIdentity(currentItems[index]) === patch.itemId) {
      matchingIndexes.push(index);
    }
  }
  if (patch.kind === "insert") {
    return applyCollectionInsertPatch(
      materialization,
      patchRecord,
      currentValue,
      currentItems,
      matchingIndexes,
      patch,
      createCollectionPatchOutcome,
    );
  }
  if (patch.kind === "delete") {
    return applyCollectionDeletePatch(
      materialization,
      patchRecord,
      currentValue,
      currentItems,
      matchingIndexes,
      patch,
      createCollectionPatchOutcome,
    );
  }
  if (matchingIndexes.length === 0) {
    throw new RangeError(
      `${patchRecord.familyKind} resource lines could not find itemId "${patch.itemId}" for patch(...)`,
    );
  }
  if (matchingIndexes.length > 1) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines cannot admit narrow patch(...) for duplicated visible itemId "${patch.itemId}"; use resourcePatch.replace(...) when item identity is ambiguous`,
    );
  }
  const [itemIndex] = matchingIndexes;
  if (patch.kind === "item") {
    const nextItemId = patchRecord.itemIdentity(patch.nextItem);
    if (nextItemId !== patch.itemId) {
      throw new TypeError(
        `${patchRecord.familyKind} resource lines require resourcePatch.item(...) to preserve item identity "${patch.itemId}"; use resourcePatch.replace(...) when the patch changes identity to "${nextItemId}"`,
      );
    }
    currentItems[itemIndex] = patch.nextItem;
    const nextValue = patchRecord.reconcile.replaceItems(
      currentValue,
      currentItems,
    );
    const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
    return createCollectionPatchOutcome(
      "item",
      patch.itemId,
      null,
      valueChanged,
      null,
      nextValue,
    );
  }
  const aspectDefinitions = patchRecord.reconcile.aspects?.definitions ?? null;
  if (aspectDefinitions === null || !(patch.aspect in aspectDefinitions)) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit itemAspect patch(...) for undeclared aspect "${patch.aspect}"`,
    );
  }
  const aspectDefinition = aspectDefinitions[patch.aspect];
  const nextItem = aspectDefinition.write(
    currentItems[itemIndex],
    patch.value,
  );
  assertAspectPatchPreservesItemIdentity(patchRecord, patch, nextItem);
  currentItems[itemIndex] = nextItem;
  const nextValue = patchRecord.reconcile.replaceItems(
    currentValue,
    currentItems,
  );
  const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
  return createCollectionPatchOutcome(
    "aspect",
    patch.itemId,
    patch.aspect,
    valueChanged,
    aspectDefinition.jsonPathProof ?? null,
    nextValue,
  );
}

function applyDirectItemScopedPatch(
  patchRecord,
  patch,
  currentValue,
  materialization,
) {
  if (patch.kind === "insert") {
    assertCollectionInsertTopologyAdmitted(patchRecord);
    const locatedItem = patchRecord.reconcile.readItem(currentValue, patch.itemId);
    return applyDirectCollectionInsertPatch(
      materialization,
      patchRecord,
      currentValue,
      locatedItem,
      patch,
      createCollectionPatchOutcome,
    );
  }
  if (patch.kind === "delete") {
    assertCollectionDeleteTopologyAdmitted(patchRecord);
    const locatedItem = patchRecord.reconcile.readItem(currentValue, patch.itemId);
    return applyDirectCollectionDeletePatch(
      materialization,
      patchRecord,
      currentValue,
      locatedItem,
      patch,
      createCollectionPatchOutcome,
    );
  }
  const locatedItem = patchRecord.reconcile.readItem(currentValue, patch.itemId);
  if (locatedItem?.found !== true) {
    throw new RangeError(
      `${patchRecord.familyKind} resource lines could not find itemId "${patch.itemId}" for patch(...)`,
    );
  }
  const aspectPatch = patch.kind === "item"
    ? null
    : applyDirectAspectPatch(patchRecord, patch, locatedItem.item);
  const nextItem = patch.kind === "item"
    ? requireIdentityPreservingItemPatch(patchRecord, patch)
    : aspectPatch.nextItem;
  const nextValue = patchRecord.reconcile.replaceItem(
    currentValue,
    patch.itemId,
    nextItem,
  );
  const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
  return createCollectionPatchOutcome(
    patch.kind === "item" ? "item" : "aspect",
    patch.itemId,
    patch.kind === "item" ? null : patch.aspect,
    valueChanged,
    aspectPatch?.jsonPathProof ?? null,
    nextValue,
  );
}

function applyCollectionInsertPatch(
  materialization,
  patchRecord,
  currentValue,
  currentItems,
  matchingIndexes,
  patch,
) {
  assertCollectionInsertTopologyAdmitted(patchRecord);
  requireIdentityPreservingInsertPatch(patchRecord, patch);
  if (matchingIndexes.length > 0) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit resourcePatch.insert(...) for duplicate itemId "${patch.itemId}"; use resourcePatch.item(...) when the item already exists`,
    );
  }
  const topologyInsertOutcome = tryApplyTopologySpecificInsertPatch(
    materialization,
    patchRecord,
    currentValue,
    patch,
  );
  if (topologyInsertOutcome !== null) {
    return topologyInsertOutcome;
  }
  const nextItems = patch.placement === "prepend"
    ? [patch.nextItem, ...currentItems]
    : [...currentItems, patch.nextItem];
  const nextValue = patchRecord.reconcile.replaceItems(currentValue, nextItems);
  const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
  return createCollectionPatchOutcome(
    "item",
    patch.itemId,
    null,
    valueChanged,
    null,
    nextValue,
  );
}

function applyDirectCollectionInsertPatch(
  materialization,
  patchRecord,
  currentValue,
  locatedItem,
  patch,
) {
  assertCollectionInsertTopologyAdmitted(patchRecord);
  requireIdentityPreservingInsertPatch(patchRecord, patch);
  if (locatedItem?.found === true) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit resourcePatch.insert(...) for duplicate itemId "${patch.itemId}"; use resourcePatch.item(...) when the item already exists`,
    );
  }
  const topologyInsertOutcome = tryApplyTopologySpecificInsertPatch(
    materialization,
    patchRecord,
    currentValue,
    patch,
    locatedItem,
  );
  if (topologyInsertOutcome !== null) {
    return topologyInsertOutcome;
  }
  const currentItems = [...patchRecord.reconcile.items(currentValue)];
  const nextItems = patch.placement === "prepend"
    ? [patch.nextItem, ...currentItems]
    : [...currentItems, patch.nextItem];
  const nextValue = patchRecord.reconcile.replaceItems(currentValue, nextItems);
  const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
  return createCollectionPatchOutcome(
    "item",
    patch.itemId,
    null,
    valueChanged,
    null,
    nextValue,
  );
}

function tryApplyTopologySpecificInsertPatch(
  materialization,
  patchRecord,
  currentValue,
  patch,
  locatedItem = undefined,
) {
  const nextValue = buildTopologySpecificInsertPatchValue({
    patchRecord,
    currentValue,
    patch,
    locatedItem,
  });
  if (nextValue === null) {
    return null;
  }
  const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
  return createCollectionPatchOutcome(
    "item",
    patch.itemId,
    null,
    valueChanged,
    null,
    nextValue,
  );
}

function requireIdentityPreservingInsertPatch(patchRecord, patch) {
  const nextItemId = patchRecord.itemIdentity(patch.nextItem);
  if (nextItemId !== patch.itemId) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.insert(...) to preserve item identity "${patch.itemId}"; use resourcePatch.replace(...) when the inserted item carries identity "${nextItemId}"`,
    );
  }
}

function createCollectionPatchOutcome(
  scope,
  itemId,
  aspect,
  valueChanged,
  jsonPathProof = null,
  nextValue = undefined,
) {
  return Object.freeze({
    nextValue,
    result: Object.freeze({
      kind: "narrowed",
      scope,
      itemId,
      aspect,
      field: null,
    }),
    diagnostics: Object.freeze({
      scope,
      itemId,
      aspect,
      field: null,
      region: null,
      summary: null,
      valueChanged,
      fieldProof: null,
      regionProof: null,
      jsonPathProof,
    }),
    valueChanged,
  });
}

function requireIdentityPreservingItemPatch(patchRecord, patch) {
  const nextItemId = patchRecord.itemIdentity(patch.nextItem);
  if (nextItemId !== patch.itemId) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.item(...) to preserve item identity "${patch.itemId}"; use resourcePatch.replace(...) when the patch changes identity to "${nextItemId}"`,
    );
  }
  return patch.nextItem;
}

function applyDirectAspectPatch(patchRecord, patch, currentItem) {
  const aspectDefinitions = patchRecord.reconcile.aspects?.definitions ?? null;
  if (aspectDefinitions === null || !(patch.aspect in aspectDefinitions)) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit itemAspect patch(...) for undeclared aspect "${patch.aspect}"`,
    );
  }
  const aspectDefinition = aspectDefinitions[patch.aspect];
  const nextItem = aspectDefinition.write(currentItem, patch.value);
  assertAspectPatchPreservesItemIdentity(patchRecord, patch, nextItem);
  return Object.freeze({
    nextItem,
    jsonPathProof: aspectDefinition.jsonPathProof ?? null,
  });
}

function assertAspectPatchPreservesItemIdentity(patchRecord, patch, nextItem) {
  const nextItemId = patchRecord.itemIdentity(nextItem);
  if (nextItemId !== patch.itemId) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.itemAspect(...) to preserve item identity "${patch.itemId}"; use resourcePatch.replace(...) when aspect "${patch.aspect}" changes identity to "${nextItemId}"`,
    );
  }
}

function assertCollectionInsertTopologyAdmitted(patchRecord) {
  const topology = patchRecord.responseLensProof?.topology ?? null;
  if (INSERT_ADMITTED_COLLECTION_TOPOLOGIES.includes(topology)) {
    return;
  }
  throw new TypeError(
    `${patchRecord.familyKind} resource lines do not admit resourcePatch.insert(...) for ${topology} topologies before advanced placement support lands`,
  );
}

export { applyItemScopedPatch };
