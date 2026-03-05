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

### Rule 5: Wire Into the Invariant System

Every new validator must be registered through the **compile-enforced** invariant pipeline:

1. Add a new variant to `InvariantId` in `invariant_id.rs`
2. Assign it to an `InvariantGroup` in the `group()` match — compile error if missing
3. Add a `ValidatorEntry` in `validator_for()` — compile error if missing
4. Update `InvariantId::ALL` to include the new variant — `all_constant_covers_every_variant` CI test catches omissions

`structural.rs::validate_topology()` automatically picks up the new validator via the `validator_for()` dispatch loop — **no manual wiring in structural.rs is needed**.

### Rule 6: Declare a Cost Tier

Every `ValidatorEntry` must specify a `ValidatorCost`:

| Cost        | Meaning               | Runs at `ValidationLevel`   |
| :---------- | :-------------------- | :-------------------------- |
| `Cheap`     | O(n) per-entity scan  | Minimal, Intermediate, Full |
| `Medium`    | O(n log n) or set ops | Intermediate, Full          |
| `Expensive` | O(n²) or global       | Full only                   |

Choose the tier that matches your validator's algorithmic complexity. This determines when it runs at commit time.
