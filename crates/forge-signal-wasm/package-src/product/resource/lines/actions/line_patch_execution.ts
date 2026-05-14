import { recordLineHistoryEntry } from "../history/record_line_history_entry.js";
import { createPatchedDiagnostics } from "../state/line_diagnostics_value.js";
import { createLinePatchInverseDescriptor } from "./line_patch_inverse.js";
import { createLocalPatchEffectEnvelope } from "../../effects/resource_effect_envelope.js";
import { createLocalPatchEffectPlan } from "../../effects/resource_effect_plan.js";
import { requireResourcePatch } from "../../reconciliation/resource_patch.js";
import { assertLinePatchRecordAdmitsPatch } from "./line_patch_admission.js";
import {
  applyDetailFieldPatch,
  applyDetailJsonPathPatch,
  applyDetailRegionPatch,
} from "./line_detail_patch_execution.js";
import { applyItemScopedPatch } from "./line_collection_patch_execution.js";
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
    : patchValue.kind === "field"
      ? applyDetailFieldPatch(materialization, patchValue, currentValue)
    : patchValue.kind === "region"
      ? applyDetailRegionPatch(materialization, patchValue, currentValue)
    : patchValue.kind === "jsonPath"
      ? applyDetailJsonPathPatch(materialization, patchValue, currentValue)
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
      field: null,
    }),
    diagnostics: Object.freeze({
      scope: "line",
      itemId: null,
      aspect: null,
      field: null,
      region: null,
      summary: null,
      valueChanged,
      fieldProof: null,
      regionProof: null,
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
      field: null,
      summary: patch.summary,
    }),
    diagnostics: Object.freeze({
      scope: "summary",
      itemId: null,
      aspect: null,
      field: null,
      region: null,
      summary: patch.summary,
      valueChanged: !areLineValuesSemanticallyEqual(nextValue, currentValue),
      fieldProof: null,
      regionProof: null,
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

export { applyPatchValue, executeLinePatch };
