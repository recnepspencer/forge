function createRuntimeLineIdentity(
  familyIdentity,
  canonicalParamIdentity,
  runtimeLineId,
  scopeId,
  compatibility,
) {
  return Object.freeze({
    family: familyIdentity,
    canonicalParams: canonicalParamIdentity,
    runtimeLineId,
    scopeId,
    ...(compatibility === null || compatibility === undefined
      ? {}
      : { compatibility }),
  });
}

export { createRuntimeLineIdentity };
