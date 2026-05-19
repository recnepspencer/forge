import {
  readPath,
} from "../values/value_paths.js";
import { cloneFormValue, stableValueDigest } from "../values/value_semantics.js";
import {
  aggregateSemanticEqualityCounters,
  compareSemanticValues,
} from "../values/semantic_equality.js";

export function dirtyFieldRecords(fieldDeclarations, form, options = {}) {
  const omittedFields = options.omittedFields ?? new Set();
  const clearedFields = options.clearedFields ?? new Set();
  const selectedFields = selectDirtyComparableFields(fieldDeclarations, {
    omittedFields,
    clearedFields,
  });
  const sourceSnapshot = form.source();
  const effectiveSnapshot = form.effective();
  const comparisons = [];
  const fields = selectedFields.fields
    .map((field) => dirtyFieldRecord(field, sourceSnapshot, effectiveSnapshot, comparisons))
    .filter(Boolean);
  return Object.freeze({
    fields: Object.freeze(fields),
    equality: aggregateSemanticEqualityCounters(comparisons),
    breadth: Object.freeze({
      declaredFields: fieldDeclarations.length,
      comparedFields: comparisons.length,
      changedFields: fields.length,
      omittedFields: selectedFields.counters.omittedFields,
      clearedFields: selectedFields.counters.clearedFields,
      sourceSnapshots: 1,
      effectiveSnapshots: 1,
    }),
  });
}

export function buildPatchPlan(fieldDeclarations, form, rawInputs, options = {}) {
  const omittedFields = options.omittedFields ?? new Set();
  const clearedFields = options.clearedFields ?? new Set();
  const selectedFields = selectPatchComparableFields(fieldDeclarations, {
    rawInputs,
    omittedFields,
    clearedFields,
  });
  const sourceSnapshot = form.source();
  const effectiveSnapshot = form.effective();
  const comparisons = [];
  const operations = [];
  const changedFields = [];
  const replacementReasons = [];
  for (const field of selectedFields.fields) {
    const change = patchChangeForField(field, sourceSnapshot, effectiveSnapshot, comparisons);
    if (change === null) {
      continue;
    }
    changedFields.push(field.id);
    if (change.replacementReason !== null) {
      replacementReasons.push(change.replacementReason);
      continue;
    }
    operations.push(...change.operations);
  }
  const replacement = replacementReasons.length === 0
    ? null
    : Object.freeze({
        scope: "wholeForm",
        fields: Object.freeze([...changedFields]),
        reason: replacementReasons[0],
        value: cloneFormValue(effectiveSnapshot),
        valueDigest: stableValueDigest(effectiveSnapshot),
      });
  const semanticDirty = operations.length > 0 || replacement !== null;
  return Object.freeze({
    semanticDirty,
    empty: !semanticDirty,
    operations: Object.freeze(operations),
    blocked: Object.freeze(rawInputBlockers(rawInputs)),
    broadReplacement: replacement !== null,
    replacement,
    equality: aggregateSemanticEqualityCounters(comparisons),
    breadth: Object.freeze({
      declaredFields: fieldDeclarations.length,
      comparedFields: comparisons.length,
      changedFields: changedFields.length,
      skippedRawInputFields: selectedFields.counters.skippedRawInputFields,
      omittedFields: selectedFields.counters.omittedFields,
      clearedFields: selectedFields.counters.clearedFields,
      sourceSnapshots: 1,
      effectiveSnapshots: 1,
    }),
    equivalenceDigest: replacement === null
      ? stableValueDigest(
          operations.map((operation) => (
            operation.kind === "removeItem"
              ? [operation.kind, operation.field, operation.itemId]
              : [operation.kind, operation.field, operation.itemId ?? null, operation.valueDigest ?? null]
          )),
        )
      : stableValueDigest(replacement),
  });
}

function selectDirtyComparableFields(fieldDeclarations, selection) {
  const counters = {
    omittedFields: 0,
    clearedFields: 0,
  };
  const fields = [];
  for (const field of fieldDeclarations) {
    if (selection.omittedFields.has(field.id)) {
      counters.omittedFields += 1;
      continue;
    }
    if (selection.clearedFields.has(field.id)) {
      counters.clearedFields += 1;
      continue;
    }
    fields.push(field);
  }
  return Object.freeze({
    fields: Object.freeze(fields),
    counters: Object.freeze(counters),
  });
}

