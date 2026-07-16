import {
  invalidateAllFamilyLines,
  invalidateFamilyLine,
} from "./family_invalidation.js";
import { createFamilyIdentity } from "../../identity/family_identity.js";
import { createFamilyDefinitionRecord } from "../../lowering/family_definition_record.js";
import { createLinePatchRecord } from "../../lowering/line_patch_record.js";
import { requireCanonicalParamIdentity } from "../../params/param_identity_factory.js";
import { resolveResourcePolicyProfile } from "../../policies/policy_profile_resolution.js";
import {
  createRequestTargetRecord,
} from "./resolved_request_descriptor.js";
import { attachResourceFamilyMetadata } from "../resource_family_metadata.js";
import { createMaterializedLine } from "./materialized_line_entry.js";
import { createSeededBinding } from "./materialized_family_binding.js";

function createMaterializedFamily(
  kind,
  signalNamespace,
  resourceLineEpoch,
  familyId,
  declaration,
  compatibility,
  effectProjectionCoordinator,
) {
  const familyScope = signalNamespace.scope(familyId);
  const familyIdentity = createFamilyIdentity(kind, familyId);
  const policy = resolveResourcePolicyProfile(declaration.policy, kind);
  const familyRecord = createFamilyDefinitionRecord(
    familyIdentity,
    declaration,
    familyScope,
    policy,
    declaration.method,
    declaration.requestBody,
    declaration.baseUrl,
    declaration.auth,
    declaration.requestContext,
    declaration.continuation,
    declaration.processingJob,
    declaration.uploadTransport,
    declaration.effects,
    createRequestTargetRecord(declaration),
    compatibility,
  );
  const familyPatchRecord = createLinePatchRecord(
    familyRecord.identity.kind,
    familyRecord.declaration.itemIdentity,
    createLineReconciliationProofRecord(familyRecord.declaration),
  );
  const linesByCanonicalKey = new Map();
  let lineCounter = 0;

  function lookupOrCreateRegistryEntry(canonicalParamIdentity, options = null) {
    const existing = linesByCanonicalKey.get(canonicalParamIdentity.canonicalKey);
    if (existing) {
      return existing;
    }
    const created = createMaterializedLine(
      canonicalParamIdentity,
      linesByCanonicalKey,
      familyIdentity,
      familyRecord,
      familyPatchRecord,
      familyScope,
      resourceLineEpoch,
      () => {
        lineCounter += 1;
        return lineCounter;
      },
      options?.initialBindingFactory ?? null,
      effectProjectionCoordinator,
    );
    linesByCanonicalKey.set(canonicalParamIdentity.canonicalKey, created);
    return created;
  }

  function createLine(rawParams) {
    const canonicalParamIdentity = requireCanonicalParamIdentity(
      familyRecord.declaration.normalizeParams(rawParams),
      kind,
    );
    return lookupOrCreateRegistryEntry(canonicalParamIdentity).handle;
  }

  function createOptionalLine(selection) {
    if (
      selection == null
      || (
        typeof selection === "object"
        && Object.keys(selection).length === 1
        && "enabled" in selection
        && selection.enabled === false
      )
    ) {
      return null;
    }
    return createLine(selection);
  }

  const family = {
    invalidate(rawParams) {
      const canonicalParamIdentity = requireCanonicalParamIdentity(
        familyRecord.declaration.normalizeParams(rawParams),
        kind,
      );
      return invalidateFamilyLine(
        linesByCanonicalKey,
        canonicalParamIdentity.canonicalKey,
      );
    },
    invalidateAll() {
      return invalidateAllFamilyLines(linesByCanonicalKey);
    },
    line: createLine,
    optionalLine: createOptionalLine,
    execute(rawParams, options) {
      return createLine(rawParams).execute(options);
    },
  };
  return Object.freeze(
    attachResourceFamilyMetadata(family, {
      familyKind: kind,
      familyId,
      patchRecord: familyPatchRecord,
      normalizeParams: familyRecord.declaration.normalizeParams,
      lookupTargetLineIdentity(canonicalKey) {
        const existing = linesByCanonicalKey.get(canonicalKey);
        if (!existing) {
          return Object.freeze({
            canonicalKey,
            runtimeLineId: null,
            residency: "declared",
          });
        }
        return Object.freeze({
          canonicalKey,
          runtimeLineId: existing.materialization.lineIdentity.runtimeLineId,
          residency: "resident",
        });
      },
      lookupResidentTargetMaterialization(canonicalKey) {
        return linesByCanonicalKey.get(canonicalKey)?.materialization ?? null;
      },
      materializeTargetMaterialization(rawParams, seedValue) {
        const canonicalParamIdentity = requireCanonicalParamIdentity(
          familyRecord.declaration.normalizeParams(rawParams),
          kind,
        );
        return lookupOrCreateRegistryEntry(
          canonicalParamIdentity,
          seedValue === undefined
            ? null
            : {
                initialBindingFactory(
                  params,
                  lineScope,
                  familyKind,
                  _load,
                  policy,
                  requestDescriptor,
                  _lineIdentity,
                  _mutationResponseDeclaration,
                  lifecycle,
                  lifecycleHistory,
                ) {
                  return createSeededBinding(
                    params,
                    lineScope,
                    familyKind,
                    seedValue,
                    policy,
                    requestDescriptor,
                    lifecycle,
                    lifecycleHistory,
                  );
                },
              },
        ).materialization;
      },
    }),
  );
}

function createLineReconciliationProofRecord(declaration) {
  if (declaration.reconcile !== undefined) {
    return declaration.reconcile;
  }
  if (declaration.responseLensProof !== undefined && declaration.detailFields !== undefined) {
    return Object.freeze({
      ...declaration.detailFields,
      responseLensProof: declaration.responseLensProof,
    });
  }
  if (declaration.responseLensProof !== undefined && declaration.detailRegions !== undefined) {
    return Object.freeze({
      ...declaration.detailRegions,
      responseLensProof: declaration.responseLensProof,
    });
  }
  if (declaration.responseLensProof !== undefined && declaration.detailJsonPaths !== undefined) {
    return Object.freeze({
      ...declaration.detailJsonPaths,
      responseLensProof: declaration.responseLensProof,
    });
  }
  if (declaration.responseLensProof !== undefined) {
    return Object.freeze({
      responseLensProof: declaration.responseLensProof,
    });
  }
  return null;
}

export { createMaterializedFamily };
