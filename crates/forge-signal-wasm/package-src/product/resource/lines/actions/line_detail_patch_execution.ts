import { areLineValuesSemanticallyEqual } from "../state/line_value_semantic_equality.js";

function applyDetailFieldPatch(materialization, patch, currentValue) {
  const patchRecord = materialization.patch;
  if (patchRecord.familyKind !== "detail") {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit detail field patch(...)`,
    );
  }
  const fieldDefinitions = patchRecord.reconcile?.definitions ?? null;
  const fieldDefinition = readDeclaredDetailDefinition(fieldDefinitions, patch.field);
  if (fieldDefinition === null) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit field patch(...) for undeclared field "${patch.field}"`,
    );
  }
  if (fieldDefinition.jsonPathProof !== undefined) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit field patch(...) for detail JSON path "${patch.field}"; use resourcePatch.jsonPath(...) instead`,
    );
  }
  const nextValue = fieldDefinition.write(currentValue, patch.value);
  const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
  materialization.binding.valueSignal.set(nextValue);
  return Object.freeze({
    result: Object.freeze({
      kind: "narrowed",
      scope: "field",
      itemId: null,
      aspect: null,
      field: patch.field,
    }),
    diagnostics: Object.freeze({
      scope: "field",
      itemId: null,
      aspect: null,
      field: patch.field,
      region: null,
      summary: null,
      valueChanged,
      fieldProof: fieldDefinition.fieldProof,
      regionProof: null,
      jsonPathProof: null,
    }),
    valueChanged,
  });
}

function applyDetailRegionPatch(materialization, patch, currentValue) {
  const patchRecord = materialization.patch;
  if (patchRecord.familyKind !== "detail") {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit detail region patch(...)`,
    );
  }
  const regionDefinitions = patchRecord.reconcile?.definitions ?? null;
  const regionDefinition = readDeclaredDetailDefinition(regionDefinitions, patch.region);
  if (regionDefinition === null) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit region patch(...) for undeclared region "${patch.region}"`,
    );
  }
  if (regionDefinition.regionProof === undefined) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit region patch(...) for non-region detail field "${patch.region}"`,
    );
  }
  const nextValue = regionDefinition.write(currentValue, patch.value);
  const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
  materialization.binding.valueSignal.set(nextValue);
  return Object.freeze({
    result: Object.freeze({
      kind: "narrowed",
      scope: "region",
      itemId: null,
      aspect: null,
      field: null,
      region: patch.region,
    }),
    diagnostics: Object.freeze({
      scope: "region",
      itemId: null,
      aspect: null,
      field: null,
      region: patch.region,
      summary: null,
      valueChanged,
      fieldProof: null,
      regionProof: regionDefinition.regionProof,
      jsonPathProof: null,
    }),
    valueChanged,
  });
}

function applyDetailJsonPathPatch(materialization, patch, currentValue) {
  const patchRecord = materialization.patch;
  if (patchRecord.familyKind !== "detail") {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit detail JSON path patch(...)`,
    );
  }
  const pathDefinitions = patchRecord.reconcile?.definitions ?? null;
  const pathDefinition = readDeclaredDetailDefinition(pathDefinitions, patch.path);
  if (pathDefinition === null) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit jsonPath patch(...) for undeclared path "${patch.path}"`,
    );
  }
  if (pathDefinition.jsonPathProof === undefined) {
    throw new TypeError(
      `${patchRecord.familyKind} resource lines do not admit jsonPath patch(...) for detail field "${patch.path}"; use resourcePatch.field(...) instead`,
    );
  }
  const nextValue = pathDefinition.write(currentValue, patch.value);
  const valueChanged = !areLineValuesSemanticallyEqual(nextValue, currentValue);
  materialization.binding.valueSignal.set(nextValue);
  return Object.freeze({
    result: Object.freeze({
      kind: "narrowed",
      scope: "jsonPath",
      itemId: null,
      aspect: null,
      field: null,
      path: patch.path,
    }),
    diagnostics: Object.freeze({
      scope: "jsonPath",
      itemId: null,
      aspect: null,
      field: null,
      region: null,
      path: patch.path,
      summary: null,
      valueChanged,
      fieldProof: null,
      regionProof: null,
      jsonPathProof: pathDefinition.jsonPathProof,
    }),
    valueChanged,
  });
}

function readDeclaredDetailDefinition(definitions, name) {
  if (
    definitions === null ||
    !Object.prototype.hasOwnProperty.call(definitions, name)
  ) {
    return null;
  }
  return definitions[name];
}

export {
  applyDetailFieldPatch,
  applyDetailJsonPathPatch,
  applyDetailRegionPatch,
};
