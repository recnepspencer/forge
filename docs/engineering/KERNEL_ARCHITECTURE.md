# Forge Kernel Architecture

**Status:** Living specification. Reflects implemented + in-progress state as of 2026-02-26.

> The Forge kernel is designed like a high-class domain-driven enterprise
> codebase: **declarative**, **explicit**, and **abstract**. Every feature
> is a contract. Every decision is logged. Every configuration is
> cascade-resolved. The goal is clean, maintainable code that scales from
> MakeCube to NURBS trim without ad-hoc wiring.

---

## 1. Design Principles

| Principle                       | What It Means                                                                                                                                                                                                                                                                                       |
| ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Declarative over imperative** | Features declare _what_ they need (policies, invariants, audit level) — the pipeline provides it. No manual boilerplate.                                                                                                                                                                            |
| **Explicit over implicit**      | All tolerance decisions are logged (D2). All config sources are traced via provenance. No hidden state.                                                                                                                                                                                             |
| **Abstract over concrete**      | Generic queries and utilities are shared infrastructure. Features adapt them through typed contracts. One pattern scales across every feature type.                                                                                                                                                 |
| **Clean breaks**                | No backwards-compatibility shims. When a pattern is replaced, the old code is deleted — not wrapped, not feature-flagged, not re-exported with a deprecation warning.                                                                                                                               |
| **Adapters, not monoliths**     | Core infrastructure (policy resolution, config cascade, tracing, tolerance queries) is general-purpose. Features don't call these systems with bespoke APIs — they declare what they need via contracts, and the pipeline adapts the general infrastructure to the feature's specific requirements. |

### Doctrine References

- **D0** — Topology-first: structure before geometry
- **D1** — Determinism: same input → same output, always
- **D2** — Explicit policy: every ambiguous decision is recorded
- **D3** — Certified firewall: topology decisions use `CertifiedTriSign`
- **D6** — Atomic via `MutableDraft`: operations are transactional

---

## 2. Crate Layering

```
forge-math          Pure math: Rational, linalg, orient3d
    ↓
forge-core          Shared types: errors, policy, tracing, envelope, tolerance
    ↓               TracedDecision, DecisionLog, DecisionId (proof substrate)
    ↓              ↘
forge-geom          forge-topo    Topology: arena, handles, state, draft, lineage, attributes
Geometry solvers        ↓         ReplayLog, LineageEvent, OpSignature (operation history)
    ↓           forge-spatial     Spatial queries: point-in-solid, AABB bounds, geometric validation
    ↘               ↘             integrity/ (gap, sliver, area, volume), classify/ (dual-path)
    ↘               ↘
forge-kernel        Feature pipeline, operations, geometry/brep stores
    ↓               proof/ (causal chain, counterfactual, region extractor, checkpoints)
    ↓
forge-io            Import/export (JSON, STEP)
forge-test          Integration tests
```

**Rules:**

- Lower crates never import higher crates
- `forge-core` defines shared _types_ (errors, policy, tracing schemas) — no geometry, no topology
- `forge-topo` owns the connectivity graph and all entity metadata (lineage, naming, attributes) — **no `forge-geom` dependency**
- `forge-spatial` is the exclusive home for functions that need **both** topology handles and geometry math
- `forge-kernel` owns domain logic, stores, and the feature pipeline
- `&dyn ToleranceProvider` and `&dyn GeometrySource` cross the crate boundary cleanly (D3 firewall)

### forge-spatial — The Spatial Query Adapter

`forge-spatial` is a bridge crate introduced to fix a layering violation: spatial
queries (point-in-solid classification, AABB bounds, geometric invariant validation)
require both topology handles (`FaceId`, `VertexId`) and geometry math (`forge-geom`).
Putting them in `forge-topo` would create an upward dependency on `forge-geom`,
which is forbidden.

**What lives in `forge-spatial`:**

| Module                       | Responsibility                                                                    |
| ---------------------------- | --------------------------------------------------------------------------------- |
| `classify/point_in_solid.rs` | Ray-casting parity classifier — point inside/outside/on-boundary a solid          |
| `classify/point_on_face.rs`  | Boundary proximity pre-check (vertex sphere, edge tube, face plane)               |
| `classify/sos.rs`            | Simulation of Simplicity predicates for degenerate orient2d/orient3d tie-breaking |
| `classify/schema.rs`         | `PointClassification`, `SpatialAccelerator` trait                                 |
| `bounds/face.rs`             | `face_bounds`, `all_face_bounds` — AABB per face                                  |
| `bounds/solid.rs`            | Hierarchical bounds — shell → region → lump → body                                |
| `integrity/area.rs`          | Zero-area face detection (using `compute_polygon_area`)                           |
| `integrity/edge_length.rs`   | Zero-length edge detection                                                        |
| `integrity/volume.rs`        | Shell signed-volume validation (outward-normal enforcement)                       |
| `integrity/sliver.rs`        | Sliver face detection — area below threshold (wraps `compute_polygon_area`)       |
| `integrity/gap.rs`           | Face-to-face gap measurement via Halton-sampled point projection                  |

**Rule: forge-topo vs forge-spatial**

> If a function touches **only** topology connectivity (handles, edges, loops,
> faces, valences) → it belongs in `forge-topo`.
>
> If a function touches **both** a topology handle (`FaceId`, `VertexId`) **and**
> a floating-point geometry calculation (distance, area, AABB, containment) →
> it belongs in `forge-spatial`.

**`forge-kernel` uses both directly** — `forge-topo` for structural mutation
(`MutableDraft`, Euler ops, lineage, attributes), `forge-spatial` (via `crate::spatial`
adapter module) for all geometry-dependent queries. There is no plan to make
`forge-spatial` the _only_ touchpoint to `forge-topo`; the two layers serve
completely different concerns.

**`grid_scale` threading:** `ToleranceConfig::get_spatial_hash_grid_scale()` (default `1e6`,
defined in `core::config::defaults::SPATIAL_HASH_GRID_SCALE`) is passed as an
explicit `f64` parameter to `forge_math::linalg::compute_spatial_hash` via
`forge_topo::ordering::compute_entity_spatial_hash(position, grid_scale)`. No
hardcoded constants anywhere below the kernel (D4: No Hardcoded Globals).

---

## 3. Core Infrastructure Catalog

These are the foundational building blocks that every layer of the kernel
depends on. They live in `forge-core` and `forge-topo` — below the kernel.

### 3.1 Typed Generational Handles (forge-topo)

Every entity is referenced by a generational handle: `FaceId`, `HalfEdgeId`,
`EdgeId`, `VertexId`, `LoopId`, `ShellId`, `BodyId`, `LumpId`, `RegionId`.
Parametric geometry uses separate handle types: `SurfaceRef`, `CurveRef`, `CoedgeRef`.

- Generational index prevents ABA problems on slot reuse
- `pack_handle(index, generation) -> u64` for HashMap keys
- Handles are `Copy + Eq + Hash + Debug` — zero-cost abstractions

### 3.2 TopologyArena — Radial-Edge Half-Edge Structure (forge-topo)

The arena stores all topological entities with explicit radial-edge pointers:

| Entity         | Key Fields                                                       |
| -------------- | ---------------------------------------------------------------- |
| `FaceData`     | outer_loop, inner_loops, shell, lineage, surface_ref             |
| `HalfEdgeData` | radial_next, next, prev, face, origin, edge, lineage, coedge_ref |
| `EdgeData`     | half_edge, curve_ref, lineage                                    |
| `VertexData`   | outgoing, lineage, provenance (3-plane intersection)             |
| `LoopData`     | half_edge, face                                                  |
| `ShellData`    | representative_face, kind (Solid/Sheet/Wire), region             |
| `RegionData`   | outer_shell, inner_shells, lump                                  |
| `LumpData`     | regions, body                                                    |
| `BodyData`     | lumps                                                            |

**Body → Lump → Region → Shell** hierarchy supports solids with internal
cavities. Every entity carries optional inline `Lineage` for provenance.

Opaque geometry references (`SurfaceRef`, `CurveRef`, `CoedgeRef`) on topology
entities maintain the D3 firewall — topology knows _that_ geometry is attached,
not _what_ it contains.

