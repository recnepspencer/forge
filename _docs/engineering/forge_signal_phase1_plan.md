# Forge Signal Phase 1 Plan

## Thesis

Phase 1 is the productization pass for `forge-signal`.

The core runtime already exists. This phase does not invent new execution semantics. It makes the current runtime coherent, accessible, and ready for direct use as a standalone library before planner work, bridge work, or more advanced incremental semantics land.

## Outcome

At the end of Phase 1:

- `SignalRuntime` is the only public runtime name
- runtime setup has one obvious public entrypoint
- transactions have one obvious linear flow
- node creation emphasizes aspects and conditions instead of config structs
- the crate documentation teaches one coherent model
- `forge-signal-easy` exists as a separate ergonomic layer built on the same runtime
- old public API clutter is removed or clearly demoted to low-level substrate

## What Phase 1 Includes

### Hard-mode API cleanup

- `SignalRuntime::builder(graph)` as the default runtime entrypoint
- `runtime.transaction(&mut ctx, |transaction| { ... })` as the standard transaction story
- `graph.node()` fluent node builder
- accessible naming as a hard rule for public methods and examples
- migration of public docs and examples to the new surface

### Easy-mode foundation

- `forge_signal::easy::*`
- `ReactiveGraph`
- input signals
- computed signals
- `get`, `set`, and `batch`
- automatic dependency capture

### Public-surface cleanup

- remove duplicate public runtime names
- stop advertising raw config-heavy constructors
- keep low-level primitives only when they are real substrate

## What Phase 1 Excludes

Phase 1 is intentionally not the place for:

- `explain(node)`
- dependency inspection APIs
- graph debug export
- output identity diffing
- structural memoization
- explicit execution planning
- parallel execution
- snapshot/restore as a first-class API
- bridge contracts
- fixed-point or speculative semantics

Those belong to later phases. Phase 1 should leave the API ready for them without trying to deliver them early.

## API Rules

### Rule 1: Accessible naming wins

Prefer names that a smart generalist can understand on first read.

Examples:

- `depends_on_aspects(...)`
- `condition(...)`
- `transaction(...)`
- `mark_dirty(...)`
- `evaluate(...)`

Do not optimize for insider elegance, shorthand, or compressed terminology.

### Rule 2: Hard mode must be beautiful too

The full-power surface is not allowed to feel like internal scaffolding.

Advanced code should read as a short linear story:

1. build runtime
2. configure graph and node policy
3. open transaction
4. mark dirty inputs
5. evaluate targets
6. commit or rollback

### Rule 3: Aspects and conditions are central

The API should visually teach that `forge-signal` is about:

- aspect-aware invalidation
- condition-aware evaluation
- deterministic transactional execution

Comparator overrides and tolerance tuning matter, but they are secondary configuration.

### Rule 4: Easy mode must not fork semantics

`forge-signal-easy` is a UX layer over the same runtime primitives. It is not a second runtime.

## Public Surface Decisions

### Keep public and primary

- `SignalRuntime`
- `SignalRuntime::builder(...)`
- `SignalGraph`
- `SignalGraph::node()`
- `SignalTransaction`
- `runtime.transaction(...)`
- `forge_signal::easy::*`

### Keep public but secondary

- explicit `runtime.begin()` and manual `commit()` / `rollback()` flow
- low-level dependency wiring such as `add_dependency(...)`
- low-level evaluation helpers used by advanced embeddings

### Keep as low-level substrate only

- raw config-backed constructors
- lower-level constructors used internally by builders

These may stay in the crate, but they should not be the advertised story.

### Remove from the public story

- `SignalRuntimeState`
- examples centered on `with_policy(...).begin()`
- examples centered on raw `NodeEvaluationConfig`

## Deliverables

### D1. Runtime front door

Requirements:

- `SignalRuntime` is the public runtime name
- runtime docs and examples use `SignalRuntime::builder(...)`
- builder exposes only meaningful setup knobs for this phase

