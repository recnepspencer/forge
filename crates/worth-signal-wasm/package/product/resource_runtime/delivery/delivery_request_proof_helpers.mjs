import assert from "node:assert/strict";

function captureDeliveryRequestState(line) {
  return {
    request: normalizeForProof(line.request()),
    diagnosticsRequest: normalizeForProof(line.diagnostics().request),
    history: normalizeForProof(line.history().lifecycle),
  };
}

function assertDeliveryRequestStateUnchanged(line, before) {
  assert.deepEqual(normalizeForProof(line.request()), before.request);
  assert.deepEqual(normalizeForProof(line.diagnostics().request), before.diagnosticsRequest);
  assert.deepEqual(normalizeForProof(line.history().lifecycle), before.history);
}

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

export {
  assertDeliveryRequestStateUnchanged,
  captureDeliveryRequestState,
};
