import { FormDeclarationError } from "../form_errors.js";
import { stableValueDigest } from "../values/value_paths.js";

export function createLayoutMeasurementStore(policy) {
  let nextSnapshotId = 1;
  let coalescedWrites = 0;
  const snapshots = [];

  return Object.freeze({
    record(snapshotContext, rows, options = {}) {
      const cause = options.cause ?? "animationFrame";
      if (!policy.observe.includes(cause)) {
        throw new FormDeclarationError("layout measurement cause is not declared for this form", {
          cause,
        });
      }
      const normalizedRows = normalizeRows(rows, snapshotContext.layout.rows);
      const snapshot = nextSnapshotArtifact(snapshotContext, normalizedRows, cause, options.frameToken ?? null);
      const previous = snapshots.at(-1);
      if (previous && previous.frameToken !== null && previous.frameToken === snapshot.frameToken) {
        const merged = coalesceSnapshots(previous, snapshot);
        snapshots[snapshots.length - 1] = merged;
        coalescedWrites += 1;
        return merged;
      }
      snapshots.push(snapshot);
      if (snapshots.length > policy.maxRetainedSnapshots) {
        snapshots.splice(0, snapshots.length - policy.maxRetainedSnapshots);
      }
      return snapshot;
    },
    report() {
      const latestSnapshot = snapshots.at(-1) ?? null;
      const report = {
        posture: "supported",
        policy: Object.freeze({
          observe: policy.observe,
          batching: policy.batching,
          maxRetainedSnapshots: policy.maxRetainedSnapshots,
        }),
        latestSnapshot,
        snapshots: Object.freeze([...snapshots]),
        counters: Object.freeze({
          costBasis: "imperativeLayoutMeasurementEventStream",
          incrementalStatus: "frameCoalesced",
          retainedSnapshots: snapshots.length,
          coalescedWrites,
          observedCauseCount: policy.observe.length,
          measuredRows: snapshots.reduce((total, snapshot) => total + snapshot.rows.length, 0),
        }),
      };
      return Object.freeze({
        ...report,
        digest: stableValueDigest({
          policy: report.policy,
          latestSnapshot,
          snapshots,
          counters: report.counters,
        }),
      });
    },
  });

  function nextSnapshotArtifact(snapshotContext, rows, cause, frameToken) {
    const artifact = {
      kind: "layoutSnapshot",
      snapshotId: nextSnapshotId++,
      frameToken,
      causes: Object.freeze([cause]),
      rows,
      layoutDigest: snapshotContext.layout.digest,
      accessibilityDigest: snapshotContext.accessibility.digest,
      hostDigest: snapshotContext.host.digest,
      semanticDigests: snapshotContext.semanticDigests,
    };
    return Object.freeze({
      ...artifact,
      snapshotDigest: stableValueDigest(artifact),
    });
  }
}

function normalizeRows(rows, declaredRows) {
  if (!Array.isArray(rows)) {
    throw new FormDeclarationError("layout measurement rows must be an array", {
      rows,
    });
  }
  const allowedRows = new Set(declaredRows.map((row) => row.id));
  return Object.freeze(rows.map((row) => normalizeRow(row, allowedRows)));
}

function normalizeRow(row, allowedRows) {
  if (!row || typeof row !== "object" || Array.isArray(row)) {
    throw new FormDeclarationError("layout measurement row must be an object", {
      row,
    });
  }
  const rowId = requireNonEmptyString(row.row, "layout measurement row");
  if (!allowedRows.has(rowId)) {
    throw new FormDeclarationError("layout measurement row must reference a declared layout row", {
      row: rowId,
    });
  }
  return Object.freeze({
    row: rowId,
    labelHeight: finiteOrNull(row.labelHeight, "labelHeight"),
    controlHeight: finiteOrNull(row.controlHeight, "controlHeight"),
    helpHeight: finiteOrNull(row.helpHeight, "helpHeight"),
    messageHeight: finiteOrNull(row.messageHeight, "messageHeight"),
  });
}

function coalesceSnapshots(previous, nextSnapshot) {
  const rowsById = new Map(previous.rows.map((row) => [row.row, row]));
  for (const row of nextSnapshot.rows) {
    rowsById.set(row.row, row);
  }
  const merged = {
    ...nextSnapshot,
    snapshotId: previous.snapshotId,
    causes: Object.freeze([...new Set([...previous.causes, ...nextSnapshot.causes])]),
    rows: Object.freeze([...rowsById.values()]),
  };
  return Object.freeze({
    ...merged,
    snapshotDigest: stableValueDigest({
      kind: merged.kind,
      snapshotId: merged.snapshotId,
      frameToken: merged.frameToken,
      causes: merged.causes,
      rows: merged.rows,
      layoutDigest: merged.layoutDigest,
      accessibilityDigest: merged.accessibilityDigest,
      hostDigest: merged.hostDigest,
      semanticDigests: merged.semanticDigests,
    }),
  });
}

function requireNonEmptyString(value, label) {
  if (typeof value !== "string" || value.length === 0) {
    throw new FormDeclarationError(`${label} must be a non-empty string`, { value });
  }
  return value;
}

function finiteOrNull(value, label) {
  if (value === undefined || value === null) {
    return null;
  }
  if (typeof value !== "number" || !Number.isFinite(value) || value < 0) {
    throw new FormDeclarationError(`layout measurement ${label} must be a non-negative finite number`, {
      value,
    });
  }
  return value;
}
