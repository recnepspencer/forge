import { stableValueDigest } from "../values/value_paths.js";

export function createResourceMergeStore() {
  let nextArtifactId = 1;
  let current = null;
  const history = [];
  return Object.freeze({
    report(entry) {
      current = Object.freeze({
        artifactId: nextArtifactId++,
        observedAtMs: Date.now(),
        source: "preview",
        ...entry,
      });
      history.push(current);
      return current;
    },
    clear(reason = "resource merge preview was cleared") {
      current = null;
      const seed = {
        kind: "resourceMergePreview",
        artifactId: nextArtifactId++,
        observedAtMs: Date.now(),
        source: "clear",
        sourceKind: "form",
        status: "ready",
        stale: false,
        request: null,
        effectDigest: null,
        sourceBranchId: null,
        targetBranchId: null,
        reason,
        conflictCount: 0,
        projectedFields: Object.freeze([]),
        projectedSections: Object.freeze([]),
        blockers: Object.freeze([]),
        messages: Object.freeze([]),
        proofDigest: null,
      };
      const artifact = Object.freeze({
        ...seed,
        resultDigest: stableValueDigest(seed),
      });
      history.push(artifact);
      return artifact;
    },
    current() {
      return current;
    },
    history() {
      return Object.freeze([...history]);
    },
  });
}
