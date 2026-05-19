import { stableValueDigest } from "../values/value_paths.js";

export function deriveCollaborationEvents(history) {
  const events = [];
  let previous = emptySnapshot();
  for (const artifact of history) {
    pushEvent(events, "postureChange", previous, artifact, previous.posture !== artifact.posture);
    pushEvent(events, "lockChange", previous, artifact, previous.lockOwnerId !== artifact.lockOwnerId);
    pushEvent(
      events,
      "leaseChange",
      previous,
      artifact,
      stableValueDigest(previous.leasedFields) !== stableValueDigest(artifact.leasedFields),
    );
    pushEvent(events, "branchChange", previous, artifact, previous.branchId !== artifact.branchId);
    pushEvent(events, "readOnlyChange", previous, artifact, previous.readOnly !== artifact.readOnly);
    pushEvent(
      events,
      "remoteUpdateChange",
      previous,
      artifact,
      previous.remoteUpdateDigest !== artifact.remoteUpdateDigest,
    );
    pushEvent(
      events,
      "presenceChange",
      previous,
      artifact,
      stableValueDigest(previous.presence) !== stableValueDigest(artifact.presence),
    );
    pushEvent(
      events,
      "commentChange",
      previous,
      artifact,
      stableValueDigest(previous.comments) !== stableValueDigest(artifact.comments),
    );
    previous = artifact;
  }
  return Object.freeze(events);
}

function pushEvent(events, kind, previous, artifact, changed) {
  if (!changed) {
    return;
  }
  const event = {
    kind,
    artifactId: artifact.artifactId,
    source: artifact.source,
    previousArtifactId: previous.artifactId,
    mode: artifact.mode,
    posture: artifact.posture,
    reason: artifact.reason,
    lockOwnerId: artifact.lockOwnerId,
    leasedFields: Object.freeze([...artifact.leasedFields]),
    branchId: artifact.branchId,
    readOnly: artifact.readOnly,
    remoteUpdateDigest: artifact.remoteUpdateDigest,
    presence: Object.freeze([...artifact.presence]),
    comments: Object.freeze([...artifact.comments]),
    previousDigest: previous.collaborationDigest,
    nextDigest: artifact.collaborationDigest,
  };
  events.push(Object.freeze({
    ...event,
    digest: stableValueDigest(event),
  }));
}

function emptySnapshot() {
  const snapshot = {
    artifactId: null,
    collaborationDigest: null,
    posture: "active",
    reason: "no collaboration posture has been reported",
    mode: null,
    actorId: null,
    lockOwnerId: null,
    leasedFields: Object.freeze([]),
    branchId: null,
    readOnly: false,
    remoteUpdateDigest: null,
    presence: Object.freeze([]),
    comments: Object.freeze([]),
    source: "clear",
  };
  return Object.freeze(snapshot);
}
