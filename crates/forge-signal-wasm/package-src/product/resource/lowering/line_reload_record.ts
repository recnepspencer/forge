function createLineReloadRecord(
  params,
  familyKind,
  load,
  policy,
  requestState,
) {
  return Object.freeze({
    params,
    familyKind,
    load,
    policy,
    requestState,
  });
}

export { createLineReloadRecord };