function selectPatchComparableFields(fieldDeclarations, selection) {
  const counters = {
    skippedRawInputFields: 0,
    omittedFields: 0,
    clearedFields: 0,
  };
  const fields = [];
  for (const field of fieldDeclarations) {
    if (selection.rawInputs.has(field.id)) {
      counters.skippedRawInputFields += 1;
      continue;
    }
    if (selection.omittedFields.has(field.id)) {
      counters.omittedFields += 1;
      continue;
    }
    if (selection.clearedFields.has(field.id)) {
      counters.clearedFields += 1;
      continue;
    }
    fields.push(field);
  }
  return Object.freeze({
    fields: Object.freeze(fields),
    counters: Object.freeze(counters),
  });
}

function dirtyFieldRecord(field, sourceSnapshot, effectiveSnapshot, comparisons) {
  const sourceValue = readPath(sourceSnapshot, field.segments);
  const effectiveValue = readPath(effectiveSnapshot, field.segments);
  const comparison = compareFieldValues(field, sourceValue, effectiveValue);
  comparisons.push(comparison);
  if (comparison.equal) {
    return null;
  }
  return Object.freeze({
    field: field.id,
    path: field.path,
    sourceDigest: stableValueDigest(sourceValue),
    effectiveDigest: stableValueDigest(effectiveValue),
    equality: comparison.counters,
  });
}

function patchChangeForField(field, sourceSnapshot, effectiveSnapshot, comparisons) {
  const sourceValue = readPath(sourceSnapshot, field.segments);
  const effectiveValue = readPath(effectiveSnapshot, field.segments);
  const comparison = compareFieldValues(field, sourceValue, effectiveValue);
  comparisons.push(comparison);
  if (comparison.equal) {
    return null;
  }
  if (field.family !== "repeated") {
    if (field.family === "attachment" || field.family === "evidence") {
      return attachmentPatchChange(field, sourceValue, effectiveValue, comparison.counters);
    }
    return Object.freeze({
      operations: Object.freeze([Object.freeze({
        kind: "set",
        field: field.id,
        locus: Object.freeze({
          path: field.path,
          segments: Object.freeze([...field.segments]),
        }),
        value: cloneFormValue(effectiveValue),
        valueDigest: stableValueDigest(effectiveValue),
        equality: comparison.counters,
      })]),
      replacementReason: null,
    });
  }
  return diffRepeatedField(field, sourceValue, effectiveValue, comparison.counters);
}

function attachmentPatchChange(field, sourceValue, effectiveValue, equality) {
  if (sourceValue == null && effectiveValue != null) {
    return Object.freeze({
      operations: Object.freeze([Object.freeze({
        kind: "attach",
        field: field.id,
        locus: locusForField(field),
        value: cloneFormValue(effectiveValue),
        valueDigest: stableValueDigest(effectiveValue),
        equality,
      })]),
      replacementReason: null,
    });
  }
  if (sourceValue != null && effectiveValue == null) {
    return Object.freeze({
      operations: Object.freeze([Object.freeze({
        kind: "detach",
        field: field.id,
        locus: locusForField(field),
        equality,
      })]),
      replacementReason: null,
    });
  }
  return Object.freeze({
    operations: Object.freeze([Object.freeze({
      kind: "attach",
      field: field.id,
      locus: locusForField(field),
      value: cloneFormValue(effectiveValue),
      valueDigest: stableValueDigest(effectiveValue),
      equality,
    })]),
    replacementReason: null,
  });
}

