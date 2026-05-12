import assert from "node:assert/strict";

function createRequestArtifactDigest(line) {
  const request = line.request();
  const diagnostics = line.diagnostics().request;
  const summary = line.diagnosticsSummary().request;
  return JSON.stringify({
    baseUrl: request.baseUrl,
    target: request.target,
    auth: request.auth.kind,
    context: {
      headerNames: diagnostics.context.headerNames,
      correlationId: request.context.correlationId,
      branchId: request.context.branchId,
      basisId: request.context.basisId,
    },
    sources: request.sources,
    continuation: request.continuation,
    processingJob: request.processingJob,
    uploadTransport: request.uploadTransport,
    diagnostics,
    summary,
    historyAvailability: line.history().availability,
  });
}

function assertSecretAbsentFromArtifacts(line, secret) {
  const digestArtifact = createRequestArtifactDigest(line);
  const diagnosticsArtifact = JSON.stringify(line.diagnostics());
  const summaryArtifact = JSON.stringify(line.diagnosticsSummary());
  const historyArtifact = JSON.stringify(line.history());
  assert.equal(digestArtifact.includes(secret), false);
  assert.equal(diagnosticsArtifact.includes(secret), false);
  assert.equal(summaryArtifact.includes(secret), false);
  assert.equal(historyArtifact.includes(secret), false);
}

export { assertSecretAbsentFromArtifacts, createRequestArtifactDigest };
