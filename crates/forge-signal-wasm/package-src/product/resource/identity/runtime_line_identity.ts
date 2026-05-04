function createRuntimeLineIdentity(
  familyIdentity,
  canonicalParamIdentity,
  runtimeLineId,
  scopeId,
) {
  return Object.freeze({
    family: familyIdentity,
    canonicalParams: canonicalParamIdentity,
    runtimeLineId,
    scopeId,
  });
}

export { createRuntimeLineIdentity };
