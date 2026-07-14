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
import { RESOURCE_RESPONSE_TOPOLOGY_COSTS } from "../../resource/response/resource_response_topology_costs.js";

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
  semanticFinalizer,
  authoringSurface,
  response,
  reconciles,
  atomicity,
  diagnostics,
  identity,
) {
  const mutationSource = readMutationResponseSource(route, authoringSurface);
  const loweredTargets = lowerMutationResponseTargets(
    route,
    authoringSurface,
    semanticFinalizer,
    response,
    reconciles,
  );
  const loweredDiagnostics = lowerMutationResponseDiagnostics(
    route,
    semanticFinalizer,
    response,
    diagnostics,
  );
  return createMutationResponseDeclaration({
    source: mutationSource,
    lensProof: createMutationResponseLensProof({
      route,
      method,
      source: mutationSource,
      readLensProof: response.lensProof,
    }),
    responseMappedFieldNames: readMutationResponseMappedFieldNames(
      response,
      loweredTargets,
      loweredDiagnostics,
    ),
    reconciliationAtomicity: lowerMutationResponseAtomicity(route, atomicity),
    targets: loweredTargets,
    diagnostics: loweredDiagnostics,
    identityMigration: lowerMutationResponseIdentityMigration(
      route,
      method,
      semanticFinalizer,
      response,
      identity,
    ),
  });
}

function lowerMutationResponseTargets(route, authoringSurface, semanticFinalizer, response, reconciles) {
  if (reconciles === undefined) {
    return Object.freeze([]);
  }
  if (!Array.isArray(reconciles)) {
    throw new TypeError(
      `${readMutationResponseSource(route, authoringSurface)} requires reconciles to be an array of declared targets`,
    );
  }
  return Object.freeze(
    reconciles.map((target, index) =>
      lowerMutationResponseTarget(route, authoringSurface, semanticFinalizer, response, target, index)),
  );
}

function lowerMutationResponseTarget(route, authoringSurface, semanticFinalizer, response, target, index) {
  const mutationSource = readMutationResponseSource(route, authoringSurface);
  if (!target || typeof target !== "object" || Array.isArray(target)) {
    throw new TypeError(
      `${mutationSource} reconciles[${index}] must be a target declaration object`,
    );
  }
  if (typeof target.params !== "function") {
    throw new TypeError(
      `${mutationSource} reconciles[${index}] requires params(mutationParams)`,
    );
  }
  const familyMetadata = requireResourceFamilyMetadata(
    target.family,
    `${mutationSource} reconciles[${index}].family`,
  );
  const reconciliation = lowerMutationResponseTargetReconciliation(
    route,
    authoringSurface,
    semanticFinalizer,
    response,
    familyMetadata,
    target,
    index,
  );
  return Object.freeze({
    targetId: `mutationTarget${index + 1}`,
    fallback: requireMutationResponseFallback(route, authoringSurface, target.fallback, index),
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
    cost: readMutationResponseTargetCost(familyMetadata.patchRecord, reconciliation),
    reconciliation,
  });
}

function requireMutationResponseFallback(route, authoringSurface, fallback, index) {
  if (!MUTATION_RESPONSE_FALLBACK_KINDS.includes(fallback)) {
    throw new TypeError(
      `${readMutationResponseSource(route, authoringSurface)} reconciles[${index}] fallback must be one of ${MUTATION_RESPONSE_FALLBACK_KINDS.join(", ")}`,
    );
  }
  return fallback;
}

function lowerMutationResponseTargetReconciliation(
  route,
  authoringSurface,
  semanticFinalizer,
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
      `${readMutationResponseSource(route, authoringSurface)} reconciles[${index}] declares more than one exact reconciliation target`,
    );
  }
  if (target.collection !== undefined) {
    return lowerCollectionReconciliation(
      route,
      semanticFinalizer,
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
    semanticFinalizer,
    response,
    familyMetadata,
    target.detail,
    index,
  );
}

function readMutationResponseMappedFieldNames(response, targets, diagnostics) {
  if (response.kind !== "detail" || response.fields === null) {
    return null;
  }
  const mappedFieldNames = new Set();
  for (const target of targets) {
    const reconciliation = target.reconciliation;
    if (reconciliation === null) {
      continue;
    }
    if (reconciliation.kind === "field") {
      mappedFieldNames.add(reconciliation.field);
      continue;
    }
    if (reconciliation.kind === "summary") {
      mappedFieldNames.add(reconciliation.summary);
    }
  }
  for (const diagnostic of diagnostics) {
    mappedFieldNames.add(diagnostic.field);
  }
  return Object.freeze([...mappedFieldNames].sort());
}

function readMutationResponseTargetCost(patchRecord, reconciliation) {
  if (reconciliation === null) {
    return Object.freeze({
      topologyTraversalBreadth: 0,
      reconstructionBreadth: 0,
    });
  }
  if (reconciliation.kind === "field") {
    return readDetailDefinitionCost(
      patchRecord.reconcile?.definitions?.[reconciliation.field]?.fieldProof?.cost ?? null,
    );
  }
  if (reconciliation.kind === "region") {
    return readDetailDefinitionCost(
      patchRecord.reconcile?.definitions?.[reconciliation.region]?.regionProof?.cost ?? null,
    );
  }
  if (reconciliation.kind === "jsonPath") {
    return readDetailDefinitionCost(
      patchRecord.reconcile?.definitions?.[reconciliation.path]?.jsonPathProof?.cost ?? null,
    );
  }
  if (reconciliation.kind === "summary") {
    return Object.freeze({
      topologyTraversalBreadth: reconciliation.summaryScope === "pageWindow" ? 1 : 0,
      reconstructionBreadth: 1,
    });
  }
  if (reconciliation.kind === "replace" || reconciliation.kind === "invalidate") {
    return Object.freeze({
      topologyTraversalBreadth: 0,
      reconstructionBreadth: 1,
    });
  }
  const topology = patchRecord.responseLensProof?.topology ?? null;
  const topologyCosts =
    topology === null ? null : RESOURCE_RESPONSE_TOPOLOGY_COSTS[topology] ?? null;
  const declaredCost =
    reconciliation.kind === "delete"
      ? topologyCosts?.itemDelete ?? topologyCosts?.item ?? null
      : reconciliation.kind === "insert"
        ? topologyCosts?.itemInsert ?? topologyCosts?.item ?? null
        : topologyCosts?.item ?? null;
  return Object.freeze({
    topologyTraversalBreadth: declaredCost?.[1] ?? 1,
    reconstructionBreadth: 1,
  });
}

function readDetailDefinitionCost(cost) {
  if (cost === null) {
    return Object.freeze({
      topologyTraversalBreadth: 1,
      reconstructionBreadth: 1,
    });
  }
  return Object.freeze({
    topologyTraversalBreadth: cost.traversalBreadth,
    reconstructionBreadth: cost.reconstructionBreadth,
  });
}

function readMutationResponseSource(route, authoringSurface) {
  return `api.url("${route}").response(...).${authoringSurface}(...)`;
}

export { createApiRouteMutationResponseDeclaration };
