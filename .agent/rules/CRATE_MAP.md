---
trigger: always_on
---

# Forge Crate Map — Quick Reference for AI Agents

> **Read this first.** This document tells you which crate owns which
> abstractions so you put code in the right place on the first try.

## Dependency Graph (actual, from Cargo.toml)

```
forge-math          ← pure math, no internal deps
  └─ forge-core     ← shared types (KernelError, GeometrySource, PolicyResult)
       ├─ forge-geom    ← stateless geometry solvers
       ├─ forge-signal  ← reactive dependency graph
       └─ forge-topo    ← topology arena + operators (also depends on forge-geom)
            └─ forge-kernel  ← policy engine, features, booleans (depends on core+geom+topo)
                 └─ forge-io     ← serialization (depends on math+topo+geom)
                      └─ forge-test  ← test harness (depends on everything)

forge-schema  ← declarative JSON schema (depends on serde only)
forge-repr    ← representation types (TriangleMesh, Viewable, Tessellatable) — no deps
forge-view    ← trace viewer + CLI (depends on forge-core)
```

## Per-Crate Responsibility Table

| Crate            | Owns                                                                                                                                                                                                     | Exports (key types)                                                                                                                                                                                                                                                                                          | FORBIDDEN                                                                     |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------- |
| **forge-math**   | Exact predicates, rational arithmetic, linear algebra. Structure: `numeric/`, `arithmetic/`, `predicates/`, `linalg/`, `coincidence/`, `data_access/`                                                    | `MathError`, `TriSign`, `CertifiedTriSign`, `GeometrySource`, `PlaneCoefficients`, `orient2d`, `orient3d`, `Rational`                                                                                                                                                                                        | No geometry types (`Plane`), no topology types (`FaceId`), no `KernelError`   |
| **forge-core**   | Shared error/policy language. Structure: `errors/`, `policy/`, `tracing/`, `envelope/`                                                                                                                   | `KernelError`, `PolicyResult<T>`, `PolicyKind`, `PolicyQuery`, `TracedDecision`, `DecisionLog`, `OperationResult`                                                                                                                                                                                            | No business logic, no arena, no geometry math                                 |
| **forge-geom**   | Stateless geometry solvers. Structure: `primitives/`, `spatial/`, `algorithms/`, `curve/`, `surface/`                                                                                                    | `Plane`, `PlaneSet`, `ImplicitVertex`, `BspTree`, `ray::*`                                                                                                                                                                                                                                                   | No arena, no `TopologyState`, no policy decisions, no `FaceId`/`VertexId`     |
| **forge-topo**   | Topology arena, handles, traversal, Euler ops, ordering, attributes. Structure: `topology/` (handles, arena, state, operations/, queries/, integrity/, history/)                                         | `TopologyArena`, `TopologyState`, `MutableDraft`, `FaceId`, `VertexId`, `HalfEdgeId`, `LoopId`, `OrderingKey`, `DeterministicOrder`, `AttributeStore`, `EntityKey`, `TagValue`, `Lineage`, `OpSignature`                                                                                                     | **No raw f64 geometry** (no `dist < EPS`), no hardcoded thresholds, no policy |
| **forge-kernel** | Policy engine, features, booleans, mesh building, ModelingContext. Structure: `core/` (context, tolerance, macros), `features/`, `operations/`, `geometry_store/`, `mesh_builder/`, `analysis/`, `brep/` | `ModelingContext`, `ToleranceConfig`, `GeometryStore`, `BooleanInput`, `BooleanOp`, `BooleanResult`, `BooleanIntrospection`, `execute_boolean`, `FeatureTree`, `NativeFeature`, `Feature` trait, `FeatureOutput`, `MakeCubeFeature`, `BooleanFeature`, `MeshBuildResult`, `make_cube`, `build_halfedge_mesh` | — (top-level, can use everything below)                                       |
| **forge-signal** | Reactive dependency graph. Structure: `handles.rs`, `schema.rs`, `graph.rs`, `evaluation/` (push, pull, context)                                                                                         | `SignalGraph`, `NodeId`, `NodeEntry`, `NodeState`, `Aspect`, `AspectVersion`, `DependencyEdge`, `DependencySnapshot`, `mark_dirty`, `evaluate`                                                                                                                                                               | No topology, no geometry                                                      |
| **forge-io**     | JSON import/export (future: STEP/STL). Structure: `json/` (schema, eval, diff, tests)                                                                                                                    | `save_model`, `load_model`, `IoError`, `VersionedModel`, `diff_models`                                                                                                                                                                                                                                       | No topology mutation, no policy                                               |
| **forge-test**   | Differential test harness, generators. Structure: `fixtures.rs`, `generators/` (planar), `harness/` (boolean), `logging.rs`                                                                              | Test fixtures, random solid generators, fuzz corpus runner                                                                                                                                                                                                                                                   | Test-only crate                                                               |
| **forge-schema** | Declarative modeling language                                                                                                                                                                            | JSON schema types                                                                                                                                                                                                                                                                                            | No kernel deps                                                                |
| **forge-repr**   | Representation types for tessellation and rendering                                                                                                                                                      | `TriangleMesh`, `Viewable`, `Tessellatable`                                                                                                                                                                                                                                                                  | No kernel deps, no geometry deps                                              |
| **forge-view**   | Trace-only: store, native viewer (egui), CLI inspector. Structure: `trace/` (store, server, viewer)                                                                                                      | `TraceStore`, `TraceViewerApp`, `forge-trace-cli`, `forge-trace-viewer`                                                                                                                                                                                                                                      | No topology mutation, no policy, no representation types                      |

