import { requireResourceFamilyMetadata } from "../../families/resource_family_metadata.js";
import {
  createIdentityMigrationDeclarationDigest,
} from "./resource_mutation_response_identity_migration_digests.js";

const MUTATION_RESPONSE_IDENTITY_FALLBACK_KINDS = Object.freeze([
  "identityMigrationUnavailable",
  "refetchRequired",
  "deliveryAwaited",
  "partialReconciliation",
]);
const MUTATION_RESPONSE_IDENTITY_ATOMICITY_KINDS = Object.freeze([
  "allOrNone",
  "partialAllowed",
]);

function lowerMutationResponseIdentityMigration(route, method, response, identity) {
  if (identity === undefined) {
    return null;
  }
  if (method === "DELETE") {
    throw new TypeError(
      `api.url("${route}").response(...).remove(...) identity migration currently admits create/update/save responses only`,
    );
  }
  if (!identity || typeof identity !== "object" || Array.isArray(identity)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity must be a declaration object`,
    );
  }
  if (typeof identity.submitted !== "function") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.submitted(mutationParams) is required`,
    );
  }
  if (typeof identity.canonical !== "function") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.canonical(responseValue, responseIdentity) is required`,
    );
  }
  if (
    identity.atomicity !== undefined
    && !MUTATION_RESPONSE_IDENTITY_ATOMICITY_KINDS.includes(identity.atomicity)
  ) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.atomicity must be one of ${MUTATION_RESPONSE_IDENTITY_ATOMICITY_KINDS.join(", ")}`,
    );
  }
  const targets = lowerMutationResponseIdentityTargets(route, identity.targets);
  return Object.freeze({
    source: `api.url("${route}").response(...).${method.toLowerCase()}(...).identity`,
    route,
    method,
    topology: response.kind,
    submitted: identity.submitted,
    response:
      typeof identity.response === "function" ? identity.response : null,
    canonical: identity.canonical,
    atomicity: identity.atomicity ?? "allOrNone",
    targets,
    declarationDigest: createIdentityMigrationDeclarationDigest(
      route,
      method,
      identity.atomicity ?? "allOrNone",
      targets,
    ),
  });
}

function lowerMutationResponseIdentityTargets(route, targets) {
  if (targets === undefined) {
    return Object.freeze([]);
  }
  if (!Array.isArray(targets)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.targets must be an array of declared migration targets`,
    );
  }
  return Object.freeze(
    targets.map((target, index) =>
      lowerMutationResponseIdentityTarget(route, target, index)),
  );
}

function lowerMutationResponseIdentityTarget(route, target, index) {
  if (!target || typeof target !== "object" || Array.isArray(target)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] must be a target declaration object`,
    );
  }
  if (typeof target.params !== "function") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] requires params(mutationParams)`,
    );
  }
  if (
    target.canonicalParams !== undefined
    && typeof target.canonicalParams !== "function"
  ) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] canonicalParams(requestParams, responseValue, canonicalIdentity, responseIdentity) must be a function when declared`,
    );
  }
  const familyMetadata = requireResourceFamilyMetadata(
    target.family,
    `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}].family`,
  );
  if (!MUTATION_RESPONSE_IDENTITY_FALLBACK_KINDS.includes(target.fallback)) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] fallback must be one of ${MUTATION_RESPONSE_IDENTITY_FALLBACK_KINDS.join(", ")}`,
    );
  }
  const scope = lowerMutationResponseIdentityTargetScope(
    route,
    target,
    index,
    familyMetadata.familyKind,
  );
  return Object.freeze({
    targetId: `migrationTarget${index + 1}`,
    fallback: target.fallback,
    canonicalParams:
      typeof target.canonicalParams === "function" ? target.canonicalParams : null,
    canonicalizeTargetParams: familyMetadata.canonicalizeTargetParams,
    readTargetLineIdentity: familyMetadata.readTargetLineIdentity,
    lookupResidentTargetMaterialization:
      familyMetadata.lookupResidentTargetMaterialization,
    family: Object.freeze({
      kind: familyMetadata.familyKind,
      familyId: familyMetadata.familyId,
    }),
    scope,
    params: target.params,
  });
}

function lowerMutationResponseIdentityTargetScope(
  route,
  target,
  index,
  familyKind,
) {
  const scopeDeclarations = [
    target.summary !== undefined ? "summary" : null,
    target.selection !== undefined ? "selection" : null,
    target.detailChild !== undefined ? "detailChild" : null,
  ].filter(Boolean);
  if (scopeDeclarations.length > 1) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] must declare at most one of summary, selection, or detailChild`,
    );
  }
  if (scopeDeclarations.length === 0) {
    return Object.freeze({ kind: "residentLine" });
  }
  if (target.summary !== undefined) {
    if (familyKind !== "collection" && familyKind !== "paged") {
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] summary targets require a collection or paged family`,
      );
    }
    if (
      !target.summary
      || typeof target.summary !== "object"
      || Array.isArray(target.summary)
      || target.summary.kind !== "summary"
      || typeof target.summary.summary !== "string"
      || target.summary.summary.length === 0
    ) {
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] summary must be { kind: "summary", summary: string }`,
      );
    }
    return Object.freeze({
      kind: "summary",
      summary: target.summary.summary,
    });
  }
  if (target.selection !== undefined) {
    if (
      !target.selection
      || typeof target.selection !== "object"
      || Array.isArray(target.selection)
      || target.selection.kind !== "visibleSelection"
    ) {
      throw new TypeError(
        `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] selection must be { kind: "visibleSelection" }`,
      );
    }
    return Object.freeze({ kind: "visibleSelection" });
  }
  if (familyKind !== "detail") {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] detailChild targets require a detail family`,
    );
  }
  if (target.canonicalParams !== undefined) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] detailChild targets do not admit canonicalParams(...) until detail-child identity rewrite support lands`,
    );
  }
  if (
    !target.detailChild
    || typeof target.detailChild !== "object"
    || Array.isArray(target.detailChild)
    || target.detailChild.kind !== "detailChild"
    || typeof target.detailChild.region !== "string"
    || target.detailChild.region.length === 0
  ) {
    throw new TypeError(
      `api.url("${route}").response(...).create/update/remove(...) identity.targets[${index}] detailChild must be { kind: "detailChild", region: string }`,
    );
  }
  return Object.freeze({
    kind: "detailChild",
    region: target.detailChild.region,
  });
}

export { lowerMutationResponseIdentityMigration };
