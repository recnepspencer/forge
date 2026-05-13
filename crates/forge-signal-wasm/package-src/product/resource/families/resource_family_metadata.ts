import { requireCanonicalParamIdentity } from "../params/param_identity_factory.js";

const RESOURCE_FAMILY_METADATA = Symbol("forgeSignal.resourceFamilyMetadata");

function attachResourceFamilyMetadata(family, options) {
  const metadata = Object.freeze({
    familyKind: options.familyKind,
    familyId: options.familyId,
    patchRecord: options.patchRecord,
    canonicalizeTargetParams(rawParams) {
      return requireCanonicalParamIdentity(
        options.normalizeParams(rawParams),
        options.familyKind,
      );
    },
    readTargetLineIdentity(rawParams) {
      const canonicalParams = requireCanonicalParamIdentity(
        options.normalizeParams(rawParams),
        options.familyKind,
      );
      return options.lookupTargetLineIdentity(canonicalParams.canonicalKey);
    },
    lookupResidentTargetMaterialization(rawParams) {
      const canonicalParams = requireCanonicalParamIdentity(
        options.normalizeParams(rawParams),
        options.familyKind,
      );
      return options.lookupResidentTargetMaterialization(
        canonicalParams.canonicalKey,
      );
    },
  });
  Object.defineProperty(family, RESOURCE_FAMILY_METADATA, {
    value: metadata,
    enumerable: false,
    configurable: false,
    writable: false,
  });
  return family;
}

function requireResourceFamilyMetadata(value, source) {
  if (
    !value
    || typeof value !== "object"
    || !(RESOURCE_FAMILY_METADATA in value)
  ) {
    throw new TypeError(`${source} requires a resource family created by this runtime`);
  }
  return value[RESOURCE_FAMILY_METADATA];
}

export {
  attachResourceFamilyMetadata,
  requireResourceFamilyMetadata,
};
