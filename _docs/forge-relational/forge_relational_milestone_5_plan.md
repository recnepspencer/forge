# Milestone 5 Plan: Commit Architecture

## Summary

Implement Phase E from
[relational_architecture.md](./relational_architecture.md) as a clean-break
commit architecture rewrite.

This milestone should treat the commit pipeline as a producer of explicit batch
artifacts, not a sequence of phases that each rediscover their own local story.
The new cross-cutting architecture rule applies directly here:

- if intermediate structural maintenance has no semantic value inside a batch,
  amortize it at the merged-batch boundary
- represent it as a batch-derived summary or delta
- make later commit phases, tracing, and return envelopes consume that artifact

The milestone should be executed in this order:

1. introduce batch-derived commit structural summaries
2. build the commit decision log around those summaries
3. introduce the unified commit result envelope
4. delete old phase-local result reshaping and tracing residue

## Ordered Slices

### Slice 1: Commit Structural Summary

Create a named batch-derived summary artifact for commit-time structural facts.

The first version should own at least:
- invariant contract mask
- inferred commit topology
- touched partitions
- bulk reservation counts

The goal is to stop recomputing merged-plan structure across prepare,
invariants, publication, and later tracing/result surfaces.

### Slice 2: Commit Decision Log

Introduce a real `CommitLog` with explicit phase spans and structured
phase/decision recording. The log should consume `CommitStructuralSummary`
instead of requiring each phase to rediscover batch shape.

### Slice 3: Commit Result Envelope

Introduce `CommitResult` as the canonical commit return envelope. It should
bundle:
- `CommitOutcome`
- canonical commit envelope
- diagnostics
- patch records
- invariant results
- complexity delta
- commit log
- phase timing

### Slice 4: Deletion Pass

Delete any phase-local helper layers, duplicate tracing vocabulary, or partial
result shims that remain once the summary/log/envelope flow is live.

## Must Preserve

- serialized authoritative mutation
- atomic publication contract
- canonical patch/replay relationship
- deterministic commit outputs
- branch/history determinism
- immutable committed reads

## Performance Rules

- batch-derived structural facts are computed once and reused
- commit topology must be a control signal, not just observability metadata
- later commit phases should consume summaries/deltas, not rescan merged intents
- no per-intent structural maintenance should survive if only batch-final truth
  matters

## Acceptance Criteria

- commit structural facts are represented by a named batch artifact
- commit tracing/logging consumes batch summaries instead of re-deriving them
- callers receive one coherent commit result envelope
- old result/tracing duplication is deleted
- `cargo test -p forge-relational --lib` remains green
