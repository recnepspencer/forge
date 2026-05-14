import {
  cloneFormValue,
  readPath,
  stableValueDigest,
} from "../values/value_paths.js";
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
  const operations = selectedFields.fields
    .map((field) => patchOperationForField(field, sourceSnapshot, effectiveSnapshot, comparisons))
    .filter(Boolean);
  return Object.freeze({
    semanticDirty: operations.length > 0,
    empty: operations.length === 0,
    operations: Object.freeze(operations),
    blocked: Object.freeze(rawInputBlockers(rawInputs)),
    broadReplacement: false,
    equality: aggregateSemanticEqualityCounters(comparisons),
    breadth: Object.freeze({
      declaredFields: fieldDeclarations.length,
      comparedFields: comparisons.length,
      changedFields: operations.length,
      skippedRawInputFields: selectedFields.counters.skippedRawInputFields,
      omittedFields: selectedFields.counters.omittedFields,
      clearedFields: selectedFields.counters.clearedFields,
      sourceSnapshots: 1,
      effectiveSnapshots: 1,
    }),
    equivalenceDigest: stableValueDigest(
      operations.map((operation) => [operation.field, operation.valueDigest]),
    ),
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

function patchOperationForField(field, sourceSnapshot, effectiveSnapshot, comparisons) {
  const sourceValue = readPath(sourceSnapshot, field.segments);
  const effectiveValue = readPath(effectiveSnapshot, field.segments);
  const comparison = compareFieldValues(field, sourceValue, effectiveValue);
  comparisons.push(comparison);
  if (comparison.equal) {
    return null;
  }
  return Object.freeze({
    kind: "set",
    field: field.id,
    locus: Object.freeze({
      path: field.path,
      segments: Object.freeze([...field.segments]),
    }),
    value: cloneFormValue(effectiveValue),
    valueDigest: stableValueDigest(effectiveValue),
    equality: comparison.counters,
  });
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
