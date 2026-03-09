# Forge Crate Overview

This document provides a thorough overview of the most important types, concepts, and architecture in each crate of the Forge geometry kernel.

---

## forge-math

**Purpose**: Exact arithmetic, certified predicates, and filtered evaluation — the mathematical foundation.

### Key Types

- **`TriSign`** (`numeric/sign.rs`): Three-valued sign (Neg/Zero/Pos) representing geometric predicate outcomes. Unlike boolean predicates, `Zero` represents genuine geometric coincidence, not a degeneracy to avoid.

- **`CertifiedTriSign`** (`numeric/sign.rs`): A newtype wrapper around `TriSign` that can ONLY be constructed inside `forge-math` via `pub(crate) fn new()`. This enforces **Doctrine D3** (Topology-Geometry Firewall) at compile time — topology functions accept `CertifiedTriSign`, making it impossible to pass raw float comparisons to topology mutations.

- **`FilteredEval`** (`arithmetic/filter.rs`): Four-stage evaluation pipeline:
  1. **f64** with Shewchuk error bounds (resolves >95% of cases)
  2. **Interval** with ULP-widened bounds (resolves >99% remaining)
  3. **Double-double** (~106-bit) (resolves >99.9% remaining)
  4. **Exact rational** (resolves everything)

- **`Rational`** (`arithmetic/rational.rs`): Exact rational arithmetic for final-stage predicate evaluation.

- **`PrecisionEscalation`** (`arithmetic/filter.rs`): Metadata recording which precision stage resolved a result.

### Predicates

- **`orient2d`**, **`orient3d`**: Orientation predicates (point-to-plane classification)
- **`in_sphere`**: Point-in-sphere predicate
- **`orient2d_grid`**, **`orient3d_grid`**: Grid-quantized predicates for spatial hashing

### Key Concept: Doctrine D3 Enforcement

Every topology decision must flow through a certified predicate. The type system prevents raw float comparisons from reaching topology code.

---

## forge-core

**Purpose**: Shared types and protocols that all Forge crates speak — errors, policy, tracing, and operation envelopes.

### Key Types

- **`KernelError`** (`errors/mod.rs`): Primary error type with structured variants:
  - `InvalidInput`: User-provided bad data
  - `TopologyViolation`: Structural invariant broken
  - `AmbiguousResult`: Geometry solver couldn't decide (requires policy)
  - `MathError`: Arithmetic/predicate failure

- **`TopologyError`** (`errors/mod.rs`): Specific topology violations (Euler formula, twin consistency, etc.)

- **`PolicyResult<T>`** (`policy/mod.rs`): Three-state return type for Doctrine D2 (Ambiguity Escalation):
  - `Ok(T)`: Unambiguous result
  - `Ambiguous(PolicyQuery)`: Geometry solver couldn't decide — kernel applies policy
  - `Err(KernelError)`: Hard failure

- **`TracedDecision`** (`tracing/mod.rs`): Records every kernel decision with:
  - `DecisionId`: Unique identifier
  - `DecisionKind`: Exact/Ambiguous/Policy
  - `DecisionTier`: Deterministic/Policy/Heuristic
  - `DecisionContext`: Structured context (e.g., `Degeneracy`, `Tolerance`, `TopologyOp`)

- **`DecisionLog`** (`tracing/mod.rs`): Collection of `TracedDecision` records, queryable and diffable. Enables causal replay and delta debugging.

- **`OperationResult<T>`** (`envelope/mod.rs`): Universal return type wrapping operation results with:
  - Value: The actual result
  - `OperationMetrics`: Duration, entity counts, predicate calls
  - `LineageDelta`: What entities were created/deleted
  - `DecisionLog`: All decisions made during the operation
  - `KernelWarning`: Non-fatal issues

### Key Concept: Structured Decision Tracing

Every operation returns an `OperationResult` that contains a complete audit trail. An AI agent can reconstruct the full state transition from the envelope alone.

---

## forge-topo

**Purpose**: Topology layer — immutable state, transactional mutations, Euler operators, and entity handles.

### Key Types

