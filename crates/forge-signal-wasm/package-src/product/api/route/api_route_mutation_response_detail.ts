import { resourcePatch } from "../../resource/reconciliation/resource_patch.js";

function lowerDetailReconciliation(
  route,
  method,
  response,
  familyMetadata,
  detail,
  index,
) {
  if (!detail || typeof detail !== "object" || Array.isArray(detail)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail must be a target declaration object`,
    );
  }
  if (familyMetadata.familyKind !== "detail") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail exact reconciliation requires a detail read family`,
    );
  }
  const patchRecord = familyMetadata.patchRecord;
  switch (detail.kind) {
    case "replace":
      requireDetailResponseLens(route, response, index);
      return Object.freeze({
        kind: "replace",
        executionKind: "exactDetail",
        materializeDeclaredTarget: method === "POST",
        targetDigest: "detail:replace",
        createPatch(responseValue) {
          return resourcePatch.replace(responseValue);
        },
      });
    case "invalidate":
      if (method !== "DELETE") {
        throw new TypeError(
          `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail invalidation is currently admitted only for remove/delete responses`,
        );
      }
      return Object.freeze({
        kind: "invalidate",
        executionKind: "exactDetailInvalidation",
        materializeDeclaredTarget: false,
        targetDigest: "detail:invalidate",
      });
    case "field":
      requireDetailResponseLens(route, response, index);
      return lowerFieldReconciliation(route, response, patchRecord, detail, index);
    case "jsonPath":
      requireDetailResponseLens(route, response, index);
      return lowerJsonPathReconciliation(
        route,
        response,
        patchRecord,
        detail,
        index,
      );
    case "region":
      requireDetailResponseLens(route, response, index);
      return lowerRegionReconciliation(route, response, patchRecord, detail, index);
    default:
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail kind must be replace, invalidate, field, jsonPath, or region`,
      );
  }
}

function lowerSummaryReconciliation(route, response, familyMetadata, summary, index) {
  if (!summary || typeof summary !== "object" || Array.isArray(summary)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] summary must be a target declaration object`,
    );
  }
  if (summary.kind !== "summary") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] summary kind must be summary`,
    );
  }
  if (typeof summary.summary !== "string" || summary.summary.length === 0) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] summary requires a non-empty summary name`,
    );
  }
  const patchRecord = familyMetadata.patchRecord;
  if (!patchRecord.summaryNames.includes(summary.summary)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] summary "${summary.summary}" is not declared on the target family`,
    );
  }
  const responseDefinition = requireSummaryResponseDefinition(
    route,
    response,
    summary.summary,
    index,
  );
  return Object.freeze({
    kind: "summary",
    executionKind: "exactSummary",
    summary: summary.summary,
    summaryScope: patchRecord.reconcile?.summaries?.patchScope ?? null,
    targetDigest: `summary:${summary.summary}`,
    extractPatchValue:
      typeof responseDefinition?.extract === "function"
        ? (responseValue) =>
          normalizeExtractedResponseField(
            summary.summary,
            responseDefinition.extract(responseValue),
          )
        : undefined,
    createPatch(responseValue, _mutationParams, extractedPatchValue) {
      return resourcePatch.summary({
        summary: summary.summary,
        value:
          responseDefinition === null
            ? responseValue
            : extractedPatchValue !== null
              ? extractedPatchValue
              : responseDefinition.read(responseValue),
      });
    },
  });
}

function requireDetailResponseLens(route, response, index) {
  if (response.kind !== "detail") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail exact reconciliation requires a detail response lens`,
    );
  }
}

function requireSummaryResponseDefinition(route, response, summaryName, index) {
  if (response.kind === "summary") {
    return null;
  }
  if (response.kind !== "detail") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] summary exact reconciliation requires a summary response lens or declared detail response field`,
    );
  }
  const responseDefinition = response.fields?.definitions?.[summaryName];
  if (responseDefinition === undefined) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] summary "${summaryName}" is not declared on the mutation response lens`,
    );
  }
  return responseDefinition;
}

function lowerFieldReconciliation(route, response, patchRecord, detail, index) {
  if (typeof detail.field !== "string" || detail.field.length === 0) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail.field requires a non-empty field name`,
    );
  }
  if (!patchRecord.fieldNames.includes(detail.field)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail.field "${detail.field}" is not declared on the target detail family`,
    );
  }
  const responseDefinition = response.fields?.definitions?.[detail.field];
  if (responseDefinition === undefined) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail.field "${detail.field}" is not declared on the mutation response lens`,
    );
  }
  return Object.freeze({
    kind: "field",
    executionKind: "exactDetail",
    materializeDeclaredTarget: false,
    field: detail.field,
    targetDigest: `detail:field:${detail.field}`,
    extractPatchValue(responseValue) {
      return typeof responseDefinition.extract === "function"
        ? normalizeExtractedResponseField(
          detail.field,
          responseDefinition.extract(responseValue),
        )
        : Object.freeze({
          kind: "present",
          value: responseDefinition.read(responseValue),
        });
    },
    createPatch(responseValue, _mutationParams, extractedPatchValue) {
      return resourcePatch.field({
        field: detail.field,
        value:
          extractedPatchValue !== null
            ? extractedPatchValue
            : responseDefinition.read(responseValue),
      });
    },
  });
}

function lowerJsonPathReconciliation(route, response, patchRecord, detail, index) {
  if (typeof detail.path !== "string" || detail.path.length === 0) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail.path requires a non-empty path name`,
    );
  }
  if (!patchRecord.jsonPathNames.includes(detail.path)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail.path "${detail.path}" is not declared on the target detail family`,
    );
  }
  const responseDefinition = response.jsonPaths?.definitions?.[detail.path];
  if (responseDefinition === undefined) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail.path "${detail.path}" is not declared on the mutation response lens`,
    );
  }
  return Object.freeze({
    kind: "jsonPath",
    executionKind: "exactDetail",
    materializeDeclaredTarget: false,
    path: detail.path,
    targetDigest: `detail:jsonPath:${detail.path}`,
    createPatch(responseValue) {
      return resourcePatch.jsonPath({
        path: detail.path,
        value: responseDefinition.read(responseValue),
      });
    },
  });
}

function lowerRegionReconciliation(route, response, patchRecord, detail, index) {
  if (typeof detail.region !== "string" || detail.region.length === 0) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail.region requires a non-empty region name`,
    );
  }
  if (!patchRecord.regionNames.includes(detail.region)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail.region "${detail.region}" is not declared on the target detail family`,
    );
  }
  const responseDefinition = response.regions?.definitions?.[detail.region];
  if (responseDefinition === undefined) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail.region "${detail.region}" is not declared on the mutation response lens`,
    );
  }
  return Object.freeze({
    kind: "region",
    executionKind: "exactDetail",
    materializeDeclaredTarget: false,
    region: detail.region,
    targetDigest: `detail:region:${detail.region}`,
    createPatch(responseValue) {
      return resourcePatch.region({
        region: detail.region,
        value: responseDefinition.read(responseValue),
      });
    },
  });
}

function normalizeExtractedResponseField(field, extraction) {
  if (!extraction || extraction.present !== true) {
    return Object.freeze({
      kind: "missing",
      field,
    });
  }
  return Object.freeze({
    kind: "present",
    field,
    value: extraction.value,
  });
}

export { lowerDetailReconciliation, lowerSummaryReconciliation };
