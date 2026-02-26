# Forge Architectural Design Document

### Components, Dependencies, and Open Decisions

### February 2026

---

# 1. What This Document Is

This is the pre-spec. It works out the architecture before we commit to 500
milestones. It answers: what are the pieces, how do they connect, what's
decided, what isn't.

Everything here should be challenged. The full engineering spec gets written
after these decisions are locked.

---

# 2. The Core Insight That Drives Everything

A CAD model is not a mesh. It's not a B-rep. It's a **specification** — a
directed acyclic graph of features, constraints, and design decisions. The
B-rep, the SDF preview, the STEP export — those are all derived projections
of the specification. They can be recomputed. The spec is the truth.

This has three consequences that shape the entire architecture:

1. **The spec graph is what lives in git.** It's JSON (or similar). You can
   diff it, merge it, branch it. Two engineers work on different subsystems,
   merge back, and the merge is a graph operation — not a binary blob collision.

2. **Every derived projection is a reactive signal.** Change a parameter, and
   only the affected signals recompute. The topology might not change. The SDF
   updates in milliseconds. The B-rep materializes in the background. This is
   why the signal architecture isn't an optimization — it's the skeleton.

3. **Every decision that produces the derived state is traced.** Every
   classification, every tolerance judgment, every policy application is a
   `TracedDecision` with a margin metric, a provenance chain, and the ability
   to replay with mutated inputs. This is how you debug NURBS edge cases with
   agent swarms. It's also how you sell to Lockheed.

---

# 3. Component Map

```
PERSISTENCE & INTERFACE
┌─────────────────────────────────────────────────────────┐
│ forge-cli          Agent/human command interface        │
│ forge-ui           SDF renderer, interactive editing    │
│ forge-io           STEP, IGES, STL, 3MF import/export  │
│ forge-persist      Git-native spec serialization        │
└──────────────────────────┬──────────────────────────────┘
                           │
KERNEL                     │
┌──────────────────────────┼──────────────────────────────┐
│ forge-kernel       Features, Booleans, fillets          │
│                    The "application" crate that          │
│                    orchestrates everything below         │
│                          │                              │
│ forge-decision     Tracing, policy, replay, causal      │
│                    chains, counterfactual engine         │
│                          │                              │
│ forge-signal       Reactive dependency graph,           │
│                    topology/geometry firewall            │
└──────────────────────────┬──────────────────────────────┘
                           │
FOUNDATION                 │
┌──────────────────────────┼──────────────────────────────┐
│ forge-topo         Halfedge mesh, Euler operators,      │
│                    immutable state, generational IDs     │
│                                                         │
│ forge-geom         Surfaces, curves, intersections,     │
│                    coedges, tolerant entities,           │
│                    analytic arbitration                  │
│                                                         │
│ forge-math         Predicates, filtered arithmetic,     │
│                    interval, rational, certified signs   │
└─────────────────────────────────────────────────────────┘
```

Dependency rule: arrows point down only. Nothing in foundation knows about
the kernel. Nothing in the kernel knows about persistence or UI.

---

# 4. Foundation Crates

## 4.1 forge-math

The leaf crate. No internal dependencies.

**What it provides:** Certified arithmetic. Every topological decision in the
kernel flows through here. The core guarantee: you get a `CertifiedTriSign`
(Neg/Zero/Pos) and the sign is _mathematically correct_, not approximately
correct.

**The filtered predicate pipeline:**

```
┌─────────────────────────────────────────────────┐
│ Input: raw f64 coordinates                      │
│                                                 │
│ Stage 1: f64 + Shewchuk error bound    (~5ns)   │
│   → sign resolved? return CertifiedTriSign      │
│   → inconclusive? ↓                             │
│                                                 │
│ Stage 2: compensated (double-double)   (~50ns)   │
│   → sign resolved? return CertifiedTriSign      │
│   → inconclusive? ↓                             │
│                                                 │
│ Stage 2.5: interval arithmetic         (~100ns)  │
│   → sign resolved? return CertifiedTriSign      │
│   → interval contains zero? ↓                   │
│                                                 │
│ Stage 3: SPLIT — depends on geometry class       │
│                                                 │
│ Stage 3A: Planar/Analytic — Exact Rational       │
│   (BigInt P/Q arithmetic)              (~500ns)  │
│   → ALWAYS resolves. return CertifiedTriSign    │
│   Scope: orient2d, orient3d, point_vs_plane,    │
│          plane-plane intersection, all linear    │
│          equations. Rational arithmetic works    │
│          because planes are linear (P/Q covers   │
│          the entire solution space).             │
│                                                 │
│ Stage 3B: Curved/Freeform — Arbitrary Precision  │
│   Float (MPFR-style, up to 512-bit mantissa)     │
│   bounded by interval/affine arithmetic          │
│   → resolved? return CertifiedTriSign           │
│   → hits fuel cap and still ambiguous? ↓        │
│   → yield to tracing engine, emit               │
│     TracedDecision::PolicyApplied                │
│     (e.g., "Force Tangent Coincidence")          │
│                                                 │
│ WHY THE SPLIT:                                   │
│ Exact rational (BigInt) is IMPOSSIBLE for        │
│ general curved geometry. Cylinder-sphere         │
│ intersections contain √2 (irrational).           │
│ Abel-Ruffini theorem: polynomials of degree ≥ 5  │
│ have no closed-form roots. NURBS SSI is a        │
│ massively high-degree algebraic system.           │
│ Passing NURBS through BigInt → infinite           │
│ bit-growth → kernel hangs forever.               │
│                                                 │
│ Stage 3A is guaranteed termination (linear).      │
│ Stage 3B is bounded termination (fuel cap +       │
│ policy escalation).                              │
└─────────────────────────────────────────────────┘
```

