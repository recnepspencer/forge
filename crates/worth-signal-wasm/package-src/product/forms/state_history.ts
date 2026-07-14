import { stableValueDigest } from "./values/value_paths.js";

export function createFormStateHistoryStore() {
  let nextArtifactId = 1;
  const history = [];

  return Object.freeze({
    recordRawInput(snapshot, entry) {
      const artifact = createArtifact(nextArtifactId++, "rawInput", snapshot, entry);
      history.push(artifact);
      return artifact;
    },
    recordDraftWrite(snapshot, entry) {
      const artifact = createArtifact(nextArtifactId++, "draftWrite", snapshot, entry);
      history.push(artifact);
      return artifact;
    },
    history() {
      return Object.freeze([...history]);
    },
    counters() {
      return Object.freeze({
        rawInputOperations: history.filter((entry) => entry.entryKind === "rawInput").length,
        fieldWriteOperations: history.filter((entry) => entry.entryKind === "draftWrite").length,
      });
    },
  });
}

export function digestFormStateHistory(history) {
  return stableValueDigest(history.map((entry) => entry.stateHistoryDigest));
}

function createArtifact(artifactId, entryKind, snapshot, entry) {
  const digestInput = {
    kind: "formStateHistory",
    artifactId,
    entryKind,
    field: entry.field,
    operation: entry.operation,
    source: entry.source ?? null,
    reason: entry.reason ?? null,
    rawValueDigest: entry.rawValueDigest ?? null,
    parsedValueDigest: entry.parsedValueDigest ?? null,
    previousDraftDigest: entry.previousDraftDigest,
    nextDraftDigest: snapshot.draftDigest,
    sourceDigest: snapshot.sourceDigest,
    effectiveDigest: snapshot.effectiveDigest,
    dirtyDigest: snapshot.dirtyDigest,
    patchPlanDigest: snapshot.patchPlanDigest,
    readinessDigest: snapshot.readinessDigest,
  };
  return Object.freeze({
    ...digestInput,
    observedAtMs: Date.now(),
    stateHistoryDigest: stableValueDigest(digestInput),
  });
}
