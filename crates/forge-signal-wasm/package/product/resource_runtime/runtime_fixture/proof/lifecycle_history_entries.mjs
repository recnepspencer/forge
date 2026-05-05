function snapshotLifecycleCore(entries) {
  return entries.map((entry) => ({
    sequence: entry.sequence,
    event: entry.event,
    lastOutcome: entry.lastOutcome,
    status: entry.status,
    freshness: entry.freshness,
    visibleValueVersion: entry.visibleValueVersion,
  }));
}

function snapshotLifecycleSupersession(entries) {
  return entries.map((entry) => ({
    sequence: entry.sequence,
    event: entry.event,
    status: entry.status,
    supersededOperation: entry.supersededOperation,
    lastSupersededOperation: entry.lastSupersededOperation,
  }));
}

export { snapshotLifecycleCore, snapshotLifecycleSupersession };
