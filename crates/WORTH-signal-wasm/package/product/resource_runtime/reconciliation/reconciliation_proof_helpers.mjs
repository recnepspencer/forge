import assert from "node:assert/strict";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

function snapshotPatchMarker(line) {
  const diagnostics = normalizeForProof(line.diagnostics());
  const historyEntry = normalizeForProof(line.history().lifecycle).at(-1);
  return {
    diagnostics: {
      patchCount: diagnostics.patchCount,
      lastPatchKind: diagnostics.lastPatchKind,
      lastPatchScope: diagnostics.lastPatchScope,
      lastPatchedItemId: diagnostics.lastPatchedItemId,
      lastPatchedAspect: diagnostics.lastPatchedAspect,
      lastPatchedSummary: diagnostics.lastPatchedSummary,
      visibleValueVersion: diagnostics.visibleValueVersion,
    },
    historyEntry: {
      event: historyEntry?.event ?? null,
      patchCount: historyEntry?.patchCount ?? null,
      lastPatchKind: historyEntry?.lastPatchKind ?? null,
      lastPatchScope: historyEntry?.lastPatchScope ?? null,
      lastPatchedItemId: historyEntry?.lastPatchedItemId ?? null,
      lastPatchedAspect: historyEntry?.lastPatchedAspect ?? null,
      lastPatchedSummary: historyEntry?.lastPatchedSummary ?? null,
      visibleValueVersion: historyEntry?.visibleValueVersion ?? null,
    },
  };
}

function assertLineStateUnchanged(line, before) {
  assert.deepEqual(normalizeForProof(line.value()), before.value);
  assert.deepEqual(normalizeForProof(line.diagnostics()), before.diagnostics);
  assert.deepEqual(normalizeForProof(line.history().lifecycle), before.history);
}

function captureLineState(line) {
  return {
    value: normalizeForProof(line.value()),
    diagnostics: normalizeForProof(line.diagnostics()),
    history: normalizeForProof(line.history().lifecycle),
  };
}

export {
  assertLineStateUnchanged,
  captureLineState,
  normalizeForProof,
  snapshotPatchMarker,
};
