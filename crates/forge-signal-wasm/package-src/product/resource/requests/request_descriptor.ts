function createResourceRequestDescriptor(
  lineIdentity,
  auth,
  context,
  continuation,
  processingJob,
  uploadTransport,
) {
  return Object.freeze({
    family: lineIdentity.family,
    canonicalParams: lineIdentity.canonicalParams,
    auth,
    context,
    continuation,
    processingJob,
    uploadTransport,
  });
}

export { createResourceRequestDescriptor };
