function createFamilyDefinitionRecord(
  identity,
  declaration,
  familyScope,
  policy,
  baseUrl,
  auth,
  requestContext,
  continuation,
  processingJob,
  uploadTransport,
  requestTarget,
  compatibility,
) {
  return Object.freeze({
    identity,
    declaration,
    familyScope,
    policy,
    baseUrl,
    auth,
    requestContext,
    continuation,
    processingJob,
    uploadTransport,
    requestTarget,
    compatibility,
  });
}

export { createFamilyDefinitionRecord };
