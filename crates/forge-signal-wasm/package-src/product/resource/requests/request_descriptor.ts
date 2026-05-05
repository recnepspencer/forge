function createResourceRequestDescriptor(
  lineIdentity,
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
    auth,
    context,
    continuation,
    processingJob,
    uploadTransport,
    sources,
  });
}

export { createResourceRequestDescriptor };
