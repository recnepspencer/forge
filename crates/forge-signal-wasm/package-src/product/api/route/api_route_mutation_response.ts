import { createMutationResponseLensProof } from "../../resource/mutation/resource_mutation_response_lens_proof.js";
import { createMutationResponseDeclaration } from "../../resource/mutation/resource_mutation_response_plan.js";
import { requireResourceFamilyMetadata } from "../../resource/families/resource_family_metadata.js";
import { resourcePatch } from "../../resource/reconciliation/resource_patch.js";

const MUTATION_RESPONSE_FALLBACK_KINDS = Object.freeze([
  "refetchRequired",
  "deliveryAwaited",
  "partialReconciliation",
  "unsupportedTarget",
]);

function createApiRouteMutationResponseDeclaration(
  route,
  method,
  response,
  reconciles,
) {
  return createMutationResponseDeclaration({
    source: `api.url("${route}").response(...).${method.toLowerCase()}(...)`,
    lensProof: createMutationResponseLensProof({
      route,
      method,
      source: `api.url("${route}").response(...)`,
      readLensProof: response.lensProof,
    }),
    targets: lowerMutationResponseTargets(route, response, reconciles),
  });
}

function lowerMutationResponseTargets(route, response, reconciles) {
  if (reconciles === undefined) {
    return Object.freeze([]);
  }
  if (!Array.isArray(reconciles)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) requires reconciles to be an array of declared targets`,
    );
  }
  return Object.freeze(
    reconciles.map((target, index) =>
      lowerMutationResponseTarget(route, response, target, index)),
  );
}

function lowerMutationResponseTarget(route, response, target, index) {
  if (!target || typeof target !== "object" || Array.isArray(target)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] must be a target declaration object`,
    );
  }
  if (typeof target.params !== "function") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] requires params(mutationParams)`,
    );
  }
  const familyMetadata = requireResourceFamilyMetadata(
    target.family,
    `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}].family`,
  );
  const fallback = requireMutationResponseFallback(route, target.fallback, index);
  return Object.freeze({
    targetId: `mutationTarget${index + 1}`,
    fallback,
    readTargetLineIdentity: familyMetadata.readTargetLineIdentity,
    lookupResidentTargetMaterialization:
      familyMetadata.lookupResidentTargetMaterialization,
    family: Object.freeze({
      kind: familyMetadata.familyKind,
      familyId: familyMetadata.familyId,
    }),
    params: target.params,
    reconciliation: lowerMutationResponseTargetReconciliation(
      route,
      response,
      familyMetadata,
      target,
      index,
    ),
  });
}

function requireMutationResponseFallback(route, fallback, index) {
  if (!MUTATION_RESPONSE_FALLBACK_KINDS.includes(fallback)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] fallback must be one of ${MUTATION_RESPONSE_FALLBACK_KINDS.join(", ")}`,
    );
  }
  return fallback;
}

function lowerMutationResponseTargetReconciliation(
  route,
  response,
  familyMetadata,
  target,
  index,
) {
  if (target.detail === undefined) {
    return null;
  }
  if (!target.detail || typeof target.detail !== "object" || Array.isArray(target.detail)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail must be a target declaration object`,
    );
  }
  if (response.kind !== "detail") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail exact reconciliation requires a detail response lens`,
    );
  }
  if (familyMetadata.familyKind !== "detail") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail exact reconciliation requires a detail read family`,
    );
  }
  const patchRecord = familyMetadata.patchRecord;
  const detail = target.detail;
  switch (detail.kind) {
    case "replace":
      return Object.freeze({
        kind: "replace",
        targetDigest: "detail:replace",
        createPatch(responseValue) {
          return resourcePatch.replace(responseValue);
        },
      });
    case "field":
      return lowerFieldReconciliation(route, response, patchRecord, detail, index);
    case "jsonPath":
      return lowerJsonPathReconciliation(
        route,
        response,
        patchRecord,
        detail,
        index,
      );
    case "region":
      return lowerRegionReconciliation(route, response, patchRecord, detail, index);
    default:
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] detail kind must be replace, field, jsonPath, or region`,
      );
  }
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
    field: detail.field,
    targetDigest: `detail:field:${detail.field}`,
    createPatch(responseValue) {
      return resourcePatch.field({
        field: detail.field,
        value: responseDefinition.read(responseValue),
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

export { createApiRouteMutationResponseDeclaration };