### 3.3 EntityBitset — Cache-Friendly Visited Sets (forge-topo)

Dense bitset over entity indices for O(1) membership testing:

```rust
let mut visited = EntityBitset::for_faces(arena);
visited.insert(face.index());
visited.union_with(&other);
for idx in visited.iter_ones() { /* ... */ }
```

Replaces `BTreeSet<u32>` in BFS traversals, validation sweeps, and
classification passes. One bit per entity — cache-friendly and allocation-free
after construction.

### 3.4 Provenance & Lineage (forge-topo, forge-core)

Every entity carries a `Lineage` from birth:

```rust
pub struct Lineage {
    pub ancestry_hash: u64,           // Merkle-DAG deterministic hash
    pub creation_op: OpSignature,     // Which operation created this entity
    pub origin_features: Vec<String>, // Feature chain (e.g. ["cube_1", "fillet_2"])
    pub parent_ancestry_hashes: Vec<u64>,
}
```

- `OpSignature` = (operation_name, invocation_id) — unique per operation call
- `LineageEvent` enum tracks entity creation/deletion/modification events
- `LineageStore` maps `EntityRef → Lineage` during a draft (live mutable state)
- `ReplayLog` + `ReplayEntry` enable deterministic replay with pre/post hash verification

Ancestry hashes are Merkle-DAG style: a child's hash incorporates its parents'.
This enables persistent naming (section 3.5) and replay verification (D1).

### 3.5 Persistent Naming (forge-topo)

Stable entity references that survive parametric rebuild:

```rust
pub struct PersistentName {
    pub ancestry_hash: u64,    // From Lineage at time-of-naming
    pub kind: EntityKind,
    pub ordinal: u32,          // Disambiguates splits
}

pub enum Selector {
    ByAncestry(u64),
    ByFeature(String),
    ByOperation(OpSignature),
    And(Box<Selector>, Box<Selector>),
    Or(Box<Selector>, Box<Selector>),
}
```

- `resolve_name(arena, name) -> Vec<EntityKey>` — 0 = deleted, 1 = normal, 2+ = split
- `resolve_selector(arena, selector) -> Vec<EntityKey>` — composable queries
- `assign_name(arena, key) -> PersistentName` — capture from live entity

### 3.6 Attributes — Semantic Tags (forge-topo)

Side-car metadata independent of connectivity:

```rust
pub enum TagValue { Text(String), Number(f64), Flag(bool) }
pub type SemanticTag = HashMap<String, TagValue>;
pub struct AttributeStore { /* EntityKey → SemanticTag */ }
```

Used for manufacturing tags (material, surface_finish, tolerance_class),
per-entity tolerances, and user-defined metadata. Query by entity or by tag name.

### 3.7 Policy System (forge-core)

Doctrine D2 in code. Geometry solvers return `PolicyResult<T>` instead of
plain `Result<T>` to distinguish clear math from ambiguity:

```rust
pub enum PolicyKind {
    CoincidentGeometry,
    NearTangency,
    SliverFace,
    GapClosure,
    PrecisionBudget,
}

pub struct PolicyQuery {
    pub kind: PolicyKind,
    pub location: EntityRef,
    pub margin: f64,          // Distance from decision boundary
    pub overridable: bool,
}

pub enum PolicyResult<T> {
    Success(T),
    Ambiguous { query: PolicyQuery, potential_value: T },
    HardError(KernelError),
}
```

Features declare which `PolicyKind` values they require via `FeatureContract::required_policies()`.
The pipeline pre-validates that all declared policies are configured before execution starts.

### 3.8 Error Hierarchy (forge-core)

Structured errors with machine-actionable remediation:

```rust
pub enum KernelError {
    TopologyViolation(TopologyError),
    AmbiguousResult(AmbiguousResult),
    ToleranceExceeded { /* ... */ },
    PrecisionEscalation { /* ... */ },
    InvalidInput { /* ... */ },
    InternalError { message: String, context: Option<ErrorContext> },
    DiagnosticFailure(DiagnosticPayload),
    ReplayMismatch { /* ... */ },
    MergeFailure(MergeError),
}

pub struct ErrorContext {
    pub scope: ErrorScope,              // Global, Feature, Entity, Operation
    pub suggested_fixes: Vec<SuggestedFix>,
    pub detail: Option<String>,
}

pub enum SuggestedFix {
    IncreaseThreshold(String, f64),
    ReduceValue(String, f64),
    RetryWithPolicy(PolicyKind),
    SplitOperation,
    ManualIntervention(String),
}
```

- `TopologyError` has specific variants: `EulerFormulaViolation`, `NonManifoldEdge`, `BrokenLoop`, etc.
- `MergeError` is a typed struct — never downgraded to `InternalError` string
- `SuggestedFix` enables AI-assisted remediation

### 3.9 Decision Tracing (forge-core)

Every kernel judgment call is recorded as a `TracedDecision`:

```rust
pub struct TracedDecision {
    pub id: DecisionId,
    pub kind: DecisionKind,      // Exact, PolicyApplied, NearBoundary, Forced
    pub tier: DecisionTier,      // Deterministic, Resolved, NearBoundary, PolicyApplied, Escalated
    pub context: DecisionContext,// Classification, Coincidence, Tolerance, Degeneracy
    pub margin: f64,             // Distance from decision boundary
    pub feature_scope: String,
    pub entity_scope: Option<EntityRef>,
    pub overridable: bool,
    pub span_id: Option<SpanId>,
    pub topology_delta: Option<TopologyDelta>,
}
```

- `DecisionTier` classifies significance — `Deterministic` (clear math) through `Escalated` (precision escalation triggered)
- `TopologyDelta` links decisions to the faces/edges/vertices they created or deleted
- `TraceEvent` enum (`Decision`, `StartSpan`, `EndSpan`) enables hierarchical span-based tracing

### 3.10 OperationResult Envelope (forge-core)

Universal metadata transport wrapping every operation:

```rust
pub struct OperationResult<T> {
    pub value: T,
    pub warnings: Vec<KernelWarning>,
    pub decision_log: DecisionLog,
    pub metrics: OperationMetrics,
    pub lineage_delta: LineageDelta,
    pub state_hash_before: Option<u64>,
    pub state_hash_after: Option<u64>,
    pub validation_results: Vec<ValidationResult>,
    pub accumulated_error_budget: f64,
}
```

- `KernelWarning`: `SliverFaceCreated`, `ShortEdgeCreated`, `AutoDecision`, `ErrorBudgetExceeded`
- `OperationMetrics`: duration, entities created/deleted/modified, exact predicate calls, policy decisions
- `LineageDelta`: summary of lineage changes per entity kind
- `absorb_metadata()` merges sub-operation audit data — nested composition
- `into_value()` auto-persists trace if `FORGE_TRACE_DIR` is set

### 3.11 ToleranceProvider (forge-core)

D3 firewall interface — topology/geometry solvers query tolerance without
owning geometric state:

```rust
pub trait ToleranceProvider {
    fn vertex_tolerance(&self, vertex_index: u32, vertex_generation: u32) -> f64;
    fn edge_tolerance(&self, edge_index: u32, edge_generation: u32) -> f64;
    fn global_default(&self) -> f64;
}
```

- `FlatToleranceProvider` — constant tolerance for all entities (fast path for planar ops)
- `GeometryState` implements `ToleranceProvider` with ISO 10303-42 scale-aware defaults
- Lower crates accept `&dyn ToleranceProvider` — never import `GeometryState` directly

### 3.12 DraftConfig — Transaction Configuration (forge-topo)

Controls behavior of `MutableDraft` transactions:

```rust
pub struct DraftConfig {
    pub per_op_hashing: bool,          // Compute hash after every Euler op
    pub deterministic_seed: u64,       // Base seed for reproducible RNG
    pub validation_level: ValidationLevel,
    pub per_op_validation: bool,       // Validate after every op (dev/CI only)
}
```

`per_op_hashing` enables full replay hash trails (D1). `per_op_validation` is
expensive and only used in debug/CI builds.

---

## 4. Three-Store Data Model

