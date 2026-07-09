function createLineReloadRecord(
  params,
  familyKind,
  load,
  policy,
  requestState,
  mutationResponseDeclaration = null,
) {
  return Object.freeze({
    params,
    familyKind,
    load,
    policy,
    requestState,
    mutationResponseDeclaration,
  });
}

export { createLineReloadRecord };
