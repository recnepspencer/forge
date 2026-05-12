import { isPlainObject } from "./value_paths.js";

export function compareSemanticValues(left, right) {
  const counters = semanticEqualityCounterSeed();
  const equal = compareSemanticValuesAtDepth(left, right, counters, 0);
  return Object.freeze({
    equal,
    counters: Object.freeze(counters),
  });
}

export function semanticEqualityCounterSeed() {
  return {
    costBasis: "fieldLocusStructuralCompare",
    valueComparisons: 0,
    objectKeyReads: 0,
    arrayEntries: 0,
    maxDepth: 0,
  };
}

export function aggregateSemanticEqualityCounters(comparisons) {
  const totals = {
    costBasis: "derivedFieldLocusStructuralCompare",
    incrementalStatus: "notIncremental",
    fieldComparisons: comparisons.length,
    deepCollectionFields: 0,
    valueComparisons: 0,
    objectKeyReads: 0,
    arrayEntries: 0,
    maxDepth: 0,
  };
  for (const comparison of comparisons) {
    totals.valueComparisons += comparison.counters.valueComparisons;
    totals.objectKeyReads += comparison.counters.objectKeyReads;
    totals.arrayEntries += comparison.counters.arrayEntries;
    totals.maxDepth = Math.max(totals.maxDepth, comparison.counters.maxDepth);
    if (isDeepCollectionComparison(comparison.counters)) {
      totals.deepCollectionFields += 1;
    }
  }
  return Object.freeze(totals);
}

function compareSemanticValuesAtDepth(left, right, counters, depth) {
  counters.valueComparisons += 1;
  counters.maxDepth = Math.max(counters.maxDepth, depth);
  if (Object.is(left, right)) {
    return true;
  }
  if (Array.isArray(left) || Array.isArray(right)) {
    if (!Array.isArray(left) || !Array.isArray(right) || left.length !== right.length) {
      return false;
    }
    counters.arrayEntries += left.length;
    return left.every((entry, index) => compareSemanticValuesAtDepth(entry, right[index], counters, depth + 1));
  }
  if (isPlainObject(left) || isPlainObject(right)) {
    if (!isPlainObject(left) || !isPlainObject(right)) {
      return false;
    }
    const leftKeys = Object.keys(left);
    const rightKeys = Object.keys(right);
    counters.objectKeyReads += leftKeys.length + rightKeys.length;
    if (leftKeys.length !== rightKeys.length) {
      return false;
    }
    return leftKeys.every((key) => (
      Object.prototype.hasOwnProperty.call(right, key) &&
      compareSemanticValuesAtDepth(left[key], right[key], counters, depth + 1)
    ));
  }
  return false;
}

function isDeepCollectionComparison(counters) {
  return counters.arrayEntries > 32 || counters.objectKeyReads > 64 || counters.maxDepth > 4;
}