- **`TopologyState`** (`topology/state.rs`): Immutable topology state with epoch versioning:
  - `epoch`: Monotonically increasing version counter
  - `topology_version`: Bumped when connectivity changes
  - `geometry_version`: Bumped when positions change
  - `topology_hash`: Structural hash (Merkle-style) for change detection
  - `arena`: `Arc<TopologyArena>` holding all entity data

- **`MutableDraft`** (`topology/state.rs`): Transactional mutation handle (Doctrine D6):
  - **Commit**: Call `.commit()` → returns new `TopologyState`
  - **Rollback**: Drop without committing → all changes discarded
  - **Auto-validation**: `.commit()` runs topology validation

- **`TopologyArena`** (`arena/mod.rs`): Arena-based entity storage with generational handles:
  - `FaceData`, `HalfEdgeData`, `VertexData`, `LoopData`: Entity storage
  - Generational handles prevent stale reference bugs
  - Slots are reusable after deletion (generation bumped)

- **Entity Handles** (`topology/handles.rs`):
  - `FaceId`, `VertexId`, `HalfEdgeId`, `LoopId`: Typed generational handles
  - All handles validate generation before access

- **`EulerOperator`** (`topology/operations/operator.rs`): Trait for topology mutations:
  - `execute()`: Pure topology logic
  - `signature()`: Unique operation identifier
  - Never called directly — always via `apply_op()` runner

- **`apply_op()`** (`topology/operations/operator.rs`): The ONLY correct way to execute topology mutations:
  1. Logs operation start for replay (D1)
  2. Calls operator's `execute()`
  3. Updates lineage tracking
  4. Records `TracedDecision`
  5. Returns `OperationResult`

### Key Concept: Immutability + Transactions

`TopologyState` is immutable. The ONLY way to mutate is through `MutableDraft`, which auto-rolls back if dropped. This enables undo/redo (keep `Arc` references to old states) and ensures determinism.

---

## forge-geom

**Purpose**: Geometry layer — analytic surfaces, spatial structures, and curve representations.

### Key Types

- **`Plane`** (`primitives/plane.rs`): Plane representation with coefficients `[a, b, c, d]` where `ax + by + cz + d = 0`.

- **`PlaneSet`** (`spatial/bsp/mod.rs`): Lightweight test double implementing `GeometrySource` for plane lookups.

- **`BspTree`** (`spatial/bsp/mod.rs`): Binary Space Partition tree for spatial queries.

- **`BvhNode`** (`spatial/bvh/mod.rs`): Bounding Volume Hierarchy node for acceleration structures.

- **`LocalCoordinateSpace`** (`spatial/local_space.rs`): Local coordinate system with scale analysis for numerical stability.

- **`Vertex`** (`primitives/implicit_vertex.rs`): Implicit vertex representation.

### Key Concept: Geometry-Geometry Firewall

Geometry is a binding layer — it may be approximate, but it carries bounded error metrics and never corrupts topology (Doctrine D3). Geometry solvers accept `&dyn GeometrySource` for plane lookups, keeping topology types out of the geometry layer.

---

## forge-kernel

**Purpose**: Application layer — features, operations, and the modeling pipeline.

### Key Types

- **`ModelingContext`** (`core/context.rs`): Kernel-level orchestration:
  - Policy decisions (tolerance, tangency, sliver handling)
  - Decision logging
  - Tolerance configuration

- **`FeatureTree`** (`features/tree.rs`): Parametric feature tree powered by `forge-signal`:
  - Owns the `SignalGraph`
  - Maps `NodeId` → `NativeFeature`
  - Caches `FeatureOutput` results
  - Manages feature dependencies

- **`Feature`** (`features/traits.rs`): Trait for parametric features:
  - `evaluate()`: Pure function (output depends only on inputs)
  - `dependencies()`: List of input `NodeId`s
  - `name()`: Feature identifier

- **`FeatureOutput`** (`features/traits.rs`): Result of feature evaluation:
  - `topology`: `TopologyState` snapshot
  - `geometry`: `GeometryStore`
  - `decision_log`: `Arc<DecisionLog>`
  - `replay_log`: `Arc<ReplayLog>`
  - `lineage_events`: `Arc<Vec<LineageEvent>>`