Acceptance:

- public crate docs show one standard runtime construction path
- no public docs rely on `SignalRuntimeState`

### D2. Transaction flow

Requirements:

- closure-based `runtime.transaction(...)`
- commit on `Ok`
- rollback on `Err`
- explicit transaction object still available for advanced control

Acceptance:

- transaction examples read as one linear story
- tests cover commit path and rollback path

### D3. Fluent node builder

Requirements:

- `graph.node()`
- `depends_on_aspects(...)`
- `condition(...)`
- `always()`
- `on_demand()`
- `debounce(...)`
- `comparator(...)`
- `tolerance(...)`
- `build()`

Acceptance:

- common docs/examples use the node builder
- aspects and conditions are visually central in examples

### D4. Easy module

Requirements:

- separate `easy` namespace
- input and computed signals
- `get`, `set`, `batch`
- auto dependency capture

Acceptance:

- easy mode uses the same underlying runtime semantics
- easy-mode examples work without extra setup noise

### D5. Documentation reset

Requirements:

- crate docs use the new runtime naming
- README teaches one coherent hard-mode story and one coherent easy-mode story
- no tutorial-quality docs use the old surface

Acceptance:

- public examples are readable by a smart generalist with no prior signal-runtime background

## PR Sequence

## PR1. Public-surface naming cleanup

Goal:

- one public runtime name
- one public runtime-construction story

Changes:

- remove `SignalRuntimeState` from the public surface
- move public examples to `SignalRuntime`
- stop exporting old names from the facade

Acceptance:

- `forge-signal` compiles and tests cleanly
- no public docs or examples mention `SignalRuntimeState`

## PR2. Hard-mode runtime ergonomics

Goal:

- clean runtime setup and transaction flow

Changes:

- finalize `SignalRuntime::builder(...)`
- finalize `runtime.transaction(...)`
- tighten builder naming and defaults
- document explicit transaction path as advanced-only

Acceptance:

- runtime setup and transaction examples fit in a short, obvious flow

## PR3. Node builder and aspect/condition-first docs

Goal:

- make node policy readable without config structs

Changes:

- finalize `graph.node()`
- emphasize `depends_on_aspects(...)` and `condition(...)`
- demote raw `NodeEvaluationConfig` to low-level usage

Acceptance:

- common examples no longer require config structs
- node builder behavior is covered by tests

## PR4. Easy-mode baseline

Goal:

- give common users a direct, low-ceremony signal surface

Changes:

- land `forge_signal::easy::*`
- add `ReactiveGraph`
- add basic input/computed/get/set/batch API
- add easy-mode tests and docs

Acceptance:

- easy mode is usable on its own
- easy mode does not invent a separate execution model

## PR5. Cleanup and consistency pass

Goal:

- remove leftovers that still weaken the public story

Changes:

- delete obsolete examples
- tighten doc wording
- clean up remaining public naming leaks
- align workspace usage where convenient, without blocking on full migration

Acceptance:

- public-facing crate materials present one coherent model

## Remaining Cleanup Decisions

These should be decided with a bias toward clarity, not compatibility:

- if an old API exists only because of history, delete it
- if an old API is real substrate for the new surface, keep it but demote it
- do not maintain duplicate public paths when one is clearly preferred

## Acceptance Checklist

Phase 1 is complete when all of the following are true:

- `SignalRuntime` is the only public runtime name
- the builder is the standard runtime entrypoint
- closure-based transactions are the standard public flow
- node creation is builder-first in docs and examples
- aspects and conditions are central in public examples
- easy mode exists under its own namespace
- public docs teach one coherent mental model
- `forge-signal` remains a standalone generic library with no bridge requirement

## Next Phase Boundary

Once Phase 1 is complete, the next major work should be Phase 2 from the vision doc:

- explainability
- dependency inspection
- graph inspection
- metrics and causality surfaces

Do not jump ahead to planning, parallelism, or bridge contracts until the public surface feels stable and legible.
