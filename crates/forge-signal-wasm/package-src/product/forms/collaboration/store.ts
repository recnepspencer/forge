import { stableValueDigest } from "../values/value_paths.js";

export function createCollaborationStore() {
  let nextArtifactId = 1;
  let current = null;
  const history = [];

  return Object.freeze({
    current() {
      return current;
    },
    report(snapshot) {
      const artifact = collaborationArtifact(nextArtifactId++, snapshot, "report");
      current = artifact;
      history.push(artifact);
      return artifact;
    },
    clear(reason = "collaboration posture was cleared") {
      current = null;
      const artifact = collaborationArtifact(nextArtifactId++, {
        posture: "active",
        reason,
        mode: null,
        actorId: null,
        lockOwnerId: null,
        leasedFields: [],
        branchId: null,
        readOnly: false,
        remoteUpdateDigest: null,
        presence: [],
        comments: [],
      }, "clear");
      history.push(artifact);
      return artifact;
    },
    history() {
      return Object.freeze([...history]);
    },
  });
}

function collaborationArtifact(artifactId, snapshot, source) {
  const artifact = {
    kind: "collaboration",
    artifactId,
    source,
    posture: snapshot.posture,
    reason: snapshot.reason,
    mode: snapshot.mode,
    actorId: snapshot.actorId,
    lockOwnerId: snapshot.lockOwnerId,
    leasedFields: Object.freeze([...snapshot.leasedFields]),
    branchId: snapshot.branchId,
    readOnly: snapshot.readOnly,
    remoteUpdateDigest: snapshot.remoteUpdateDigest,
    presence: Object.freeze([...snapshot.presence]),
    comments: Object.freeze([...snapshot.comments]),
  };
  return Object.freeze({
    ...artifact,
    collaborationDigest: stableValueDigest(artifact),
  });
}
