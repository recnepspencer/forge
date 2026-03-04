# Validator QA Rules

> Every validator must prove it can detect the disease, not just that it doesn't crash on healthy input.

## The Poison Test Contract

For every validator function `validate_X(arena) -> Result<(), KernelError>`:

### Rule 1: Minimum 2 Tests Per Validator

1. **Positive proof**: Build valid topology (e.g., a cube via `MakeVertexFace` + `SplitEdge`), run the validator, assert `Ok(())`.
2. **Poison injection**: Build valid topology, then **bypass operators** and directly corrupt the arena via `arena_mut()` to inject the exact invariant violation. Assert the validator returns `Err` with the **specific error variant** (not just "any error").

### Rule 2: At Least One Subtle Poison

Every validator must have at least one poison test that targets a non-obvious case:

- Inner loop corruption, not just outer loop
- Multi-hole faces, not just single-loop faces
- Shared vertices between faces, not just isolated faces

### Rule 3: Real-World Regression Anchors

When a validator is written in response to a discovered operator bug, a third test must reproduce the **exact corruption pattern** from the real bug (e.g., run the buggy operator sequence, assert the validator catches it).

### Rule 4: No Coupling Between Validators

Validators are pure read-only functions: `fn(&TopologyArena) -> Result<(), KernelError>`. They must not depend on each other, must not mutate state, and must not require geometry unless explicitly in the geometry-dependent batch.

### Rule 5: Wire Into Dispatcher

Every new validator must be registered in `structural.rs::validate_topology()` at the appropriate `ValidationLevel` tier. If it is not wired into the dispatcher, it does not exist.
