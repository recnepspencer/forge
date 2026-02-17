# File Naming Dictionary

Deterministic naming conventions so agents instinctively know what a file does and where to put new code.

## Rust Source Files

| Suffix | Contains | Example |
|--------|----------|---------|
| `schema.rs` | Data shapes: structs, enums, type aliases | `extrude_schema.rs` |
| `eval.rs` | Pure business logic, math, evaluation | `extrude_eval.rs` |
| `topo.rs` | Topology mutations (Euler operator impls) | `extrude_topo.rs` |
| `facade.rs` | Adapters bridging external/complex systems | `step_facade.rs` |
| `tests.rs` | Unit tests for the containing module | `tests.rs` |
| `mod.rs` | Table of Contents — exports only, no logic | `mod.rs` |

## When a Feature is a Single File

Small features that don't warrant a full directory can be a single file with a descriptive name. Split into a directory when:
- The file exceeds ~400 lines
- The feature has distinct schema, eval, and topo concerns
- Tests grow beyond a few cases

## Module-Level Files (Non-Feature)

| Name | Contains | Example Crate |
|------|----------|---------------|
| `handles.rs` | Typed generational handles | `forge-topo` |
| `state.rs` | State management, transactions | `forge-topo` |
| `operator.rs` | Trait definitions for operators | `forge-topo` |
| `validate.rs` | Invariant checking | `forge-topo` |
| `sign.rs` | Predicate types | `forge-math` |
| `error.rs` | Error taxonomy | `forge-math` |
| `context.rs` | Modeling context, policies | `forge-kernel` |

## Documentation Files

| Name | Purpose |
|------|---------|
| `architecture.md` | Crate structure and dependency chain |
| `tolerance-policy.md` | Tolerance thresholds and their rationale |
| `exactness-contract.md` | Precision guarantee model |
| `FILE_NAMING.md` | This file |