- **`GeometryStore`** (`geometry_store/mod.rs`): Side-car storage for geometry:
  - Per-face planes
  - Per-vertex positions
  - Implements `GeometrySource` for geometry solvers

- **`BooleanInput`** (`operations/boolean/schema.rs`): Input to Boolean operations:
  - `target`: First solid
  - `tool`: Second solid
  - `op`: Union/Intersection/Subtraction

- **`BooleanResult`** (`operations/boolean/schema.rs`): Result of Boolean operation:
  - `topology`: Resulting `TopologyState`
  - `geometry`: Resulting `GeometryStore`
  - `face_classifications`: How each face was classified

### Boolean Operation Pipeline

1. **Split**: Split faces of both solids along mutual intersections
2. **Classify**: Label each split face as inside/outside the other solid (using `CertifiedTriSign`)
3. **Assemble**: Collect correct faces based on operation type

### Key Concept: Feature-Sliced Architecture

Each feature (Extrude, Boolean, Fillet) is a self-contained module following the Bento Box pattern:
- `schema.rs`: Data definitions
- `eval.rs`: Business logic
- `topo.rs`: Euler operator implementations
- `tests.rs`: Unit tests

---

## forge-signal

**Purpose**: Reactive signal graph for feature dependencies — enables parametric modeling.

### Key Types

- **`SignalGraph`** (`graph.rs`): Arena-based dependency graph:
  - Nodes are allocated with generational handles (`NodeId`)
  - Dependency edges track upstream → downstream relationships
  - Tombstoned nodes are skipped during traversal

- **`NodeId`** (`handles.rs`): Generational handle to a signal node (prevents stale references).

- **`NodeState`** (`schema.rs`): Three-state invalidation:
  - `Clean`: Value is current, no recomputation needed
  - `MaybeStale`: Transitive dependency changed — check before using
  - `Dirty`: Direct dependency changed — must recompute

- **`Aspect`** (`schema.rs`): Which aspect of a feature output a node subscribes to:
  - `Topology`: Connectivity, face count, etc.
  - `Geometry`: Positions, dimensions, etc.

- **`AspectVersion`** (`schema.rs`): Per-aspect version counters:
  - `topology`: u64
  - `geometry`: u64

- **`DependencyEdge`** (`schema.rs`): Records which `Aspect` a downstream node reads from an upstream node.

- **`NodeEntry`** (`schema.rs`): Internal storage for a signal node:
  - `state`: Current `NodeState`
  - `aspect_version`: Current `AspectVersion`
  - `dependencies`: Upstream edges
  - `subscribers`: Downstream nodes
  - `dep_snapshot`: Snapshot of upstream versions at last clean evaluation
  - `trace_summary`: Last evaluation trace (for diff-on-re-eval)

- **`EvaluationContext`** (`evaluation/context.rs`): Explicit context object (Doctrine D8):
  - Not thread-local — passed explicitly
  - Tracks evaluation state, cycle detection, etc.

### Evaluation Flow

1. **Push Phase** (`evaluation/push.rs`): `mark_dirty()` propagates dirty state synchronously:
   - Marks direct dependencies as `Dirty`
   - Marks transitive dependencies as `MaybeStale`
   - Detects cycles

2. **Pull Phase** (`evaluation/pull.rs`): `evaluate()` lazily recomputes:
   - Checks if node is `Clean` (version-gated skip)
   - If `MaybeStale`, checks upstream versions against snapshot
   - If `Dirty` or versions changed, recomputes
   - Updates `AspectVersion` and snapshot

### Key Concept: Multi-Aspect Versioning

Topology and geometry versions are independent. A geometry-only change (e.g., dragging an extrude depth) won't trigger re-evaluation of nodes that only subscribe to topology. This enables efficient incremental updates.

---

## forge-schema

**Purpose**: Versioned declarative command schema for the agent API.

### Key Types

- **`Command`** (`lib.rs`): Modeling command enum:
  - `AddBlock`: Create axis-aligned block
  - `AddHole`: Create cylindrical hole
  - `AddFillet`: Apply fillet to edges
  - `BooleanUnion`, `BooleanSubtract`: Boolean operations
  - `SetAttribute`: Set semantic attribute

