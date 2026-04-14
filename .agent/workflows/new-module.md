---
description: Create a new module or feature directory following the Bento Box pattern and naming conventions
---

# New Module Workflow (v2)

## Step 1: Identify the Layer

Refer to `architecture.md` to ensure correct dependency direction:

- **math**: Pure numbers only. Internal structure: `numeric/`, `arithmetic/`, `predicates/`, `linalg/`, `coincidence/`, `data_access/`.
- **core**: Shared error/policy types. Internal structure: `errors/`, `policy/`, `tracing/`, `envelope/`.
- **geom**: Stateless solvers only. Internal structure: `primitives/`, `spatial/`, `algorithms/`, `curve/`, `surface/`.
- **signal**: Reactive graph. Internal structure: `evaluation/` (push, pull, context).
- **topo**: Connectivity and Generational safety. Internal structure: `topology/` (handles, arena, state, operations/, queries/, integrity/, history/).
- **kernel**: Policy, Feature Tree, and Orchestration. Internal structure: `core/` (context, tolerance, macros), `features/`, `operations/` (boolean, fillet, ...), `geometry_store/`, `mesh_builder/`, `analysis/`, `brep/`.
- **io**: Serialization formats. Internal structure: `json/` (schema, eval, diff, tests).
- **test**: Test infrastructure. Internal structure: `fixtures.rs`, `generators/` (planar), `harness/` (boolean), `logging.rs`.
- **repr**: Representation types (`TriangleMesh`, `Viewable`, `Tessellatable`). No deps.
- **view**: Trace viewer + CLI. Internal structure: `trace/` (store, server, viewer).

## Step 2: Determine Directory Placement

| Module Type                                   | Location                                                                                  |
| --------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Modeling operation (boolean, fillet, extrude) | `forge-kernel/src/operations/<name>/`                                                     |
| Feature tree entry                            | `forge-kernel/src/features/<name>/`                                                       |
| Geometry solver                               | `worth-geom/src/primitives/` or `worth-geom/src/spatial/` or `worth-geom/src/algorithms/` |
| Topology operation                            | `forge-topo/src/topology/operations/`                                                     |
| Topology query                                | `forge-topo/src/topology/queries/`                                                        |
| Math utility                                  | `worth-math/src/numeric/` or `worth-math/src/arithmetic/`                                 |
| Data-access traits                            | `worth-math/src/data_access/`                                                             |
| IO format handler                             | `forge-io/src/<format>/` (e.g., `forge-io/src/json/`)                                     |
| Test generator                                | `forge-test/src/generators/`                                                              |
| Test harness                                  | `forge-test/src/harness/`                                                                 |
| Representation type                           | `forge-repr/src/`                                                                         |

## Step 3: The Template (Feature Module)

Every directory must contain a `mod.rs` that acts as a Table of Contents:

```rust
//! DOMAIN: <Feature Name>
//!
//! <Brief description of the domain>
//!
//! INVARIANTS:
//! - <Key invariant 1>
//!
//! DEPENDENCIES: <list>

mod schema;
mod eval;
#[cfg(test)]
mod tests;

pub use schema::{...};
pub use eval::{...};
```

### Standard files:

- `schema.rs` — Data shapes (Structs, Enums)
- `eval.rs` — Pure business logic and algorithms
- `topo.rs` — Stateful topology mutations (if applicable)
- `tests.rs` — Unit tests (or `tests/` directory if multiple test files)