A solid in Forge is not a single blob — it is three independent stores with
distinct responsibilities, change rates, and precision requirements. This
separation is a foundational architectural decision.

```
┌─────────────────────────────────────────────────────────────┐
│  KernelState                                                │
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │ TopologyState│  │GeometryState│  │     BrepState       │ │
│  │             │  │             │  │                     │ │
│  │ Connectivity │  │ Face planes  │  │ NURBS surfaces     │ │
│  │ graph only — │  │ Vertex pos   │  │ Edge curves        │ │
│  │ no geometry  │  │ (exact +f64) │  │ UV trim coedges    │ │
│  │             │  │             │  │ Parametric bindings │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
│       D0               D3                NURBS-ready        │
└─────────────────────────────────────────────────────────────┘
```

### 4.1 TopologyState — Pure Structure (forge-topo)

The connectivity graph: faces, half-edges, edges, vertices, loops, shells,
solids. **No geometric data whatsoever.** This is Doctrine D0 in action.

- Lives in `forge-topo` — the lowest layer that can be used independently
- Handles are generational (`FaceId`, `VertexId`, etc.) for safe slot reuse
- Mutations go through `MutableDraft` → `commit()` (D6 atomic transactions)
- Validation at commit: Euler formula, edge uses, manifold checks

### 4.2 GeometryState — Planar Foundation (forge-kernel)

Side-car storage mapping topology handles to geometric meaning:

- **Face planes** — `HashMap<u64, Plane>` keyed by packed `(index, generation)`
- **Vertex positions** — `HashMap<u64, ExactPosition>` with dual representation:
  - `exact: [Rational; 3]` — exact arithmetic for topology decisions (D3)
  - `approx: [f64; 3]` — cached IEEE754 for BVH, AABB, rendering
  - `is_exact` flag distinguishes genuine exact results from f64-promoted values
  - `symbolic_planes` tracks the defining 3-plane intersection when known

**Key design decisions:**

- Implements `GeometrySource` (from `forge-math`) so `forge-geom` solvers
  can query planes without importing `GeometryState`
- Implements `ToleranceProvider` with ISO 10303-42 scale-aware defaults
- `GeometryView` trait abstracts over both `GeometryState` (immutable snapshot)
  and `GeometryPatch` (mid-transaction overlay) — callers don't know which
- Supports exact coordinate transforms via `LocalCoordinateSpace` (Rational arithmetic preserved through transform/inverse_transform)

### 4.3 BrepState — Parametric Boundary Representation (forge-kernel)

The NURBS/analytic layer, deliberately separated from `GeometryState` to
keep the planar foundation clean and fast:

- **Surfaces** — `Vec<BrepSlot<SurfaceData>>` with generational `SurfaceRef` handles
- **Curves** — `Vec<BrepSlot<CurveGeom>>` with generational `CurveRef` handles
- **Coedges** — `Vec<BrepSlot<Coedge>>` with generational `CoedgeRef` handles
- **Bindings** — face→surface, halfedge→coedge, edge→curve attachments

**Why separate from GeometryState?**

- Planar booleans (the 90% case) don't touch `BrepState` at all — zero overhead
- NURBS surface fitting, curve evaluation, and UV trimming are expensive;
  isolating them means planar code never pays for curved complexity
- `BrepState` can be empty for planar solids (and most current operations)
- Clean layering: `GeometryState` is the universal foundation, `BrepState` is
  the extension layer for advanced parametric geometry

### 4.4 Transactional Patches — Copy-on-Write During Operations

Each store has a transactional **patch** overlay for mid-operation mutations:

| Snapshot        | Patch           | Commit Behavior                                                 |
| --------------- | --------------- | --------------------------------------------------------------- |
| `TopologyState` | `MutableDraft`  | `commit()` validates Euler formula, returns new `TopologyState` |
| `GeometryState` | `GeometryPatch` | Insert/remove overlays → `commit()` applies to base             |
| `BrepState`     | `BrepPatch`     | Insert/remove overlays → `commit()` applies to base             |

`KernelDraft` bundles all three patches into a single transactional unit:

```rust
pub struct KernelDraft {
    draft: MutableDraft,
    geom_patch: GeometryPatch,
    brep_patch: BrepPatch,
    original_topo: TopologyState,  // for guaranteed-safe rollback
}

impl KernelDraft {
    pub fn as_parts_mut(&mut self) -> (&mut MutableDraft, &mut GeometryPatch, &mut BrepPatch);
    pub fn commit(self) -> Result<KernelState, KernelError>;
    pub fn rollback(self) -> KernelState;
}
```

**Invariant:** Commit or rollback is all-or-nothing across all three stores.
Drop without commit = automatic rollback. The original `TopologyState` is
stored internally so geometry/topology are always paired correctly.

### 4.5 FeatureOutput — The Domain Result

`FeatureOutput` carries all three stores as its domain payload:

```rust
pub struct FeatureOutput {
    pub topology: TopologyState,
    pub geometry: GeometryState,
    pub brep: BrepState,
}
```

No audit metadata here — that lives in the `OperationResult<FeatureOutput>`
envelope (see section 8). This separation means `FeatureOutput` is pure domain data:
serializable, cloneable, diffable.

**Already implemented:** All three stores, all three patches, `KernelDraft`,
`KernelState`, `BRepWorkspace`, `GeometryView` trait.

---

## 5. Query & Adapter Pattern

The kernel's infrastructure (policy resolution, config cascade, tolerance
queries, tracing) is **general-purpose**. Features don't call these systems
with bespoke APIs — they declare what they need, and the pipeline adapts.

```
┌──────────────────────────────────────────────────────┐
│  General Infrastructure (forge-core, forge-topo)      │
│                                                       │
│  PolicyQuery / PolicyResult<T>   ← any feature can    │
│  ToleranceProvider               ← query these        │
│  KernelSpan / TracedDecision     ← generically        │
│  ResolvedConfig                                       │
│  PersistentName / Selector                            │
│  Lineage / LineageStore                               │
│  AttributeStore                                       │
│  EntityBitset                                         │
└──────────────────┬───────────────────────────────────┘
                   │  Features adapt via contracts
                   │
┌──────────────────▼───────────────────────────────────┐
│  Feature Adapters (FeatureContract declarations)      │
│                                                       │
│  BooleanFeature:                                      │
│    required_policies: [CoincidentGeometry, ...]       │
│    entity_origins: [SplitOperator, MergeOperator]     │
│    surface_types: [Planar]                            │
│    config_overrides: Some(boolean-specific tweaks)    │
│                                                       │
│  FilletFeature:                                       │
│    required_policies: [NearTangency, SliverFace]      │
│    entity_origins: [EulerOperator]                    │
│    surface_types: [Cylinder, Sphere, Nurbs]           │
│    config_overrides: Some(fillet-specific tweaks)     │
└──────────────────────────────────────────────────────┘
```

**This is not just policies.** The adapter pattern applies to:

| General Infrastructure             | Feature Adapter                                                                                                |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `PolicyQuery` / `PolicyResult<T>`  | `FeatureContract::required_policies()` declares which policies; pipeline pre-validates them                    |
| `ResolvedConfig` cascade           | `FeatureContract::config_overrides()` injects feature-level overrides into the cascade                         |
| `InvariantKind` validation         | `FeatureContract::post_invariants()` selects which invariants to check post-execution                          |
| `KernelSpan` tracing               | `FeatureContract::audit_level()` controls trace granularity (None/Summary/Full)                                |
| `Lineage` / `OpSignature`          | `FeatureContract::entity_origins()` declares what kinds of lineage events this feature produces                |
| `SurfaceRef` / `CurveRef` bindings | `FeatureContract::surface_types()` declares expected geometry — validates B-Rep output                         |
| `PersistentName` / `Selector`      | Steps use `resolve_persistent_selection` — general naming infrastructure adapted per feature's selection needs |
| `ToleranceProvider`                | `StepContract::precision_sensitive()` flags steps that need scale-aware tolerance — pipeline provides it       |

The pipeline executor (`FeaturePipeline::execute`) reads these declarations and
wires the general infrastructure accordingly. Features never manually set up
tracing, resolve config, or validate policies — they declare, the pipeline provides.

