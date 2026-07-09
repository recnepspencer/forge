function createReadyLineUpload(transportKind = "none") {
  return Object.freeze({
    kind: "ready",
    transportKind,
    uploadId: null,
    descriptor: null,
    finalizeRequired: false,
    awaitingProcessing: false,
    message: null,
  });
}

function createPreparedLineUpload(result, transportKind) {
  return Object.freeze({
    kind: "prepared",
    transportKind,
    uploadId: result.uploadId,
    descriptor: result.descriptor,
    finalizeRequired: result.finalizeRequired,
    awaitingProcessing: false,
    message: result.message,
  });
}

function createUploadedLineUpload(result, transportKind) {
  return Object.freeze({
    kind: "uploaded",
    transportKind,
    uploadId: result.uploadId,
    descriptor: null,
    finalizeRequired: result.finalizeRequired,
    awaitingProcessing: result.awaitingProcessing,
    message: result.message,
  });
}

export {
  createPreparedLineUpload,
  createReadyLineUpload,
  createUploadedLineUpload,
};