- **`EntityRef`** (`lib.rs`): Reference to topological entity:
  - `ByFeature`: Reference by feature name + optional selector
  - `ByIndex`: Reference by sequential operation index

- **`EdgeSelector`** (`lib.rs`): Selector for edges in fillet/chamfer:
  - `AllEdges`: All edges of an entity
  - `IntersectionEdges`: Edges where two features meet

### Key Concept: Token Efficiency

Commands are designed for minimal token cost (~100-200 tokens vs ~2,000 for COM APIs). All types derive `Serialize`/`Deserialize` for JSON/Protobuf transport.

---

## forge-repr

**Purpose**: Representation traits for converting kernel geometry to visual representations.

### Key Types

- **`TriangleMesh`** (`schema/mod.rs`): Output container for tessellation.

- **`Viewable`** (`traits/mod.rs`): Trait for SDF (Signed Distance Field) representation.

- **`Tessellatable`** (`traits/mod.rs`): Trait for mesh generation.

### Key Concept: Separation of Concerns

The kernel and geometry solvers are NEVER aware of these traits — they are consumed by UI and export layers only.

---

## forge-io

**Purpose**: File format support.

### Key Types

- **`VersionedModel`** (`json/mod.rs`): Versioned JSON serialization format.

- **`IoError`** (`lib.rs`): IO error type:
  - `Io`: Standard IO error
  - `Json`: JSON serialization error
  - `VersionMismatch`: Schema version incompatibility

### Functions

- `save_model()`: Serialize model to JSON
- `load_model()`: Deserialize model from JSON
- `diff_models()`: Compare two models, return `ModelChange` list

---

## forge-test

**Purpose**: Test infrastructure.

### Modules

- **`fixtures`**: Reusable test fixtures and topology builders
- **`generators`**: Random polyhedra and Boolean pair generators
- **`harness`**: Self-consistency harness for corpus fuzzing
- **`logging`**: Universal test logging helpers

---

## forge-view

**Purpose**: Trace inspection infrastructure.

### Modules

- **`trace`**: In-memory trace store, REST API server, native egui viewer

### Binaries

- `forge-trace-server`: HTTP server on port 9091
- `forge-trace-viewer`: Native egui desktop app
- `forge-trace-cli`: CLI inspector for AI drill-down

---

## Architecture Summary

### Data Flow

1. **User/AI Agent** → `Command` (forge-schema)
2. **Command** → `Feature` evaluation (forge-kernel)
3. **Feature** → `SignalGraph` dependency resolution (forge-signal)
4. **Feature** → `EulerOperator` execution (forge-topo)
5. **EulerOperator** → `CertifiedTriSign` predicates (forge-math)
6. **Predicates** → `FilteredEval` pipeline (forge-math)
7. **Result** → `OperationResult` envelope (forge-core)
8. **Envelope** → `DecisionLog` tracing (forge-core)
9. **Final State** → `TopologyState` + `GeometryStore` (forge-topo + forge-kernel)

### Key Invariants

- **D1 (Determinism)**: Same inputs → same outputs. Always.
- **D2 (Explicit Policy)**: Ambiguous decisions escalate to policy, never silent failures.
- **D3 (Topology-Geometry Firewall)**: All topology decisions use `CertifiedTriSign`. Raw `f64` comparisons are compile errors.
- **D6 (Atomic Transactionality)**: All topology changes go through `MutableDraft`. Commit on success, drop to rollback.
- **D8 (Parallel Safety)**: Explicit context objects, not thread-local.

### Signal System Flow

1. **Feature Registration**: `FeatureTree::register_feature()` allocates `NodeId`, wires dependencies
2. **Dependency Change**: `mark_dirty()` propagates dirty state (push phase)
3. **Evaluation Request**: `FeatureTree::evaluate_feature()` triggers pull phase
4. **Pull Phase**: `evaluate()` lazily recomputes dirty nodes, checks versions
5. **Feature Execution**: `Feature::evaluate()` produces `FeatureOutput`
6. **Trace Flush**: `DecisionLog` → `TraceSummary` → `NodeEntry.trace_summary`

This architecture ensures correctness by construction, with every decision traceable and replayable.
