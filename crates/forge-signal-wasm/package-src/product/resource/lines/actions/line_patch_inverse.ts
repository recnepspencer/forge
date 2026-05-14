import {
  createItemScopedInverseDescriptor,
  snapshotCompactInversePreimage,
} from "./line_collection_patch_inverse.js";

function createLinePatchInverseDescriptor(materialization, patch, currentValue) {
  if (patch.kind === "replace") {
    return null;
  }
  if (patch.kind === "field") {
    return createDetailFieldInverseDescriptor(materialization, patch, currentValue);
  }
  if (patch.kind === "region") {
    return createDetailRegionInverseDescriptor(materialization, patch, currentValue);
  }
  if (patch.kind === "jsonPath") {
    return createDetailJsonPathInverseDescriptor(materialization, patch, currentValue);
  }
  if (patch.kind === "summary") {
    return createSummaryInverseDescriptor(materialization, patch, currentValue);
  }
  if (patch.kind === "insert" || patch.kind === "delete") {
    return createCollectionStructuralInverseDescriptor(currentValue);
  }
  return createItemScopedInverseDescriptor(materialization, patch, currentValue);
}

function createCollectionStructuralInverseDescriptor(currentValue) {
  const previousValue = snapshotCompactInversePreimage(() => currentValue);
  if (previousValue.kind === "unavailable") {
    return null;
  }
  return Object.freeze({
    kind: "compactPatchInverse",
    mode: "CompactInversePatch",
    preimage: "responseValue",
    scope: "line",
    itemId: null,
    aspect: null,
    summary: null,
    field: null,
    patch: Object.freeze({
      kind: "replace",
      nextValue: previousValue.value,
    }),
    cost: Object.freeze({
      retainedValueCount: 1,
      retainedResponsePreimage: true,
    }),
  });
}

function createDetailRegionInverseDescriptor(materialization, patch, currentValue) {
  const regionDefinitions = materialization.patch.reconcile?.definitions ?? null;
  if (regionDefinitions === null || !(patch.region in regionDefinitions)) {
    throw new TypeError(
      `${materialization.patch.familyKind} resource lines do not admit region patch(...) for undeclared region "${patch.region}"`,
    );
  }
  const regionDefinition = regionDefinitions[patch.region];
  if (regionDefinition.regionProof === undefined) {
    throw new TypeError(
      `${materialization.patch.familyKind} resource lines do not admit region patch(...) for non-region detail field "${patch.region}"`,
    );
  }
  const previousRegionValue = snapshotCompactInversePreimage(() =>
    regionDefinition.read(currentValue),
  );
  if (previousRegionValue.kind === "unavailable") {
    return null;
  }
  return Object.freeze({
    kind: "compactPatchInverse",
    mode: "CompactInversePatch",
    preimage: "detailRegionValue",
    scope: "region",
    itemId: null,
    aspect: null,
    summary: null,
    field: null,
    region: patch.region,
    patch: Object.freeze({
      kind: "region",
      region: patch.region,
      value: previousRegionValue.value,
    }),
    cost: Object.freeze({
      retainedValueCount: 1,
      retainedResponsePreimage: false,
    }),
  });
}

function createDetailFieldInverseDescriptor(materialization, patch, currentValue) {
  const fieldDefinitions = materialization.patch.reconcile?.definitions ?? null;
  if (fieldDefinitions === null || !(patch.field in fieldDefinitions)) {
    throw new TypeError(
      `${materialization.patch.familyKind} resource lines do not admit field patch(...) for undeclared field "${patch.field}"`,
    );
  }
  const previousFieldValue = snapshotCompactInversePreimage(() =>
    fieldDefinitions[patch.field].read(currentValue),
  );
  if (previousFieldValue.kind === "unavailable") {
    return null;
  }
  return Object.freeze({
    kind: "compactPatchInverse",
    mode: "CompactInversePatch",
    preimage: "detailFieldValue",
    scope: "field",
    itemId: null,
    aspect: null,
    summary: null,
    field: patch.field,
    patch: Object.freeze({
      kind: "field",
      field: patch.field,
      value: previousFieldValue.value,
    }),
    cost: Object.freeze({
      retainedValueCount: 1,
      retainedResponsePreimage: false,
    }),
  });
}

function createDetailJsonPathInverseDescriptor(materialization, patch, currentValue) {
  const pathDefinitions = materialization.patch.reconcile?.definitions ?? null;
  if (pathDefinitions === null || !(patch.path in pathDefinitions)) {
    throw new TypeError(
      `${materialization.patch.familyKind} resource lines do not admit jsonPath patch(...) for undeclared path "${patch.path}"`,
    );
  }
  const pathDefinition = pathDefinitions[patch.path];
  if (pathDefinition.jsonPathProof === undefined) {
    throw new TypeError(
      `${materialization.patch.familyKind} resource lines do not admit jsonPath patch(...) for detail field "${patch.path}"`,
    );
  }
  if (optionalDetailJsonPathTerminalWasAbsent(pathDefinition, currentValue)) {
    return null;
  }
  const previousPathValue = snapshotCompactInversePreimage(() =>
    pathDefinition.read(currentValue),
  );
  if (previousPathValue.kind === "unavailable") {
    return null;
  }
  return Object.freeze({
    kind: "compactPatchInverse",
    mode: "CompactInversePatch",
    preimage: "detailJsonPathValue",
    scope: "jsonPath",
    itemId: null,
    aspect: null,
    summary: null,
    field: null,
    path: patch.path,
    patch: Object.freeze({
      kind: "jsonPath",
      path: patch.path,
      value: previousPathValue.value,
    }),
    cost: Object.freeze({
      retainedValueCount: 1,
      retainedResponsePreimage: false,
    }),
  });
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

function optionalDetailJsonPathTerminalWasAbsent(pathDefinition, currentValue) {
  const jsonPathProof = pathDefinition.jsonPathProof;
  if (jsonPathProof?.policy.absence !== "readAsNull") {
    return false;
  }
  const terminal = jsonPathProof.path.at(-1);
  if (typeof terminal === "number") {
    return false;
  }
  const parentContainer = readDetailJsonPathTerminalParentForInverse(
    currentValue,
    jsonPathProof,
  );
  if (parentContainer === null) {
    return false;
  }
  return Object.getOwnPropertyDescriptor(parentContainer, terminal) === undefined;
}

function readDetailJsonPathTerminalParentForInverse(currentValue, jsonPathProof) {
  if (jsonPathProof.path.length === 1) {
    return isJsonPathInverseContainer(currentValue, jsonPathProof.path[0])
      ? currentValue
      : null;
  }
  let current = currentValue;
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

export { createLinePatchInverseDescriptor };