function diffRepeatedField(field, sourceValue, effectiveValue, equality) {
  if (!Array.isArray(sourceValue) || !Array.isArray(effectiveValue)) {
    return Object.freeze({
      operations: Object.freeze([]),
      replacementReason: "repeatedReorderRequiresWholeReplace",
    });
  }
  const sourceIds = sourceValue.map((item) => collectionItemId(field, item));
  const effectiveIds = effectiveValue.map((item) => collectionItemId(field, item));
  const sourceMap = new Map(sourceValue.map((item) => [collectionItemId(field, item), item]));
  const effectiveMap = new Map(effectiveValue.map((item) => [collectionItemId(field, item), item]));
  const retainedSourceIds = sourceIds.filter((itemId) => effectiveMap.has(itemId));
  const retainedEffectiveIds = effectiveIds.filter((itemId) => sourceMap.has(itemId));
  if (!sameIdOrder(retainedSourceIds, retainedEffectiveIds)) {
    return Object.freeze({
      operations: Object.freeze([]),
      replacementReason: "repeatedReorderRequiresWholeReplace",
    });
  }
  const removedIds = sourceIds.filter((itemId) => !effectiveMap.has(itemId));
  const insertedIds = effectiveIds.filter((itemId) => !sourceMap.has(itemId));
  const placement = insertionPlacement(retainedEffectiveIds, effectiveIds, insertedIds, field);
  if (insertedIds.length > 0 && placement === null) {
    return Object.freeze({
      operations: Object.freeze([]),
      replacementReason: "repeatedMixedPlacementRequiresWholeReplace",
    });
  }
  const operations = [];
  for (const itemId of removedIds) {
    operations.push(Object.freeze({
      kind: "removeItem",
      field: field.id,
      locus: locusForField(field),
      itemId,
      equality,
    }));
  }
  for (const itemId of retainedSourceIds) {
    const sourceItem = sourceMap.get(itemId);
    const nextItem = effectiveMap.get(itemId);
    const itemComparison = compareSemanticValues(sourceItem, nextItem);
    if (itemComparison.equal) {
      continue;
    }
    operations.push(Object.freeze({
      kind: "replaceItem",
      field: field.id,
      locus: locusForField(field),
      itemId,
      value: cloneFormValue(nextItem),
      valueDigest: stableValueDigest(nextItem),
      equality,
    }));
  }
  if (insertedIds.length > 0) {
    const orderedIds = placement === "prepend" ? [...insertedIds].reverse() : insertedIds;
    for (const itemId of orderedIds) {
      const nextItem = effectiveMap.get(itemId);
      operations.push(Object.freeze({
        kind: "insertItem",
        field: field.id,
        locus: locusForField(field),
        itemId,
        placement,
        value: cloneFormValue(nextItem),
        valueDigest: stableValueDigest(nextItem),
        equality,
      }));
    }
  }
  return Object.freeze({
    operations: Object.freeze(operations),
    replacementReason: null,
  });
}

function insertionPlacement(retainedIds, effectiveIds, insertedIds, field) {
  if (insertedIds.length === 0) {
    return null;
  }
  if (retainedIds.length === 0) {
    return field.resourceLocus?.placement ?? "append";
  }
  const prefixRetained = sameIdOrder(
    effectiveIds.slice(0, retainedIds.length),
    retainedIds,
  );
  const suffixRetained = sameIdOrder(
    effectiveIds.slice(effectiveIds.length - retainedIds.length),
    retainedIds,
  );
  if (suffixRetained && !prefixRetained) {
    return "prepend";
  }
  if (prefixRetained && !suffixRetained) {
    return "append";
  }
  return null;
}

function sameIdOrder(left, right) {
  return left.length === right.length && left.every((value, index) => value === right[index]);
}

function locusForField(field) {
  return Object.freeze({
    path: field.path,
    segments: Object.freeze([...field.segments]),
  });
}

function collectionItemId(field, item) {
  if (field.collectionIdentity.kind === "resolver") {
    return String(field.collectionIdentity.resolver(item));
  }
  return String(readPath(item, [field.collectionIdentity.field]));
}

function compareFieldValues(field, sourceValue, effectiveValue) {
  const comparison = compareSemanticValues(sourceValue, effectiveValue);
  return Object.freeze({
    field: field.id,
    equal: comparison.equal,
    counters: comparison.counters,
  });
}

export function rawInputBlockers(rawInputs) {
  return [...rawInputs.values()]
    .filter((rawInput) => rawInput.committed !== true)
    .map((rawInput) => ({
      kind: "uncommittedRawInput",
      field: rawInput.field,
      reason: "raw input has not crossed a parse/commit boundary",
    }));
}
