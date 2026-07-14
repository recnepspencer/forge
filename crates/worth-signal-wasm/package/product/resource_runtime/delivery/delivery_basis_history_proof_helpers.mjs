import assert from "node:assert/strict";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

function projectBasisProof(line) {
  const history = line.history();
  const summary = line.diagnosticsSummary();
  const diagnostics = line.diagnostics();
  return {
    requestBasisId: line.request().context.basisId,
    diagnosticsBasis: normalizeForProof(diagnostics.basis),
    summaryBasis: {
      count: summary.counts.basisAdvanceCount,
      currentBasisId: summary.latest.basisCurrentId,
      fromBasisId: summary.latest.basisAdvanceFromId,
      toBasisId: summary.latest.basisAdvanceToId,
    },
    historyBasis: normalizeForProof(history.basis),
    lifecycleBasis: normalizeForProof(
      history.lifecycle.map((entry) => ({
        sequence: entry.sequence,
        event: entry.event,
        currentBasisId: entry.currentBasisId,
        basisAdvanceCount: entry.basisAdvanceCount,
        lastBasisAdvanceFromId: entry.lastBasisAdvanceFromId,
        lastBasisAdvanceToId: entry.lastBasisAdvanceToId,
      })),
    ),
    availability: normalizeForProof(history.availability),
    branch: normalizeForProof(history.branch),
    replay: normalizeForProof(history.replay),
  };
}

function assertBasisProofUnchanged(line, before) {
  assert.deepEqual(projectBasisProof(line), before);
}

export {
  assertBasisProofUnchanged,
  normalizeForProof,
  projectBasisProof,
};