**Already implemented:** `FeatureContract`, `StepContract`, `FeaturePipeline::execute`,
`declare_feature!`, `declare_step!`.

---

## 6. Runtime Abstractions

The kernel's runtime concerns collapse to two: _what settings to use_ and _what happened during execution_.

```
┌───────────────────────────────────────────────────┐
│  Pipeline Executor                                │
│                                                   │
│  1. Resolve config cascade → ResolvedConfig       │
│  2. Enter KernelSpan                              │
│  3. Execute feature/operation                     │
│  4. Exit span → collect DecisionLog + metrics     │
└───────────────┬───────────────────────────────────┘
                │
    ┌───────────┴───────────┐
    │                       │
 &ResolvedConfig         KernelSpan
 (explicit param,        (scope-based,
  read-only)              write-only)
```

### 6.1 KernelConfig — Declarative Configuration Root

One struct absorbs all scattered config/policy structs into a unified,
serializable root:

```rust
pub struct KernelConfig {
    pub tolerance: ToleranceSection,   // spatial, angular, sliver, gap, etc.
    pub policy: PolicySection,         // boolean fallback rules
    pub solver: SolverSection,         // iterative solver params
    pub validation: ValidationSection, // checkpoint config
    pub precision: PrecisionSection,   // bit-length threshold
}
```

**Key properties:**

- Every section implements `ConfigSection` (independent `defaults()` + `validate()`)
- All magic numbers live in `core::config::defaults` — single source of truth
- `Serialize + Deserialize` — future UI settings panel, JSON/TOML presets
- `UnitSystem` field prevents silent meters/mm scale mismatches on import

**Already implemented:** `KernelConfig`, all sections, `ConfigSection` trait, named defaults, `UnitSystem`.

### 6.2 Cascade Resolution

Configuration overrides follow a 4-level cascade (highest precedence last):

```
Session Default → Model Override → Feature Override → Operation Override
```

Each override level is **sparse** — only populated fields take effect:

```rust
pub struct ConfigOverride {
    pub tolerance: Option<ToleranceOverride>,
    pub solver: Option<SolverOverride>,
    pub validation: Option<ValidationOverride>,
    pub policy: Option<PolicyOverride>,
    pub precision: Option<PrecisionOverride>,
}
```

Resolution produces a **frozen `ResolvedConfig`** with provenance tracking:

```rust
pub struct ResolvedConfig {
    config: KernelConfig,          // effective values
    provenance: ConfigProvenance,  // which scope set each value
}

impl ResolvedConfig {
    pub fn source_of(&self, field: &str) -> Option<&ConfigSource>;
    pub fn scale_factor(&self) -> f64;
    pub fn scaled_vertex_tolerance(&self) -> f64;
    pub fn cross_validate(&self) -> Result<(), KernelError>;
}
```

Cross-validation catches inter-section invariant violations after cascade
(e.g., `max_gap_closure > spatial_tolerance * ambiguity_band_factor`).

**Already implemented:** `resolve_config()`, `cascade!` macro, `ResolvedConfig`, `ConfigOverride`, `ConfigProvenance`, `ConfigSource`, `ConfigScope`, cross-validation.

### 6.3 KernelSpan — Scope-Based Decision Logging

Replaces `&mut ModelingContext` for write-only logging (~40 of 50 call sites
were write-only). Uses `Arc<Mutex<SpanCollector>>` for thread-safe parallel
propagation.

```rust
pub struct KernelSpan;

impl KernelSpan {
    pub fn enter(name: &str) -> KernelSpanGuard;
    pub fn record_decision(decision: TracedDecision);
    pub fn record_warning(warning: KernelWarning);
    pub fn is_active() -> bool;
    pub fn current_handle() -> Option<KernelSpanHandle>;
    pub fn attach(handle: KernelSpanHandle) -> KernelSpanGuard;
}
```

**Parallel safety:** Worker threads receive a `KernelSpanHandle` (cloneable
`Arc`) and call `KernelSpan::attach()` — decisions flow back to the parent
span's collector. No zombie spans on Rayon workers.

**RAII guard** collects output on drop, restoring nested spans:

```rust
pub struct SpanOutput {
    pub decision_log: DecisionLog,
    pub warnings: Vec<KernelWarning>,
    pub metrics: OperationMetrics,
    pub lineage_delta: LineageDelta,
    pub config_snapshot: Option<ResolvedConfig>,
}
```

**Already implemented:** `KernelSpan`, `KernelSpanGuard`, `KernelSpanHandle`, `SpanOutput`, nested span support.

### 6.4 ModelingContext — Transitional Shell

`ModelingContext` still exists as a transitional wrapper around `KernelConfig`
plus decision logging state. As callers migrate to `&ResolvedConfig` +
`KernelSpan`, it shrinks.

**Migration path:**

1. Config reads: `ctx.get_tolerance_config()` → `&ResolvedConfig` param
2. Decision logging: `ctx.log_decision()` → `KernelSpan::record_decision()`
3. Eventually: `ModelingContext` becomes a thin construction helper

**Not a goal:** Preserving `&mut ModelingContext` in operation signatures.
The `&mut` threading tax exists only because logging was coupled to config
reading. With `KernelSpan` handling logging implicitly, operation functions
take `&ResolvedConfig` (explicit, read-only) and nothing else.

---

## 7. Three-Tier Feature Pipeline

Every feature — from MakeCube to NURBS Trim — goes through the same pipeline.
Simple features degrade to no-ops. The pipeline never invents its own config
system; it composes with `ResolvedConfig`, `KernelSpan`, `OperationFinalizer`,
and `KernelDraft`.

```
┌─────────────────────────────────────────────────────┐
│  Tier 0: Command Dispatch                           │
│  Bridges forge-schema Commands → FeatureTree        │
├─────────────────────────────────────────────────────┤
│  Tier 1: Feature Pipeline                           │
│  Compiler-enforced FeatureContract (sealed)         │
│  Typed inputs → execute → finalize → audit          │
├─────────────────────────────────────────────────────┤
│  Tier 2: Operation Pipeline                         │
│  StepContract-driven sub-operation sequencing       │
│  PipelineBuilder for typed intermediate state       │
├─────────────────────────────────────────────────────┤
│  Tier 3: Shared Steps + Shared Ops                  │
│  Reusable atomic operations across features         │
└─────────────────────────────────────────────────────┘
```

### 7.1 Tier 0: Command Dispatch

Bridges `forge-schema::Command` variants to `NativeFeature` construction
and `FeatureTree` insertion. One match arm per command variant — compiler
enforces exhaustiveness.

```rust
pub struct CommandDispatcher<'a> {
    tree: &'a mut FeatureTree,
}

impl<'a> CommandDispatcher<'a> {
    pub fn dispatch(&mut self, cmd: &Command) -> Result<NodeId, KernelError>;
}
```

**Already implemented:** `engine/dispatch.rs`.

### 7.2 Tier 1: Feature Contracts and Pipeline

#### FeatureContract (Sealed Supertrait)

Every feature must declare its contract. `Feature` requires `FeatureContract`
as a supertrait — no contract = compile error.

```rust
pub trait FeatureContract {
    fn feature_kind(&self) -> &'static str;
    fn required_policies(&self) -> &[PolicyKind];
    fn entity_origins(&self) -> &[EntityOriginKind];
    fn euler_ops(&self) -> &[EulerOpKind];
    fn surface_types(&self) -> &[SurfaceKind];
    fn post_invariants(&self) -> &[InvariantKind];
    fn audit_level(&self) -> AuditLevel;
    fn persistent_output(&self) -> bool { true }
    fn config_overrides(&self) -> Option<ConfigOverride> { None }
}
```

Implemented via `declare_feature!` macro:

```rust
declare_feature!(MakeCubeFeature,
    kind: "make_cube",
    policies: [],
    origins: [EntityOriginKind::EulerOperator],
    euler_ops: [EulerOpKind::MakeVertexFace, EulerOpKind::MakeEdgeFace],
    surfaces: [SurfaceKind::Planar],
    invariants: [InvariantKind::ManifoldEdges],
    audit: AuditLevel::Summary,
    persistent: true,
);
```

