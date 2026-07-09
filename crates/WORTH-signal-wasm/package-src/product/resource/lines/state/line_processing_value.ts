function createReadyLineProcessing(completionKind = "none") {
  return Object.freeze({
    kind: "ready",
    completionKind,
    jobId: null,
    message: null,
  });
}

function createAcceptedLineProcessing(job, completionKind) {
  return Object.freeze({
    kind: "accepted",
    completionKind,
    jobId: job.jobId,
    message: job.message,
  });
}

function createInProgressLineProcessing(job, completionKind) {
  return Object.freeze({
    kind: "processing",
    completionKind,
    jobId: job.jobId,
    message: job.message,
  });
}

function createUploadAwaitingLineProcessing(upload, completionKind) {
  return Object.freeze({
    kind: "processing",
    completionKind,
    jobId: upload.uploadId,
    message: upload.message,
  });
}

export {
  createAcceptedLineProcessing,
  createInProgressLineProcessing,
  createReadyLineProcessing,
  createUploadAwaitingLineProcessing,
};
