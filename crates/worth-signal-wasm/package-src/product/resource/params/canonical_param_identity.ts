const CANONICAL_PARAM_IDENTITY_BRAND = Symbol(
  "WorthSignal.resourceParamIdentity",
);

function isCanonicalParamIdentity(value) {
  return (
    Boolean(value) &&
    value[CANONICAL_PARAM_IDENTITY_BRAND] === "resourceParamIdentity"
  );
}

function brandCanonicalParamIdentity(canonicalParamSnapshot, canonicalKey) {
  return Object.freeze({
    params: canonicalParamSnapshot,
    canonicalKey,
    [CANONICAL_PARAM_IDENTITY_BRAND]: "resourceParamIdentity",
  });
}

export { brandCanonicalParamIdentity, isCanonicalParamIdentity };
