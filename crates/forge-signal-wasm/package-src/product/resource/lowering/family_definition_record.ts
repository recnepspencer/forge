function createFamilyDefinitionRecord(
  identity,
  declaration,
  familyScope,
  policy,
  method,
  requestBody,
  baseUrl,
  auth,
  requestContext,
  continuation,
  processingJob,
  uploadTransport,
  effects,
  requestTarget,
  compatibility,
) {
  return Object.freeze({
    identity,
    declaration,
    familyScope,
    policy,
    method,
    requestBody,
    baseUrl,
    auth,
    requestContext,
    continuation,
    processingJob,
    uploadTransport,
    effects,
    requestTarget,
    compatibility,
  });
}

export { createFamilyDefinitionRecord };