Stage 2.5 (interval) is specifically for curved geometry where you're
evaluating surface equations rather than pure determinants. For the classic
planar predicates (orient2d, orient3d), stages 1→2→3A suffice and are
guaranteed exact. For curved geometry ("is this point inside this cylinder's
trim region"), the pipeline goes 1→2→2.5→3B, where 3B uses arbitrary
precision floats with a hard fuel cap — never BigInt rationals.

**Core types:**

```rust
pub enum TriSign { Neg, Zero, Pos }

/// Only constructible inside forge-math. Topology functions accept ONLY this.
pub struct CertifiedTriSign(pub(crate) TriSign);

/// Interval with tracked error bounds
pub struct Interval { pub lo: f64, pub hi: f64 }

/// Exact rational number (BigInt/BigInt) — PLANAR ONLY
pub struct Rational { ... }

/// Arbitrary precision float for curved geometry — NEVER exact, but bounded
pub struct MpFloat { mantissa: BigInt, exponent: i64, precision_bits: u32 }

/// Which precision stage resolved a decision
pub enum PrecisionMode {
    Float64,
    Compensated,
    Interval,
    ExactRational,       // Stage 3A — planar only, always resolves
    ArbitraryPrecision,  // Stage 3B — curved, bounded by fuel cap
    PolicyFallback,      // Stage 3B exhaust — yielded to policy engine
}

/// Attached to every certified result — tells you how it was computed
pub struct PrecisionCertificate {
    pub resolved_at: PrecisionMode,
    pub float_agreed: bool,        // did the fast path get it right?
    pub margin: f64,               // how close to zero (for risk ranking)
    pub bits_consumed: Option<u32>, // for 3A/3B: how much precision was needed
}
```

**Core predicates:**

- `orient2d(a, b, c)` — sign of 2D triangle
- `orient3d(a, b, c, d)` — sign of tetrahedron (the workhorse)
- `in_sphere(a, b, c, d, e)` — insphere test
- `point_vs_plane(point, plane)` — the most common classification

**Settled decisions:**

- [x] ~~BigInt library~~ → **`num-bigint`** (pure Rust). Stage 3A fires <1%
      of calls on small numbers (<200 bits) — the 10x GMP speed gap is
      irrelevant. Portability wins (WASM, Windows, no C deps).
- [x] ~~Shewchuk predicates~~ → **Full port** of Shewchuk's C code to Rust.
      Gives us full control over all predicates and extensibility for custom
      ones. The `robust` crate covers basics but can't be extended.
- [x] ~~MPFR binding for Stage 3B~~ → **Deferred** to Phase 4+. Curved
      geometry doesn't exist yet. When needed, try pure Rust first
      (`astro-float` / `dashu-float`); add `rug` as optional feature gate
      only if profiling shows it's too slow.
- [x] ~~Bit-growth budget for rational arithmetic~~ → Resolved by the 3A/3B
      split. Stage 3A (rational) is for linear equations only — bit-growth is
      bounded by the number of input planes. Stage 3B (arbitrary precision float)
      has a hard fuel cap on precision bits (default 512) and yields to policy
      if exhausted.

---

## 4.2 forge-topo

**Purpose:** The topological data structure. Pure connectivity — which faces
exist, how edges link them, how loops bound faces. Knows _nothing_ about
where things are in space.

**Depends on:** `forge-math` (for hashing only, not predicates)

**Core types (✔ exists, partially implemented):**

```rust
/// Immutable topology snapshot. The only way to read the mesh.
pub struct TopologyState {
    pub epoch: u64,
    pub topology_version: u64,   // bumps when connectivity changes
    pub geometry_version: u64,   // bumps when embedding changes
    pub topology_hash: u128,     // Merkle-style structural hash
    arena: Arc<TopologyArena>,
}

/// Transactional mutation handle. Commit or drop (auto-rollback).
pub struct MutableDraft { ... }

/// Generational handles (thunderdome). Stale access = immediate panic.
pub struct FaceId(pub(crate) thunderdome::Index);
pub struct EdgeId(pub(crate) thunderdome::Index);   // ← NEW: undirected 3D edge
pub struct VertexId(pub(crate) thunderdome::Index);
pub struct HalfEdgeId(pub(crate) thunderdome::Index);
pub struct LoopId(pub(crate) thunderdome::Index);
```

**Entity data (what's stored per entity):**

```rust
pub struct FaceData {
    pub outer_loop: LoopId,
    pub inner_loops: SmallVec<[LoopId; 2]>,  // holes in the face
    pub surface: SurfaceRef,                  // → forge-geom (opaque ID)
    pub lineage: Lineage,
}

/// The undirected 3D boundary between two faces.
/// Owns only topological connectivity — the representative HalfEdgeId.
///
/// Geometric edge data (3D curve + tolerance tube) lives in
/// `forge-geom::CurveGeom`, mirroring how `VertexGeom` holds vertex
/// positions and tolerance spheres. `EdgeData` holds an opaque `CurveRef`
/// only as a cross-crate lookup key — it does NOT own or compare f64 values.
pub struct EdgeData {
    pub halfedge: HalfEdgeId,   // one halfedge in the radial ring (entry point)
    pub curve: Option<CurveRef>,// opaque ID → forge-geom::CurveGeom (None = planar)
    pub lineage: Lineage,
    // NO tolerance here. Tube radius lives in forge-geom::CurveGeom.tolerance.
}

/// The directed 2D boundary. Each Edge has a ring of HalfEdges linked
/// via `radial_next`. For manifold edges (the common case), the ring has
/// exactly 2 halfedges. For non-manifold edges (3+ faces sharing an edge),
/// the ring is longer. For boundary edges (open shells), `radial_next == self`.
pub struct HalfEdgeData {
    pub edge: EdgeId,           // parent undirected edge
    pub next: HalfEdgeId,
    pub prev: HalfEdgeId,
    pub radial_next: HalfEdgeId, // next halfedge in the radial ring (NMT-capable)
    pub face: FaceId,
    pub vertex: VertexId,       // vertex at the START of this halfedge
    pub coedge: Option<CoedgeRef>,  // UV-space curve on this face's surface
    pub direction: bool,        // true = aligned with Edge's 3D curve direction
    pub lineage: Lineage,
}

pub struct VertexData {
    pub outgoing: HalfEdgeId,   // one outgoing halfedge (arbitrary choice)
    pub lineage: Lineage,
    // NO position stored here. Position is derived from geometry.
}

pub struct LoopData {
    pub first_edge: HalfEdgeId,
    pub face: FaceId,
}
```

**Key design: vertices have no coordinates, edges have no curves (in topology).**
For planar geometry, a vertex's position is the intersection of 3+ planes —
computed on demand from the original plane coefficients. For curved geometry,
a vertex's position is stored in `forge-geom::VertexGeom` (with a tolerance
sphere), and an edge's 3D curve geometry + tolerance tube are stored in
`forge-geom::CurveGeom`. `forge-topo` holds only opaque `CurveRef` handles
— it never stores, reads, or compares `f64` geometry values.

**Geometry mirror types in forge-geom:**

```rust
// forge-geom — mirrors VertexGeom for edges
pub struct CurveGeom {
    /// The 3D curve parametric definition (None = planar implicit intersection)
    pub kind: Option<CurveKind>,
    /// Certified error bound — the 3D tube of uncertainty around this edge
    pub tolerance: f64,
    /// How this edge geometry was created and its tolerance derived
    pub provenance: CurveProvenance,
}
```

Tolerance tube radius is always read from `CurveGeom.tolerance` via the
`ToleranceProvider` trait — never stored in the topo arena.

**Why Edge is a first-class entity:** In flat polygon meshes, edges are
implicit (just a straight line between two vertices). In a curved B-Rep, an
edge has its own 3D geometry (e.g., a NURBS curve) and its own 3D tolerance
tube. Without `EdgeId`, the 3D curve would need to live on `HalfEdgeData` —
but that duplicates it across twins and creates asymmetric state. `EdgeId`
provides the shared key; `HalfEdgeData` owns the per-face UV coedge and
directional orientation. This is the standard ACIS/Parasolid/OpenCASCADE
pattern.

**Euler operators:** `split_edge`, `join_faces`, `make_vertex_face`,
`kill_edge_vertex`, etc. All are pure functions:
`(MutableDraft, params) → Result<(), TopologyError>`. All go through
`apply_op()` which logs for replay.

**Validation:** `validate_topology()` checks:

- Radial ring closure (every halfedge's `radial_next` ring returns to start)
- Previous consistency (`he.prev.next == he`)
- Vertex continuity (`next(he).origin` is a valid edge endpoint)
- Vertex outgoing (`v.outgoing.origin == v`)
- Loop closure (following `next` returns to start)
- Hierarchy integrity (Face→Shell→Region→Lump→Solid chain)
- Euler formula: V - E + F = 2 - 2G + R
- Shell consistency (solid shells have no boundary edges)
- Manifold enforcement (solid shell edges have valence ≤ 2)
- Consistent orientation

**Manifold Policy Doctrine (D8):**

The kernel is **2-manifold by default, NMT-aware by data structure**.

- `radial_next` supports radial rings of arbitrary length, enabling NMT
  representation during intermediate construction phases (e.g., boolean
  face classification, multi-body imprinting).
- **Solid shells** enforce the 2-manifold invariant at commit time:
  every edge must have radial valence exactly 2. Edges with valence > 2
  cause `TopologyError::NonManifoldEdge` at `MutableDraft::commit()`.
- **Open shells** allow boundary edges (valence 1, `radial_next == self`)
  and manifold edges (valence 2). Valence > 2 is still rejected.
- Wire edges (antennae from `MakeEdgeVertex`, where both halfedges share
  the same face) are explicitly exempted from manifold checks — they are
  valid topological construction features, not manifold defects.
- Euler operators (`JoinFaces`, `KillEdgeMakeLoop`) enforce `valence == 2`
  as a precondition because they are defined only for manifold edges.
  This is correct — they reject early rather than corrupting topology.
- Future NMT support (honeycombs, sheet-metal mid-surfaces) will
  generalize the Euler operators to accept radial insertion parameters
  and relax the commit-time check per-shell.

**Settled decisions:**

- [x] ~~Structural sharing for TopologyState~~ → **Clone-on-write** for now.
      Models are <1k entities. Signal graph memoizes outputs so snapshots are less
      frequent than expected. Revisit persistent HAMT (`im` crate) only if
      profiling shows memory pressure on real models (Phase 5+).
- [x] ~~SurfaceRef / CoedgeRef handle type~~ → **Generational handles**
      (thunderdome). Same pattern as topology handles. ABA safety is
      non-negotiable — silent corruption in geometry references is
      undebuggable.

---

## 4.3 forge-geom

**Purpose:** All geometry. Surfaces, curves, intersections, evaluation. This
is the most complex crate and where the hybrid pipeline lives.

**Depends on:** `forge-math`

**This crate does NOT depend on forge-topo.** Geometry doesn't know about
halfedges. It provides surfaces and curves that topology _references_ via
opaque IDs. This separation is critical — it means you can swap geometry
implementations without touching topology.

### 4.3.1 The Geometry Store

Everything lives in an arena, similar to topology:

```rust
pub struct GeometryStore {
    surfaces: Arena<SurfaceData>,
    curves: Arena<CurveData>,
    vertices: Arena<VertexGeom>,
}
```

**Surfaces** store their parametric definition, never evaluated coordinates:

```rust
pub enum SurfaceKind {
    // Analytic — closed-form evaluation, exact derivatives
    Plane { normal: [f64; 3], offset: f64 },
    Cylinder { origin: [f64; 3], axis: [f64; 3], radius: f64 },
    Cone { apex: [f64; 3], axis: [f64; 3], half_angle: f64 },
    Sphere { center: [f64; 3], radius: f64 },
    Torus { center: [f64; 3], axis: [f64; 3], major_r: f64, minor_r: f64 },

    // Freeform — numerical evaluation
    Nurbs(NurbsSurface),
}

pub struct SurfaceData {
    pub kind: SurfaceKind,
    pub id: SurfaceId,
    pub domain: ParameterDomain,   // valid (u,v) range
}
```

This is the Gemini doc's first principle: the parametric definition is
separated from any 3D evaluated position.

**Curves** follow the same pattern:

```rust
pub enum CurveKind {
    Line { origin: [f64; 3], direction: [f64; 3] },
    Circle { center: [f64; 3], normal: [f64; 3], radius: f64 },
    Ellipse { center: [f64; 3], major: [f64; 3], minor: [f64; 3] },

    Nurbs(NurbsCurve),

    // Symbolic — the Gemini doc's "lazy parametric evaluation"
    // NOT discretized into points. Stored as the intersection of two surfaces.
    // BUT: immediately computes a cached SP-curve approximation (see below).
    SurfaceIntersection {
        surface_a: SurfaceId,
        surface_b: SurfaceId,
        /// Tightly bounded polynomial approximation (B-spline)
        /// computed at creation time. Downstream broad-phase AABB,
        /// rendering, and non-topological queries evaluate the
        /// SP-curve (fast). Only rigorous topological stitching
        /// falls back to the exact symbolic surfaces (slow, bounded).
        sp_curve_cache: SpCurveApproximation,
    },
}

/// Lazy-but-Cached SP-curve approximation for SurfaceIntersection curves.
///
/// The algebraic complexity explosion problem: if you Boolean A∩B to get
/// Symbolic Edge 1, then Boolean C through Edge 1, the solver must now find
/// roots of three simultaneous surfaces. By the 4th Boolean, evaluating a
/// single point requires solving a 5D non-linear system. Performance drops
/// from milliseconds to hours.
///
/// The fix: SurfaceIntersection is Lazy but Cached.
pub struct SpCurveApproximation {
    /// B-spline control points approximating the true intersection curve
    pub control_points: Vec<[f64; 3]>,
    /// Knot vector
    pub knots: Vec<f64>,
    /// Certified maximum deviation from the true symbolic curve
    pub error_bound: f64,
    /// Parameter range
    pub domain: (f64, f64),
}
```

**Why SurfaceIntersection curves matter:** When a third Boolean step needs to
intersect a new surface with an existing edge, it can evaluate the intersection
of _three abstract surfaces_ directly — instead of intersecting against a
lossy polyline approximation of the edge. This prevents the catastrophic error
accumulation that kills chained Booleans.

**Why the SP-curve cache is mandatory:** Without it, the algebraic complexity
explodes exponentially. Each chained Boolean adds another surface to the
system. The SP-curve gives downstream consumers a fast, bounded-error
polynomial to evaluate against, while the symbolic definition remains
available for the rare cases that require exact refinement (topological
stitching, near-tangency classification). This is the same lazy-but-cached
pattern used in production kernels for exactly this reason.

### 4.3.2 Coedges (UV-Space Anchoring)

This is the Gemini doc's most critical insight for surviving curved Booleans.

Every edge in the model has _two_ coedges — one for each adjacent face. A
coedge is a 2D curve in the face's parameter space (u, v).

```rust
pub struct Coedge {
    /// The edge's path in this face's (u,v) parameter space
    pub uv_curve: ParametricCurve2D,
    /// The face's surface this coedge is anchored to
    pub surface: SurfaceId,
}
```

**Why this saves you:** After 6 chained Booleans on curved surfaces, 3D
floating-point drift causes edges to separate from their surfaces — the edge
"floats" above the surface by 1e-8, then 1e-6, then it's a visible gap. The
UV coedge _cannot_ separate from the surface because it's defined _in the
surface's parameter space_. Evaluate `surface.point_at(u, v)` and you're on
the surface by definition.

Topological stitching happens in UV space, where "exactness" is far more
achievable even for curved surfaces.

For planar geometry, coedges are trivial (straight lines in UV). They
become essential when curved surfaces enter the picture.

### 4.3.3 Tolerant Vertices

Here's the reconciliation between the Gemini doc (you need tolerant entities)
and the old spec (ban mutable tolerance — it causes creep).

```rust
pub struct VertexGeom {
    /// Best-known 3D position
    pub position: [f64; 3],

    /// Certified error bound — this is a SPHERE of uncertainty
    pub tolerance: f64,

    /// How this vertex was created and how its tolerance was determined
    pub provenance: VertexProvenance,
}

pub struct VertexProvenance {
    /// Which surfaces defined this vertex (for re-derivation)
    pub defining_surfaces: SmallVec<[SurfaceId; 3]>,
    /// Which precision stage resolved the position
    pub precision: PrecisionCertificate,
    /// The traced decision (for audit, replay, counterfactual)
    pub decision_id: DecisionId,
}
```

**Tolerance governance: coalescence over write-once.** The original spec
proposed write-once tolerance (a vertex gets its tolerance at creation and
it never widens). This sounds mathematically pure but generates slivers.

**The sliver problem:** If Vertex A has a locked tolerance of `1.0e-7`, and
a subsequent Boolean cut passes exactly `1.01e-7` away from Vertex A, the
write-once rule forces creation of a new Vertex B. The result: a microscopic
edge of length `0.01e-7`. Slivers are fatal — they cause normal-vector
singularities, ruin STEP exporters, and cause subsequent Booleans to hang in
infinite subdivision loops because the features are smaller than machine
epsilon.

**The fix: Tolerance Coalescence.** Tolerance _can_ widen, but only via an
explicit, policy-governed `ToleranceCoalescence` decision:

```rust
pub enum DecisionKind {
    // ... existing variants ...

    /// Two vertices were too close to coexist (gap < sliver_area_min).
    /// Both were destroyed and replaced with a single merged vertex
    /// at a slightly wider tolerance. Fully traced and auditable.
    ToleranceCoalesced {
        consumed_vertices: SmallVec<[VertexId; 2]>,
        new_vertex: VertexId,
        old_tolerances: SmallVec<[f64; 2]>,
        new_tolerance: f64,
    },
}
```

**The coalescence protocol:**

1. A new intersection creates a candidate vertex near an existing vertex.
2. If the gap is less than the `sliver_area_min` policy threshold, the engine
   does NOT create a second vertex.
3. Instead, it destroys both vertices, creates a single merged vertex with a
   slightly wider tolerance sphere that encompasses both original positions.
4. The decision is logged as `DecisionKind::ToleranceCoalesced` with full
   provenance: which vertices were consumed, what the old/new tolerances
   were, and the gap distance.
5. If the gap is _above_ the sliver threshold, normal snapping rules apply
   (snap if within tolerance sphere, create new vertex otherwise).

This maintains strict traceability (every widening is a logged, auditable
policy decision) without shattering the topology into micro-slivers. The key
invariant: tolerance can only grow, never shrink, and every growth event is a
first-class `TracedDecision`.

### 4.3.4 Analytic Arbitration

Before the numerical solver runs, the engine compares _abstract surface
definitions_:

```rust
pub enum SurfaceRelation {
    /// Parameters match within machine epsilon — same surface
    Coincident,
    /// Surfaces are analytically known to not intersect (e.g., parallel planes)
    Disjoint,
    /// Must run numerical intersection
    General,
}

pub fn classify_surface_pair(a: &SurfaceData, b: &SurfaceData) -> SurfaceRelation {
    // Compare parametric definitions directly.
    // Two cylinders with same origin, axis, radius → Coincident.
    // Two parallel planes → Disjoint.
    // Everything else → General.
}
```

When surfaces are `Coincident`, the Boolean engine skips numerical
intersection entirely and falls back to deterministic tie-breaking (lower
`SurfaceId` wins). The Boolean becomes a pure 2D graph merge in parameter
space. This is how you survive the coplanar/co-cylindrical cases that destroy
other kernels.

### 4.3.5 Surface-Surface Intersection

For `General` surface pairs, we need actual intersection computation. This
is organized by how hard it is:

**Tier 1: Exact or semi-exact (analytic pairs)**

- Plane-Plane → line (exact)
- Plane-Cylinder → line or ellipse (exact)
- Plane-Sphere → circle (exact)
- Plane-Cone → conic section (exact)
- Sphere-Sphere → circle (exact)

**Tier 2: Hard analytic (the cylinder-cylinder problem)**

- Cylinder-Cylinder → degree-4 space curves
- This single case causes more kernel failures industry-wide than any other.
- Five sub-cases: skew axes, parallel axes, intersecting axes, coaxial,
  near-tangent.
- Topology classification FIRST (how many curves, what connectivity),
  geometry extraction SECOND (approximate the curves numerically).

**Tier 3: General numeric (NURBS)**

- Recursive Bézier subdivision (via knot insertion / Oslo algorithm)
- **Broad-phase bounding** via the **Convex Hull Property** of the control
  net — NOT interval arithmetic. IA fatally over-widens due to the Dependency
  Problem: evaluating a cubic B-spline `C(t)` with IA on `t ∈ [0,1]` produces
  a bounding box that often encompasses the entire model, causing the
  subdivision algorithm to think everything intersects and subdivide to
  `FuelExhausted`. The curve/surface is strictly contained within the convex
  hull of its control points — this is tight by construction and nearly free
  to compute.
- Swept-sphere / AABB hierarchies built over control points for ultra-tight
  intersection culling.
- **Narrow-phase certification** via interval/affine arithmetic on
  _residuals_ — certifying that a candidate intersection point actually
  satisfies both surface equations within bounds. This is the legitimate
  use of IA: verifying a solution, not bounding the solution space.
- Fuel-bounded iteration (never hang — return progress + `FuelExhausted`)
- Near-tangency fallback: when subdivision hits the local flatness threshold
  and stalls, switch to signed distance field sampling (polynomial cost
  instead of exponential)

**Settled decisions:**

- [x] ~~Newton-Raphson vs. subdivision~~ → **Hybrid.** Subdivision for
      topology classification (safe, can't jump branches), Newton-Raphson for
      final geometry refinement within locked topology (fast, quadratic
      convergence). Industry standard approach.
- [x] ~~NURBS infrastructure~~ → **Build from scratch.** B-spline evaluation
      is ~500 lines of well-documented math. SSI must be custom regardless
      (no Rust crate does robust SSI). Full control over knot insertion, basis
      function derivatives, and the Oslo algorithm is essential.
- [x] ~~Affine arithmetic vs. interval arithmetic for subdivision bounds~~ →
      **Resolved.** Neither. Use the Convex Hull Property of B-splines for
      broad-phase bounding (tight, fast, correct). Reserve interval arithmetic
      for narrow-phase residual certification only. Affine arithmetic is an
      option for near-tangent residual cases where standard IA over-widens.

---

## 4.4 forge-decision

**Purpose:** The tracing, policy, and replay engine. This is separate from
forge-core because it's substantial — it's not just an error enum, it's a
full causal reasoning system.

**Depends on:** `forge-math` (for precision certificates)

**This is what you sell to Lockheed, and what powers the agent debug loop.**

### Tiered Tracing

**Critical design correction:** The original spec said "Every kernel decision.
Every one. No exceptions." This is architecturally correct _in intent_ but
fatal _in practice_. A recursive NURBS subdivision might evaluate `orient3d`
10 million times in two seconds. If every one allocates a `TracedDecision`
struct with UUIDs, strings, and precision certificates, the kernel runs out
of RAM on a simple chamfer.

**The solution: two-tier tracing.**

```
┌─────────────────────────────────────────────────────────┐
│ TIER 1: Micro-decisions (ephemeral)                      │
│                                                         │
│ Condition: Fast-path float agreed with exact math,       │
│ margin is huge (well above threshold).                   │
│                                                         │
│ Action: Increment a per-operation telemetry counter.     │
│ No allocation. No struct. No UUID. Just:                │
│   counters.exact_agreed += 1;                           │
│   counters.min_margin = min(counters.min_margin, m);    │
│                                                         │
│ Cost: ~2ns per decision (cache-line increment)           │
├─────────────────────────────────────────────────────────┤
│ TIER 2: Macro-decisions (persisted)                      │
│                                                         │
│ Condition: ANY of the following:                         │
│   • Fast-path float DISAGREED with exact math            │
│   • Margin is near boundary (within 10x of threshold)   │
│   • Vertex was snapped to existing                       │
│   • Tolerance was coalesced                              │
│   • Policy was applied (any PolicyKind)                  │
│   • Fuel was exhausted                                   │
│   • Stage 3B (arbitrary precision) was reached           │
│                                                         │
│ Action: Allocate full TracedDecision, log to decision DB.│
│                                                         │
│ Cost: ~500ns per decision (acceptable — these are rare)  │
└─────────────────────────────────────────────────────────┘
```

**The telemetry counters** roll up into the `OperationResult` as aggregate
metrics, giving the agent a summary ("12M micro-decisions, 47 macro-decisions,
min margin 0.034") without the memory cost.

### Core Types

```rust
/// Macro-decisions only — the ones that matter for debugging and audit.
pub struct TracedDecision {
    pub id: DecisionId,
    pub kind: DecisionKind,
    pub margin: f64,              // distance to threshold (risk metric)
    pub precision: PrecisionCertificate,
    pub context: DecisionContext,
    pub scope: EntityScope,       // which entities were affected
    pub overridable: bool,        // can an agent change this?
    pub timestamp_fuel: u64,      // deterministic "time" (fuel counter, not wall clock)
}

/// Ephemeral aggregate — cheap per-operation counters for Tier 1.
pub struct MicroDecisionCounters {
    pub exact_agreed: u64,         // fast-path matched exact
    pub total_evaluated: u64,      // total predicate evaluations
    pub min_margin: f64,           // closest call (even if resolved)
    pub max_precision_stage: PrecisionMode,  // highest stage reached
}

pub enum DecisionKind {
    /// Resolved exactly — zero ambiguity
    Exact,
    /// Near a threshold but resolved with confidence
    NearBoundary { threshold: f64 },
    /// Ambiguous, resolved by ModelingContext policy
    PolicyApplied { policy: PolicyKind, default_used: bool },
    /// Vertex snapped to existing within tolerance sphere
    ToleranceSnap { target_vertex: VertexId, distance: f64 },
    /// Surfaces declared analytically coincident
    AnalyticCoincidence { surface_a: SurfaceId, surface_b: SurfaceId },
    /// Two near-coincident vertices coalesced to prevent sliver generation
    ToleranceCoalesced {
        consumed_vertices: SmallVec<[VertexId; 2]>,
        new_vertex: VertexId,
        old_tolerances: SmallVec<[f64; 2]>,
        new_tolerance: f64,
    },
    /// Stage 3B arbitrary precision exhausted fuel cap
    PrecisionFuelExhausted {
        bits_consumed: u32,
        fuel_cap: u32,
        fallback_policy: PolicyKind,
    },
    /// Could not resolve — safe default applied, flagged for review
    Unresolved { fallback: String },
}

pub enum DecisionContext {
    Classification { point: [f64; 3], result: TriSign },
    SurfaceIntersection { surface_a: SurfaceId, surface_b: SurfaceId },
    Coincidence { entity_a: EntityRef, entity_b: EntityRef },
    Tolerance { measured: f64, threshold: f64 },
    FuelExhausted { iterations_completed: u64, progress: f64 },
}
```

### The Decision Log

```rust
pub struct DecisionLog {
    decisions: Vec<TracedDecision>,
    micro_counters: MicroDecisionCounters,  // Tier 1 aggregate
}

impl DecisionLog {
    /// All decisions, sorted by risk (lowest margin first)
    pub fn by_margin_ascending(&self) -> Vec<&TracedDecision>;
    /// Only decisions where the fast path disagreed with exact
    pub fn divergent(&self) -> Vec<&TracedDecision>;
    /// Only decisions that could be overridden
    pub fn overridable(&self) -> Vec<&TracedDecision>;
    /// True if zero Unresolved decisions exist
    pub fn is_clean(&self) -> bool;
    /// Diff against another log (for detecting what changed between runs)
    pub fn diff(&self, other: &DecisionLog) -> DecisionDelta;
    /// Tier 1 aggregate summary (safe to always include in responses)
    pub fn micro_summary(&self) -> &MicroDecisionCounters;
}
```

### Audience Projection Layer

The `DecisionLog` is the single source of truth. Internal debugging and
customer-facing output are **projections** of the same data, not separate
collection systems. This avoids dual-tracing overhead while keeping the
internal/customer boundary clean.

**Principle: one collection, multiple lenses.**

```
                          ┌──────────────────────────┐
                          │      DecisionLog         │
                          │  (Tier 1 + Tier 2 data)  │
                          └────────┬─────────────────┘
                                   │
              ┌────────────────────┼────────────────────┐
              ▼                    ▼                     ▼
     Dev / Agent Lens      Customer Lens         Compliance Lens
  ─────────────────────  ───────────────────  ───────────────────
  by_margin_ascending()  project_customer()   export_compliance()
  divergent()            → SemanticDecision   → stable, versioned
  micro_summary()          human-readable       audit trail
  raw TracedDecision       no predicate guts    entity provenance
```

**The translation layer** converts raw `TracedDecision` variants into
human-readable semantic explanations:

```rust
/// Customer-visible decision summary. Stable API — versioned independently
/// of internal TracedDecision variants.
pub struct SemanticDecision {
    pub summary: String,            // "Two surfaces are near-tangent within 1e-9"
    pub entity: EntityRef,          // which face/edge/vertex was affected
    pub category: SemanticCategory, // ToleranceChange, PolicyOverride, Failure
    pub severity: Severity,         // Info, Warning, Error
}

pub enum SemanticCategory {
    ToleranceChange,     // vertex tolerance widened
    PolicyOverride,      // ambiguity resolved by policy
    AnalyticCoincidence, // surfaces declared identical
    PrecisionEscalation, // exact arithmetic was required
    FailureExplanation,  // why an operation failed
}

/// Project a full DecisionLog down to customer-visible summaries.
pub fn project_customer_decisions(log: &DecisionLog) -> Vec<SemanticDecision>;

/// Translate a single TracedDecision into a human sentence.
/// This is where internal → customer translation lives (~200 lines of
/// pattern matching, not a separate crate).
///
/// Example:
///   PrecisionFuelExhausted { bits: 512, .. }
///     → "Intersection is near-tangent and numerically unstable.
///        Increase tolerance or modify geometry."
pub fn explain(decision: &TracedDecision) -> String;
```

**What each audience sees from the same operation:**

| Audience          | Output                             | Example                                                                |
| ----------------- | ---------------------------------- | ---------------------------------------------------------------------- |
| Dev / agent       | `log.by_margin_ascending()`        | "47 macro-decisions, min margin 0.034, 2 divergent"                    |
| Customer CLI      | `project_customer_decisions(&log)` | "Vertex tolerance widened from 1e-7 to 2e-7 (sliver prevention)"       |
| Compliance export | `export_compliance_trace(&log)`    | Versioned JSON: entity provenance, policy decisions, determinism proof |

**What customers never see:** predicate counters, precision stage escalation
details, interval arithmetic bounds, halfedge rewiring internals,
intersection recursion depth. These remain accessible through the dev lens
for kernel engineers and debugging agents.

**What customers always see:** what changed, why it changed, whether it's
deterministic, and whether it's stable. The `explain()` function is the
single translation boundary between internal complexity and external
clarity.

### The Operation Envelope

```rust
/// Every kernel operation returns this. No exceptions.
pub struct OperationResult<T> {
    pub value: T,
    pub decisions: DecisionLog,       // Tier 2 macro-decisions + Tier 1 counters
    pub warnings: Vec<KernelWarning>,
    pub metrics: OperationMetrics,
    pub lineage_delta: LineageDelta,
    pub state_hash_before: u128,
    pub state_hash_after: u128,
}
```

### Causal Chains (for debugging and compliance)

Given a problematic entity, trace back through every decision that created it:

```rust
pub struct CausalChain {
    pub target: EntityRef,
    pub steps: Vec<CausalStep>,
    pub summary: ChainSummary,     // < 200 tokens, agent-consumable
}

pub struct CausalStep {
    pub operation: OpSignature,
    pub decisions: Vec<TracedDecision>,
    pub semantic_summary: String,  // "split by plane intersection"
}
```

### Counterfactual Replay

"What would have happened if this decision went the other way?"

```rust
pub fn replay_with_override(
    replay_log: &ReplayLog,
    decision_id: DecisionId,
    override_value: TriSign,
) -> CounterfactualResult;

pub struct CounterfactualResult {
    pub original_hash: u128,
    pub counterfactual_hash: u128,
    pub entities_changed: Vec<EntityRef>,
    pub still_valid: bool,         // does the counterfactual pass validation?
}
```

### Minimal Region Extraction

Given a failure, extract the smallest sub-mesh that reproduces it:

```rust
pub fn extract_minimal_repro(
    state: &TopologyState,
    failing_entity: EntityRef,
    n_ring: usize,                 // neighborhood size
) -> MinimalRepro;

pub struct MinimalRepro {
    pub sub_topology: TopologyState,
    pub sub_geometry: GeometrySlice,
    pub replay_log: ReplayLog,     // just the operations that matter
    pub serialized: Vec<u8>,       // standalone, loadable test case
}
```

**Open decisions:**

- [x] ~~Storage for decision logs~~ → Resolved. See §6.3 (three-tier
      progression: in-memory → SQLite → Postgres).
- [ ] Token budget for causal chain summaries. 200 tokens? 500? Needs
      experimentation with real chains to find the right compression level.
- [ ] How much fuel is enough? Fuel budgets per operation type need tuning.
      Start generous, tighten based on profiling.

---

## 4.5 forge-signal

**Purpose:** The reactive dependency graph. Every computed value in the system
is a signal node. Changes propagate automatically. The topology firewall
prevents unnecessary recomputation.

**Depends on:** Nothing below `forge-core` — pure graph infrastructure.

### Why Not Salsa

Early design considered the `salsa` crate (rust-analyzer's incremental
framework). Salsa solves the generic memoization problem well, but Forge
requires domain-specific semantics that Salsa cannot express natively:

- **Multi-granularity aspect signals** — a single feature output carries
  separable topology and geometry aspects with independent version counters.
  Salsa tracks queries, not aspects within a query.
- **Topology change firewall** — dirty propagation must respect aspect
  boundaries: a geometry-only change must never dirty a topology-only consumer.
  This requires first-class aspect-aware edges in the graph, not just value
  equality checks.
- **Generational handle integration** — Salsa's database-trait pattern couples
  poorly with our `thunderdome`-backed arena pattern. The adapter overhead
  (wrapping every `TopologyState` for `Clone + Eq` equality) adds complexity
  for no benefit.
- **Evaluation scheduling control** — future GPU dispatch and parallel feature
  evaluation require explicit control over when and how computations are
  scheduled. Salsa's opaque evaluation model prevents this.

`forge-signal` is purpose-built for Forge's domain. It is not a simplification
— it delivers the full reactive capability set the architecture requires.

### Push-Pull Hybrid Evaluation

When a parameter changes, the engine **pushes** a "dirty" notification through
the dependency graph in O(edges) time. But actual recomputation is **pulled**
lazily — only when a downstream consumer reads the value. Signals that are
off-screen, not currently needed for the active operation, or behind a topology
change firewall are never recomputed.

```rust
/// Push phase: mark a node dirty and propagate through dependency edges.
pub fn mark_dirty(graph: &mut SignalGraph, node: NodeId, aspect: Aspect)
    -> Result<(), KernelError>;

/// Pull phase: evaluate a node and all its dirty transitive dependencies.
pub fn evaluate<F>(graph: &mut SignalGraph, node: NodeId, compute: &mut F)
    -> Result<(), KernelError>
where F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, KernelError>;
```

### Three-State Invalidation

Forge extends the standard clean/dirty model with a critical intermediate
state: **MaybeStale**. When a signal's dependency's dependency changes, the
signal is marked `MaybeStale` rather than `Dirty`. On read, the engine walks
up the graph and checks version counters. If the direct dependency didn't
actually change its value (as determined by structural hashing for topology
signals), the `MaybeStale` signal reverts to `Clean` without any
recomputation.

```
┌─────────────────────────────────────────────────────────┐
│ Parameter changed                                        │
│   → Direct dependents: marked DIRTY                      │
│   → Transitive dependents: marked MAYBE_STALE            │
│                                                         │
│ On read of a MAYBE_STALE node:                           │
│   → Walk up to nearest DIRTY ancestor                    │
│   → Recompute the DIRTY ancestor                         │
│   → Compare version counter of result                    │
│   → Same version? → This node reverts to CLEAN           │
│       (topology didn't actually change — FIREWALL HIT)  │
│   → Different version? → This node is now DIRTY          │
│       (genuine change — recompute)                       │
└─────────────────────────────────────────────────────────┘
```

This mechanism implements the **topology change firewall**. The vast majority
of interactive parameter edits — dragging a dimension, tweaking a fillet
radius — do not change the model's topological structure. With three-state
invalidation, these edits only propagate geometry updates through the graph.
Topology signals, and everything that depends _only_ on topology (selectors,
feature references, assembly mates), remain untouched.

### Multi-Granularity Signals

Each feature's output signal carries separable **aspects** with independent
version counters: topological structure and geometric embedding. A downstream
signal can subscribe to only the aspect it requires.

```rust
pub enum Aspect {
    Topology,  // connectivity, face/edge/vertex structure
    Geometry,  // positions, dimensions, embedding
}

pub struct AspectVersion {
    pub topology: u64,
    pub geometry: u64,
}
```

| Consumer                | Subscribes to | Why                                    |
| ----------------------- | ------------- | -------------------------------------- |
| Fillet edge selector    | Topology only | Only cares which edges exist           |
| SDF preview renderer    | Geometry only | Only cares about positions             |
| STEP exporter           | Both          | Needs complete B-rep                   |
| Mass property analyzer  | Both          | Needs shape + dimensions               |
| Feature name/ref lookup | Topology only | Structure-dependent, position-agnostic |

Dirty notifications propagate only along matching aspect edges, minimizing
unnecessary recomputation.

### Automatic Dependency Discovery

Dependencies are **not declared manually**. The engine discovers them at
evaluation time by tracking which signals are read during each computation.
This means dependencies are dynamic — a selector might depend on different
edges after a topology change, and the graph rewires itself automatically.
There are no stale dependency declarations.

```rust
/// The compute closure receives a &SignalGraph reference.
/// Any node read via graph.get_value(dep_id) during evaluation
/// is automatically registered as a dependency of the evaluating node.
let mut compute = |id: NodeId, graph: &SignalGraph| -> Result<AspectVersion, _> {
    // Reading dependency outputs here auto-registers the dependency edge.
    let dep_output = graph.get_value(dep_id)?;
    // ... compute ...
    Ok(AspectVersion::new(topo_v, geom_v))
};
```

### Why This Matters for Performance

In SolidWorks, changing a dimension in feature 5 of a 200-feature model
triggers a complete sequential rebuild from feature 5 forward. In Forge, the
same change propagates through the dependency graph, hits multiple topology
change firewalls (where the topology didn't actually change), and typically
recomputes only 3–5 geometry signals rather than 195 feature rebuilds.
Interactive edits that take 5–30 seconds in SolidWorks complete in under 50
milliseconds in Forge.

### Parallelism

The `SignalGraph` owns an `EvaluationContext` per evaluation pass. Independent
branches of the dependency graph can be evaluated in parallel — the context
tracks per-node dirty state without lock contention. The current implementation
is single-threaded (sequential pull), but the data structures are designed for
future parallel dispatch:

- No shared mutable state during evaluation (each node's compute is isolated)
- Version counters are atomic-ready (currently `u64`, trivially `AtomicU64`)
- Node evaluation results are stored per-node, not in shared collections

### Core Types

```rust
pub struct SignalGraph {
    nodes: Arena<NodeEntry>,        // thunderdome-backed
    edges: Vec<DependencyEdge>,     // (from, to, aspect)
}

pub struct NodeEntry {
    state: NodeState,               // Clean / MaybeStale / Dirty
    version: AspectVersion,         // per-aspect version counters
    trace_summary: Option<TraceSummary>,
}

pub struct DependencyEdge {
    from: NodeId,
    to: NodeId,
    aspect: Aspect,
}
```

### Features as Signal Nodes

Every modeling feature (extrude, Boolean, fillet) is a signal node:

```
[sketch params] → [sketch solver] → [extrude] → [boolean] → [fillet]
                                         ↓             ↓          ↓
                                    [topo aspect] [topo aspect] [topo aspect]
                                    [geom aspect] [geom aspect] [geom aspect]
```

Each feature node produces two output aspects: topology and geometry. The
signal graph tracks dependencies on each independently, giving us the
topology firewall.

**Settled decisions:**

- [x] ~~Salsa vs custom~~ → **Custom `forge-signal`.** Salsa's database-trait
      pattern couples poorly with generational handles. Forge needs
      aspect-granular signals and evaluation scheduling control that Salsa
      cannot express. Purpose-built for the domain.
- [x] ~~Query granularity~~ → **One node per feature, two aspects per node.**
      `Aspect::Topology` and `Aspect::Geometry` carry independent version
      counters. Dirty propagates only along matching aspect edges.
- [x] ~~Thread-safe evaluation~~ → **Single-threaded now, parallel-ready.**
      Data structures designed for future parallel dispatch. No shared mutable
      state during evaluation.
- [x] ~~Cycle detection~~ → **Built-in.** The `evaluate` function detects
      cycles during graph traversal and returns `KernelError`.
- [x] ~~Arena allocator for signal nodes~~ → **thunderdome.** Same
      generational handle pattern as topology.

---

## §4.5 Assembly Hierarchy: Solid → Lump → Region → Shell

### Motivation

Both engineers and boolean algorithms need to reason about _disjoint_ or _nested_
solids within a single arena. STEP uses this hierarchy explicitly; boolean
operations create it implicitly.

### The Four Levels

| Level      | Meaning                                                                                                                                                | Status                                             |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------- |
| **Solid**  | A complete mechanical solid — what the user thinks of as "one part". Corresponds to a STEP `MANIFOLD_SOLID_BREP` or a Forge `Feature` output.          | ✅ `BodyData` / `BodyId` in `forge-topo` arena     |
| **Lump**   | A maximal connected volume of material. A Solid has ≥ 1 Lumps. After a boolean difference that separates geometry, each fragment becomes its own Lump. | ✅ `LumpData` / `LumpId` in `forge-topo` arena     |
| **Region** | A bounded volume defined by shells. One outer Shell (bounds solid material) + zero or more inner Shells (cavity walls).                                | ✅ `RegionData` / `RegionId` in `forge-topo` arena |
| **Shell**  | A maximal connected manifold surface mesh. Outer shells have `ShellOrientation::Outer`; inner cavity walls have `ShellOrientation::Inner`.             | ✅ `ShellData` / `ShellId` in `forge-topo` arena   |

### Ownership: All Levels Live in `forge-topo`

**Decision (deviation from original plan):** The original design placed Solid
(Body) in `forge-kernel` via `FeatureOutput`. This was changed — all four
hierarchy levels are owned by `forge-topo`'s `TopologyArena`. Rationale:

1. **Euler formula completeness**: The generalized Euler formula
   `V - E + F - L = 2(S - G)` accounts for connected components (Lumps). If
   hierarchy entities live outside the arena, the validator can't verify the
   formula without cross-crate coupling.
2. **Atomic transactions (Doctrine D6)**: `MutableDraft::commit()` validates
   the entire topology atomically. Splitting the hierarchy across crates would
   create a split-brain: half commits in topo, half in kernel. Rollback would
   be inconsistent.
3. **STEP roundtrip fidelity**: STEP requires `MANIFOLD_SOLID_BREP → Lump →
Region → Shell`. With all levels in the arena, `forge-io` can
   serialize/deserialize them directly.

### Hierarchy Structure

```
Solid  → forge-topo arena; owns ≥ 1 Lumps
Lump   → forge-topo arena; owns ≥ 1 Regions; parent → Solid
Region → forge-topo arena; owns 1 outer Shell + N inner Shells; parent → Lump
Shell  → forge-topo arena; parent → Region
```

### Invariants

- `MakeVertexFace` creates the full Solid→Lump→Region→Shell chain.
- `KillVertexFace` destroys the full chain (inverse).
- Every Shell must be owned by exactly one Region (orphan detection at commit).
- Every Region must be owned by exactly one Lump.
- Every Lump must be owned by exactly one Solid.
- `RegionData` distinguishes its outer shell from inner shells at the type level.

---

# 5. The Hybrid Boolean Pipeline

This is the heart of the kernel. It's where exact planar math and tolerant
curved math merge into one pipeline.

## 5.1 Overview

Every Boolean operation follows this sequence:

```
1. BROAD PHASE
   AABB tree → candidate face pairs that might intersect

2. ANALYTIC ARBITRATION
   For each candidate pair, compare surface definitions:
   - Coincident → skip numerical solver, route to 2D graph merge
   - Disjoint → skip entirely
   - General → proceed to intersection

3. SURFACE-SURFACE INTERSECTION
   Compute intersection curves between General pairs:
   - Planar pairs: exact (plane-plane = line, clip to face bounds)
   - Analytic pairs: semi-exact (plane-cylinder = ellipse, etc.)
   - NURBS pairs: subdivision + certified bounds

4. FACE SPLITTING
   Split faces along intersection curves. New edges get coedges
   (UV-space anchoring) on both adjacent faces.

5. FACE CLASSIFICATION
   For each split face, classify as Inside/Outside/Boundary relative
   to the other solid:
   - Planar: exact predicates → CertifiedTriSign
   - Curved: precision escalation → CertifiedTriSign or PolicyRequired

6. ASSEMBLY
   Select faces per Boolean type:
   - Union: outside(A) + outside(B) + shared coplanar (same normal)
   - Intersection: inside(A) + inside(B)
   - Subtraction: outside(A) + inside(B) flipped

7. TOLERANT STITCHING
   Stitch the assembled faces. Vertex snapping uses tolerance spheres.
   Edge matching uses UV-space coedge alignment. Every snap is a
   TracedDecision.

8. VALIDATION
   Euler formula, orientation, manifoldness, watertightness.
```

Steps 2, 4 (coedges), and 7 are the Gemini doc additions. Steps 1, 3, 5, 6,
8 are from the original spec. Together they form the hybrid pipeline.

## 5.2 What Makes This Different From Parasolid

| Aspect              | Parasolid                                | Forge                                            |
| ------------------- | ---------------------------------------- | ------------------------------------------------ |
| Tolerance           | Global epsilon, silently widened         | Per-vertex, coalescent (policy-governed, traced) |
| Coincidence         | Heuristic detection                      | Analytic arbitration on surface definitions      |
| Edge anchoring      | 3D curves only (drift after chained ops) | UV coedges (locked to surface)                   |
| Classification      | Float, sometimes wrong                   | Precision escalation to exact                    |
| Decisions           | Black box                                | Every decision traced, replayable, overridable   |
| Intersection curves | Immediately discretized                  | Symbolic (`SurfaceIntersection`) until forced    |

## 5.3 The Planar Fast Path

When both operands are planar-only (the common case for early development and
many real parts), the entire hybrid pipeline simplifies:

- Analytic arbitration reduces to coplanar detection (exact)
- SSI reduces to plane-plane intersection (exact line)
- Classification uses `orient3d` → `CertifiedTriSign` (exact)
- Tolerant stitching is unnecessary (vertex positions are exact)
- Coedges are trivial (straight lines in UV)

The planar path has zero tolerance decisions. Zero ambiguity. Zero policy.
Every decision is `DecisionKind::Exact`. This is the foundation you build
and test first, and the curved path extends it without breaking it.

---

# 6. State, Persistence, and Git

## 6.1 The Spec Graph Format

The spec graph is the source of truth. It serializes to JSON (or a similar
human-readable, diffable format).

```json
{
  "forge_version": "0.1.0",
  "features": [
    {
      "id": "extrude-1",
      "type": "Extrude",
      "params": { "profile": "sketch-1", "depth": 25.0 },
      "depends_on": ["sketch-1"]
    },
    {
      "id": "fillet-1",
      "type": "Fillet",
      "params": {
        "edges": {
          "selector": "intersection_of",
          "args": ["extrude-1", "extrude-2"]
        },
        "radius": 3.0
      },
      "depends_on": ["boolean-1"]
    }
  ],
  "policy": {
    "tolerance": 1e-7,
    "tangency_threshold": 0.01,
    "sliver_area_min": 1e-10
  }
}
```

This is what gets committed to git. `git diff` shows:

```diff
- "depth": 25.0
+ "depth": 30.0
```

**WARNING: Standard git merges are NOT sufficient.** A standard git merge is
purely syntactic, not geometric. It is oblivious to the Topological Naming
Problem (TNP).

**The TNP trap:** Agent A adds a hole intersecting Face 5. Agent B adds a
fillet to Face 5. Git merges the JSON seamlessly — the text lines don't
conflict. But Agent A's hole just split Face 5 into Face 5a and Face 5b.
When the merged JSON evaluates, Agent B's fillet crashes because Face 5 no
longer exists as defined. TNP is the #1 cause of parametric model collapse,
and relying on raw git text merges means agents will continuously commit
"successful" merges that produce broken models.

**The fix: a semantic Git merge driver.** Git is configured to pass `.forge`
files to a custom CLI (`forge merge`). During a merge, the CLI:

1. Parses both branches' spec graphs into memory.
2. Performs a structural (not textual) three-way merge of the feature DAGs.
3. Re-evaluates the reactive signal graph on the merged result.
4. Tracks lineage signatures (`OpSignature`) through the merge.
5. Rewrites topological references in the merged JSON to point at the
   correct post-merge entities (e.g., Face 5 → Face 5a if that's the
   surviving reference after Agent A's hole).
6. If a reference truly disappears (the face was consumed entirely), the
   CLI flags a **Semantic Merge Conflict** — a structured error that tells
   the agent _why_ the reference is invalid and _which_ operation broke it,
   rather than a raw text diff.

```bash
# .gitattributes
*.forge merge=forge-semantic

# .git/config
[merge "forge-semantic"]
  name = Forge Semantic Merge Driver
  driver = forge merge %O %A %B %P
```

The merge driver is a Phase 3 deliverable (alongside the spec graph and
signal infrastructure). Until it exists, concurrent branch editing of the
same model region is explicitly unsupported.

## 6.2 Derived State (NOT in git)

The B-rep, decision logs, SDF cache, and tessellation are derived. They're
either:

- Recomputed on demand (B-rep, SDF)
- Cached locally (tessellation)
- Stored alongside for debugging (decision logs for the current state)

Decision logs for _historical_ operations (the audit trail) are a different
question — see 6.3.

## 6.3 Persistence for Dev Tooling and Audit

The kernel itself is stateless — pure functions on immutable TopologyState.
No database in the kernel.

Above the kernel, the dev tooling and audit system needs queryable storage:

**What needs querying:**

- Decision logs across thousands of test runs ("show me all NearBoundary
  decisions in the last week's CI")
- Test corpus management (10,000+ test cases, growing monotonically)
- Regression tracking ("did this commit increase the divergence rate?")
- Aerospace audit trail ("trace every decision in this part back to source")

**Three-tier storage progression:**

```
┌─────────────────────────────────────────────────────────┐
│ TIER 1: The Spec Graph (JSON in Git)                     │
│                                                         │
│ What: Feature DAG, parameters, policy config.           │
│ Where: .forge files in the repo, versioned by git.      │
│ Why: Diffable, mergeable (via semantic driver), auditable│
│ via git history. This is the source of truth.           │
├─────────────────────────────────────────────────────────┤
│ TIER 2: Local Dev / Agent Loop (Embedded SQLite)         │
│                                                         │
│ What: Macro-decision logs, micro-decision aggregates,   │
│ test corpus, regression baselines.                      │
│ Where: Single .sqlite file, shipped inside the CLI      │
│ binary. No server, no network, no Docker.               │
│ Why: A single Boolean operation may generate thousands  │
│ of macro-decisions. SQLite writes locally in             │
│ microseconds. It supports the exact relational SQL      │
│ queries the agent needs:                                │
│   SELECT * FROM decisions WHERE margin < 1e-6           │
│   SELECT operation, COUNT(*) FROM decisions             │
│     WHERE kind = 'ToleranceCoalesced' GROUP BY operation│
│ CAD traceability is highly relational — a TracedDecision│
│ references a VertexId which belongs to a FaceId which   │
│ was created by an Extrude feature. Document DBs either  │
│ duplicate massive data or do slow app-level joins.      │
├─────────────────────────────────────────────────────────┤
│ TIER 3: Aerospace / Cloud Swarm (Postgres)               │
│                                                         │
│ What: Aggregated decision metrics, cross-repo regression│
│ tracking, multi-agent counterfactual results.           │
│ Where: Centralized Postgres, same SQL schema as SQLite. │
│ Why: Teams of agents running 10,000 counterfactuals in  │
│ the cloud need centralized aggregation. Point the same  │
│ schema at Postgres — the application code doesn't change│
│ When: NOT for local dev. NOT for v1. This tier exists   │
│ when you have enterprise customers running agent swarms.│
└─────────────────────────────────────────────────────────┘
```

**Why not Mongo?** CAD traceability is highly relational. A `TracedDecision`
references a `VertexId`, which belongs to a `FaceId`, which was created by
an `Extrude` feature. Document DBs either duplicate massive amounts of data
or require incredibly slow application-level joins. The data model is
inherently relational — use a relational database.

**The audit trail for compliance** is serialized decision logs in the git
repo alongside the spec graph. Each commit includes the decisions that
produced the current state. The full chain is reconstructible from git
history. This is both the simplest approach and the most auditable — git
provides the immutable append-only log for free.

---

# 7. The CLI and Agent Interface

The CLI is how agents (and power-user humans) interact with Forge. It operates
on the spec graph.

```bash
# Create a feature
forge add extrude --profile sketch-1 --depth 25.0

# Inspect the decision log for the last operation
forge decisions --sort-by margin

# See the riskiest decisions across the whole model
forge decisions --filter near-boundary --sort-by margin

# Extract a minimal repro for a failing entity
forge debug extract-repro --entity face-37 --n-ring 3

# Replay a decision with a different outcome
forge debug counterfactual --decision d-1294 --override Pos

# Validate the current state
forge validate --full

# Export
forge export step --output part.step

# Git-native workflow
forge status          # which features changed since last commit
forge diff HEAD~1     # what parameters changed
```

For agents, the same operations are available as a structured JSON API:

```json
{
  "command": "add_feature",
  "type": "Extrude",
  "params": { "profile": "sketch-1", "depth": 25.0 }
}
```

Response:

```json
{
  "status": "ok",
  "feature_id": "extrude-1",
  "decisions": {
    "total": 12,
    "exact": 12,
    "near_boundary": 0,
    "unresolved": 0,
    "min_margin": 0.034
  },
  "topology_changed": true,
  "entities_created": { "faces": 6, "edges": 12, "vertices": 8 }
}
```

The agent debug loop:

```
1. Agent creates feature
2. Inspects decision summary → sees a NearBoundary warning
3. Extracts minimal repro for the affected entity
4. Runs counterfactual → discovers the alternative produces invalid topology
5. Logs the case as a regression test
6. Commits the fix
```

This is the same loop a human debugger would follow, but automated. The
tracing infrastructure makes it possible.

---

# 8. Build Order and Dependencies

What gets built first, what depends on what, and why.

## 8.1 Dependency Graph

```
forge-math           (leaf — no deps)
    ↓
forge-topo           (depends on forge-math)
    ↓
forge-geom           (depends on forge-math)
    ↓
forge-decision       (depends on forge-math)
    ↓
forge-signal         (depends on forge-topo)
    ↓
forge-kernel         (depends on all above)
    ↓
forge-io             (depends on forge-kernel, forge-geom)
forge-persist        (depends on forge-kernel)
forge-cli            (depends on everything)
```

Note: `forge-topo` and `forge-geom` are siblings, not parent-child. They
communicate through opaque IDs. `forge-kernel` is the first crate that
combines them.

## 8.2 Build Phases (What, Not How)

### Phase 1: Math + Planar Topology

Build `forge-math` (predicates, filtered pipeline) and `forge-topo` (halfedge
mesh, Euler operators, immutable state). At the end of this phase you can
construct a cube from 6 planes and validate its topology.

**Depends on:** Nothing (greenfield)
**Produces:** Certified predicates, valid halfedge meshes, replay infrastructure

### Phase 2: Planar Booleans + Decision Engine

Build `forge-decision` (tracing, policy, operation envelope) and the planar
Boolean pipeline in `forge-kernel`. At the end of this phase you can union
two cubes and get a traced, replayable, validated result.

**Depends on:** Phase 1
**Produces:** Working planar Booleans with full traceability

### Phase 3: Spec Graph + Signals + Git

Build `forge-signal` (reactive graph) and `forge-persist` (JSON serialization,
git integration). Wire features into the signal graph. At the end of this
phase you can define a parametric model, change a parameter, and only the
affected parts recompute. You can commit the spec to git.

**Depends on:** Phase 2
**Produces:** Parametric feature tree, reactive updates, git-native persistence

### Phase 4: Geometry Store + Analytic Surfaces

Build `forge-geom` (surface hierarchy, geometry store, analytic surfaces).
Implement coedges, tolerant vertices, analytic arbitration. At the end of
this phase the data structures for curved geometry exist.

**Depends on:** Phase 2 (needs the decision engine)
**Produces:** Surface/curve types, coedge infrastructure, tolerance model

### Phase 5: Curved Booleans

Extend the Boolean pipeline to handle analytic surface pairs (plane-cylinder,
cylinder-cylinder, etc.). This is where the hybrid pipeline comes together.
At the end of this phase you can Boolean a cylinder through a box.

**Depends on:** Phase 4
**Produces:** Mixed planar/analytic Booleans with traced tolerance decisions

### Phase 6: Fillets + Chamfers

Rolling-ball fillets, chamfers, variable radius. Depends on curved surfaces
existing. Corner patches, cascade detection, failure reporting.

**Depends on:** Phase 5
**Produces:** Edge blending with full traceability

### Phase 7: NURBS + General SSI

NURBS surface representation, general surface-surface intersection via
subdivision, Bézier clipping, distance-field fallback. This is the long pole.

**Depends on:** Phase 5 (extends the curved Boolean pipeline)
**Produces:** General freeform geometry Booleans

### Phase 8: IO + CLI + Agent Interface

STEP import/export, STL/3MF export, the CLI, the agent JSON API. Import
healing with traced decisions.

**Depends on:** Phases 3 (persistence) + 7 (NURBS for full STEP coverage)
**Produces:** The product interface

---

# 9. Open Design Decisions

These need to be resolved before the full spec.

## 9.1 Settled

| Decision                                      | Resolution             | Rationale                                                                                         |
| --------------------------------------------- | ---------------------- | ------------------------------------------------------------------------------------------------- |
| Topology-geometry separation                  | Yes, absolute          | The core architectural invariant                                                                  |
| Exact predicates for planar                   | Yes, filtered pipeline | Eliminates 95% of failure modes                                                                   |
| Immutable TopologyState                       | Yes, with MutableDraft | Already implemented, enables undo/git                                                             |
| Generational handles                          | Yes, thunderdome       | Prevents ABA corruption                                                                           |
| JSON spec graph in git                        | Yes                    | Diffable, mergeable, agent-readable. JSON over TOML/DSL — universally tooled, agents R/W natively |
| Tiered decision tracing                       | Yes                    | Micro (counters) + Macro (full struct). No "trace everything" RAM explosion                       |
| Coedges for curved edges                      | Yes                    | Required to survive chained curved Booleans                                                       |
| Tolerant vertices (coalescence)               | Yes                    | Localized epsilon with policy-governed widening, prevents sliver hell                             |
| Analytic arbitration                          | Yes                    | Skips solver for coincident surfaces                                                              |
| Symbolic intersection curves (SP-cached)      | Yes                    | Symbolic for exactness + SP-curve cache for performance                                           |
| SDF for real-time preview                     | Yes                    | Decoupled from B-rep computation                                                                  |
| Three-tier storage (JSON → SQLite → Postgres) | Yes                    | Spec in git, dev tooling in SQLite, enterprise in Postgres                                        |
| Fuel-bounded iteration                        | Yes                    | Deterministic, no wall-clock branching                                                            |
| First-class Edge entity                       | Yes                    | EdgeData owns shared 3D curve + tolerance; HalfEdge owns UV coedge + direction                    |
| Convex Hull NURBS bounding                    | Yes                    | Control-net convex hull for broad-phase; IA only for narrow-phase residual certification          |
| Custom `forge-signal` reactive graph          | Yes                    | Purpose-built: push-pull hybrid, three-state invalidation, aspect-granular topology firewall      |
| Predicate pipeline 3A/3B split                | Yes                    | Exact rational for planar, arbitrary precision for curved                                         |
| Semantic git merge driver                     | Yes (Phase 3)          | Required to prevent TNP-induced model collapse on merges                                          |

## 9.2 All Decisions Settled

All pre-spec decisions have been resolved. The following table records each
decision and its resolution for reference.

| #   | Decision                    | Resolution                                                                                         |
| --- | --------------------------- | -------------------------------------------------------------------------------------------------- |
| 1   | BigInt library              | `num-bigint` (pure Rust). Portability wins; exact path is <1% of calls on small numbers.           |
| 2   | Shewchuk predicates         | Full port to Rust. Full control + extensibility for custom predicates.                             |
| 3   | SurfaceRef / CoedgeRef type | Generational handles (thunderdome). Same ABA-safe pattern as topology.                             |
| 4   | TopologyState sharing       | Clone-on-write now. Persistent HAMT deferred until profiling demands it (Phase 5+).                |
| 5   | MPFR binding (Stage 3B)     | Deferred to Phase 4+. Pure Rust first; `rug` as optional feature gate if too slow.                 |
| 6   | Newton vs subdivision       | Hybrid — subdivision for topology, Newton for refinement. Industry standard.                       |
| 7   | NURBS infrastructure        | Build from scratch. Full control over evaluation, derivatives, Oslo algorithm. SSI must be custom. |
| 8   | Sketch solver               | Custom, built fully in Rust with custom UI library. Not deferred — long-term build.                |
| 9   | SDF implementation          | Hybrid — per-feature composition for planar, mesh-based for curved.                                |
| 10  | Lineage hash in git diff    | Serialize both feature-level and entity-level lineage in spec.                                     |
| 11  | Signal graph framework      | Custom `forge-signal`. Salsa rejected — poor generational-handle fit, no aspect-granular signals.  |
| 12  | Signal granularity          | One node per feature, two aspects per node (`Topology`, `Geometry`). Aspect-aware firewall.        |

## 9.3 Open — Can Defer Past Full Spec

| Decision                                     | Why It Can Wait                                        |
| -------------------------------------------- | ------------------------------------------------------ |
| STEP AP214 vs AP203                          | Export format details don't affect kernel architecture |
| 3MF metadata schema                          | Export detail                                          |
| Assembly constraints                         | Assemblies are Phase 8+                                |
| 2D drafting                                  | Product feature, not kernel                            |
| Variable-radius fillet interpolation method  | Detail within fillet implementation                    |
| Class-A surface fitting convergence strategy | NURBS detail, Phase 7+                                 |
| Multi-user concurrent editing                | Enterprise feature, needs Postgres, not yet            |

---

# 10. What Exists vs What's New

Mapping existing code to this architecture:

| Component                        | Status         | Notes                                      |
| -------------------------------- | -------------- | ------------------------------------------ |
| `KernelError` taxonomy           | ✔ Exists       | Structured variants, machine-actionable    |
| `PolicyResult<T>`                | ✔ Exists       | Three-state: Ok / Ambiguous / Err          |
| `TracedDecision`                 | ✔ Exists       | Has DecisionId, Kind, Tier, Context        |
| `DecisionLog`                    | ✔ Exists       | Queryable, diffable                        |
| `OperationResult<T>`             | ✔ Exists       | Envelope with metrics, lineage, decisions  |
| `TopologyState`                  | ✔ Exists       | Immutable, epoch-versioned, Arc<Arena>     |
| `MutableDraft`                   | ✔ Exists       | Transactional commit/rollback              |
| `TopologyArena`                  | ✔ Exists       | thunderdome-backed generational handles    |
| Entity handles                   | ✔ Exists       | FaceId, VertexId, HalfEdgeId, LoopId       |
| `EulerOperator` + `apply_op()`   | ✔ Exists       | Trait + runner pattern                     |
| `Lineage`                        | ✔ Exists       | Origin feature, creation op, ancestry hash |
| **Filtered predicates**          | ❌ Needs build | forge-math core                            |
| **CertifiedTriSign**             | ❌ Needs build | The certified sign newtype                 |
| **Interval arithmetic**          | ❌ Needs build | For curved geometry                        |
| **Geometry Store**               | ❌ Needs build | forge-geom (surfaces, curves)              |
| **Coedges**                      | ❌ Needs build | UV-space anchoring                         |
| **Tolerant vertices**            | ❌ Needs build | Write-once localized epsilon               |
| **Analytic arbitration**         | ❌ Needs build | Surface comparison before solving          |
| **Surface-surface intersection** | ❌ Needs build | The hard problem                           |
| **Signal graph**                 | ❌ Needs build | forge-signal                               |
| **Boolean pipeline**             | ❌ Needs build | forge-kernel core                          |
| **Spec graph serialization**     | ❌ Needs build | forge-persist                              |
| **CLI**                          | ❌ Needs build | forge-cli                                  |

---

# 11. Risk Map

Where the hard problems are, in order of severity:

1. **Cylinder-cylinder intersection topology** (Phase 5) — more kernels fail
   on this than any other single problem. Five sub-cases, near-tangency is
   unbounded complexity. Mitigation: topology-first (classify before extracting
   geometry), distance-field fallback for near-tangent.

2. **Chained Boolean stability on curves** (Phase 5–7) — error accumulates
   across operations. Mitigation: symbolic intersection curves, UV coedges,
   tolerant vertices with provenance.

3. **NURBS surface-surface intersection** (Phase 7) — the general case is a
   research problem. No known algorithm handles all cases robustly.
   Mitigation: fuel-bounded subdivision, distance-field fallback, policy
   escalation for truly degenerate cases. The tracing system means agent
   swarms can continuously find and fix edge cases.

4. **Fillet corner patches** (Phase 6) — where 3+ fillets meet at a vertex,
   the blending surface is a hard geometric construction. Cascade detection
   (fillet consuming a face) is where incumbent kernels crash. Mitigation:
   detect-before-execute, transactional rollback, structured error reporting.

5. **Signal graph parallel evaluation** (Phase 3) — Parallel feature
   evaluation requires careful dependency tracking to maintain determinism.
   `forge-signal`'s data structures are parallelism-ready (per-node state,
   no shared mutation during evaluation), but the dispatch scheduler and
   deterministic ordering for parallel branches need implementation.

6. **STEP import healing** (Phase 8) — real-world STEP files are messy.
   Gaps, inconsistent normals, missing faces. Healing must be deterministic
   and traced. Mitigation: staged pipeline with per-stage decision logging.

---

# 12. Product Manufacturing Information (GD&T / PMI)

## The Two-Tolerance Model

Forge maintains a strict separation between two kinds of tolerances that are
easy to conflate but must never be mixed:

| Concept                     | Type                                | Home                              | Meaning                                                                    |
| --------------------------- | ----------------------------------- | --------------------------------- | -------------------------------------------------------------------------- |
| **Geometric uncertainty**   | `f64` on `VertexGeom` / `CurveGeom` | `forge-geom`                      | "How accurately does the kernel know where this point is, mathematically?" |
| **Specification tolerance** | `ToleranceZone` (PMI)               | `forge-schema` / `AttributeStore` | "How much deviation from nominal is acceptable for manufacturing?"         |

The first is a _kernel_ property — it's about floating-point error bounds and
SSI solver residuals. The second is a _design intent_ property — it's about
what the part is allowed to be in the physical world. Conflating them is
a classic CAD kernel bug (e.g. snapping geometry to GD&T zone boundaries).

## Where GD&T Lives

The existing architecture already has the right homes for every GD&T concept.
No new crates are needed — GD&T is purely additive:

### Annotations → `AttributeStore` (forge-topo)

GD&T annotations are semantic metadata attached to topology entities.
`AttributeStore` already provides the side-car tag mechanism:

```
face_42  ← TagValue::GdtAnnotation(Flatness { tolerance_mm: 0.002 })
face_43  ← TagValue::GdtAnnotation(Position { diameter_mm: 0.010, datum_refs: [A, B, C] })
edge_17  ← TagValue::GdtAnnotation(Cylindricity { tolerance_mm: 0.001 })
```

No topology changes required. No geometry changes required.

### Schema Types → `forge-schema`

`forge-schema` is the declarative JSON schema crate (serde-only, no kernel
deps). ASME Y14.5 / ISO 1101 tolerance types belong here:

```rust
// forge-schema
pub enum ToleranceZoneType { Cylindrical, Spherical, Projected, TwoLine }

pub struct DatumReference { label: char, modifier: DatumModifier }

pub struct GdtAnnotation {
    characteristic: ToleranceCharacteristic, // Flatness, Cylindricity, Position, ...
    tolerance_value: f64,                    // mm or degrees
    datum_refs: Vec<DatumReference>,         // [A, B] or [A, B(M), C]
    zone_type: ToleranceZoneType,
    material_condition: MaterialCondition,   // MMC / LMC / RFS
}

pub struct DatumReferenceFrame {
    primary:   DatumDefinition,
    secondary: Option<DatumDefinition>,
    tertiary:  Option<DatumDefinition>,
}
```

### Validation → `forge-kernel::analysis::pmi`

GD&T validation (is this face within its flatness spec?) is a `forge-kernel`
analysis operation — it queries both topology (which faces), geometry (actual
positions), and PMI annotations (what the spec says):

```rust
// forge-kernel::analysis::pmi
pub fn evaluate_flatness(
    face_id: FaceId,
    topo: &TopologyState,
    geom: &GeometryStore,
    annotation: &GdtAnnotation,
    ctx: &mut ModelingContext,
) -> OperationResult<PmiEvaluation>
```

Every evaluation is a `TracedDecision` in the `DecisionLog` — which is exactly
the compliance audit trail aerospace customers pay for.

### Import/Export → `forge-io`

STEP AP242 carries PMI as `draughting_model` entities alongside the B-rep.
IGES Section D carries GD&T as entity type 402/212. `forge-io` reads and
writes these as `GdtAnnotation` tags on the topology entities they reference.

## Why the Architecture Is Already Set Up for This

1. **`AttributeStore` was designed for exactly this** — arbitrary semantic
   tags on topology entities. GD&T is the primary use case.

2. **`DecisionLog` + `OperationResult` envelope** — compliance validation
   reports are just `TracedDecision` records. The audit trail is free.

3. **`forge-schema` is already separate** — GD&T types don't pollute the
   kernel. They're JSON-serializable spec-layer types.

4. **`VertexGeom::tolerance` is the right thing** — it does NOT need to be a
   GD&T tolerance. It stays as geometric uncertainty. This is correct.

5. **`ToleranceProvider` returns uncertainty, not specification** — this
   distinction is enforced at the type level. A GD&T evaluator receives
   `GdtAnnotation`, not a `ToleranceProvider`.

## What Is Explicitly Not Needed

- No new crate for GD&T (it's `forge-schema` + `forge-kernel::analysis`)
- No changes to `forge-topo` (GD&T uses `AttributeStore` which already exists)
- No changes to the `ToleranceProvider` trait (different concept entirely)
- No feature recognition engine for _basic_ GD&T — annotations are attached
  explicitly by the CAD operator or imported from STEP; Forge doesn't need to
  infer them from geometry shape

Feature recognition (inferring that 4 faces form a "slot" for auto-annotation)
is an _optional_ higher-order analysis operation and is out of scope for v1.

---

# 13. One-Page Summary

**Forge is a spec graph that derives geometry, not a geometry engine that
stores specs.**

The spec graph lives in git. It's JSON. Agents and humans read and write it
through the CLI. Every derived quantity — topology, geometry, B-rep, SDF — is
a reactive signal that recomputes only when its dependencies change.

The Boolean pipeline is hybrid: exact for planar (certified signs, zero
ambiguity), tolerant for curved (precision escalation, coedges, coalescent
tolerance, analytic arbitration). The predicate pipeline splits at Stage 3:
exact rational (BigInt) for planar, arbitrary precision float (MPFR) for
curved. Every macro-decision is traced, replayable, and overridable; every
micro-decision is counted for aggregate telemetry.

The tracing system serves three audiences simultaneously: it's the debugging
infrastructure that makes it possible to build the kernel (agent swarms
finding and fixing edge cases), it's the compliance system that aerospace
companies pay for, and it's the observability layer that lets the signal
graph work correctly.

Build order: math → topo → planar booleans + decisions → signals + git →
curved geometry → curved booleans → fillets → NURBS → IO + CLI.

The hardest problems are cylinder-cylinder intersection, chained curved
Boolean stability, and general NURBS SSI. The architecture doesn't eliminate
these problems — it contains them within traced, fuel-bounded, policy-governed
boundaries so they can be attacked incrementally by agent swarms running
against a monotonically growing test corpus.
