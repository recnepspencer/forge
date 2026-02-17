# Forge Kernel — AI Agent Guide

Quick-start reference for AI coding agents working on the Forge geometry kernel.

---

## Crate Map

```
forge-math   → Pure numerics, predicates, linear algebra (no dependencies)
forge-core   → Shared types: KernelError, PolicyResult, OperationResult, GeometrySource
forge-geom   → Stateless geometry: planes, surfaces, BSP, intersections (→ math, core)
forge-topo   → Halfedge mesh, Euler operators, TopologyState, diff (→ geom, math, core)
forge-kernel → Policy engine, ModelingContext, features (→ topo, geom, math, core)
forge-io     → Import/export (→ everything)
forge-test   → Integration tests & fixtures (→ everything)
```

**Dependencies flow DOWN only.** Never add an upward `use` or Cargo dependency.

---

## Error Taxonomy

All errors use `KernelError` from `forge-core`:

| Variant | When |
|---------|------|
| `InvalidInput` | Bad parameters (wrong IDs, degenerate input) |
| `TopologyViolation` | Stale handles, Euler formula violations |
| `AmbiguousResult` | Geometry near tolerance boundary |
| `ToleranceExceeded` | Residual above threshold |
| `InternalError` | Logic bugs, invariant failures |

Every variant carries `context: Option<ErrorContext>` with `scope`, `suggested_fix`, and related entity info.

---

## Key Abstractions

### OperationResult\<T\>

Every `apply_op()` call returns `Result<OperationResult<T>, KernelError>`.

```rust
let result = apply_op(&mut draft, SplitEdge { edge })?;
let output = result.into_value();       // The actual operator output
let warnings = result.get_warnings();   // Non-fatal observations
let metrics = result.get_metrics();     // Timing, entity counts
```

### TopologyDiff

```rust
use forge_topo::diff::compute_diff;
let diff = compute_diff(before_arena, after_arena, epoch_before, epoch_after);
println!("Added: {}, Removed: {}", diff.total_added(), diff.total_removed());
```

### DecisionLog

```rust
use forge_core::DecisionLog;
let decisions = ctx.get_decisions();
let marginal = ctx.get_most_marginal(5);    // 5 most marginal decisions
let merged = ctx.get_decisions_by_kind(&DecisionKind::MergedVertices);
```

---

## Adding a New Feature

Follow the `/add-feature` workflow in `.agent/workflows/`.

1. **Create a feature directory** under the appropriate crate
2. **Use the Bento Box pattern**: `mod.rs` (manifest), `schema.rs` (data), `eval.rs` (logic), `topo.rs` (mutations), `tests.rs`
3. **All mutations go through Euler operators** via `apply_op()`
4. **Return structured errors** — never `panic!()`, `unwrap()`, or `expect()`
5. **Pass minimal values** — no `&TopologyState` in function signatures

---

## Conventions Cheat Sheet

| Rule | Do | Don't |
|------|-----|-------|
| IDs | `FaceId`, `VertexId` (typed handles) | `u32`, `usize` |
| Errors | `KernelError::InvalidInput { ... }` | `panic!("bad")` |
| Mutation | `MutableDraft` + `apply_op()` + `commit()` | Direct arena writes |
| Docs | `///` for public, `//!` for modules | No `//` inline comments |
| Tests | Straight-line, no loops/conditionals | Complex test logic |
| Functions | Named as verbs, single responsibility | `fn do_this_and_that()` |
| Files | ≤ 400 lines, subdivide if larger | Monolithic modules |

---

## Running Tests

```bash
cargo test --workspace              # All tests
cargo test -p forge-topo            # Single crate
cargo test -p forge-topo diff       # Single module
```

---

## Current Status

See [`PHASE_STATUS.md`](../PHASE_STATUS.md) for the milestone status table.
See [`DEVELOPMENT_BLUEPRINT.MD`](../DEVELOPMENT_BLUEPRINT.MD) for full specifications.
