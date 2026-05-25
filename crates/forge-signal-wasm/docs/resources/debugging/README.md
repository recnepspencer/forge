# Inspecting And Debugging Resources

Use this section when the resource already exists and the question is now:

- what is this line doing right now?
- why did the visible value change?
- why did it not change?
- can this line restore or replay exact truth?
- is delivery or compatibility the real reason something drifted?

## Start Here

- [Inspect A Resource Line](./inspect-a-resource-line.md)
  The grouped first reads for ordinary debugging.
- [Check Status, Freshness, And History](./check-status-settlement-and-history.md)
  Status, freshness, retry posture, and retained history truth.
- [Why Did This View Update?](./why-did-this-view-update.md)
  Patch, delivery, visible selection, and effect provenance.
- [Why Didn't This View Update?](./why-didnt-this-view-update.md)
  Freshness, continuity, denied restore/replay, and unavailable branch proof.
- [Read Delivery And Compatibility](./read-delivery-and-compatibility.md)
  External definitions, basis drift, and packet-owned updates.
- [Restore, Replay, And Recover](./restore-replay-and-recover.md)
  Exact replay, exact restore, and rollback of the latest effect.

## The Main Mental Model

Start high, then go lower only when you need proof.

1. `line.summary()`
2. `line.status()` / `line.freshness()`
3. `line.history()`
4. `line.history().verificationPackage()`

That order keeps the common path small while still leaving the proof-bearing
history lane available when the grouped summary is not enough.

## Related Docs

- [Line Inspection](../line-inspection.md)
- [Caching And Refresh](../caching/README.md)
- [Branch-Native Resource Effects](../effects/README.md)
- [History And Restore](../../resource-contracts/history-and-restore.md)
