import { recordLineHistoryEntry } from "../history/record_line_history_entry.js";
import { createPatchedDiagnostics } from "../state/line_diagnostics_value.js";
import { createLinePatchInverseDescriptor } from "./line_patch_inverse.js";
import { createLocalPatchEffectEnvelope } from "../../effects/resource_effect_envelope.js";
import { createLocalPatchEffectPlan } from "../../effects/resource_effect_plan.js";
import { requireResourcePatch } from "../../reconciliation/resource_patch.js";
import { assertLinePatchRecordAdmitsPatch } from "./line_patch_admission.js";

import { areLineValuesSemanticallyEqual } from "../state/line_value_semantic_equality.js";

function executeLinePatch(materialization, patch) {
  const patchValue = requireResourcePatch(
    patch,
    materialization.patch.familyKind,
  );
  const previousDiagnostics = materialization.binding.diagnosticsSignal();
  if (previousDiagnostics.pendingOperation !== null) {
    throw new TypeError(
      `${materialization.patch.familyKind} resource lines do not admit patch(...) while reload is pending`,
    );
  }
  const currentValue = materialization.binding.valueSignal();
  if (currentValue === null) {
    throw new TypeError(
      `${materialization.patch.familyKind} resource lines do not admit patch(...) before visible value exists`,
    );
  }
  assertLinePatchRecordAdmitsPatch(materialization.patch, patchValue);
  const effectPlan = createLocalPatchEffectPlan(
    materialization,
    previousDiagnostics,
    createLinePatchInverseDescriptor(materialization, patchValue, currentValue),
  );
  const patchOutcome =
    applyPatchValue(materialization, patchValue, currentValue);
  const effectEnvelope = createLocalPatchEffectEnvelope(
    effectPlan,
    patchValue,
    patchOutcome.diagnostics,
  );
  const diagnostics = createPatchedDiagnostics(
    previousDiagnostics,
    patchValue,
    patchOutcome.diagnostics,
    effectEnvelope,
  );
  materialization.binding.diagnosticsSignal.set(diagnostics);
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    "patched",
  );
  return patchOutcome.result;
}

function applyPatchValue(materialization, patchValue, currentValue) {
  return patchValue.kind === "replace"
    ? applyReplacePatch(materialization, patchValue, currentValue)
    : patchValue.kind === "summary"
      ? applySummaryPatch(materialization, patchValue, currentValue)
      : applyItemScopedPatch(materialization, patchValue, currentValue);
}

function applyReplacePatch(materialization, patch, currentValue) {
  assertReplacePatchPreservesResponseTopology(materialization.patch, patch);
  const valueChanged = !areLineValuesSemanticallyEqual(
    patch.nextValue,
    currentValue,
  );
  materialization.binding.valueSignal.set(patch.nextValue);
  return Object.freeze({
    result: Object.freeze({
      kind: "replaced",
      scope: "line",
      itemId: null,
      aspect: null,
    }),
    diagnostics: Object.freeze({
      scope: "line",
      itemId: null,
      aspect: null,
      summary: null,
      valueChanged,
      jsonPathProof: null,
    }),
    valueChanged,
  });
}

function assertReplacePatchPreservesResponseTopology(patchRecord, patch) {
  if (
    patchRecord.reconcile === null ||
    typeof patchRecord.reconcile.items !== "function"
  ) {
    return;
  }
  patchRecord.reconcile.items(patch.nextValue);
}

function applySummaryPatch(materialization, patch, currentValue) {
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
  const currentItems = patchRecord.reconcile.items(currentValue);
  const nextValue = summaryDefinitions[patch.summary].write(
    currentValue,
    patch.value,
  );
  assertSummaryPatchPreservesItems(
    patchRecord,
    patch.summary,
    currentItems,
    nextValue,
  );
  materialization.binding.valueSignal.set(nextValue);
  return Object.freeze({
    result: Object.freeze({
      kind: "narrowed",
      scope: "summary",
      itemId: null,
      aspect: null,
      summary: patch.summary,
    }),
    diagnostics: Object.freeze({
      scope: "summary",
      itemId: null,
      aspect: null,
      summary: patch.summary,
      valueChanged: !areLineValuesSemanticallyEqual(nextValue, currentValue),
      jsonPathProof: null,
    }),
    valueChanged: !areLineValuesSemanticallyEqual(nextValue, currentValue),
  });
}

function assertSummaryPatchPreservesItems(
  patchRecord,
  summary,
  currentItems,
  nextValue,
) {
  const nextItems = patchRecord.reconcile.items(nextValue);
  if (currentItems.length !== nextItems.length) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines require resourcePatch.summary(...) to preserve reconciled items; use resourcePatch.replace(...) when summary "${summary}" changes item membership`,
    );
  }
  for (let index = 0; index < currentItems.length; index += 1) {
    if (patchRecord.itemIdentity(currentItems[index]) !== patchRecord.itemIdentity(nextItems[index])) {
      throw new TypeError(
        `${patchRecord.familyKind} resource lines require resourcePatch.summary(...) to preserve reconciled item identity order; use resourcePatch.replace(...) when summary "${summary}" changes item membership`,
      );
    }
    if (currentItems[index] !== nextItems[index]) {
      throw new TypeError(
        `${patchRecord.familyKind} resource lines require resourcePatch.summary(...) to preserve item objects; use resourcePatch.replace(...) when summary "${summary}" changes item contents`,
      );
    }
  }
}

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
    typeof patchRecord.reconcile.readItem === "function" &&
    typeof patchRecord.reconcile.replaceItem === "function"
  ) {
    return applyDirectItemScopedPatch(patchRecord, patch, currentValue, materialization);
  }
  const currentItems = [...patchRecord.reconcile.items(currentValue)];
  const matchingIndexes = [];
  for (let index = 0; index < currentItems.length; index += 1) {
    if (patchRecord.itemIdentity(currentItems[index]) === patch.itemId) {
      matchingIndexes.push(index);
    }
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
    materialization.binding.valueSignal.set(nextValue);
    return Object.freeze({
      result: Object.freeze({
        kind: "narrowed",
        scope: "item",
        itemId: patch.itemId,
        aspect: null,
      }),
      diagnostics: Object.freeze({
        scope: "item",
        itemId: patch.itemId,
        aspect: null,
        summary: null,
        valueChanged,
        jsonPathProof: null,
      }),
      valueChanged,
    });
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
  materialization.binding.valueSignal.set(nextValue);
  return Object.freeze({
    result: Object.freeze({
      kind: "narrowed",
      scope: "aspect",
      itemId: patch.itemId,
      aspect: patch.aspect,
    }),
    diagnostics: Object.freeze({
      scope: "aspect",
      itemId: patch.itemId,
      aspect: patch.aspect,
      summary: null,
      valueChanged,
      jsonPathProof: aspectDefinition.jsonPathProof ?? null,
    }),
    valueChanged,
  });
}

function applyDirectItemScopedPatch(patchRecord, patch, currentValue, materialization) {
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
  materialization.binding.valueSignal.set(nextValue);
  return Object.freeze({
    result: Object.freeze({
      kind: "narrowed",
      scope: patch.kind === "item" ? "item" : "aspect",
      itemId: patch.itemId,
      aspect: patch.kind === "item" ? null : patch.aspect,
    }),
    diagnostics: Object.freeze({
      scope: patch.kind === "item" ? "item" : "aspect",
      itemId: patch.itemId,
      aspect: patch.kind === "item" ? null : patch.aspect,
      summary: null,
      valueChanged,
      jsonPathProof: aspectPatch?.jsonPathProof ?? null,
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

export { applyPatchValue, executeLinePatch };
