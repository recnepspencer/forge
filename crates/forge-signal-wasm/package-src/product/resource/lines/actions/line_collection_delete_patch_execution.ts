import { areLineValuesSemanticallyEqual } from "../state/line_value_semantic_equality.js";

import {
  buildTopologySpecificDeletePatchValue,
} from "./line_collection_topology_delete_patch_execution.js";

const DELETE_ADMITTED_COLLECTION_TOPOLOGIES = Object.freeze([
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

function applyCollectionDeletePatch(
  materialization,
  patchRecord,
  currentValue,
  currentItems,
  matchingIndexes,
  patch,
  createCollectionPatchOutcome,
) {
  assertCollectionDeleteTopologyAdmitted(patchRecord);
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
  const topologyDeleteValue = buildTopologySpecificDeletePatchValue({
    patchRecord,
    currentValue,
    patch,
    matchingItems: matchingIndexes.map((index) => currentItems[index]),
  });
  if (topologyDeleteValue !== null) {
    const valueChanged = !areLineValuesSemanticallyEqual(
      topologyDeleteValue,
      currentValue,
    );
    materialization.binding.valueSignal.set(topologyDeleteValue);
    return createCollectionPatchOutcome("item", patch.itemId, null, valueChanged);
  }
  const [itemIndex] = matchingIndexes;
  const nextItems = [...currentItems];
  nextItems.splice(itemIndex, 1);
  const nextValue = patchRecord.reconcile.replaceItems(currentValue, nextItems);
  const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
  materialization.binding.valueSignal.set(nextValue);
  return createCollectionPatchOutcome("item", patch.itemId, null, valueChanged);
}

function applyDirectCollectionDeletePatch(
  materialization,
  patchRecord,
  currentValue,
  locatedItem,
  patch,
  createCollectionPatchOutcome,
) {
  assertCollectionDeleteTopologyAdmitted(patchRecord);
  const topologyDeleteValue = buildTopologySpecificDeletePatchValue({
    patchRecord,
    currentValue,
    patch,
    matchingItems:
      locatedItem?.found === true
        ? [locatedItem.item]
        : [],
    locatedItem,
  });
  if (topologyDeleteValue !== null) {
    const valueChanged = !areLineValuesSemanticallyEqual(
      topologyDeleteValue,
      currentValue,
    );
    materialization.binding.valueSignal.set(topologyDeleteValue);
    return createCollectionPatchOutcome("item", patch.itemId, null, valueChanged);
  }
  if (locatedItem?.found !== true) {
    throw new RangeError(
      `${patchRecord.familyKind} resource lines could not find itemId "${patch.itemId}" for patch(...)`,
    );
  }
  const currentItems = [...patchRecord.reconcile.items(currentValue)];
  const nextItems = currentItems.filter(
    (item) => patchRecord.itemIdentity(item) !== patch.itemId,
  );
  const nextValue = patchRecord.reconcile.replaceItems(currentValue, nextItems);
  const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
  materialization.binding.valueSignal.set(nextValue);
  return createCollectionPatchOutcome("item", patch.itemId, null, valueChanged);
}

function assertCollectionDeleteTopologyAdmitted(patchRecord) {
  const topology = patchRecord.responseLensProof?.topology ?? null;
  if (DELETE_ADMITTED_COLLECTION_TOPOLOGIES.includes(topology)) {
    return;
  }
  throw new TypeError(
    `${patchRecord.familyKind} resource lines do not admit resourcePatch.delete(...) for ${topology} topologies before exact deletion support lands`,
  );
}

export {
  applyCollectionDeletePatch,
  applyDirectCollectionDeletePatch,
  assertCollectionDeleteTopologyAdmitted,
};