## "I Need To…" Decision Table

| I need to…                                 | Put it in                                        | Import from                                                                                      |
| ------------------------------------------ | ------------------------------------------------ | ------------------------------------------------------------------------------------------------ |
| Compare floats for a geometry decision     | `forge-geom`                                     | `forge_math::numeric::sign::CertifiedTriSign`                                                    |
| Make a topology decision based on geometry | `forge-topo`                                     | Call a `forge-geom` function, use its result                                                     |
| Add a tolerance threshold                  | `forge-kernel::core::tolerance::ToleranceConfig` | Kernel passes individual `f64` values down                                                       |
| Handle an ambiguous geometric result       | Return `PolicyResult<T>` from `forge-geom`       | `forge_core::PolicyResult`                                                                       |
| Create/mutate topology                     | `forge-topo` via `MutableDraft`                  | `forge_topo::state::MutableDraft`                                                                |
| Add a new error variant                    | `forge-core::KernelError`                        | `forge_core::KernelError`                                                                        |
| Return an error from `forge-math`          | Use `MathError`                                  | `forge_math::MathError` (NOT `KernelError`)                                                      |
| Access plane data from a lower layer       | Implement `GeometrySource` trait                 | `forge_math::GeometrySource`                                                                     |
| Write a hardcoded `const EPS`              | **DON'T.** Use `ToleranceConfig`                 | `forge_kernel::core::ToleranceConfig`                                                            |
| Write `unwrap()` or `expect()`             | **DON'T** (outside `#[cfg(test)]`)               | Return `Result<T, KernelError>` or `MathError`                                                   |
| Write `dist < 1e-8` in forge-topo          | **DON'T.** Move the comparison to `forge-geom`   | Import result from `forge-geom`                                                                  |
| Pass `ToleranceConfig` to a lower crate    | **DON'T.** Pass individual `f64` values          | `forge-kernel` destructures config                                                               |
| Use a raw `u32` as an ID                   | **DON'T.** Use typed handles                     | `forge_topo::handles::{FaceId, VertexId, ...}`                                                   |
| Save/load a model as JSON                  | `forge-io::json`                                 | `forge_io::{save_model, load_model}` (re-exported) or `forge_io::json::{save_model, load_model}` |
| Diff two JSON models                       | `forge-io::json::diff`                           | `forge_io::json::diff::{diff_models, ModelChange}`                                               |
| Build a halfedge mesh from BSP             | `forge-kernel::mesh_builder`                     | `forge_kernel::mesh_builder::{build_halfedge_mesh, make_cube, MeshBuildResult}`                  |
| Register a feature in the tree             | `forge-kernel::features::tree`                   | `forge_kernel::features::tree::FeatureTree`                                                      |
| Tag a face with metadata                   | `forge-topo::attributes`                         | `forge_topo::attributes::{AttributeStore, EntityKey, TagValue}`                                  |
| Sort entities deterministically            | `forge-topo::ordering`                           | `forge_topo::ordering::{OrderingKey, DeterministicOrder}`                                        |
| Mark a signal node dirty                   | `forge-signal::evaluation::push`                 | `forge_signal::evaluation::mark_dirty`                                                           |
| Evaluate a signal graph                    | `forge-signal::evaluation::pull`                 | `forge_signal::evaluation::evaluate`                                                             |
| Inspect test traces (AI)                   | Run `/testing-and-tracing` workflow              | `forge-trace-cli list/show/decisions/issues`                                                     |
| View traces in GUI                         | `forge-view` binary                              | `cargo run --bin forge-trace-viewer <DIR>`                                                       |
| Auto-persist traces from tests             | Set `FORGE_TRACE_DIR` env var                    | `OperationResult::into_value()` handles it                                                       |
| Tessellate a solid for rendering           | `forge-repr`                                     | `forge_repr::{TriangleMesh, Tessellatable, Viewable}`                                            |

## Key API Paths