**Already implemented:** `FeatureContract`, `Feature` trait, `FeatureInputs`,
`declare_feature!`, `AuditLevel`, `InvariantKind`, `EntityOriginKind`,
`EulerOpKind`, `SurfaceKind`.

#### Feature Trait

```rust
pub trait Feature: FeatureContract + Debug + Any {
    type Inputs: FeatureInputs;
    fn parse_inputs(&self, raw: &HashMap<NodeId, FeatureOutput>) -> Result<Self::Inputs, KernelError>;
    fn execute_typed(&self, inputs: &Self::Inputs, ctx: &mut ModelingContext) -> Result<FeatureOutput, KernelError>;
    fn dependencies(&self) -> Vec<NodeId>;
    fn name(&self) -> &str;
}
```

All audit metadata (decisions, warnings, metrics, lineage, hashes, error
budget) lives in `OperationResult<FeatureOutput>` — no `Arc<DecisionLog>`
or `Arc<ReplayLog>` on the output struct.

**Already implemented:** `Feature`, `FeatureOutput`, `MakeCubeFeature`,
`BooleanFeature`.

#### FeaturePipeline Executor

7-stage lifecycle:

1. **Resolve config** — cascade `KernelConfig` + feature overrides → `ResolvedConfig`
2. **Pre-validate policies** — fail-fast before any topology mutation
3. **Parse + validate inputs** — typed DTOs with semantic validation
4. **Snapshot topology hash** — before execution, for hash boundary tracking
5. **Execute** — business logic with active `KernelSpan`
6. **Post-validate invariants** — only on success, respects `ValidationConfig`
7. **Finalize + audit** — drain decisions into `OperationResult` envelope

```rust
impl FeaturePipeline {
    pub fn execute<F: Feature>(
        feature: &F,
        raw_inputs: &HashMap<NodeId, FeatureOutput>,
        ctx: &mut ModelingContext,
    ) -> Result<OperationResult<FeatureOutput>, KernelError>;
}
```

**Already implemented:** `engine/executor.rs`.

#### Adding a New Feature (Compiler-Enforced Chain)

1. Add `NativeFeature::Fillet(FilletFeature)` → compiler forces all `match` arms
2. `impl Feature for FilletFeature` → requires `FeatureContract` (supertrait)
3. `FeatureContract` → forces declaring policies, invariants, audit level, origins
4. `Feature::Inputs` → forces typed input struct + `parse_inputs` + `validate`

### 7.3 Tier 2: Operation Pipeline and Step Contracts

For multi-step operations within a feature. Two APIs:

**`OperationPipeline::run_step`** — for simple cases or complex data deps:

```rust
impl<'a> OperationPipeline<'a> {
    pub fn run_step<S, R, F>(&mut self, step: &S, execute: F) -> Result<R, KernelError>
    where
        S: StepContract,
        F: FnOnce(&mut ModelingContext) -> Result<R, KernelError>;
}
```

Each step gets: policy pre-validation, decision checkpointing, trace span,
and audit entry collection — automatically.

**`PipelineBuilder::then`** — for linear pipelines with typed state threading:

```rust
let (result, audit) = PipelineBuilder::start(ctx, selection)
    .then(&ResolveSelection, |sel, ctx| resolve_edge_chain(&sel, ctx))?
    .then(&ClassifyConvexity, |edges, ctx| classify_convexity(&edges, ctx))?
    .then(&ConstructSurface, |conv, ctx| construct_blend(&conv, radius, ctx))?
    .then(&ApplyEulerOps, |blend, ctx| apply_fillet_topology(&blend, ctx))?
    .then(&ValidateManifold, |topo, ctx| validate_manifold(&topo).map(|_| topo))?
    .finish();
```

Steps are declared via `declare_step!`:

```rust
declare_step!(ClassifySurfacePair,
    name: "classify_surface_pair",
    policies: [PolicyKind::CoincidentGeometry, PolicyKind::NearTangency],
    precision_sensitive: true,
);
```

**Already implemented:** `StepContract`, `declare_step!`, `OperationPipeline`,
`PipelineBuilder`, `StepAuditEntry`, `OperationAuditRecord`.

### 7.4 Tier 3: Shared Steps and Shared Ops

Cross-feature reusable code lives in two places, distinguished by granularity:

#### `shared_steps/` — Pipeline-Aware Reusable Steps

Each is a `declare_step!` invocation + implementation. These are _pipeline-level_
units: they have step contracts, produce audit entries, and participate in
the pipeline's tracing and policy framework.

| Step                           | Policies                           | Precision | Used By                                           |
| ------------------------------ | ---------------------------------- | --------- | ------------------------------------------------- |
| `resolve_persistent_selection` | []                                 | false     | Boolean, Fillet, Chamfer, Shell, Extrude, Pattern |
| `classify_surface_pair`        | [CoincidentGeometry, NearTangency] | true      | Boolean, Fillet, Chamfer                          |
| `classify_edge_convexity`      | [NearTangency]                     | true      | Fillet, Chamfer                                   |
| `certify_boundary`             | [CoincidentGeometry]               | true      | Boolean, Fillet, Chamfer, Shell                   |
| `construct_surface`            | []                                 | true      | Fillet, Chamfer, Shell, Extrude                   |
| `apply_euler_ops`              | []                                 | false     | All (D6 transactional)                            |
| `validate_manifold`            | []                                 | false     | All                                               |
| `detect_slivers`               | [SliverFace]                       | true      | Boolean, Fillet, Chamfer                          |

**Status:** Step declarations exist in `operations/pipeline/steps.rs`. When
the step library grows, each step gets its own file in `operations/shared_steps/`.

#### `shared_ops/` — Pure Algorithmic Utilities

Cross-operation algorithms that are too high-level for `forge-geom`/`forge-topo`
but shared across multiple features. These are **not** pipeline steps — they are
pure functions or small structs with no step contract, no audit, no tracing.

| Utility                          | Purpose                                   | Used By               |
| -------------------------------- | ----------------------------------------- | --------------------- |
| `VertexMatchKey`                 | Exact-rational vertex dedup across solids | Boolean, Pattern      |
| `build_face_coincidence_prepass` | BVH-accelerated face pairing              | Boolean, Fillet       |
| `compute_face_centroid`          | Face centroid for spatial queries         | Boolean, Mesh builder |

**Rule:** If two features need the same algorithm, it goes in `shared_ops/`.
If two features need the same _pipeline step_ (with contracts, policies,
audit), it goes in `shared_steps/`.

Boolean's internal phases (EMBER → split → classify → assemble) are NOT
shared steps. They are internal to boolean and live in `operations/boolean/`.
The shared layers capture only _cross-feature_ operations.

**Already implemented:** `shared_ops/vertex_identity.rs`, `shared_ops/coincidence.rs`,
`shared_ops/centroid.rs`.

---

## 8. OperationResult as Canonical Envelope

`OperationResult<T>` is the universal metadata transport. The pipeline uses
it — never works around it.

```
Feature::execute_typed  →  Result<FeatureOutput, KernelError>
                              (domain result only)

FeaturePipeline::execute  →  OperationResult<FeatureOutput>
                              (envelope: decisions, metrics, warnings,
                               lineage, hashes, error budget)
```

- `FeatureTree` stores `OperationResult<FeatureOutput>` per node
- Sub-operation absorption works envelope-to-envelope
- `OperationFinalizer` drains `ModelingContext` → envelope exactly once
- No `Arc<DecisionLog>` or `Arc<ReplayLog>` on `FeatureOutput`

---

## 9. Error Taxonomy

Two levels of error types serve different purposes:

**`KernelError`** (forge-core) — the universal error type across all crates:
structured variants with `ErrorContext`, `SuggestedFix`, and `DiagnosticPayload`.
See section 3.8 for the full hierarchy.

**`PipelineError`** (forge-kernel) — pipeline-specific failures that wrap into
`KernelError`:

```rust
pub enum PipelineError {
    PolicyNotConfigured { kind: PolicyKind, feature: String },
    InvariantViolation { kind: InvariantKind, detail: String },
    InputParseFailure { expected: String, actual: String },
    InputValidationFailure { message: String },
    StepExecutionFailed { step: String, source: Box<KernelError> },
}
```

