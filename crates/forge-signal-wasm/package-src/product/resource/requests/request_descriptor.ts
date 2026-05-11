function createResourceRequestDescriptor(
  lineIdentity,
  target,
  baseUrl,
  method,
  body,
  auth,
  context,
  continuation,
  processingJob,
  uploadTransport,
  effects,
  sources,
) {
  return Object.freeze({
    family: lineIdentity.family,
    canonicalParams: lineIdentity.canonicalParams,
    target,
    baseUrl,
    method,
    body,
    auth,
    context,
    continuation,
    processingJob,
    uploadTransport,
    effects,
    sources,
  });
}

export { createResourceRequestDescriptor };
