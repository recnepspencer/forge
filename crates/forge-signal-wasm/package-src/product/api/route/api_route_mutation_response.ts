import { createMutationResponseLensProof } from "../../resource/mutation/resource_mutation_response_lens_proof.js";
import { createMutationResponseDeclaration } from "../../resource/mutation/resource_mutation_response_plan.js";
import { requireResourceFamilyMetadata } from "../../resource/families/resource_family_metadata.js";
import { lowerMutationResponseDiagnostics } from "./api_route_mutation_response_diagnostics.js";
import { lowerMutationResponseAtomicity } from "./api_route_mutation_response_atomicity.js";
import { lowerMutationResponseIdentityMigration } from "../../resource/mutation/identity/resource_mutation_response_identity_migration.js";
import { lowerCollectionReconciliation } from "./api_route_mutation_response_collection.js";
import {
  lowerDetailReconciliation,
  lowerSummaryReconciliation,
} from "./api_route_mutation_response_detail.js";

const MUTATION_RESPONSE_FALLBACK_KINDS = Object.freeze([
  "deletionUnavailable",
  "placementUnavailable",
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
  atomicity,
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
    reconciliationAtomicity: lowerMutationResponseAtomicity(route, atomicity),
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
  return Object.freeze({
    targetId: `mutationTarget${index + 1}`,
    fallback: requireMutationResponseFallback(route, target.fallback, index),
    readTargetLineIdentity: familyMetadata.readTargetLineIdentity,
    lookupResidentTargetMaterialization:
      familyMetadata.lookupResidentTargetMaterialization,
    materializeTargetMaterialization:
      familyMetadata.materializeTargetMaterialization,
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
  if (target.collection !== undefined) {
    return lowerCollectionReconciliation(
      route,
      method,
      response,
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
  return lowerDetailReconciliation(
    route,
    method,
    response,
    familyMetadata,
    target.detail,
    index,
  );
}

export { createApiRouteMutationResponseDeclaration };