Wraps via `impl From<PipelineError> for KernelError` with `ErrorContext`.

**Already implemented:** `engine/errors.rs`, `forge-core/src/errors/schema.rs`.

---

## 10. Directory Layout

```
forge-kernel/src/
│
├── engine/                          ← Feature evaluation engine (Tier 0+1)
│   ├── mod.rs                       ← Table of Contents
│   ├── traits.rs                    ← Feature trait, FeatureOutput
│   ├── tree.rs                      ← FeatureTree, NativeFeature enum, SignalGraph
│   ├── intent.rs                    ← PrimitiveSpec (dual SDF/B-Rep)
│   ├── contract.rs                  ← FeatureContract, AuditLevel, InvariantKind, etc.
│   ├── executor.rs                  ← FeaturePipeline::execute (7-stage lifecycle)
│   ├── macros.rs                    ← declare_feature! macro
│   ├── invariants.rs                ← validate_invariant dispatch
│   ├── dispatch.rs                  ← Tier 0: CommandDispatcher
│   ├── errors.rs                    ← PipelineError taxonomy
│   ├── wrappers.rs                  ← MakeCubeFeature, BooleanFeature
│   └── tests.rs
│
├── geometry_state/                  ← GeometryState (planar foundation)
│   ├── mod.rs                       ← GeometryView trait
│   ├── schema.rs                    ← GeometryState, ExactPosition
│   ├── eval.rs                      ← Position lookup helpers
│   ├── patch.rs                     ← GeometryPatch (transactional overlay)
│   ├── coalescence.rs               ← Vertex snap/coalesce
│   └── split_propagation.rs         ← Curve propagation on edge splits
│
├── brep/                            ← BrepState (parametric extension)
│   ├── state.rs                     ← BrepState (surfaces, curves, coedges)
│   └── patch.rs                     ← BrepPatch (transactional overlay)
│
├── core/
│   ├── kernel_state.rs              ← KernelState (Topo + Geom + Brep bundle)
│   ├── kernel_draft.rs              ← KernelDraft (three-patch transaction)
│   ├── brep_workspace.rs            ← BRepWorkspace (lifecycle wrapper)
│   ├── config/
│   │   ├── schema.rs                ← KernelConfig, sections, ConfigSection trait
│   │   ├── defaults.rs              ← All named constants (single source of truth)
│   │   ├── overrides.rs             ← Sparse ConfigOverride
│   │   ├── provenance.rs            ← ConfigScope, ConfigSource, ConfigProvenance
│   │   └── resolve.rs               ← resolve_config(), ResolvedConfig
│   ├── context/
│   │   ├── schema.rs                ← ModelingContext (transitional)
│   │   ├── accessors.rs             ← Policy getters/setters
│   │   ├── decision_logging.rs      ← Decision recording + spans
│   │   ├── policy_resolution.rs     ← 4-layer cascade resolution
│   │   ├── sub_operations.rs        ← Sub-operation metadata absorption
│   │   ├── counterfactual.rs        ← Classification overrides for replay
│   │   └── topology_delta.rs        ← Arena snapshots + entity tracking
│   ├── tracing/
│   │   └── span.rs                  ← KernelSpan, KernelSpanGuard, KernelSpanHandle
│   └── finalization.rs              ← OperationFinalizer, TopologyHashBoundary
│
├── shared_ops/                      ← Cross-operation pure algorithms
│   ├── mod.rs
│   ├── vertex_identity.rs           ← VertexMatchKey (exact dedup)
│   ├── coincidence.rs               ← BVH face coincidence prepass
│   └── centroid.rs                  ← Face centroid computation
│
├── operations/
│   ├── pipeline/                    ← Step-level pipeline (Tier 2+3)
│   │   ├── step_contract.rs         ← StepContract, declare_step!
│   │   ├── builder.rs               ← OperationPipeline, PipelineBuilder
│   │   ├── steps.rs                 ← Step library declarations (Tier 3)
│   │   └── tests.rs
│   │
│   ├── boolean/                     ← Boolean operations
│   │   ├── contract.rs              ← ParametricBooleanContract, EmberBooleanContract
│   │   ├── schema.rs                ← BooleanInput/Output types
│   │   ├── test_helpers.rs          ← Centralized test fixtures
│   │   ├── parametric/              ← split → classify → assemble → postprocess
│   │   ├── ember/                   ← BSP quantize → merge → extract
│   │   ├── shared/                  ← Boolean-internal shared utilities
│   │   ├── debug/                   ← Debug/diagnostic helpers
│   │   ├── brutality/               ← Adversarial test suites (MB1-MB8)
│   │   └── tests/                   ← Unit tests
│   │
│   ├── fillet/                      ← (placeholder)
│   ├── chamfer/                     ← (placeholder)
│   ├── shell/                       ← (placeholder)
│   ├── extrude/                     ← (placeholder)
│   ├── loft/                        ← (placeholder)
│   └── sweep/                       ← (placeholder)
│
├── proof/                           ← Decision query & replay infrastructure
│   ├── causal_chain/                ← Causal chain reconstruction from decision + lineage logs
│   │   ├── schema.rs                ← CausalChain, CausalStep, ChainSummary
│   │   └── eval.rs                  ← query_causal_chain, query_causal_summary
│   ├── counterfactual/              ← Decision replay with forced overrides
│   │   ├── schema.rs                ← CounterfactualResult, DecisionOverride, EntityDelta
│   │   └── eval.rs                  ← replay_decision, replay_all_near_boundary
│   ├── region_extractor/            ← N-ring BFS extraction for minimal repro cases
│   │   ├── schema.rs                ← ExtractedRegion, SerializedPlane, SerializedHalfEdge
│   │   └── eval.rs                  ← extract_n_ring
│   ├── checkpoint/                  ← P0.5 invariant checkpoint system
│   │   ├── schema.rs                ← ValidationConfig, ValidationCheckpoint, ValidationResult
│   │   └── diagnose.rs              ← PipelineStage, PipelineDiagnostic (non-fatal mid-pipeline)
│   ├── invariants.rs                ← Structural proof validators (INV-1/2/3: hash chain, lineage, causal)
│   └── tests/                       ← PV suites (pv_p0_*, pv_p2_*, pv_p3_*)
│
├── mesh_builder/                    ← Primitive construction
├── primitives/                      ← Primitive features
└── queries/                         ← Spatial queries
```

### Where New Code Goes

| Scenario                             | Location                                                                     |
| ------------------------------------ | ---------------------------------------------------------------------------- |
| New feature (e.g. Fillet)            | `operations/fillet/` with `contract.rs`, `eval.rs`, `tests.rs`               |
| Algorithm shared by 2+ features      | `shared_ops/` — pure function, no step contract                              |
| Pipeline step shared by 2+ features  | `operations/pipeline/steps.rs` (or `shared_steps/` when the file gets large) |
| Boolean-internal algorithm           | `operations/boolean/shared/` — not exported                                  |
| New policy kind                      | `forge-core/src/policy/schema.rs` — compiler forces all match arms           |
| New invariant kind                   | `engine/contract.rs` + one match arm in `engine/invariants.rs`               |
| New error variant                    | `forge-core/src/errors/schema.rs` with `ErrorContext`                        |
| New config section                   | `core/config/schema.rs` + `core/config/defaults.rs`                          |
| New proof layer validator            | `proof/invariants.rs` — pure validation function                             |
| New checkpoint type                  | `proof/checkpoint/schema.rs` — add `ValidationCheckpoint` variant            |
| Proof query/reconstruction tool      | `proof/` subdirectory with `schema.rs` + `eval.rs`                           |
| Proof validation test suite (PV)     | `proof/tests/` — PV-prefixed test files                                      |
| Operation-specific replay logic      | `operations/{feature}/shared/counterfactual.rs`                              |
| Spatial integrity query (gap/sliver) | `forge-spatial/integrity/` — **not** in kernel                               |
| Dual-path classifier                 | `forge-spatial/classify/` — winding number, cross-check                      |

---

