function createUploadDiagnostics(upload) {
  if (upload.kind !== "prepared") {
    return upload;
  }
  return Object.freeze({
    ...upload,
    descriptor: Object.freeze({
      kind: upload.descriptor.kind,
      url: upload.descriptor.url,
      method: upload.descriptor.method,
      headerNames: Object.freeze(
        Object.keys(upload.descriptor.headers).sort(),
      ),
      fieldNames: Object.freeze(
        Object.keys(upload.descriptor.fields).sort(),
      ),
      objectKey: upload.descriptor.objectKey,
      expiresAt: upload.descriptor.expiresAt,
    }),
  });
}

export { createUploadDiagnostics };
