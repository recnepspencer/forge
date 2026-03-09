# Forge Signal Full Parallel Execution Design

## Purpose

This document defines the architecture and acceptance contract for full parallel execution in
`forge-signal`.

It exists to prevent a repeat of the watered-down story where same-stage parallel precompute is
quietly described as mature parallel evaluation.

The current truthful model remains:

- serial planning
- serial execution snapshot build
- optional same-stage parallel precompute
- serial apply

Full parallel execution is a separate architecture.

## Goals

- preserve deterministic final graph state
- preserve diagnostics, explanation, and provenance fidelity
- preserve rollback semantics
- make parallel work selection explicit and measurable
- allow harness-driven parity validation between serial and parallel execution

## Non-Goals

- no hidden relaxed consistency model
- no best-effort parallelism that silently changes behavior
- no concurrent apply until deterministic merge semantics exist
- no executor design that depends on `easy`

## Execution Model

### Phase 1: plan

Planning stays deterministic and serial.

Outputs:

- stage order
- task order within each stage
- per-task execution policy metadata
- per-stage parallel eligibility metadata

### Phase 2: snapshot

Execution snapshot build stays serial and immutable.

This is the causal anchor for the wave and must remain single-truth.

### Phase 3: parallel precompute

Eligible tasks within a stage may precompute in parallel against the immutable snapshot.

Requirements:

- no shared mutable graph state
- task-local dependency capture only
- task-local output/result payload only
- task-local diagnostics staging only

### Phase 4: patch-buffered merge

Prepared results from workers are merged into a deterministic patch buffer rather than applied
directly to the live graph.

Requirements:

- stable merge order by planned task order
- deterministic conflict handling
- worker-local metadata retained until merge
- no graph writes from worker threads

### Phase 5: apply

Apply may become parallel only after deterministic merge semantics exist for all side effects.

Until then:

- patch merge may be parallel-safe internally
- graph apply remains ordered by the merged patch sequence

Current recommendation:

- keep final apply serial for now
- do not claim full parallel apply until downstream suppression, diagnostics mutation, and rollback
  semantics are all modeled as deterministic patches

## Required Runtime Pieces

### Executor policy

The executor must choose serial vs parallel work based on:

- stage width
- optional task cost class
- worker count
- configured execution mode

### Worker pool

Scoped per-stage thread spawning is not the desired end-state.

The mature executor should use:

- reusable worker pool
- deterministic task chunking
- bounded queueing

### Patch model

Each worker result must be representable as a patch payload:

- node state update
- aspect version update
- dependency snapshot handle update
- trace/causality update
- diagnostics deltas
- telemetry deltas
- suppression side effects if applicable

### Conflict model

Same-stage tasks should not conflict on target node writes.

If future semantics introduce shared mutation surfaces, conflicts must be:

- explicitly modeled
- deterministically merged
- rejected if unsafe

## Diagnostics Contract

Parallel execution must preserve:

- task outcome classification
- execution record determinism
- explanation/provenance surfaces
- flow/failure/rollback diagnostics

Parallel execution may change timing.

It must not change:

- final node states
- trace summaries
- execution record semantics
- rollback results
- explanation causal meaning

## Harness Acceptance Tests

Full parallel execution is not done until all of these pass through `forge-harness`:

1. serial vs full-parallel final state parity
2. serial vs full-parallel diagnostics parity
3. serial vs full-parallel explanation/provenance parity
4. replay parity between serial and full-parallel captures
5. rollback parity after injected apply failures
6. memoization parity
7. partition-scoped invalidation parity
8. output-identity suppression parity

## Performance Acceptance Tests

Full parallel execution is not done until it demonstrates:

- no regression on narrow stages that should remain serial
- measurable improvement on wide expensive stages
- no unbounded diagnostics blow-up
- no rollback or failure-path corruption

## Implementation Sequence

1. preserve current truthful staged-parallel-precompute model
2. introduce executor pool abstraction
3. introduce deterministic patch payload representation
4. stage worker-local diagnostics/telemetry deltas
5. merge prepared results into ordered patch sets
6. validate parity through harness suites
7. only then consider concurrent apply

## Completion Standard

The runtime may only say "full parallel execution" when:

- graph writes are driven through deterministic merged patch semantics
- harness parity and replay acceptance suites pass
- performance acceptance suites show the mode is useful rather than ceremonial
- docs stop needing the qualifier "parallel precompute only"