## 11. Architecture Rules

### Data Model

- **DO:** Keep `GeometryState` and `BrepState` as independent stores — planar ops never touch `BrepState`
- **DO:** Use `GeometryView` trait when code should work with both snapshots and patches
- **DO:** Use `KernelDraft` for all transactional mutations (topology + geometry + brep atomically)
- **DON'T:** Put NURBS/curve data in `GeometryState` — that's what `BrepState` is for
- **DON'T:** Access `ExactPosition.approx` for topology decisions — use `exact()` via `classify_point_exact`
- **DON'T:** Put geometry data in `TopologyState` — structure only (D0)

### Adapters & Sharing

- **DO:** Declare feature needs via `FeatureContract` — let the pipeline adapt infrastructure
- **DO:** Put cross-feature algorithms in `shared_ops/` (pure functions, no pipeline coupling)
- **DO:** Put cross-feature pipeline steps in `shared_steps/` (with `StepContract` and audit)
- **DON'T:** Duplicate algorithms between features — extract to `shared_ops/` or `shared_steps/`
- **DON'T:** Put feature-internal code in shared directories

### Config

- **DO:** Pass `&ResolvedConfig` to functions that need tolerance values
- **DON'T:** Pass `&mut KernelConfig` — config is immutable during execution
- **DO:** Copy `f64` values to stack locals before hot loops
- **DO:** Destructure `ResolvedConfig` into individual `f64` params at the `forge-geom`/`forge-topo` boundary
- **DON'T:** Import `ResolvedConfig` in lower crates

### Logging

- **DO:** Use `KernelSpan::record_decision()` for write-only logging
- **DO:** Use `KernelSpan::handle()` + `attach()` for parallel work
- **DON'T:** Thread `&mut` just for logging

### Serialization

- **DO:** `KernelConfig` and `ConfigOverride` are `Serialize + Deserialize`
- **DO:** `ResolvedConfig` tracks provenance
- **DON'T:** Serialize `ResolvedConfig` — serialize the inputs (config + overrides)

### Proof Infrastructure

- **DO:** Treat the `DecisionLog` as the universal proof substrate — all proof layers consume it
- **DO:** Put proof schemas (`CausalChain`, `CounterfactualResult`, etc.) in `proof/` subdirectory `schema.rs` files — keep types importable without pulling in algorithms
- **DO:** Make counterfactual replay generic — accept operation-specific replay functions as closures, not hardcoded boolean-only logic
- **DO:** Run proof checkpoints through the existing `InvariantKind` + `ValidationConfig` system
- **DO:** Put spatial measurement queries (gap, sliver, area) in `forge-spatial/integrity/`
- **DON'T:** Build operation-specific replay logic into the generic counterfactual module — each feature provides its own `ReplayFn`
- **DON'T:** Run manifold invariants at NMT-intermediate checkpoints — respect `TopologyMode`
- **DON'T:** Store proof metadata in the topology arena — proof results flow through `OperationResult<T>`
- **DON'T:** Put proof query infrastructure (causal chain, counterfactual, region extractor) in a generic "analysis" directory — it is first-class proof infrastructure

### Clean Breaks

- **DON'T:** Keep deprecated wrappers around old APIs
- **DON'T:** Re-export deleted types with deprecation attributes
- **DON'T:** Add feature flags for old vs new behavior
- **DO:** Delete old code when the replacement is tested and working

---

## 12. Scalability: Feature Roadmap

The pipeline supports the full roadmap without architectural redesign:

| Feature        | Contract                                                                          | Pipeline Style                     | Step Library Usage                  |
| -------------- | --------------------------------------------------------------------------------- | ---------------------------------- | ----------------------------------- |
| **MakeCube**   | `policies: [], invariants: [ManifoldEdges]`                                       | No sub-steps                       | `validate_manifold`                 |
| **Boolean**    | `policies: [CoincidentGeometry, ...], invariants: []`                             | Wraps `execute_boolean`            | Pre/post only                       |
| **Fillet**     | `policies: [NearTangency, SliverFace], invariants: [ManifoldEdges, G1Continuity]` | 6-step `PipelineBuilder`           | Full step library                   |
| **Chamfer**    | Same as fillet minus G1                                                           | Same pipeline shape                | Same steps                          |
| **Shell**      | `policies: [CoincidentGeometry], invariants: [ManifoldEdges]`                     | 4-step                             | Selection, surface, Euler, validate |
| **Extrude**    | `policies: [], invariants: [ManifoldEdges]`                                       | 3-step                             | Selection, surface, Euler           |
| **Loft/Sweep** | `policies: [NearTangency]`                                                        | Multi-step with surface fitting    | `construct_surface` handles NURBS   |
| **NURBS Trim** | `policies: [CoincidentGeometry, NearTangency]`                                    | Multi-step                         | 2D boolean on parametric surface    |
| **Pattern**    | Inherits from source                                                              | Iterates source feature's pipeline | Delegates to source                 |

### What Grows Additively

- `InvariantKind` enum — new variant + one match arm in `validate_invariant`
- `PolicyKind` enum — compiler forces all match arms
- Shared step library — new `declare_step!` + implementation
- `shared_ops/` utilities — new file per algorithm
- `PipelineBuilder` composite states — explicit about data dependencies

### What Doesn't Change

- Internal operation architecture (Boolean keeps EMBER/parametric)
- Three-store data model (`TopologyState` + `GeometryState` + `BrepState`)
- Transactional commit/rollback via `KernelDraft`
- Surface/curve representation dispatch
- Precision escalation path
- `OperationSpace` coordinate transforms

---

## 13. Future: Deterministic Parallelism

Sequential execution is correct for determinism. Parallelism is achievable
when features demand it (e.g., multi-edge fillet):

**Strategy:** Fork-then-merge with deterministic key sorting.

```rust
let per_edge_contexts: Vec<_> = edge_chains
    .par_iter()
    .map(|chain| {
        let _guard = KernelSpan::attach(span_handle.clone());
        let result = classify_edge_chain(chain, &config);
        (chain.ordering_key(), result)
    })
    .collect();

// Merge in deterministic order
per_edge_contexts.sort_by_key(|(key, _)| *key);
```

Requires `ModelingContext::fork_local()` / `merge_local()` (~200 LOC).
Built when the first parallel feature demands it.

---

## 14. Vision: Spec Graph & Reactive Signals

The long-term architecture is a **spec graph** as the single source of truth:

- A DAG of features, constraints, and decisions — serialized as JSON, git-diffable
- Every computed value is a dependency-tracked node in a **reactive signal graph**
  (already implemented: `FeatureTree` uses `SignalGraph` for invalidation)
- Every classification, tolerance judgment, and precision escalation is recorded
  with margin metrics — enabling AI-assisted remediation and counterfactual replay
- The full decision trace is machine-readable: an agent can identify a failed
  classification, adjust the policy, and replay the operation

This is not a future rewrite — it's the direction the existing infrastructure
converges toward. `FeatureTree` + `OperationResult` + `KernelSpan` +
`ReplayLog` + `PersistentName` are the building blocks already in place.

---

## 15. Proof System Architecture

The proof system transforms the kernel's write-only decision log into a
**queryable, replayable, and certifiable** evidence chain. It serves three
audiences with the same infrastructure: kernel engineers (debug + regression),
aerospace/defense (compliance audit trail), and AI agents (self-inspection +
trust). The `DecisionLog` and `TracedDecision` (already in `forge-core`) are
the substrate; everything else is built on top.

### 15.1 Five-Layer Model

Proof is layered — each layer validates orthogonal properties with independent
algorithms. A failure in one layer does not invalidate the others.

| Layer | Name                      | Primary Crate(s)                        | What It Validates                                                |
| ----- | ------------------------- | --------------------------------------- | ---------------------------------------------------------------- |
| 1     | Topological Invariants    | `forge-topo` + `forge-kernel/proof/`    | Euler formula, manifoldness, orientation, loop closure           |
| 2     | Dual-Path Verification    | `forge-spatial/classify/`               | Independent algorithm agreement (ray casting vs. winding number) |
| 3     | Redundant Numerical Modes | `forge-math` + `forge-kernel` (policy)  | Float vs. interval vs. rational result comparison                |
| 4     | Causal Replay & Witnesses | `forge-kernel/proof/`                   | Decision trace queries, counterfactual replay, region extraction |
| 5     | Self-Consistency Fuzzing  | `forge-kernel/proof/tests/` (MB series) | Algebraic identity testing at scale (A∪B=B∪A, A∩∅=∅)             |