```
forge_math::numeric::sign::TriSign     — Neg / Zero / Pos
forge_math::numeric::sign::CertifiedTriSign — Compile-time proof of exact evaluation
forge_math::predicates::orient2d       — Exact 2D orientation predicate
forge_math::predicates::orient3d       — Exact 3D orientation predicate
forge_math::data_access::GeometrySource — Trait for anonymous data access (Rule 3.1)
forge_math::data_access::PlaneCoefficients — Typed plane coefficient struct
forge_math::MathError                  — Error type for forge-math

forge_core::KernelError                — Primary error type for all other crates (errors/)
forge_core::PolicyResult<T>            — Three-state return (policy/)
forge_core::PolicyKind                 — CoincidentGeometry / NearTangency / SliverFace / ...
forge_core::TracedDecision             — Recorded kernel decision (tracing/)
forge_core::DecisionKind               — Exact / PolicyApplied / Forced
forge_core::DecisionContext            — Tolerance / Degeneracy context
forge_core::DecisionLog                — Collection of traced decisions (tracing/decision_log)
forge_core::OperationResult<T>         — Universal return envelope (envelope/)

forge_geom::Plane                      — Analytic plane (re-exported from primitives::plane)
forge_geom::Aabb                       — Axis-aligned bounding box (re-exported from primitives::aabb)
forge_geom::BvhNode                    — Bounding volume hierarchy (re-exported from spatial::bvh)
forge_geom::PlaneSet                   — Vec<Plane> implementing GeometrySource
forge_geom::primitives::ray::compute_ray_plane_intersection  — Ray-plane hit point
forge_geom::primitives::ray::dominant_projection_axes        — Best 2D projection for a face
forge_geom::spatial::bsp::{build_convex_polyhedron, BspConfig} — BSP construction

forge_topo::handles::{FaceId, VertexId, HalfEdgeId, LoopId}  — Typed generational handles
forge_topo::state::TopologyState       — Immutable topology snapshot
forge_topo::state::MutableDraft        — Transaction for topology mutation (D6)
forge_topo::arena::TopologyArena       — The actual halfedge data store
forge_topo::classify::classify_point_in_solid    — Ray-parity point classification
forge_topo::ordering::OrderingKey      — Composite key for deterministic entity sorting
forge_topo::ordering::DeterministicOrder — Trait for entities that produce ordering keys
forge_topo::attributes::AttributeStore — Side-car semantic tag storage
forge_topo::lineage::Lineage           — Provenance record per entity
forge_topo::lineage::OpSignature       — Unique operation signature

forge_signal::graph::SignalGraph       — The reactive dependency graph
forge_signal::handles::NodeId          — Typed generational handle for signal nodes
forge_signal::schema::{Aspect, AspectVersion, NodeState}  — Signal state types
forge_signal::evaluation::{mark_dirty, evaluate}  — Push/pull evaluation engine
forge_signal::evaluation::context::EvaluationContext — Parallel-safe dep tracking

forge_kernel::core::ModelingContext    — Policy + tolerance + decision log (core/context.rs)
forge_kernel::core::ToleranceConfig    — All numeric thresholds (core/tolerance.rs)
forge_kernel::core::tolerance::*Policy — TolerancePolicy, TangencyPolicy, SliverPolicy, etc.
forge_kernel::check_tolerance!         — Macro for logging tolerance decisions (core/macros.rs)
forge_kernel::features::tree::FeatureTree  — Parametric feature graph
forge_kernel::features::tree::NativeFeature — Enum of all features (for serde)
forge_kernel::features::traits::{Feature, FeatureOutput}  — Feature interface
forge_kernel::mesh_builder::{build_halfedge_mesh, make_cube, MeshBuildResult}  — BSP→mesh
forge_kernel::operations::boolean::{BooleanInput, BooleanOp, BooleanResult, execute_boolean}
forge_kernel::geometry_store::GeometryStore — Face-plane and vertex-position map

forge_io::{save_model, load_model}         — Re-exports from json/ module
forge_io::json::schema::VersionedModel     — Versioned JSON envelope
forge_io::json::eval::{save_model, load_model}  — JSON file I/O
forge_io::json::diff::{diff_models, ModelChange} — Model diffing
forge_io::IoError                          — IO error type

forge_repr::TriangleMesh               — Tessellated mesh for rendering
forge_repr::Viewable                   — Trait: SDF + bounding box
forge_repr::Tessellatable              — Trait: tessellate to TriangleMesh

forge_view::trace::store::TraceStore   — In-memory trace file manager
forge_view::trace::store::TraceFile    — On-disk JSON trace format
forge_view::trace::store::TraceMeta    — One-line trace summary (name, counts, hash)
forge_view::trace::viewer::TraceViewerApp — Native egui trace viewer app
forge_view::trace::server::build_router — Axum REST API for trace browsing
```
