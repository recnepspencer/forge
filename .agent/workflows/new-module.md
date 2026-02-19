---
description: Create a new module or feature directory following the Bento Box pattern and naming conventions
---

# New Module Workflow (v2)

## Step 1: Identify the Layer

Refer to `architecture.md` to ensure correct dependency direction:
- **math**: Pure numbers only. Internal structure: `numeric/`, `arithmetic/`, `predicates/`, `linalg/`, `coincidence/`.
- **core**: Shared traits (`GeometrySource`, `PolicyResult`).
- **geom**: Stateless solvers only. Internal structure: `primitives/`, `spatial/`, `algorithms/`, `curve/`, `surface/`.
- **signal**: Reactive graph. Internal structure: `evaluation/` (push, pull, context).
- **topo**: Connectivity and Generational safety. Internal structure: `topology/` (handles, arena, state, operations/, queries/, integrity/, history/).
- **kernel**: Policy, Feature Tree, and Orchestration. Internal structure: `core/`, `features/`, `operations/` (boolean, fillet, ...), `geometry_store/`, `mesh_builder/`, `analysis/`, `brep/`.

## Step 2: Determine Directory Placement

| Module Type | Location |
|-------------|----------|
| Modeling operation (boolean, fillet, extrude) | `forge-kernel/src/operations/<name>/` |
| Feature tree entry | `forge-kernel/src/features/<name>/` |
| Geometry solver | `forge-geom/src/primitives/` or `forge-geom/src/spatial/` or `forge-geom/src/algorithms/` |
| Topology operation | `forge-topo/src/topology/operations/` |
| Topology query | `forge-topo/src/topology/queries/` |
| Math utility | `forge-math/src/numeric/` or `forge-math/src/arithmetic/` |

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