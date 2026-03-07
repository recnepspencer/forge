# forge-signal Boundary Contract

`forge-signal` is a domain-free reactive runtime for deterministic evaluation.

## What this crate owns

- Evaluation dependency graph scheduling and invalidation.
- Deterministic ordering for event-subscriber flush.
- Checkpoint staging/finalize/rollback semantics.

## What this crate does not own

- Host structural graphs (B-Rep, mesh topology, feature spec storage).
- Geometry numerics, spatial acceleration, or CAD-domain mutation logic.

## Two graph kinds

1. Evaluation dependency graph:
   - Must be a DAG.
   - Cycles are invalid and rejected.
2. Structural host graph:
   - May be cyclic.
   - Lives in host domain crates.
   - Is consumed by signal compute closures as opaque snapshots/views.

## Raw-path compute contract

- `forge-signal` does not require per-field reactive lookups during compute.
- Host algorithms may consume tightly packed snapshots directly.
- Reactive overhead is confined to invalidation/scheduling boundaries.

## Integration expectation for host domains

- Emit effects/invalidation from structural mutation chokepoints.
- Keep structural truth in domain storage.
- Use signal DAG for derived-state refresh and orchestration.
