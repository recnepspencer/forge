import { createMutationResponseLensProof } from "../../resource/mutation/resource_mutation_response_lens_proof.js";
import { createMutationResponseDeclaration } from "../../resource/mutation/resource_mutation_response_plan.js";
import { requireResourceFamilyMetadata } from "../../resource/families/resource_family_metadata.js";
import { resourcePatch } from "../../resource/reconciliation/resource_patch.js";
import { lowerMutationResponseDiagnostics } from "./api_route_mutation_response_diagnostics.js";
import {
  lowerMutationResponseIdentityMigration,
} from "../../resource/mutation/identity/resource_mutation_response_identity_migration.js";

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
  diagnostics,
  identity,
) {
  return createMutationResponseDeclaration({
    source: `api.url("${route}").response(...).${method.toLowerCase()}(...)`,
    lensProof: createMutationResponseLensProof({
      route,
      method,
      source: `api.url("${route}").response(...)`,
      readLensProof: response.lensProof,
    }),
    targets: lowerMutationResponseTargets(route, method, response, reconciles),
    diagnostics: lowerMutationResponseDiagnostics(route, method, response, diagnostics),
    identityMigration: lowerMutationResponseIdentityMigration(
      route,
      method,
      response,
      identity,
    ),
  });
}

function lowerMutationResponseTargets(route, method, response, reconciles) {
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
      lowerMutationResponseTarget(route, method, response, target, index)),
  );
}

function lowerMutationResponseTarget(route, method, response, target, index) {
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
      method,
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
  method,
  response,
  familyMetadata,
  target,
  index,
) {
  const declaredReconciliationCount =
    Number(target.detail !== undefined)
    + Number(target.collection !== undefined)
    + Number(target.summary !== undefined);
  if (declaredReconciliationCount === 0) {
    return null;
  }
  if (declaredReconciliationCount > 1) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] declares more than one exact reconciliation target`,
    );
  }
  if (method !== "PUT") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) exact reconciliation targets are currently admitted only for update/save responses`,
    );
  }
  if (target.collection !== undefined) {
    return lowerCollectionItemReconciliation(
      route,
      familyMetadata,
      target.collection,
      index,
    );
  }
  if (target.summary !== undefined) {
    return lowerSummaryReconciliation(
      route,
      response,
      familyMetadata,
      target.summary,
      index,
    );
  }
  return lowerDetailReconciliation(route, response, familyMetadata, target.detail, index);
}

function lowerDetailReconciliation(route, response, familyMetadata, detail, index) {
  if (!detail || typeof detail !== "object" || Array.isArray(detail)) {
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
  switch (detail.kind) {
    case "replace":
      return Object.freeze({
        kind: "replace",
        executionKind: "exactDetail",
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

function lowerCollectionItemReconciliation(route, familyMetadata, collection, index) {
  if (!collection || typeof collection !== "object" || Array.isArray(collection)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection must be a target declaration object`,
    );
  }
  if (collection.kind !== "item") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection kind must be item`,
    );
  }
  if (familyMetadata.familyKind === "detail") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection item reconciliation requires a collection or paged read family`,
    );
  }
  const patchRecord = familyMetadata.patchRecord;
  if (!patchRecord.narrowItem || typeof patchRecord.itemIdentity !== "function") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) reconciles[${index}] collection item reconciliation requires a target family with item reconciliation`,
    );
  }
  return Object.freeze({
    kind: "item",
    executionKind: "exactCollectionItem",
    targetDigest: "collection:item",
    readItemId(responseValue) {
      return patchRecord.itemIdentity(responseValue);
    },
    createPatch(responseValue) {
      const itemId = patchRecord.itemIdentity(responseValue);
      return resourcePatch.item({
        itemId,
        nextItem: responseValue,
      });
    },
  });
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
    createPatch(responseValue) {
      return resourcePatch.summary({
        summary: summary.summary,
        value: responseDefinition === null
          ? responseValue
          : responseDefinition.read(responseValue),
      });
    },
  });
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
    executionKind: "exactDetail",
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
