function createFamilyDefinitionRecord(
  identity,
  declaration,
  familyScope,
  policy,
  auth,
  requestContext,
  continuation,
  processingJob,
  uploadTransport,
  compatibility,
) {
  return Object.freeze({
    identity,
    declaration,
    familyScope,
    policy,
    auth,
    requestContext,
    continuation,
    processingJob,
    uploadTransport,
    compatibility,
  });
}

export { createFamilyDefinitionRecord };
