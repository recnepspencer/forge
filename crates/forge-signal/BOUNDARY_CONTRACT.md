# forge-signal Boundary Contract

`forge-signal` is a domain-free reactive runtime for deterministic evaluation.

## What this crate owns

- Evaluation dependency graph scheduling and invalidation.
- Deterministic semantic artifacts for diagnostics, explanation, provenance, and replay.
- Deterministic ordering for event-subscriber flush.
- Checkpoint staging/finalize/rollback semantics.
- In-place graph mutation with sparse undo-log hard rewind.
- Runtime condition gating at the scheduling boundary.
- Arena-aligned node metadata for node-scale execution policy lookup.
- Scratch-backed traversal/evaluation/GC after warmup.

## What this crate does not own

- Host structural or state graphs.
- Host-domain numerics, acceleration structures, or mutation logic.
- Re-entrant graph traversal through shared scratch state.

## Two graph kinds

1. Evaluation dependency graph:
   - Must be a DAG.
   - Cycles are invalid and rejected.
2. Host state graph:
   - May be cyclic.
   - Lives in embedding crates.
   - Is consumed by signal compute closures as opaque snapshots/views.

## Raw-path compute contract

- `forge-signal` does not require per-field reactive lookups during compute.
- Host algorithms may consume tightly packed snapshots directly.
- Reactive overhead is confined to invalidation/scheduling boundaries.
- No overlay-read semantics exist in this crate.
- Scratch-backed graph passes are single-threaded and non-reentrant in this phase.

## Integration expectation for host domains

- Emit effects/invalidation from host mutation chokepoints.
- Keep source-of-truth state in host storage.
- Use the signal DAG for derived-state refresh and orchestration.
- Treat transaction failure as hard rewind: callers must not expect partial graph state to survive.
- Do not assume node-slot reuse preserves node-scoped metadata; generation-safe metadata guards that boundary.

## Parallel semantic contract

- Serial, staged-parallel, and full-parallel execution must converge to the same canonical semantic artifacts for logically equivalent runs.
- Observable semantic artifacts are canonicalized before retention; completion order is never the source of truth.
- Replay events plus stable task/segment identifiers are the authoritative retained truth in every runtime policy.
- Explanation and provenance artifacts are policy-dependent: they may be eagerly retained, deterministically reconstructed, or intentionally unavailable depending on the configured runtime policy.
- Callers that care about this distinction should use explicit retained/reconstructed accessors rather than assuming eager availability.
- Transaction boundary transitions and semantic merge/finalization remain intentionally serial for determinism.
- Current-run replay/diagnostics/explanation/provenance truth is runtime-owned here; durable storage and cross-run analysis belong outside this crate.
- Core storage width is selected at build time through the crate storage profile (`compact`, `standard`, or `extended`); runtime artifacts surface the active profile identifier so capture/replay consumers do not guess.
- Market presets (`game_engine`, `fintech`, `kernel`) are configuration conveniences layered on top of the same deterministic runtime policy contract.

See [PARALLEL_CERTIFICATION.md](./PARALLEL_CERTIFICATION.md) for the release gates, failure matrix, and deterministic-equivalence contract.