**Layer independence is an invariant.** Layer 4 (causal replay) must never
depend on Layer 2 (dual-path), and vice versa. Each layer can be tested,
disabled, and extended in isolation.

### 15.2 How Proof Maps to the Crate Hierarchy

Proof types are **distributed across layers** because the layering rules
demand it — lower crates never import higher crates. This is intentional,
not accidental:

| Type                                          | Lives In              | Why                                                    |
| --------------------------------------------- | --------------------- | ------------------------------------------------------ |
| `TracedDecision`, `DecisionLog`, `DecisionId` | `forge-core`          | Universal proof substrate — used by every layer        |
| `ReplayLog`, `LineageEvent`, `OpSignature`    | `forge-topo`          | Topology operation history — topo owns its own lineage |
| `CausalChain`, `CausalStep`, `ChainSummary`   | `forge-kernel/proof/` | Bridges topo lineage + core decisions                  |
| `ExtractedRegion`                             | `forge-kernel/proof/` | Uses `GeometryState` (kernel-owned)                    |
| `CounterfactualResult`, `DecisionOverride`    | `forge-kernel/proof/` | Uses `BooleanInput` (kernel-owned)                     |
| `ValidationConfig`, `ValidationCheckpoint`    | `forge-kernel/proof/` | Configuration is kernel-level                          |
| `ValidationResult`                            | `forge-kernel/proof/` | Pipeline-coupled result type                           |

### 15.3 Pipeline Integration — Checkpoints

Proof validation hooks into the feature pipeline through `ValidationCheckpoint` +
`ValidationConfig`. The checkpoint system is config-driven:

```rust
pub enum ValidationCheckpoint {
    PostCommit,       // After every topology commit
    PostBoolean,      // After boolean operations
    PostImport,       // After STEP/JSON import
    OnDemand,         // Explicit API call
}

pub struct ValidationConfig {
    checkpoints: Vec<ValidationCheckpoint>,  // Which checkpoints are active
    include_geometric: bool,                 // Include expensive geometric checks?
    entity_limit: usize,                     // Skip validation on large models (0=no limit)
}
```

`ValidationResult` entries flow into `OperationResult<T>.validation_results` —
the existing envelope absorbs proof data without structural changes.

**Critical rule — NMT-intermediate checkpoints:**

> Proof checkpoints must respect the current topology mode. Layer 1 manifold
> invariants are enforced only at manifold-strict checkpoints (post-commit on
> final results, post-import). NMT-intermediate checkpoints validate only
> structural invariants (twin reciprocity, loop closure, Euler formula per-shell).
> Running manifold checks during boolean intermediates produces false positives.

The `diagnose_arena()` function exists specifically for this: it runs non-fatal
structural diagnostics at intermediate pipeline stages without aborting.

### 15.4 Causal Chain — Provenance Reconstruction

`query_causal_chain(target, replay_log, decision_log, lineage_events)` walks
the operation history from present to origin, collecting every `CausalStep`
that created or modified the target entity. The result is:

- An ordered list of `CausalStep` entries with operation signatures, decisions, and topology hashes
- A `ChainSummary` budgeted to < 200 tokens for agent consumption
- The tightest margin across all decisions (the riskiest judgment in the entity's history)

This is the query layer that makes the decision log _useful_ — it transforms
a flat log into entity-centric provenance graphs.

### 15.5 Counterfactual Replay — "What If?"

`replay_decision(input, original_log, original_hash, override)` re-executes
an operation with a forced classification override and compares the result:

- `DecisionOverride` specifies which decision to flip (e.g., Inside→Outside)
- The replay produces a `CounterfactualResult` with the original and counterfactual topology hashes
- `EntityDelta` reports how many faces/edges/vertices changed
- `CounterfactualValidation` reports whether the counterfactual topology is structurally valid

**Architectural note:** Counterfactual replay is currently boolean-specific
(it imports `BooleanInput`). As features grow, the replay mechanism should
become generic — accept an operation-specific `ReplayFn` closure rather than
hardcoding boolean logic. Each feature provides its own replay implementation.

### 15.6 Region Extraction — Minimal Reproduction

`extract_n_ring(arena, geometry_state, seed_face, depth)` BFS-expands from a
seed face through edge adjacency, collecting all faces, halfedges, vertices,
and their geometry into a self-contained, serializable `ExtractedRegion`.

This enables:

- **Delta-debug:** Shrink a failing 1000-face model down to the 5-face neighborhood around the bug
- **Standalone test cases:** Serialize the region to JSON, reconstruct a `TopologyArena`, replay
- **Fuzzer isolation:** Extract the minimal region that reproduces a fuzzer-found failure

### 15.7 What Grows Additively

| Growth                       | Mechanism                                          |
| ---------------------------- | -------------------------------------------------- |
| New proof layer              | New subdirectory under `proof/`, new PV test suite |
| New checkpoint type          | `ValidationCheckpoint` variant + one match arm     |
| New structural invariant     | New `validate_*` function in `proof/invariants.rs` |
| New operation's replay logic | `operations/{feature}/shared/counterfactual.rs`    |
| New MetaBoss test series     | New PV test file in `proof/tests/`                 |
| Dual-path classifier         | New file in `forge-spatial/classify/`              |

---

## 16. Edge Cases

| Edge Case             | Mitigation                                                                              |
| --------------------- | --------------------------------------------------------------------------------------- |
| **Parallelism**       | `Arc<Mutex<SpanCollector>>` + `handle()/attach()`                                       |
| **Undo/Redo**         | `SpanOutput` stores `config_snapshot` — replay uses exact original config               |
| **Unit Mismatch**     | `UnitSystem` field + `cross_validate()`                                                 |
| **Stale Override**    | `resolve_config()` validates override sources are live                                  |
| **Config Drift**      | `ResolvedConfig` frozen at span entry — in-flight ops immune                            |
| **Local Tolerances**  | Per-entity tolerances in `forge-topo` `AttributeStore`, gated by `KernelConfig::policy` |
| **Entity Splits**     | `PersistentName::resolve` returns `Vec` — 0 = deleted, 2+ = split                       |
| **Replay Divergence** | `ReplayLog` stores pre/post hashes — mismatch → `ReplayMismatch` error                  |

---

## 17. Performance

Pipeline overhead per step: <100ns (one policy lookup + one checkpoint +
one `Vec::push`). Less than 4 `orient3d` predicate calls. Negligible even
for NURBS operations with 50+ steps.

---

## 18. What Was Replaced

| Before                                      | After                                                                   |
| ------------------------------------------- | ----------------------------------------------------------------------- |
| 7 scattered config structs                  | `KernelConfig` with 5 sections                                          |
| `&mut ModelingContext` in 50+ signatures    | `&ResolvedConfig` (~6 sites) + `KernelSpan` (implicit)                  |
| `FeatureOutput` carrying `Arc<DecisionLog>` | `OperationResult<FeatureOutput>` envelope                               |
| Ad-hoc per-feature wiring                   | `FeaturePipeline::execute` 7-stage lifecycle                            |
| Manual policy pre-checks                    | `FeatureContract::required_policies()` auto-validated                   |
| Manual invariant checks                     | `FeatureContract::post_invariants()` auto-validated                     |
| `features/` directory                       | `engine/` directory (clean break)                                       |
| Geometry + B-Rep data in one store          | `GeometryState` (planar) + `BrepState` (parametric) — separate stores   |
| Single patch type                           | `GeometryPatch` + `BrepPatch` + `MutableDraft` bundled in `KernelDraft` |
| Duplicated algorithms across features       | `shared_ops/` (pure) + `shared_steps/` (pipeline-aware)                 |
| Bespoke per-feature infrastructure wiring   | Adapter pattern via `FeatureContract` declarations                      |
