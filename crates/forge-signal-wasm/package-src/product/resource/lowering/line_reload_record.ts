function createLineReloadRecord(
  params,
  familyKind,
  load,
  policy,
  requestDescriptor,
) {
  return Object.freeze({
    params,
    familyKind,
    load,
    policy,
    requestDescriptor,
  });
}

export { createLineReloadRecord };
