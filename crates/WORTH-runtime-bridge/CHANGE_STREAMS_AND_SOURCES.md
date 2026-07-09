# Change Streams And Sources

This guide covers the bridge surfaces that bring truth into the bridge and move
invalidations outward in structured form.

These are advanced integration topics.
They matter whenever you are binding real hosts, not just running the smallest
everyday path.

## Why This Layer Exists

The bridge should not assume one in-process truth source and one simple sink
forever.

It needs public contracts for:

- source capability declaration
- source materialization
- change stream delivery
- replay and resume
- backpressure-aware consumer behavior

## Source Contracts

Source-facing integration belongs to the bridge because truth access and truth
view materialization are part of the bridge boundary.

Public advanced types here include:

- `SourceDeclaration`
- `AdmittedSourceContract`
- `BridgeSourceCapability`
- `BridgeSourceCapabilitySet`
- `PlannedSourceReadPacketSet`
- `MaterializedTruthViewPacketSet`

These types describe what a source can provide and how the bridge is allowed to
materialize truth from it.

## Change Stream Contracts

Stream-facing integration belongs to the bridge because routed change delivery,
checkpointing, and replay all affect the meaning of cross-runtime transfer.

Public advanced and specialist types here include:

- `ChangeStreamDeclaration`
- `AdmittedConsumerContract`
- `PlannedChangeStreamWindow`
- `StreamReplayMode`
- `StreamResumeMode`
- `ConsumerCheckpointToken`
- `CanonicalStreamReplayRecord`

## Delivery Must Preserve Meaning

Whether a consumer reads live, resumes from checkpoint, or replays a retained
window, the bridge should preserve:

- member truth
- ordering semantics where declared
- replay meaning
- typed failure surfaces

That is why streams are a bridge contract, not just an adapter convenience.

## When You Need This Guide

Reach here when you are:

- authoring or binding a source adapter
- working with truth-view materialization policy
- integrating a multi-consumer stream path
- reasoning about replay, checkpoint, or resume behavior
- diagnosing source capability or transport failure

If you are just building, routing, evaluating, and speculating in the default
path, you do not need this layer first.
