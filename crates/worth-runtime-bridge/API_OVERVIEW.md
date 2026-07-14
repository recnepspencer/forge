# API Overview

This guide explains how the public `worth-runtime-bridge` surface is organized.

The most important idea is simple:

- the bridge should read like one product

Even though the crate has deep protocol and certification machinery under the
hood, the public memory model should stay short and job-shaped.

## Public Surface Map

The bridge exposes one intended public surface:

- `worth_runtime_bridge::facade`

Inside that single facade, the product still has three categories of work:

- standard path
- explicit control
- replay and certification

Those categories are about usage shape, not competing public identities.

## Standard Path

Use this for normal bridge work.

This is the standard path:

- build a bridge
- route a truth change
- evaluate the current or explicit truth view
- open a speculative session
- discard or promote
- inspect diagnostics

The core memory should be:

```rust
use worth_runtime_bridge::facade::*;

let bridge = RuntimeBridge::builder()
    .with_truth_source(relational_source)
    .with_compute_sink(signal_sink)
    .build()?;

let route = bridge.route(change)?;
let evaluation = bridge.evaluate_current(route.target())?;
let session = bridge.speculate(spec_request)?;
let comparison = session.compare_to_main();
let diagnostics = bridge.diagnostics();
```

If you are new to the crate or writing ordinary application code, start here.

## Explicit Control

Use this when the job is still public and intentional, but no longer part of
the ordinary daily path.

Typical reasons to drop deeper into the explicit-control part of the facade:

- explicit truth-view selection beyond the common default
- bulk planning or delivery-oriented orchestration
- structural comparison or merge-aware flows
- stream control and multi-consumer behavior
- policy-shaped runtime refinement

These capabilities are important, but they should not dominate the first-read
story of the crate.

The main advanced domains are:

- policy and runtime posture
- truth-view materialization and source contracts
- bulk routing and planning
- stream delivery, replay, and resume
- structural comparison and merge-aware flows
- explicit writeback authority integration

## Replay And Certification

Use this when you are intentionally doing protocol-lab work rather than normal
application integration.

Typical reasons to use the replay and certification part of the facade:

- replay and canonical proof work
- certification harness integration
- low-level record or artifact inspection
- explicit family-aware writeback internals
- raw protocol surfaces that exist for trust, parity, or forensics

This layer exists on purpose.
It just should not be the default mental model for most users.

The main specialist domains are:

- canonical route, evaluation, and replay records
- certification and workload proof artifacts
- preview replay bundles and promotion proof surfaces
- low-level packet, slice, and retained protocol records
- parity, reconstruction, and offline audit surfaces

## Core Everyday Types

These are the main concepts most callers should care about first.

### `RuntimeBridge`

The runtime facade.
It is the ordinary front door for bridge work.

Everyday operations center here:

- `RuntimeBridge::builder()`
- `bridge.route(...)`
- `bridge.evaluate_current(...)`
- `bridge.evaluate(...)`
- `bridge.speculate(...)`
- `bridge.diagnostics()`

### `RuntimeBridgeBuilder`

The one obvious setup door.

Typical setup work:

- bind truth source
- bind branch-head source
- bind compute sink
- register mappings
- refine runtime policy or diagnostics if needed
- `build()`

### Route Result

The route result represents the bridge's canonical routing outcome for a truth
change.

In everyday work, the important use is usually:

```rust
let route = bridge.route(change)?;
let evaluation = bridge.evaluate_current(route.target())?;
```

### `BridgeTruthViewEvaluationRequest`

Use this when you need an explicit truth basis instead of the default current
view.

The common constructors are:

- `for_branch_head(...)`
- `for_branch_snapshot(...)`
- `for_historical_commit(...)`

### `BridgeSpeculativeSessionHandle`

The session-shaped speculation handle.

This is where branch-local preview work should live.
It owns:

- comparison to main
- discard
- promote

Speculation should feel scoped and intentional, not smeared across unrelated
top-level methods.

### Diagnostics Handle

`bridge.diagnostics()` is the normal diagnostics door.

It should be the first place you go to ask:

- what was the last thing the bridge did?
- why did this route happen?
- what truth view was used for this evaluation?
- what happened to this speculative session?
- what was promoted?

## Safe Defaults

The crate should be easy to use correctly without extensive policy homework.

That means the everyday surface is designed around defaults like:

- current truth view
- standard diagnostics tier
- canonical routing mode
- explicit but ordinary promotion flow

Advanced control still exists.
It just should not be mandatory for first success.

## What The API Tries To Hide In Everyday Work

The everyday surface intentionally tries to keep these off the happy path:

- phase-by-phase validation and admission verbs
- raw lowering and canonicalization sequences
- replay-first orchestration
- record-family inventory thinking
- low-level mapper-envelope assembly

Those are real bridge capabilities.
They are simply not meant to be the default memory model.

## Suggested Learning Order

For most readers, the smoothest path is:

1. [`README.md`](./README.md)
2. [`QUICKSTART.md`](./QUICKSTART.md)
3. [`DAILY_WORKFLOWS.md`](./DAILY_WORKFLOWS.md)
4. `facade`
5. diagnostics
6. only then the deeper explicit-control or replay-oriented types on `facade`

That order matches how the bridge is meant to feel in real use.
