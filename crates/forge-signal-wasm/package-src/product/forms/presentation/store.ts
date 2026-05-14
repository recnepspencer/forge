import { stableValueDigest } from "../values/value_paths.js";

export function createPresentationStore() {
  let nextArtifactId = 1;
  const history = [];
  const settlements = new Map();
  const externalLanes = new Map();

  return Object.freeze({
    settlementFor(laneId, token) {
      if (token === null) {
        return null;
      }
      return settlements.get(settlementKey(laneId, token)) ?? null;
    },
    externalLane(laneId) {
      return externalLanes.get(laneId) ?? null;
    },
    reportExternalLane(laneId, lane, scope, update) {
      const current = externalLanes.get(laneId) ?? null;
      if (
        current &&
        current.token !== null &&
        update.token !== null &&
        current.token !== update.token &&
        update.supersessionHandoff === "handoff"
      ) {
        history.push(externalLaneArtifact(nextArtifactId++, laneId, lane, scope, {
          status: current.status,
          target: current.target,
          reason: `${laneId} presentation handed off to ${update.token}`,
          token: current.token,
          supersededByToken: update.token,
        }, "handoff"));
      }
      const artifact = externalLaneArtifact(nextArtifactId++, laneId, lane, scope, update, "report");
      externalLanes.set(laneId, artifact);
      history.push(artifact);
      return artifact;
    },
    clearExternalLane(laneId, lane, scope, reason = null) {
      externalLanes.delete(laneId);
      const artifact = externalLaneArtifact(nextArtifactId++, laneId, lane, scope, {
        status: "ready",
        target: null,
        reason: reason ?? `${laneId} presentation was cleared`,
        token: null,
      }, "clear");
      history.push(artifact);
      return artifact;
    },
    acknowledge(lane) {
      return recordSettlement(lane, "acknowledged", lane.reason);
    },
    timeout(lane, reason = null) {
      return recordSettlement(lane, "timedOut", reason ?? `${lane.id} presentation settlement timed out`);
    },
    history() {
      return Object.freeze([...history]);
    },
  });

  function recordSettlement(lane, resultKind, reason) {
    if (lane.token === null || lane.acknowledgement.required !== true) {
      return settlementArtifact(lane, resultKind === "timedOut" ? "noOp" : "ignored", reason);
    }
    const artifact = settlementArtifact(lane, resultKind, reason);
    settlements.set(settlementKey(lane.id, lane.token), artifact);
    history.push(artifact);
    return artifact;
  }

  function settlementArtifact(lane, resultKind, reason) {
    const artifact = {
      kind: "presentationSettlement",
      artifactId: nextArtifactId++,
      observedAtMs: Date.now(),
      laneId: lane.id,
      lane: lane.lane,
      scope: lane.scope,
      token: lane.token,
      resultKind,
      reason,
    };
    return Object.freeze({
      ...artifact,
      settlementDigest: stableValueDigest(artifact),
    });
  }
}

function settlementKey(laneId, token) {
  return `${laneId}::${token}`;
}

function externalLaneArtifact(artifactId, laneId, lane, scope, update, source) {
  const artifact = {
    kind: "presentationLaneUpdate",
    artifactId,
    observedAtMs: Date.now(),
    laneId,
    lane,
    scope,
    status: update.status,
    target: update.target ?? null,
    reason: update.reason,
    token: update.token ?? null,
    supersededByToken: update.supersededByToken ?? null,
    source,
  };
  return Object.freeze({
    ...artifact,
    presentationDigest: stableValueDigest(artifact),
  });
}
