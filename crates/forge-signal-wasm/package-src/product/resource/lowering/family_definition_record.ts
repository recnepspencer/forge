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
  });
}

export { createFamilyDefinitionRecord };
