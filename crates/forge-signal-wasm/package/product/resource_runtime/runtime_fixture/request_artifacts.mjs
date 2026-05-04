import assert from "node:assert/strict";

function createRequestArtifactDigest(line) {
  return JSON.stringify({
    auth: line.request().auth.kind,
    context: {
      headers: line.request().context.headers,
      correlationId: line.request().context.correlationId,
      branchId: line.request().context.branchId,
      basisId: line.request().context.basisId,
    },
    continuation: line.request().continuation,
    diagnostics: line.diagnostics().request,
    summary: line.diagnosticsSummary().request,
    historyAvailability: line.history().availability,
  });
}

function assertSecretAbsentFromArtifacts(line, secret) {
  const diagnosticsArtifact = JSON.stringify(line.diagnostics());
  const summaryArtifact = JSON.stringify(line.diagnosticsSummary());
  const historyArtifact = JSON.stringify(line.history());
  assert.equal(diagnosticsArtifact.includes(secret), false);
  assert.equal(summaryArtifact.includes(secret), false);
  assert.equal(historyArtifact.includes(secret), false);
}

export { assertSecretAbsentFromArtifacts, createRequestArtifactDigest };
