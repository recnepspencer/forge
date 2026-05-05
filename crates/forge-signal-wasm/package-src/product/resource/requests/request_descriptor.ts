function createResourceRequestDescriptor(
  lineIdentity,
  target,
  baseUrl,
  auth,
  context,
  continuation,
  processingJob,
  uploadTransport,
  sources,
) {
  return Object.freeze({
    family: lineIdentity.family,
    canonicalParams: lineIdentity.canonicalParams,
    target,
    baseUrl,
    auth,
    context,
    continuation,
    processingJob,
    uploadTransport,
    sources,
  });
}

export { createResourceRequestDescriptor };
