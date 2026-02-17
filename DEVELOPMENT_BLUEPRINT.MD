# Forge Kernel Development Plan v5
### The Complete Engineering Specification

---

# Part 1: How to Read This Document

This plan has **10 phases, 58 milestones, 41 kernel validation suites (KV), 20 test case parts (TC), and 8 performance gates.** That's a lot. Here's how it's organized so you don't get lost.

**Phases** are major capability stages. Each phase produces something testable.

**Milestones** are the atomic work units inside each phase. Each milestone is scoped for an AI coding agent to receive as a self-contained spec, implement, and validate. Milestones within a phase are sequential unless noted otherwise.

**KV suites** (Kernel Validation) are internal correctness tests that validate architectural invariants. They exist to catch structural problems early — before they become expensive. Named `KV-01` through `KV-41`.

**TC parts** (Test Cases) are the 20 real-world mechanical parts that validate end-to-end kernel capability. Named `TC-01` through `TC-20`. A test case is "green" when it passes both internal validation and differential comparison against Open CASCADE.

**Performance gates** are hard numeric thresholds. A phase is not complete until its gates are met or consciously deferred with a tracked issue.

**Risk markers** on each milestone:
- ✅ = Well-understood engineering, low risk
- 🟡 = Moderate complexity, known approaches exist
- 🔴 = Hard problem, requires careful implementation
- 🧪 = Research-grade, may require iteration on approach

---

# Part 2: Global Doctrines

These nine rules apply to every line of code in every phase. They are not guidelines — they are invariants that the test harness enforces.

### D0 — Topology-First Doctrine
All topological decisions must be certified:
- **Planar/linear geometry:** exact predicates returning `CertifiedTriSign` (three-valued: Neg/Zero/Pos)
- **Curved geometry:** certified bounds (affine arithmetic / interval arithmetic + subdivision) with explicit policy at the ambiguity boundary

Geometry is a binding layer — it may be approximate, but it carries bounded error metrics and never corrupts topology.

### D1 — Determinism & Replay
Every operation is deterministic given: `input state + parameters + policy + seed`. Every failure produces a replay log and minimal reproduction case. The test harness verifies this by running the same operation 100× and comparing topology hashes.

### D2 — Explicit Policy at Ambiguity
When classification falls within tolerance of an ambiguous boundary, the kernel either:
- Returns a `PolicyRequired` result for the caller to resolve, or
- Applies the explicit `ModelingContext` policy and logs a `ToleranceDecision`

Silent heuristic decisions are forbidden. Every judgment call is traceable.

### D3 — Topology–Geometry Firewall
Topology creation/mutation is driven only by certified classification + explicit policy decisions. Geometry can propose candidates, but cannot finalize adjacency, splits, or merges. 

### D4 — Canonical Orientation Doctrine
Every solid is globally oriented with outward normals and consistent halfedge winding. The topology validator enforces this strictly after every operation and import.

### D5 — Sliver Generation Budget
Operations must avoid creating faces with area < threshold through alternative split strategies or forced policy decisions at creation time. No operation may produce more than N new slivers without an explicit "waive gate" log.

### D6 — Atomic Transactionality
Every modeling operation is atomic. It either produces a valid, manifold new state or returns a structured error. No partial mutations are ever observable. The kernel guarantees `State A → Result<State B, Error>`.

### D7 — Visualization Subordination
The B-Rep is the single source of truth. SDF and mesh visualizations are derived, lossy artifacts. They never inform topological decisions (except as explicit hints). The kernel guarantees eventually consistent visualization, but never authoritative visualization.

### D8 — Environmental Isolation
The kernel's output is independent of the compute environment. CPU feature flags, thread scheduling, and math library differences must not alter bit-exact results. We enforce strict floating-point modes (no fast-math) and deterministic parallelism.

### D9 — AI-Observability Doctrine
Every kernel operation must return a structured result envelope containing: the output value, a log of all tolerance decisions made, any warnings raised, performance metrics, and a lineage delta. Silent side effects are forbidden — an agent must be able to reconstruct the full state transition from the result envelope alone.

---

# Part 3: Project Initialization

This section tells you exactly how to set up the project before any kernel code exists.

### Step 1: Create the Workspace

```
forge/
├── Cargo.toml              # workspace root
├── crates/
│   ├── forge-math/          # exact arithmetic, predicates, filtered evaluation
│   ├── forge-topo/          # halfedge mesh, Euler operators, topology state
│   ├── forge-geom/          # analytic surfaces, NURBS, curves
│   ├── forge-kernel/        # features, booleans, fillets — the "application" crate
│   ├── forge-io/            # STEP, IGES, STL, native JSON serialization
│   └── forge-test/          # differential harness, fuzzer generators, KV suites
├── tests/
│   ├── kv/                  # kernel validation suites (KV-01 through KV-41)
│   ├── tc/                  # test case specs (TC-01 through TC-20)
│   └── differential/        # Open CASCADE comparison harness
├── benches/                 # criterion benchmarks
└── docs/
    ├── architecture.md      # this document
    ├── exactness-contract.md
    └── tolerance-policy.md
```

### Step 1.5: Compiler & Environment Safety

To enforce **D8 (Environmental Isolation)**, the build must reject non-deterministic math optimizations.

In `.cargo/config.toml`:
```toml
[build]
rustflags = [
    "-C", "target-cpu=x86-64-v3", # Baseline standard, not host-native
    "-C", "overflow-checks=on",
    "-F", "unsafe_code",          # explicit allow required
]
```

In `forge-math/src/lib.rs`:
```rust
#[cfg(feature = "strict_env")]
pub fn init_fpu() {
    // Reset FPU control words to standard IEEE 754 default
    // Disable DAZ/FTZ (denormals are valid geometry)
    // Set rounding mode to Nearest
}
```

### Step 2: Crate Dependency Order

```
forge-math          (no internal deps)
  └─→ forge-topo    (depends on forge-math)
        └─→ forge-geom   (depends on forge-math, forge-topo)
              └─→ forge-kernel (depends on all above)
                    └─→ forge-io     (depends on all above)

forge-test depends on everything (test-only)
```

### Step 3: CI Pipeline (Milestone 0.1)

From the very first commit, CI enforces:
- `cargo build --workspace` passes
- `cargo test --workspace` passes
- `cargo clippy --workspace -- -D warnings` passes
- `cargo miri test -p forge-math` passes (checks unsafe code in math crate)
- Benchmark regressions flagged (criterion, stored in repo)
- Determinism replay tests pass (once introduced in 0.4)

**CI blocks all merges unless all gates pass.** This is KV-01.

### Step 4: First Files to Create

In `forge-math/src/`:
```
lib.rs
sign.rs          → TriSign enum + CertifiedTriSign newtype
rational.rs      → BigInt-backed rational arithmetic
filter.rs        → three-stage filtered evaluation trait
predicates.rs    → orient2d, orient3d, in_sphere
```

In `forge-topo/src/`:
```
lib.rs
state.rs         → TopologyState (epoch-versioned arena)
mesh.rs          → Face, HalfEdge, Vertex with stable IDs
euler.rs         → Euler operators as pure functions on TopologyState
lineage.rs       → Lineage struct, provenance tracking
traverse.rs      → face_edges(), vertex_ring(), etc.
validate.rs      → topological validity checker
```

In `forge-test/src/`:
```
lib.rs
harness.rs       → differential test runner scaffold
generators.rs    → random polyhedra, random boolean pairs
kv.rs            → KV suite runner
```

---

# Part 4: Development Phases

## Phase Sequence

Development proceeds in a **single linear sequence.** Reactive engine milestones are woven into the phase where they naturally belong — no separate track to remember.

```
Phase 0 → 0.5 → 1 → 1B → 1C → 2 → 2B → 2C → 3 → 4 → 5 → 6A → 6B → 7 → 8
```

**Phase 1C (Reactive Substrate)** introduces the signal graph core and topology firewall. All subsequent features are built as signal nodes, ensuring reactivity is foundational rather than retrofitted.

**Signal granularity rule:** Features (Extrude, Boolean, Fillet) are signal nodes. Euler operators remain procedural functions on `MutableDraft` inside each feature's `evaluate()`. Signals operate at feature granularity, not Euler operator granularity.

---

## Phase 0: Foundation Layer
**Duration target:** 3–4 weeks
**Goal:** Math primitives, certified predicates, exact arithmetic with safety valves, and the test harness.
**Crate:** `forge-math` + `forge-test` scaffold
**Depends on:** Nothing
**Unlocks:** Phase 0.5, Phase 1

---

### Milestone 0.1 — Project Scaffold & CI ✅
**What:** Set up the workspace, CI, and empty crate structure per Part 3.
**Acceptance:**
- KV-01: CI blocks merges unless all gates pass
- All crates compile (even if mostly empty)
- Criterion benchmark scaffold runs (even with no real benchmarks yet)

---

### Milestone 0.2 — Filtered Arithmetic Core 🟡 ✅
**What:** The three-stage predicate pipeline that makes exact arithmetic practical.

**Implementation:**
- Define the `FilteredEval` trait:
  - Stage 1: `f64` evaluation with Shewchuk-style error bounds → returns `Some(sign)` if bound resolves it
  - Stage 2: double-double (compensated) arithmetic → resolves 99%+ of remaining cases
  - Stage 3: exact `Rational` (BigInt numerator/denominator) → resolves everything
- Implement core predicates behind this filter:
  - `orient2d(a, b, c)` → sign of 3×3 determinant
  - `orient3d(a, b, c, d)` → sign of 4×4 determinant
  - `in_sphere(a, b, c, d, e)` → insphere test

**Acceptance:**
- Unit tests for non-degenerate and degenerate configurations
- Benchmark: Stage 1 fast-path dominates for random inputs (>95% of calls resolve at Stage 1)

---

### Milestone 0.2.1 — Tri-Valued Predicates + CertifiedTriSign ✅
**What:** Predicates return three values (Neg/Zero/Pos), not two. Zero is meaningful — it represents genuine geometric coincidence, not a degeneracy to perturb away.

**Implementation:**
```rust
// In forge-math/src/sign.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriSign { Neg, Zero, Pos }

/// Can only be constructed inside the predicate module.
/// Topology mutation functions accept ONLY this type.
/// Passing a raw float comparison to a topology function
/// is a compile error.
pub struct CertifiedTriSign(TriSign);

impl CertifiedTriSign {
    /// Only callable from within forge-math predicates
    pub(crate) fn new(sign: TriSign) -> Self { Self(sign) }
    pub fn sign(&self) -> TriSign { self.0 }
}
```

**Why this matters:** Traditional kernels use Simulation of Simplicity (SoS) to perturb away all zeros. But when a user designs two faces to be flush, that's intentional coincidence. SoS would perturb it into a micro-gap or micro-overlap, creating exactly the sliver faces that plague SolidWorks. By returning `Zero` explicitly, Forge can handle flush geometry correctly.

**Acceptance:**
- KV-02: Coplanar points produce `Zero` deterministically (not perturbed)
- KV-02: Near-coplanar points produce `Neg`/`Pos` with the correct sign
- KV-03: Test crate that attempts to use a raw `f64` comparison in a topology function fails to compile

---

### Milestone 0.2.2 — Explicit Coincidence Framework ✅
**What:** When predicates return Zero, the kernel needs rules for what to do. This milestone builds those rules.

**Implementation:**
- Coincidence primitives:
  - `Coplanar(PlaneId, PlaneId)`
  - `Collinear(LineId, LineId)`
  - `CoincidentPoint(PointId, PointId)`
- `CoincidenceGraph`: union-find clustering of coincident entities
  - Deterministic tie-breaking: cluster representatives selected by stable ID ordering
  - Merge rules: coplanar faces merge; overlapping edges merge; coincident vertices merge
  - "Flush union" semantics: two solids sharing a face produce a single solid with no internal face

**Why this matters:** This is what makes `union(cube_A, cube_B)` where they share a face produce one clean block, not two blocks with a paper-thin internal face.

**Acceptance:**
- KV-04: Two cubes in flush union → single block, V=8 E=12 F=6, no slivers
- KV-05: Suite of coplanar overlap cases — all merge deterministically
- All results produce identical topology hashes across 100 runs (D1 enforcement)

---

### Milestone 0.2.3 — Lazy Exact Evaluation + Bit-Growth Budget ✅
**What:** Prevent exact arithmetic from becoming a performance trap.

**The problem:** Each exact rational operation doubles (or worse) the bit-length of the result. After 10 chained operations, you can have thousand-digit numerators. The kernel grinds to a halt.

**Implementation:**
- Track bit-length of every exact `Rational` result
- Define a `PrecisionEscalationPolicy` in the `ModelingContext`:
  - Bit-length threshold (e.g., 512 bits)
  - When exceeded: round to double-double precision and re-certify the sign
  - If re-certification is ambiguous: log a `ToleranceDecision` and apply policy
- The key insight: you almost never need the exact value — you need the exact *sign*. Compute the sign exactly (via the filtered pipeline), then store the *value* at working precision.

**Implementation note:** Start simple. The "coordinate dependency DAG" (symbolic lazy evaluation) is the ideal long-term solution, but for now, a bit-length check with a rounding pressure valve is sufficient. Upgrade to symbolic DAG if profiling shows it's needed.

**Acceptance:**
- KV-06: 10 sequential exact operations keep bit-length bounded under typical workloads
- KV-07: Crafted case that exceeds budget → triggers logged escalation deterministically
- Performance: exact path is rare (<1% of calls in the fuzz corpus) and bounded in cost

---

### Milestone 0.3 — Plane Primitive & Implicit Vertex 🟡
**What:** The fundamental geometric primitives that everything else is built on.

**Implementation:**
- `Plane`: coefficients stored as `f64` with certified bounds + optional exact rational fallback
- `ImplicitVertex`: defined by 3+ plane references, position derived on demand
  - When exactly 3 planes: solve 3×3 system
  - When 4+ planes (overconstrained): choose the best-conditioned triple, verify all planes satisfied within tolerance
  - If inconsistent beyond tolerance: return `PolicyRequired` or error
- `PlaneRelation` queries: point-vs-plane classification using `orient3d` → `CertifiedTriSign`

**Acceptance:**
- Cube from 6 planes: all 8 vertices resolve correctly
- KV-08: Overconstrained apex (4+ planes meeting at a point) → deterministic triple selection + validation against all constraints
- Perturbed near-coplanar planes produce consistent orientation

---

### Milestone 0.4 — Determinism, Replay, and Diagnostics 🟡
**What:** Infrastructure that makes every failure reproducible.

**Implementation:**
- Deterministic RNG with seed propagation (for fuzzers and tie-breaking)
- Deterministic ordering rules: stable sort by ID → lineage → spatial hash (in that order)
- Replay log: captures operations + parameters + policy configuration + seeds + state hashes
- Error taxonomy: `KernelError` enum with structured diagnostic payloads
- Minimal repro extraction: given a failure replay log, strip to minimal reproduction case

**Acceptance:**
- KV-09: Same operation executed 100× → identical topology hash + identical decision log every time
- KV-10: Every failure produces a repro artifact; replaying it reproduces the exact failure

---

## Phase 0.5: State & Versioning Substrate
**Duration target:** 1–2 weeks
**Goal:** Establish the immutable state model that all topology operations use. This prevents a catastrophic rewrite when the reactive engine (Phase 1C) needs undo/redo and incremental updates.
**Crate:** `forge-topo` (foundation layer)
**Depends on:** Phase 0
**Unlocks:** Phase 1, Phase 1C

---

### Milestone 0.5.1 — Epoch-Versioned Topology State 🔴
**What:** Define `TopologyState` as an immutable (or copy-on-write) data structure. Every topology operation is a pure function:

```rust
fn boolean_union(
    state: &TopologyState, 
    solid_a: SolidId, 
    solid_b: SolidId,
    ctx: &ModelingContext
) -> Result<TopologyState, KernelError>
```

**Why this matters:** If topology operations mutate state in-place, then undo/redo requires deep-cloning the entire mesh before every operation. If operations produce new immutable states with structural sharing, undo is just "point to the previous state." The reactive engine in Phase 1C depends on this — without it, you'd have to rewrite every operation.

**Implementation:**
- Start simple: `TopologyState` wraps an `Arc<Arena>` with clone-on-write semantics
- Epoch counter increments on each new state
- Snapshot = keeping an `Arc` reference to a previous epoch
- Upgrade path: if profiling reveals excessive cloning, switch to a persistent data structure (HAMT or similar) with structural sharing. Don't build this upfront.

**Acceptance:**
- KV-11: Apply ops A→B→C, keep `Arc` references to each state. Old states remain valid and readable.
- KV-12: 1,000 small edits don't grow memory linearly (structural sharing works)
- State hashes are stable and deterministic

---

### Milestone 0.5.2 — Topology Hashing & Change Firewall 🟡
**What:** Every topological entity (face, edge, vertex) gets a structural hash. Aggregate solid hash = hash of all entity hashes. This enables the "topology change firewall" in Phase 1C: if a parameter change doesn't alter the topology hash, downstream topology-dependent computations are skipped.

**Implementation:**
- Per-entity hash: computed from entity type + connectivity + lineage
- Solid hash: Merkle-style aggregate of all entity hashes
- Two version counters per solid: `topology_version` and `geometry_version`
  - These are separate because most parameter edits change geometry without changing topology

**Acceptance:**
- KV-13: A geometry-only change (moving a vertex without changing connectivity) does not alter topology hash
- KV-14: A topology change (adding a face) does alter topology hash deterministically

---

## Phase 1: Topology Engine
**Duration target:** 2–3 weeks
**Goal:** Halfedge mesh with lineage tracking, built on the immutable state substrate.
**Crate:** `forge-topo`
**Depends on:** Phase 0 + 0.5
**Unlocks:** Phase 2

---

### Milestone 1.1 — Halfedge Mesh Core 🟡
**What:** The fundamental topological data structure.

**Implementation:**
- Arena-allocated entities with stable IDs (via `slotmap` or `thunderdome`):
  - `Face`: references a loop of halfedges
  - `HalfEdge`: references next, prev, twin, face, vertex
  - `Vertex`: references one outgoing halfedge
- Euler operators as pure functions returning new `TopologyState`:
  - `make_vertex_face` — create a face with a single vertex
  - `split_edge` — subdivide an edge, inserting a new vertex
  - `join_faces` — merge two adjacent faces by removing their shared edge
  - `kill_edge_vertex` — remove an edge and one of its vertices
- Traversal iterators: `face_edges()`, `vertex_ring()`, `edge_faces()`, `boundary_loops()`
- Topological validity checker:
  - Every halfedge has a twin
  - Every face loop is closed (following `next` returns to start)
  - Euler formula: V - E + F = 2 for genus-0 solids
  - No non-manifold edges

**Acceptance:**
- Build a cube via Euler operators: V=8 E=12 F=6, all traversals correct
- KV-15: Validity checker catches: missing twins, broken loops, non-manifold edges
- All operations return new `TopologyState` (never mutate in place)

---

### Milestone 1.2 — Euler Lineage Tracking 🔴
**What:** Every topological entity carries provenance from birth. This is the foundation for persistent naming (Phase 5 selectors).

**Implementation:**
```rust
pub struct Lineage {
    origin_feature: FeatureId,   // which feature created this entity
    creation_op: OpId,           // which Euler operation
    ancestry_hash: u128,         // deterministic hash of parent lineage
}
```
- Every Euler operator updates lineage deterministically:
  - `split_edge` → two new edges carry parent edge's lineage + split op ID
  - `join_faces` → merged face carries both parents' lineage + "dominant parent" selection (deterministic: lower ID wins)
- Lineage hash = hash(parent lineage, operation ID, operation parameters)

**Why this matters:** In Phase 5, selectors need to find "the edges where Extrude-3 meets Extrude-1." Without lineage, you'd need fragile heuristics (nearest edge, geometric matching). With lineage, you query: "edges whose ancestry includes both Extrude-3 and Extrude-1." This is what makes feature references survive topology changes.

**Acceptance:**
- KV-16: Same operation sequence → identical lineage hashes every time
- KV-17: Splitting a face → children carry ancestry reflecting parent + split operation

---

### Milestone 1.3 — BSP-Based Polyhedron Construction 🟡
**What:** Given a set of planes, construct the bounded convex polyhedron.

**Implementation:**
- BSP tree partitioning: recursively split space with each plane
- Each face is a convex polygon defined by plane intersections
- Vertices are `ImplicitVertex` instances (three-plane intersections from 0.3)
- Handle coincidence: when a plane passes exactly through an existing vertex, use the `CoincidenceGraph` from 0.2.2

**Acceptance:**
- Tetrahedron (4 planes), cube (6 planes), hexagonal prism (8 planes): correct face counts, vertex positions, all faces planar
- KV-18: Near-degenerate plane sets → consistent construction via coincidence handling

---

### Milestone 1.4 — Point-in-Solid Classification 🟡
**What:** Given a point and a solid, determine inside/outside/on-boundary.

**Implementation:**
- Ray casting against the halfedge mesh
- Uses tri-valued predicates: ray-face intersection can return Zero (ray hits edge or vertex)
- Degenerate cases handled deterministically via the coincidence framework (not perturbation)
- Returns: `Inside | Outside | OnBoundary(FaceId)`

**Acceptance:**
- 10,000 random points classified against a known cube: matches analytical ground truth
- KV-19: Ray degeneracy suite — ray hits vertex, edge, and face tangentially — all handled deterministically

---

## Phase 1B: AI-Native Kernel Affordances
**Duration target:** 1–2 weeks
**Goal:** Ensure every kernel operation emits the metadata that future AI consumers (and the Feature Tree) depend on. This hardens the "AI substrate" before Boolean complexity lands.
**Crate:** `forge-core` + `forge-kernel`
**Depends on:** Phase 1 (and D9 enforcement)
**Unlocks:** Phase 2 (providing confidence that Boolean ops emit proper diagnostics)

---

### Milestone 1B.1 — Structured Error Taxonomy Audit 🟡
**What:** Verify that every `KernelError` variant carries machine-actionable context + remediation hints.

**Implementation:**
- Audit `KernelError` variants: ensure no "stringly typed" errors exist.
- Add `suggested_fixes` to applicable errors (e.g., `FilletRadiusTooLarge` → `max_allowed`).
- Add `scope` field: identifying which feature/entity caused the error.

**Acceptance:**
- KV-50: Integration tests verify diagnostic payload completeness for all error types.

---

### Milestone 1B.2 — Operation Result Envelope 🟡
**What:** Enforce D9. Every kernel operation returns a rich envelope, not just a value.

**Implementation:**
```rust
pub struct OperationResult<T> {
    pub value: T,
    pub decisions: Vec<ToleranceDecision>,
    pub warnings: Vec<KernelWarning>,
    pub metrics: OperationMetrics,
    pub lineage_delta: LineageDelta, // The "diff" of what changed
}
```

**Acceptance:**
- All Euler operators updated to return `OperationResult`.
- `lineage_delta` accurately reflects created/destroyed entities.

---

### Milestone 1B.3 — Decision Log Protocol 🟡
**What:** Formalize the decision log as a queryable artifact.

**Implementation:**
- `DecisionLog` trait with query methods (by feature, by margin, by type).
- Every decision carries a stable `DecisionId` for later override.

**Acceptance:**
- KV-51: Query API functions correctly (returns empty for planar ops).

---

### Milestone 1B.4 — Topology Diff Infrastructure 🟡
**What:** Machine-readable diff between two `TopologyState` snapshots.

**Implementation:**
- Compare two snapshots by entity ID (cheap due to stable generational handles).
- Output: `added_faces`, `removed_faces`, `modified_entities`.
- This is the foundation for "git-style" workflows (branching/merging) using the feature tree from Phase 2.

**Acceptance:**
- KV-52: Diff is accurate and deterministic for split/join operations.

---

### Milestone 1B.5 — Blueprint Accessibility for AI Agents 🟡
**What:** Ensure this roadmap itself is consumable by AI agents.

**Implementation:**
- Create `PHASE_STATUS.md` at root: machine-parseable tracking of current progress.
- Create `docs/ai-agent-guide.md`: "How to read this repo" for AI agents.

**Acceptance:**
- AI agent can find status and rules within 2 file reads.

---


Phase 1C: Reactive Substrate
Duration target: 2–3 weeks (Updated to account for lifecycle and concurrency hardening)
Goal: Implement the fine-grained signal graph that all subsequent features plug into. Every feature from Phase 2 onward is a signal node in a reactive dependency graph. This phase ensures the engine is safe from circular references, thread-stealing data races, and memory leaks.
Crate: forge-signal (new crate)
Depends on: Phase 1B (needs TopologyState, topology hashing from 0.5.2, OperationResult from 1B.2)
Unlocks: Phase 2 (Boolean features are signal nodes from day one)

Signal granularity rule: Features are signal nodes. Euler operators remain procedural inside MutableDraft transactions. A SplitEdge is not a reactive node — it's an internal step inside Extrude::evaluate(). The feature's output is a signal. The feature's input parameters are signals.

Milestone 1C.1 — Signal Graph Core & State Transitions 🟡
What: The reactive dependency engine. Inspired by Angular signals / SolidJS, but heavily adapted for pure, headless evaluation.

Implementation:

Arena-allocated dependency graph with stable generational node IDs (thunderdome or similar).

Three-state invalidation: Clean(Version) / MaybeStale / Dirty.

Push Phase: When an input changes, synchronously push Dirty notifications to direct dependents, and MaybeStale to all downstream dependents.

Pull Phase: Actual recomputation is lazy — triggered only when a consumer reads the value (e.g., rendering, export, or a downstream query).

Evaluation: When evaluating a MaybeStale node, the engine walks up to dependencies. If the dependencies recompute but return the same structural hash (version unchanged), the MaybeStale node is reverted to Clean without executing its own computation.

Acceptance:

Chain of 1,000 dependent signals: change one input at the top, verify minimal recomputation at the bottom.

Performance gate: Push propagation < 1ms for a 10,000-node graph.

Milestone 1C.2 — Multi-Aspect Topology Firewall ✅
What: Feature outputs carry separable topology and geometry aspects with independent version counters. This is the core optimization that prevents global rebuilds on local dimension tweaks.

Implementation:

Each feature signal has two outputs/aspects: topology_version and geometry_version.

Downstream signals explicitly declare which aspect(s) they read. (e.g., A fillet's edge selector subscribes only to upstream topology).

Topology change firewall: A geometry-only change (e.g., dragging an extrude depth that doesn't alter face connectivity) pushes MaybeStale to topology-dependent signals. When pulled, they check the upstream topology_version. Because it didn't increment, they revert to Clean without recomputation.

Acceptance:

KV-60: Parametric block with fillets. Change a dimension that doesn't alter topology → fillet selectors do not re-evaluate.

Change a dimension that adds a face → selectors do re-evaluate.

Milestone 1C.3 — Graph Lifecycle & Arena Memory Safety 🔴
What: Strict memory management. In a parametric CAD model, features are constantly created, deleted, and replaced. Without a GC, the arena will leak, or worse, panic on dangling subscriptions.

Implementation:

Explicit Unregister Protocol: When a feature is deleted, its node ID is tombstoned in the arena.

Subscription Severing: The deletion routine must walk the node's immediate edges and cleanly sever all downstream push-routes and upstream pull-dependencies.

Weak References for Push: Upstream nodes hold weak generational handles to their subscribers. If an upstream node attempts to push MaybeStale to a node that has been dropped, it safely no-ops and purges the dead handle from its subscriber list.

Garbage Collection Epochs: The arena performs a deferred compact/cleanup pass when the system is idle or the dropped-node count exceeds a threshold, preventing memory fragmentation.

Acceptance:

KV-61: Add and delete 10,000 features in a loop. Memory profile must remain flat (no arena leaks).

KV-62: Delete a node sitting between a parameter and an active feature. Parameter change must not panic the kernel when attempting to push.

Milestone 1C.4 — Cycle Detection & Deterministic Parallelism 🔴
What: Defending the graph against user-created circular references and ensuring dependency discovery survives Rayon parallel execution (enforcing Doctrine D8).

Implementation:

Cycle Detection (Push-Time): During the Dirty/MaybeStale push traversal, maintain a generation counter or visited-set. If a push route encounters a node already in the current traversal stack, instantly abort the push phase and return a structured KernelError::CircularReference(NodeId).

Parallel-Safe Discovery: - Abandon naive thread-local stacks for dependency tracking (which break under Rayon work-stealing).

Implement an explicit EvaluationToken or a span-based context tracker. When a feature evaluates, it is passed a Context object. All reads of upstream signals must pass through this context (ctx.read(signal_id)), securely mapping the dependency regardless of which OS thread executes the closure.

Acceptance:

KV-63: Construct a circular constraint (Feature A width = Feature B width; Feature B width = Feature A width). The kernel immediately returns a structured error; no stack overflows occur.

KV-64: Evaluate 5 independent branches of a feature tree in parallel. The resulting graph topology and evaluation output must be bit-identical to a single-threaded run.
---

## Phase 2: Planar Booleans
**Duration target:** 3–4 weeks
**Goal:** CSG on planar-faced polyhedra with exact predicates. The architecture proves itself here. Ends with the Feature Tree and serialization — the first real features to put in the reactive graph.
**Crate:** `forge-kernel` (depends on `forge-math` + `forge-topo`)
**Depends on:** Phase 1C
**Unlocks:** Phase 3, TC-01 through TC-05

---

### Milestone 2.0 — Spatial Acceleration ✅
**What:** AABB tree for fast face-pair pruning and ray queries.

**Implementation:**
- Build AABB tree at the start of each Boolean operation from current solid's faces
- Provides: `query_faces_in_box()`, `query_edges_near_plane()`, `ray_first_hit()`
- Rebuild strategy (not incremental) — sufficient for planar models up to ~10k faces
- Replaces brute-force traversal in point-in-solid (1.4) and face splitting (2.1)

**Acceptance:**
- **Performance gate:** Point-in-solid on 5,000-face polyhedron: < 1ms with BVH

---

### Milestone 2.1 — Face Splitting 🟡
**What:** Given two polyhedra, compute where their faces intersect and split them along intersection edges.

**Implementation:**
- Use AABB tree to find candidate overlapping face pairs
- For each pair: compute plane-plane intersection line, clip to face boundaries
- Split faces along intersection edges, producing new halfedges with proper lineage
- Handle coincident/coplanar overlaps via `CoincidenceGraph` rules from 0.2.2
- All topological decisions require `CertifiedTriSign`

**Acceptance:**
- Cube-cube intersections at various offsets: correct face counts, topology validity
- Coplanar and flush cases handled correctly (via coincidence framework)

---

### Milestone 2.2 — Face Classification & Assembly 🟡
**What:** Given split faces, classify each as inside/outside/boundary and assemble the Boolean result.

**Implementation:**
- Classify each face of each input solid relative to the other solid:
  - Outside: face is entirely outside the other solid
  - Inside: face is entirely inside the other solid
  - On-boundary: face lies on the boundary of the other solid (coplanar)
- Assemble output by selecting faces per Boolean type:
  - **Union:** outside-of-A + outside-of-B + shared-coplanar (same normal)
  - **Intersection:** inside-of-A + inside-of-B
  - **Subtraction:** outside-of-A + inside-of-B (flipped normals)
- Merge coplanar adjacent faces where possible (simplify topology)
- Lineage: assembled faces carry lineage from their source solid + Boolean operation

**Acceptance:**
- Boolean union/intersection/subtraction of overlapping cubes: correct V/E/F, correct volume (analytical), correct point classification
- Differential comparison against Open CASCADE: volumetric agreement
- **Performance gate:** Boolean of two 1,000-face planar solids < 100ms

---

### Milestone 2.3 — Sketch → Extrude → Cut Pipeline ✅
**What:** The first end-to-end modeling workflow.

**Implementation:**
- 2D sketch representation: line segments and arcs in a plane
- Extrude: convert closed sketch profile to a prism (set of bounding planes), construct polyhedron via BSP (1.3)
- Cut: extrude the cutting profile, then Boolean-subtract from the existing solid

**Test cases unlocked:**
- **TC-01 Flat Bracket:** Rectangular extrude + two through-holes (polygonal approximation of circles) ✓
- **TC-02 L-Bracket:** Two extrudes at 90° + holes on angled plane ✓
- **TC-03 Motor Mount Plate:** Central bore + bolt circle + corner holes ✓
- **TC-04 T-Slot Extrusion:** Complex sketch profile, single extrude ✓
- **TC-05 Keyway Shaft:** Polygonal cylinder + prismatic keyway cut ✓

> **Note:** TC-03 and TC-05 use cylindrical geometry, approximated as polygonal prisms in this phase. They upgrade to exact cylinders in Phase 3.

---

### Milestone 2.4 — Corpus Fuzzing (Planar) 🟡
**What:** Start building the differential testing corpus. This is a compounding moat — start it early.

**Implementation:**
- Random convex polyhedra generator: N random planes → convex solid
- Random Boolean pair generator: two random solids at random positions + random Boolean type
- Random concave polyhedra: union of K random convex polyhedra
- Execute each case in Forge and Open CASCADE
- Compare via volumetric point-in-solid sampling (10,000 points per case)
- Classify mismatches: topological bug / boundary noise / degenerate input
- Corpus grows monotonically — cases are never discarded
- Target: 1,000+ cases per CI run

**Acceptance:**
- < 0.1% mismatch rate on random convex-convex Booleans
- Every mismatch classified and pinned as a regression test

---

### Milestone 2.5 — Planar Edge-Case Battery 🟡
**What:** Handcrafted deterministic cases for the hard planar scenarios.

**Test cases:**
- Coplanar face overlaps (partial, full, exact-edge-aligned)
- Edge-on-edge intersections
- Vertex-on-face intersections
- Solids touching but not intersecting (shared vertex or edge only)
- Solids with identical geometry (union should produce the same solid)

**Acceptance:**
- No crashes, no non-manifold output
- Every ambiguous case produces explicit logged decisions (should be zero for planar — if any occur, investigate)

---

### Milestone 2.6 — Boolean Result Introspection Contract 🟡
**What:** Enforce observability for every Boolean operation to prevent quiet degradation.

**Implementation:**
- Extend `OperationResult<T>` for Boolean operations to include:
  - `face_classification_counts`: (inside/outside/boundary) for each input solid.
  - `split_counts`: Number of edges/faces split during intersection.
  - `coplanar_merge_events`: List of entities merged due to coincidence.
  - `topology_diff_summary`: Machine-readable summary of the resulting diff.
- **Contract:** No Boolean returns without this structured payload.

**Acceptance:**
- KV-53: Integration tests verify that every Boolean result contains a non-empty classification and lineage summary.

---

### Milestone 2.7 — Feature Tree 🟡
**What:** The parametric model as a graph of features. First real features (sketch, extrude, Boolean) wired into the signal graph from 1C.

**Implementation:**
- Feature definition: input parameters (signals) → `evaluate()` → output signals (topology + geometry)
- Feature ordering: determined by the dependency graph, not a linear list
- `register_feature` factory: wraps any feature with validation, caching, dependency registration, error handling
- Features operate on `TopologyState` snapshots from 0.5 — pure functional transforms

**Acceptance:**
- TC-01 as feature tree: sketch → extrude → two cuts. Change hole diameter → only cut features and downstream recompute.

---

### Milestone 2.8 — Native Serialization 🟡
**What:** Save and load the specification graph. Required for AI agent workflows.

**Implementation:**
- JSON serialization of: features, parameters, selectors, constraints, policy configuration
- Round-trip fidelity: serialize → deserialize → evaluate = bit-identical topology, geometry within epsilon
- Diffable: two serialized models can be compared to identify parameter changes / added features
- Version header for forward compatibility

**Why JSON and not a custom DSL:** JSON is AI-readable, machine-diffable, and available today. A custom DSL is a product feature for humans and can come later without any kernel changes.

**Acceptance:**
- TC-01 through TC-05: serialize → deserialize → evaluate → bit-identical topology

---

## Phase 2B: Operation Observability & Traceability
**Duration target:** 1–2 weeks
**Goal:** Enforce D9 (AI-Observability) as a hard architectural invariant. Every kernel operation — from Euler operators to Booleans to feature evaluations — returns a universal envelope that captures what happened, what was decided, what was ambiguous, and what assumptions were made. An AI agent must be able to reconstruct the full state transition from the envelope alone.
**Crate:** `forge-core` + `forge-topo` + `forge-kernel`
**Depends on:** Phase 2
**Unlocks:** Phase 2C (representations need observable operations to trace recomputation)

**Execution model for ambiguity:** The kernel never pauses mid-operation — that would violate D6 (Atomic Transactionality). Instead, every decision is resolved using the `ModelingContext`'s configured policies. If no policy is configured, a safe default is applied. Everything is logged. After the operation completes, the caller (human or agent) can query the decision log, review margins, and override specific decisions — triggering a targeted re-evaluation. This "complete-then-review" model guarantees atomicity while providing full transparency.

---

### Milestone 2B.1 — Unified Operation Envelope 🟡
**What:** Redesign `OperationResult<T>` as the universal return type for every kernel function.

**Implementation:**
```rust
pub struct OperationResult<T> {
    pub value: T,
    pub decision_log: DecisionLog,
    pub warnings: Vec<KernelWarning>,
    pub metrics: OperationMetrics,
    pub lineage_delta: LineageDelta,
    pub state_hash_before: u128,
    pub state_hash_after: u128,
}

pub struct OperationMetrics {
    pub wall_time: Duration,
    pub entities_created: u32,
    pub entities_destroyed: u32,
    pub exact_predicate_calls: u64,
    pub policy_decisions_made: u32,
}
```
- `OperationResult` is generic: every kernel function from `split_edge` to `execute_boolean` to `Feature::evaluate()` returns through this wrapper.
- `state_hash_before/after` enables cheap diff detection without full topology comparison.

**Acceptance:**
- The type compiles and is importable from `forge-core`.
- Unit test: construct an envelope, serialize it, deserialize it, verify all fields roundtrip.

---

### Milestone 2B.2 — Decision Classification Taxonomy 🟡
**What:** Every decision the kernel makes carries a structured, machine-actionable classification.

**Implementation:**
```rust
pub struct TracedDecision {
    pub id: DecisionId,
    pub kind: DecisionKind,
    pub margin: f64,
    pub feature_scope: Option<FeatureId>,
    pub entity_scope: Option<EntityRef>,
    pub overridable: bool,
    pub context: DecisionContext,
}

pub enum DecisionKind {
    /// Predicate resolved exactly — zero ambiguity.
    Exact,
    /// Ambiguity detected, resolved by configured ModelingContext policy.
    PolicyApplied { policy: PolicyKind, default_used: bool },
    /// Near a threshold but resolved with confidence.
    /// Logged for transparency even though no policy was needed.
    NearBoundary { threshold: f64 },
    /// Could not be resolved by policy — safe default applied, flagged for review.
    Ambiguous { fallback_applied: String },
    /// Hard constraint forced a specific outcome (e.g., manifoldness requirement).
    Forced { reason: String },
}

pub enum DecisionContext {
    Classification { point: [f64; 3], result: String },
    Coincidence { entity_a: EntityRef, entity_b: EntityRef },
    Tolerance { measured: f64, threshold: f64 },
    Degeneracy { description: String },
}
```
- No unstructured strings in the hot path. `DecisionKind` is an enum, not a string.
- `margin` quantifies "how close to the threshold" — enables sorting by risk.
- `overridable` tells the agent whether this decision can be changed without a full rebuild.

**Acceptance:**
- All existing `ToleranceDecision` usages migrated to `TracedDecision`.
- KV-50 (updated): Every decision in a Boolean operation has a valid `DecisionKind` and non-negative `margin`.

---

### Milestone 2B.3 — Decision Log as First-Class Artifact 🟡
**What:** The `DecisionLog` is a queryable, serializable, diffable artifact — not just a `Vec`.

**Implementation:**
```rust
pub struct DecisionLog {
    decisions: Vec<TracedDecision>,
}

impl DecisionLog {
    pub fn by_feature(&self, id: FeatureId) -> Vec<&TracedDecision>;
    pub fn by_kind(&self, kind: DecisionKind) -> Vec<&TracedDecision>;
    pub fn by_margin_ascending(&self) -> Vec<&TracedDecision>;
    pub fn ambiguous_only(&self) -> Vec<&TracedDecision>;
    pub fn overridable_only(&self) -> Vec<&TracedDecision>;
    pub fn is_clean(&self) -> bool; // true if zero Ambiguous decisions
    pub fn summary(&self) -> DecisionSummary;
}

pub struct DecisionSummary {
    pub total: usize,
    pub exact: usize,
    pub policy_applied: usize,
    pub near_boundary: usize,
    pub ambiguous: usize,
    pub forced: usize,
    pub min_margin: f64,
}
```
- Serializable via serde alongside the feature tree.
- Diffable: compare two `DecisionLog`s to see what changed between parameter edits.
- `is_clean()` is the quick check: "did this operation make any assumptions I should review?"

**Acceptance:**
- KV-51 (updated): Query API returns correct results for Boolean operations with known decision patterns.
- Serialize → deserialize roundtrip preserves all decision fields.

---

### Milestone 2B.4 — Envelope Wiring: Euler & Topology Layer 🟡
**What:** Retrofit the topology layer so every Euler operator flows through the envelope.

**Implementation:**
- The `apply_op` runner in `forge-topo/src/operator.rs` wraps every operator's output in `OperationResult`.
- Euler operators that detect degenerate cases (e.g., `split_edge` on a zero-length edge) log a `TracedDecision` rather than silently handling or returning a raw error.
- Topology traversal functions that use safety limits (e.g., `MAX_TRAVERSAL_ITERATIONS` in `traverse.rs`) log a `TracedDecision::Forced` if the limit is hit.
- `MutableDraft::commit()` computes `state_hash_after` and populates `lineage_delta`.

**Acceptance:**
- All Euler operators return `OperationResult`.
- `lineage_delta` accurately reflects entities created, destroyed, and modified.

---

### Milestone 2B.5 — Envelope Wiring: Boolean & Feature Layer 🟡
**What:** Boolean operations and feature evaluations return the unified envelope.

**Implementation:**
- `execute_boolean` returns `OperationResult<BooleanResult>` instead of `Result<BooleanResult, KernelError>`.
- `BooleanIntrospection` fields (face counts, split counts, coplanar merges) merged into `OperationMetrics` rather than existing as a separate struct. One envelope, one place to look.
- `Feature::evaluate()` returns `OperationResult<FeatureOutput>`. The feature tree aggregates decision logs from child features into the parent's log.
- The `ModelingContext` provides the configured policies; the envelope records which policies were applied.

**Acceptance:**
- KV-53 (updated): Every Boolean result contains a non-empty `DecisionLog` with classification summary and split counts.
- Feature tree evaluation produces an aggregated `DecisionLog` that includes decisions from all child features.

---

### Milestone 2B.6 — Agent Readiness Gate 🟡
**What:** Validation suite proving the envelope system is sufficient for AI agent workflows.

**Implementation:**
- **Token budget test:** TC-01 (flat bracket) expressed as `forge-schema` JSON payload < 200 tokens. Execute, verify correct topology.
- **Roundtrip fidelity test:** Serialize `OperationResult` → deserialize → verify identical `DecisionLog` and topology hash.
- **Reconstruction test:** Given only the `OperationResult` (no access to intermediate state), an agent can determine: what entities were created, what decisions were made, whether any ambiguity exists, and what the margin of closest decision was.
- **Override test:** Take a Boolean result with a `PolicyApplied` decision, override it, re-evaluate, verify the decision log reflects the override and the topology changes appropriately.

**Acceptance:**
- All four tests pass.
- This is the gate: if an agent can't understand what happened from the envelope alone, Phase 2B is not complete.

---

## Phase 2C: Representation Architecture
**Duration target:** 2–3 weeks
**Goal:** Define the foundational abstractions that enable three concurrent model representations — SDF for real-time preview, topological complex for feature operations, and eventually B-rep for manufacturing export — to coexist as lazy, signal-driven projections of a single specification graph. Validate with planar geometry before curved surfaces add complexity.
**Crate:** `forge-core` (traits) + `forge-view` (SDF evaluator) + `forge-kernel` (integration)
**Depends on:** Phase 2B (representations need observable operations for traced recomputation)
**Unlocks:** Phase 3 (curved geometry must follow representation patterns from day one)

**Design principle:** Features declare *mathematical intent*, not geometric output. A cube feature doesn't produce a mesh — it produces a specification (6 half-space inequalities) that can be *projected* into any representation on demand. The projection is lazy, cached, and wired into the signal graph. This means adding a new representation (e.g., point cloud for LiDAR comparison) requires implementing one trait — not modifying every feature.

---

### Milestone 2C.1 — GeometricIntent Trait 🔴
**What:** The universal abstraction that separates what a feature computes from how it's represented. This is the most architecturally critical trait in the kernel after `GeometrySource`.

**Implementation:**
```rust
// In forge-core — available to all crates without upward dependencies.

/// A feature's mathematical specification, independent of representation.
/// Features implement this to declare what they ARE, not how to render them.
pub trait GeometricIntent: Send + Sync {
    /// Half-space inequalities defining the solid region.
    /// Used by SDF evaluators and point-in-solid classifiers.
    fn half_spaces(&self) -> &[HalfSpace];

    /// Plane set for exact topological construction.
    /// Used by BSP builder and Boolean engine.
    fn plane_set(&self) -> PlaneSet;

    /// Bounding box for spatial acceleration.
    fn bounds(&self) -> BoundingBox;

    /// Intent kind for representation dispatch.
    fn intent_kind(&self) -> IntentKind;
}

pub enum IntentKind {
    /// Defined entirely by planar half-spaces (Phase 2).
    Planar,
    /// Defined by analytic surfaces — cylinder, sphere, cone, torus (Phase 3+).
    Analytic,
    /// Defined by NURBS surfaces (Phase 6+).
    Freeform,
}

pub struct HalfSpace {
    pub normal: [f64; 3],
    pub offset: f64,
}
```
- `GeometricIntent` lives in `forge-core` (the adapter rule — lower layers define traits, upper layers implement them).
- `IntentKind` enables representation providers to dispatch efficiently: a planar SDF evaluator doesn't need to handle NURBS cases.
- The trait is extensible: Phase 3 adds `fn analytic_surfaces(&self)`, Phase 6 adds `fn nurbs_surfaces(&self)`. Existing implementations don't break.

**Why this matters:** Without this abstraction, every new representation would require modifying every feature. With it, a feature declares intent once, and N representation providers consume it independently. This is the difference between O(features × representations) code and O(features + representations) code.

**Acceptance:**
- `MakeCubeFeature` implements `GeometricIntent` with 6 half-spaces.
- `BooleanFeature` implements `GeometricIntent` by composing child intents.
- The trait compiles at the `forge-core` layer without pulling in topology or kernel types.

---

### Milestone 2C.2 — RepresentationAspect System 🟡
**What:** Extend the signal graph's aspect model to support arbitrary representation targets.

**Implementation:**
```rust
// In forge-signal — the reactive engine.

pub enum Aspect {
    /// Topological structure (face/edge/vertex connectivity).
    Topology,
    /// Geometric embedding (vertex positions, surface equations).
    Geometry,
    /// Real-time preview representation (SDF, approximate mesh).
    Preview,
    /// Manufacturing-quality export (NURBS B-rep, STEP).
    Export,
}
```
- Each aspect has an independent version counter in the signal graph.
- A downstream signal declares which aspects it reads. The signal graph routes dirty notifications only along matching aspect edges.
- **Preview aspect** subscribes to `Geometry` changes only (topology changes don't affect SDF evaluation of existing features). This is the generalization of the topology change firewall to all representations.
- **Export aspect** subscribes to both `Topology` and `Geometry` (B-rep must reflect full model state).
- Adding a future aspect (e.g., `Simulation` for FEA mesh) requires adding one enum variant and implementing the corresponding provider — zero changes to existing features or signals.

**Acceptance:**
- Signal graph correctly routes aspect-specific dirty notifications.
- A geometry-only change marks `Preview` as `MaybeStale` but does not mark `Topology` as dirty.

---

### Milestone 2C.3 — RepresentationProvider Trait 🟡
**What:** The pluggable pattern for converting `GeometricIntent` into concrete representation outputs.

**Implementation:**
```rust
// In forge-core — the trait definition.

/// Converts a GeometricIntent into a concrete representation for a specific Aspect.
pub trait RepresentationProvider: Send + Sync {
    /// Which aspect this provider serves.
    fn aspect(&self) -> Aspect;

    /// Can this provider handle the given intent kind?
    fn supports(&self, kind: IntentKind) -> bool;
}

// In forge-view — the SDF implementation.

pub trait SdfProvider: RepresentationProvider {
    /// Evaluate the signed distance field at a point.
    /// Negative = inside, zero = on boundary, positive = outside.
    fn evaluate_sdf(&self, intent: &dyn GeometricIntent, point: &[f64; 3]) -> f64;

    /// Evaluate SDF over a grid for visualization.
    fn evaluate_grid(
        &self,
        intent: &dyn GeometricIntent,
        bounds: &BoundingBox,
        resolution: [u32; 3],
    ) -> Vec<f64>;
}
```
- Providers are composable: `BooleanSdfProvider` computes `min(sdf_a, sdf_b)` for union, `max(sdf_a, sdf_b)` for intersection, `max(sdf_a, -sdf_b)` for subtraction.
- Providers are registered in the `ModelingContext`, making them globally accessible and swappable (e.g., swap a fast approximate SDF for a precise but slower one).
- The pattern is designed for extensibility: adding NURBS B-rep export in Phase 6A means implementing `BrepProvider` — not modifying every feature.

**Acceptance:**
- Provider trait compiles at the `forge-core` layer.
- A test provider can be registered and retrieved from `ModelingContext`.

---

### Milestone 2C.4 — Planar SDF Evaluator 🟡
**What:** The first concrete representation provider. Proves the architecture works with trivial geometry before curved surfaces arrive.

**Implementation:**
- Implement `SdfProvider` for `IntentKind::Planar` in `forge-view`.
- SDF for a convex solid defined by half-spaces: `max(signed_distance_to_each_half_space)`.
- SDF for Boolean compositions:
  - Union: `min(sdf_a, sdf_b)`
  - Intersection: `max(sdf_a, sdf_b)`
  - Subtraction: `max(sdf_a, -sdf_b)`
- Wire the SDF evaluator as a signal node with `Preview` aspect dependency.
- The signal node reads `GeometricIntent` from upstream features and caches the SDF grid.
- On geometry change: SDF grid recomputed. On topology-only change: SDF grid untouched (firewall).

**Acceptance:**
- SDF evaluation of a unit cube at 10 test points matches analytical signed distance.
- SDF evaluation of `union(cube_A, cube_B)` produces correct inside/outside classification for 100 grid points.
- SDF grid recomputation takes < 1ms for a single cube at 32³ resolution.

---

### Milestone 2C.5 — Firewall Proof Suite 🟡
**What:** The critical integration test that validates the entire three-representation architecture.

**Implementation:**
Test case: parametric cube → Boolean union → change cube width.

1. **SDF signal recomputes** ✓ (geometry changed → `Preview` aspect dirty → SDF grid regenerated).
2. **Topology hash unchanged** ✓ (cube still has 6 faces, 12 edges, 8 vertices — connectivity didn't change).
3. **Feature selectors skip re-evaluation** ✓ (topology version didn't increment → `MaybeStale` reverts to `Clean`).
4. **OperationResult envelope** ✓ (the rebuild's decision log shows zero ambiguous decisions for this geometry change).

Second test: add a Boolean cut that changes topology.

1. **SDF signal recomputes** ✓ (new half-spaces added).
2. **Topology hash changes** ✓ (new faces/edges created).
3. **Feature selectors re-evaluate** ✓ (topology version incremented → selectors query new topology).
4. **OperationResult envelope** ✓ (decision log shows the new split events and face classifications).

**Acceptance:**
- KV-60 (brought forward): Both test cases pass with correct signal propagation behavior verified programmatically (not just by output correctness, but by asserting which signals were and were not recomputed).

---

### Milestone 2C.6 — Representation Architecture Documentation 🟡
**What:** Written contract that all Phase 3+ features must follow.

**Implementation:**
- Create `docs/representation-model.md` explaining:
  - The `GeometricIntent → RepresentationProvider → Aspect` pipeline.
  - How to add a new feature (implement `GeometricIntent`).
  - How to add a new representation (implement `RepresentationProvider`).
  - The signal graph wiring pattern for representation nodes.
  - Performance expectations: SDF evaluation budget, caching strategy, invalidation rules.
- Reference from crate-level documentation in `forge-core`, `forge-view`, and `forge-kernel`.

**Acceptance:**
- An AI agent can read `docs/representation-model.md` and correctly implement a new feature that works with existing representations, without reading any other documentation.

---

## Phase 3: Curved Geometry (Analytic Surfaces)
**Duration target:** 5–6 weeks (this is the hardest phase before NURBS)
**Goal:** Introduce analytic surfaces, certified classification for curved geometry, and the tolerance policy infrastructure. Fillets require this — which is why fillets are Phase 4, not Phase 3.
**Crate:** `forge-geom` + `forge-kernel`
**Depends on:** Phase 2C
**Unlocks:** Phase 4, TC-10, TC-14, TC-17

---

### Milestone 3.0 — Modeling Context & Tolerance Policy 🟡
**What:** The policy infrastructure that governs all curved geometry decisions. This is Forge's architectural differentiator over incumbent kernels.

**Implementation:**
```rust
pub struct ModelingContext {
    pub tolerance: TolerancePolicy,      // spatial tolerance for coincidence
    pub tangency: TangencyPolicy,        // near-tangent surface handling
    pub sliver: SliverPolicy,            // minimum face area before merge
    pub gap_closure: GapClosurePolicy,   // max gap for automatic sewing
    pub precision: PrecisionEscalationPolicy, // from 0.2.3
    
    // Every tolerance-driven decision is logged here
    pub decisions: Vec<ToleranceDecision>,
}

pub struct ToleranceDecision {
    pub feature: FeatureId,
    pub location: Point3,
    pub margin: f64,           // how close to the threshold
    pub decision: DecisionKind,
    pub overridable: bool,
}
```
- Query API: "show every tolerance decision in this model" → returns the full decision log
- Override API: "re-evaluate this decision with tighter tolerance" → triggers targeted recomputation

**Why here and not Phase 0:** Phases 0–2 use exact predicates on planar geometry. There are zero tolerance decisions — that's the whole point of exact predicates. Introducing tolerance infrastructure before it's needed creates pressure to use it where it shouldn't be. Phase 3 is the exact moment curved surface interactions create genuine ambiguity.

**Acceptance:**
- KV-20: Same near-tangent case under different policies → different but internally consistent results, both replayable

---

### Milestone 3.0.1 — The Exactness Contract (Documentation) 🟡
**What:** A written design decision that every AI agent working on Phases 3+ must read.

**The contract:**
> For planar/linear geometry, "exact" means algebraically exact — predicates return the mathematically correct answer with zero error.
>
> For curved geometry, "exact" means "certified bounds with known error." Classification is correct within the bounds. When bounds cannot resolve a decision, the `ModelingContext` policy applies and the decision is logged.
>
> This distinction is permanent and by design. Curved geometry is not algebraically exact because irrational numbers (√2, π) cannot be represented exactly in any finite arithmetic. The goal is not exact answers — it is certified answers with bounded, measurable, controllable error.

This is saved as `docs/exactness-contract.md` and referenced in the crate-level documentation for `forge-geom`.

---

### Milestone 3.1 — Analytic Surface Library ✅
**What:** The surface types that cover ~80% of mechanical CAD geometry.

**Implementation:**
- Surface types: `Plane`, `Cylinder`, `Cone`, `Sphere`, `Torus`
- Each supports: point evaluation, normal computation, UV parameterization
- Robust parameter storage: axis vectors normalized, radii positive, canonical forms enforced

**Acceptance:**
- Unit tests for evaluation and normals against known values
- KV-21: UV seam consistency across all surface types (U=0 and U=2π map to the same point)

---

### Milestone 3.2 — Certified Curved Classification Primitives 🔴
**What:** The tools for making certified topological decisions about curved geometry.

**Implementation:**
- Certified bounds for surface evaluation over parameter boxes:
  - Given a UV rectangle on a surface, compute a bounding box for all points in that region
  - Prefer affine arithmetic over naive interval arithmetic (avoids the dependency problem)
  - Interval arithmetic acceptable as fallback where affine is complex
- Certified patch-pair classification:
  - Given two surface patches (UV rectangles), determine: `Disjoint | Intersecting | Ambiguous`
  - `Disjoint`: bounding boxes don't overlap → certified no intersection
  - `Intersecting`: sign change detected in interval evaluation → certified intersection exists
  - `Ambiguous`: can't resolve → needs further subdivision or strategy switch

**Acceptance:**
- KV-22: Cases that fail under naive interval arithmetic (due to dependency problem) resolve correctly under affine arithmetic

---

### Milestone 3.2.5 — Trim Loop Kernel 🔴
**What:** Robust representation and validation of parameter-space trim loops.

**Implementation:**
- Represent trim loops as closed sequences of UV curves.
- Validation: verify loop closure, orientation (clockwise for holes, counter-clockwise for outer), and self-intersection detection.
- Point-in-trim classification: stable winding number or ray-casting in UV space.
- Handle "near-closed" trims through deterministic healing.

**Acceptance:**
- KV-45: Trim validity suite — loops close, holes work, near-closure handled deterministically.

---

### Milestone 3.3 — Surface-Surface Intersection (Analytic Pairs) 🔴
**What:** Intersection of analytic surface pairs where exact or semi-exact solutions exist.

**Implementation:**
- Plane-Plane → line (exact)
- Plane-Cylinder → line or ellipse (exact)
- Plane-Sphere → circle (exact)
- Plane-Cone → conic section (exact)
- Cylinder-Sphere, Sphere-Sphere → circle (exact)

**Acceptance:**
- KV-23: Sampled points on intersection curves lie on both surfaces within tolerance
- Differential comparison against Open CASCADE for all analytic pairs

---

### Milestone 3.3.1 — Cylinder-Cylinder SSI 🔴
**What:** The single hardest analytic SSI case. Gets its own milestone because it causes more kernel failures industry-wide than any other surface pair.

**Case taxonomy (all must be handled):**
- Skew axes (general position): 0 or 2 intersection curves (degree-4 space curves)
- Parallel axes: 0 or 2 intersection curves (degenerate, lines or ellipses)
- Intersecting axes: 0, 1, or 2 intersection curves
- Near-tangent: wide region of near-contact → governed by TangencyPolicy

**Implementation:**
- Classify topology first (how many curves, what connectivity) using certified bounds.
  - Explicit Outputs:
    - Number of intersection components.
    - For each component: connectivity (open/closed), UV parameter-space mapping on each surface, and certified region partitioning boundary.
- Extract geometry second (approximate the degree-4 curves numerically).
- Geometry may be approximate but must be bounded and attached to correct topology.

**Acceptance:**
- KV-24: Named cylinder-cylinder suite with expected topology counts for each case in the taxonomy
- KV-46: Classifier returns correct component count/connectivity even when curve extraction is coarse.
- Differential comparison against Open CASCADE

---

### Milestone 3.4 — Booleans on Mixed Geometry 🔴
**What:** Extend the Boolean engine from Phase 2 to handle curved faces.

**Implementation:**
- Split faces along surface-surface intersection curves (not just plane-plane lines)
- Classification uses certified bounds from 3.2 + policy from 3.0 at ambiguity
- Topology-geometry separation enforced: topological decisions from certified classification, geometric embedding computed separately
- Any ambiguous classification triggers a logged `ToleranceDecision`

**Test cases unlocked:**
- **TC-10 Turned Spacer:** Multiple diameter transitions (cylinder-plane Booleans) ✓
- **TC-14 Pipe Tee:** Three cylinders meeting at T (cylinder-cylinder Boolean — the hard case) ✓
- **TC-17 Lattice Cube:** 75 Boolean subtractions (cylinder-cylinder near-intersections) ✓
- **TC-19 Welded Structural Node:** Non-orthogonal cylinder-cylinder Booleans at 0°/45°/90°/135° ✓

**Acceptance:**
- KV-25: Every ambiguous case triggers a logged decision or explicit `PolicyRequired`
- All unlocked TCs pass differential comparison

---

### Milestone 3.4.5 — Revolve Operation 🟡
**What:** Revolve a 2D profile around an axis to produce a solid of revolution.

**Implementation:**
- Profile shape determines surface types:
  - Line parallel to axis → cylinder
  - Line at angle to axis → cone
  - Line perpendicular to axis → planar annulus
  - Arc → torus or sphere section
- Full or partial revolution (e.g., 180° revolve)

**Test cases upgraded:**
- **TC-10 Turned Spacer:** Now with exact analytic surfaces (not polygonal approximation) ✓
- **TC-13 Involute Gear (single tooth):** Involute curve sketch → extrude → pattern ✓

---

### Milestone 3.5 — Adaptive Distance-Field Classification 🧪🔴
**What:** Fallback strategy when subdivision stalls near tangency. Transforms exponential convergence into polynomial cost.

**The problem:** When two surfaces are nearly tangent over a wide region, subdivision depth explodes (each level only narrows the uncertainty by a factor of θ, where θ is the tangency angle). You can't subdivide your way out.

**Implementation:**
```rust
enum IntersectionStrategy {
    // Normal case: surfaces cross transversally
    // Subdivision resolves in 4-8 levels, cost manageable
    Subdivision { depth: u32 },
    
    // Near-tangent case: compute signed distance field between
    // the two surfaces in the tangent region.
    // The distance function is smooth and well-conditioned even
    // when the intersection curve isn't.
    // Cost: O(n²) in grid resolution, not exponential in depth.
    DistanceField { 
        region: BoundingBox,
        grid_resolution: u32,
    },
    
    // Truly degenerate: surfaces within manufacturing tolerance
    // of tangent. This is a DESIGN question, not math.
    // Route to TangencyPolicy + log ToleranceDecision.
    WithinTolerance {
        region: BoundingBox,
        max_gap: f64,
        policy_decision: ToleranceDecision,
    },
}
```
- Automatic strategy selection: start with subdivision, detect stalling (depth > threshold), switch to distance field
- Lipschitz bounds certify sign constancy inside grid cells
- Conservative classification: if unresolved, refine grid or escalate to policy

**Acceptance:**
- **Performance gate:** Near-tangent cylinder-cylinder classification < 500ms, polynomial scaling
- **Performance gate:** Subdivision depth < 16 levels for 99% of fuzz corpus
- KV-26: No missed sign changes — constructed thin-region test cases all detected

---

### Milestone 3.6 — Curved Corpus Fuzzing 🔴
**What:** Extend the fuzz corpus to curved geometry.

**Implementation:**
- Random analytic solid generator: random cylinders, spheres, cones at random positions
- Near-tangency injection: deliberately create cases near the tangent boundary
- Track per-case: policy decision frequency, non-manifold outputs, divergence from Open CASCADE
- Extend the corpus from 2.4 — all planar cases still run

**Acceptance:**
- Zero crashes
- Non-manifold output only if explicitly allowed by policy (normally not)
- Every ambiguity explicitly logged

---

### Milestone 3.7 — Ambiguity Surface Mapping 🟡 
**What:** Provide spatial and surface context for ambiguous decisions.

**Implementation:**
- When a decision is marked as `Ambiguous`, the `ToleranceDecision` must include:
  - `region_bounds`: World-space AABB of the uncertainty region.
  - `surface_pair_ids`: IDs of the surfaces involved.
  - `confidence_interval`: Quantitative metric of uncertainty (distance/angle margin).
  - `applied_strategy`: Which algorithm was used (Subdivision / DistanceField / Policy).

**Acceptance:**
- KV-54: Ambiguity reports contain valid spatial coordinates and surface handles.

---

### Milestone 3.8 — Strategy Telemetry 🟡
**What:** Cost and complexity signals for AI planners.

**Implementation:**
- Per-Boolean telemetry:
  - `%_patches_subdivision`: Percentage of surface patches resolved via subdivision.
  - `%_patches_distance_field`: Percentage resolved via Distance Field.
  - `%_patches_policy`: Percentage requiring explicit policy.
  - `max_subdivision_depth`: Highest depth reached during the operation.

**Acceptance:**
- KV-55: Telemetry payload is present in every curved Boolean `OperationResult`.

---

## Phase 4: Fillets & Chamfers
**Duration target:** 3–4 weeks
**Goal:** Edge blending. Chamfers are mostly planar; fillets require the analytic surface infrastructure from Phase 3.
**Crate:** `forge-kernel`
**Depends on:** Phase 3
**Unlocks:** TC-06 through TC-09

---

### Milestone 4.1 — Chamfer on Linear Edges 🟡
**What:** Chamfers are the simplest edge blend — two offset planes trimming adjacent faces.

**Implementation:**
- Given an edge and two offset distances, compute two new planar faces
- Trim adjacent faces to the chamfer planes
- Handle chamfer-chamfer intersection at corners (three chamfer faces meeting at a vertex)

**Test cases unlocked:**
- **TC-07 Hex Standoff:** 0.5mm chamfers on all 12 edges of a hex prism ✓

**Acceptance:**
- KV-27: Corner chamfer cases — multiple chamfers meeting at a vertex produce clean topology

---

### Milestone 4.2 — Constant-Radius Fillet on Planar Edges 🔴
**What:** Rolling-ball fillet producing cylindrical blend surfaces.

**Implementation:**
- Cylindrical blend surface between two planar faces, tangent to both
- Trim adjacent faces to the tangent lines
- Corner patches: spherical surfaces where multiple fillets meet
- Convex edges (exterior fillet) and concave edges (interior fillet) — different trim geometry

**Test cases unlocked:**
- **TC-06 Electronics Enclosure:** 3mm exterior fillets + 1mm interior fillets on shelled box ✓
- **TC-09 Mounting Block:** 5mm/2mm/1mm fillets converging at shared geometry ✓

**Acceptance:**
- KV-28: Convex and concave corner patches — topology valid, surfaces tangent at boundaries

---

### Milestone 4.2.1 — Topological Consumption & Cascade 🔴
**What:** What happens when a fillet radius is larger than an adjacent face.

**The problem:** If you fillet an edge with radius 5mm but the adjacent face is only 4mm wide, the fillet "consumes" that face entirely. The face must be deleted, its neighbors must be re-trimmed, and potentially other fillets that referenced it must update. This cascade is where incumbent kernels crash.

**Implementation:**
- Detect consumption before executing: if fillet would consume a face, flag it
- Graph rewrite rules: delete consumed face, propagate boundary changes, preserve manifoldness
- Lineage updates: consumed faces logged, dependent features notified
- Transactional: if cascade leads to non-manifold state, roll back the entire operation

**Acceptance:**
- KV-29: "Radius exceeds face width" suite — kernel either cascades cleanly or returns a structured error. Never crashes, never produces non-manifold output.

---

### Milestone 4.3 — Fillet on Curved Edges 🔴
**What:** Fillets where at least one adjacent face is curved.

**Implementation:**
- Cylinder-plane fillets (the common case for bore-to-flat-face transitions)
- Mixed-curvature edge fillets
- Near-tangent fillet behavior governed by `TangencyPolicy`

**Test cases unlocked:**
- **TC-08 Round Knob:** 5mm fillet approaching tangency on cylindrical body + 1mm fillets on grip flutes ✓

**Acceptance:**
- KV-30: Near-tangent fillet suite — results match expected policy outcomes, all decisions logged

---

### Milestone 4.4 — Variable Fillets & Failure Detection 🟡
**What:** Fillets with changing radius, and pre-execution failure detection.

**Implementation:**
- Variable-radius fillet: linear interpolation between two radii along an edge
- Pre-execution analysis: compute whether the requested fillet would cause self-intersection, face consumption, or other failure
- Structured error reporting: "Fillet radius 5mm exceeds wall thickness 4mm at edge E17, 35mm from start"

**Acceptance:**
- KV-31: Predictive failure suite — no partial corruption on failure (transactional rollback)

---

### Milestone 4.5 — Fillet Regression Fuzzer 🔴
**What:** Automated fillet stress testing.

**Implementation:**
- Random solids + random edge selections + random radii
- Invariants checked per case:
  - Manifoldness
  - No self-intersections (or explicitly flagged)
  - No orphaned topology
  - Lineage remains consistent
- Cases that expose bugs are pinned as regression tests

---

### Milestone 4.6 — Cascade Event Reporting 🔴
**What:** Structured metadata for complex topology churn.

**Implementation:**
- When a fillet/chamfer consumes a face or edge, emit:
  - `entities_deleted`: List of IDs removed.
  - `entities_replaced`: ID-to-ID mapping for direct replacements.
  - `lineage_remap_summary`: Distribution of parent lineage into children.
  - `selector_impact_hints`: Potential breakages for downstream selectors.

**Acceptance:**
- KV-56: Consumption cascade produces a valid `lineage_delta` including all remaps.

---

## Phase 5: Sweep, Pattern & Selectors
**Duration target:** 3–4 weeks
**Goal:** Sweep profiles along paths, replicate features via patterns, and build the selector system that enables persistent feature references across rebuilds.
**Crate:** `forge-kernel`
**Depends on:** Phase 4
**Unlocks:** TC-11 through TC-13, TC-16; selectors unlock robust downstream feature references

---

### Milestone 5.1 — Sweep Along Path 🔴
**What:** Sweep a closed 2D profile along a 3D path curve.

**Implementation:**
- Path types: line segment, circular arc, helix
- Surface type determined by profile shape + path shape:
  - Circle profile + line → cylinder
  - Circle profile + arc → torus section
  - Circle profile + helix → parametric sweep surface
  - General profile → general sweep surface
- Self-intersection detection for tight-radius sweeps

**Test cases unlocked:**
- **TC-11 Pipe Elbow:** Annular profile along 90° arc ✓
- **TC-12 Helical Spring:** Ø2mm wire along helix ✓

**Acceptance:**
- KV-32: Self-intersection detection suite — flagged correctly for tight radii

---

### Milestone 5.2 — Linear & Circular Pattern 🟡
**What:** Replicate features efficiently.

**Implementation:**
- Linear pattern: N copies along a direction with spacing
- Circular pattern: N copies around an axis with angular spacing
- Seed-and-copy architecture: pattern references the feature definition, not N independent copies
- Pattern-aware Booleans: batch all pattern instances into a single Boolean pass (performance)

**Test cases unlocked:**
- **TC-03 Motor Mount (upgraded):** Bolt circle as circular pattern ✓
- **TC-13 Spur Gear:** Single tooth patterned 30 times ✓
- **TC-16 Heat Sink:** 10 parallel fins as linear pattern ✓

**Acceptance:**
- KV-33: Pattern determinism — entity IDs and lineage hashes stable across rebuilds

---

### Milestone 5.2.1 — Pattern Identity Semantics 🟡
**What:** Define stable identity and lineage for patterned instances.

**Implementation:**
- Pattern instance ID = `(pattern_feature_id, instance_index, seed_hash)`.
- Lineage includes instance ID to ensure persistent naming.
- Selectors can target specific instance indices or semantic groups (e.g., "middle 5 instances").

**Acceptance:**
- KV-47: Changing instance count doesn't relabel existing instances; only new indices appear/disappear.

---

### Milestone 5.3 — Pattern Performance & Stability 🟡
**What:** Verify patterns scale.

**Acceptance:**
- 1,000-instance linear pattern: completes without crash
- Batched Boolean doesn't alter topology beyond what policy allows
- Lineage remains consistent through pattern copies

---

### Milestone 5.4 — Selector System 🔴
**What:** How features reference topology across rebuilds. Selectors use lineage queries, not fragile entity IDs.

**Implementation:**
- Selectors query by:
  - **Lineage** (primary): "edges whose ancestry includes Extrude-3 and Extrude-1"
  - **Topological adjacency** (secondary): "edges between face A and face B"
  - **Semantic role** (when available): "the bore face of the counterbore feature"
- Selectors never use "nearest edge by distance" as primary key (fragile under parameter changes)
- Re-evaluation on topology change: if the topology hash changed, re-run the selector query
- If topology unchanged (firewall from 1C.2): skip re-evaluation entirely

**Acceptance:**
- TC-06 (enclosure) with fillets via selectors: change box width → fillets reattach to correct edges automatically

---

### Milestone 5.5 — Ambiguity Resolution Protocol 🔴
**What:** When a selector returns multiple matches after a topology change.

**Implementation:**
- Return a structured result:
  - Candidate list (all matching entities)
  - Why it's ambiguous (what changed in topology)
  - Similarity scoring (how well each candidate matches the original reference)
- Default behavior: if one candidate scores significantly higher, auto-resolve and log
- Otherwise: return `AmbiguityRequired` for user/agent to resolve
- Agent API supports: "resolve ambiguity for selector X by choosing candidate Y"

**Acceptance:**
- Constructed test: add a feature that splits a referenced edge into two → selector returns both → ambiguity protocol activates

---

### Milestone 5.6 — Operation Context & Undo 🟡
**What:** Undo/redo using the epoch-versioned state from Phase 0.5.

**Implementation:**
- `OperationContext`: begin (keep `Arc` reference to current state) → compute → commit (new state) or rollback (restore old `Arc`)
- Undo stack: list of `Arc<TopologyState>` references
- Redo: step forward through the stack
- Since `TopologyState` is immutable with structural sharing, undo is O(1) — just switch which `Arc` is "current"

**Acceptance:**
- Build TC-01, undo the extrude → solid disappears. Redo → solid returns with correct geometry.

---

## Phase 6A: NURBS Geometry Binding
**Duration target:** 3–4 weeks
**Goal:** NURBS as a geometry layer on top of exact topology. No NURBS-driven topological decisions yet.
**Crate:** `forge-geom`
**Depends on:** Phase 5
**Unlocks:** Phase 6B, high-quality rendering and STEP export

**Checkpoint:** After Phase 6A, Forge can represent any solid as exact topology + NURBS geometry. This is already sufficient for rendering, FEA meshing, and STEP export of all models from Phases 2–5.

---

### Milestone 6A.1 — NURBS Representation 🟡
**What:** B-spline curves and surfaces.

**Implementation:**
- B-spline curve: degree, knot vector, control points, weights (for rational B-splines)
- B-spline surface: tensor product of two knot vectors + control point grid
- de Boor evaluation: point + derivatives at any parameter value
- Fitting: least-squares approximation of point clouds with NURBS

**Acceptance:**
- Unit tests against known NURBS evaluation references (The NURBS Book test cases)

---

### Milestone 6A.2 — Geometry Binding + Seam Authority Model 🔴
**What:** Attach NURBS surfaces to topological faces, with shared edges managed by a single authoritative curve.

**Implementation:**
- Any face in the halfedge mesh can be backed by a NURBS surface
- Trim curves: derived by projecting topological edge loops onto the face's NURBS surface in UV space
- **Seam Authority Model:** Shared edges have one authoritative 3D curve. Each adjacent face derives its UV trim by projection, not by averaging:
  ```rust
  struct SharedSeam {
      authoritative_curve: NurbsCurve3D,
      face_a_uv_trim: NurbsCurve2D,  // derived by projection onto face A
      face_b_uv_trim: NurbsCurve2D,  // derived by projection onto face B
      max_deviation: f64,             // validation metric
  }
  ```
- Authority selection: the surface with higher continuity order owns the seam
- **Invariant:** No seam has dual authoritative curves. One curve is authoritative; all others project.

**Why this instead of averaging:** Averaging two edge curves ("split the difference") moves both surfaces away from their true positions. Projection from a single authority keeps one surface exact and the other within bounded deviation. This prevents the "both surfaces are wrong by half" problem.

**Acceptance:**
- Cube wrapped in planar NURBS: seam gaps exactly zero
- TC-10 (turned spacer) wrapped in NURBS: max seam deviation < 1e-10
- KV-34: Seam uniqueness invariant enforced — no dual curves

---

### Milestone 6A.2.1 — Projection Robustness 🔴
**What:** The closest-point projection solver that derives UV trims from authoritative 3D curves.

**Implementation:**
- Newton-based closest-point projection from 3D curve to surface
- Fallback strategies for convergence failure:
  - Subdivision of the curve into smaller segments
  - Alternate initial guesses (sample along surface boundary)
  - Policy escalation: log failure + apply `GapClosurePolicy`
- Output metrics per projection: residual distance, iterations, deviation from surface

**Acceptance:**
- KV-35: Projection failure suite — solver fails gracefully, never corrupts topology

---

### Milestone 6A.3 — Shell Operation 🔴
**What:** Offset a solid's faces by a thickness to create a thin-walled shell.

**Implementation:**
- Planar faces: offset plane (exact)
- Analytic curved faces: offset surface (exact for cylinders/spheres, analytic for cones/tori)
- NURBS faces: approximate offset via control point displacement + refinement
- Collision handling: detect where offset faces intersect each other (thin-wall collisions)

**Test cases upgraded:**
- **TC-06 Electronics Enclosure:** Shell operation with proper offset ✓

**Acceptance:**
- KV-36: Thin-wall collision suite — collisions detected and handled, not silently corrupted

---

### Milestone 6A.4 — Watertight Tessellation 🟡
**What:** Tessellation rules that guarantee watertight meshes by construction.

**Implementation:**
- Shared seam sampling: both faces adjacent to a shared edge use identical 3D sample points on the authoritative seam curve
- Per-face tessellation in UV space, with boundary samples locked to seam points
- Watertight mesh verifier: no open edges, consistent winding, normal coherence

**Acceptance:**
- KV-37: Watertight tessellation verified on TC-17 (lattice cube) + 100 random fuzzed models

---

## Phase 6B: Certified NURBS Topology Classification
**Duration target:** 5–7 weeks (the hardest phase in the kernel)
**Goal:** Boolean operations between freeform NURBS bodies. Topology-geometry separation and the tolerance policy are critical.
**Crate:** `forge-kernel` + `forge-geom`
**Depends on:** Phase 6A
**Unlocks:** TC-20 (full)

---

### Milestone 6B.1 — Subdivision + Certified Bounds 🧪🔴
**What:** Determine NURBS patch-pair intersection topology via recursive subdivision with certified bounds.

**Implementation:**
- Recursive Bézier subdivision of both surfaces
- Convex-hull clipping to prune non-overlapping patches (tighter than AABB)
- Affine arithmetic for certified bounds (avoids interval arithmetic dependency problem)
- Integration with `IntersectionStrategy` from 3.5: automatic fallback to distance-field when subdivision stalls

**Acceptance:**
- KV-38: "Dependency problem" suite — cases that stall under naive interval arithmetic resolve correctly under affine/convex-hull bounds

---

### Milestone 6B.2 — Certified NURBS Boolean Assembly 🧪🔴
**What:** Assemble Boolean results from NURBS patch classifications.

**Implementation:**
- Same union/intersection/subtraction assembly logic as Phase 2, operating on NURBS-backed topology
- `SliverPolicy` applies: faces below area threshold auto-merged with neighbors
- `TangencyPolicy` applies: near-tangent regions handled by policy with full logging
- All decisions logged via `ModelingContext`

**Acceptance:**
- Differential comparison against Open CASCADE on selected NURBS cases
- Topology invariants hold; every ambiguity explicitly logged

---

### Milestone 6B.3 — Intersection Curve Extraction (Marching) 🔴
**What:** After topology is locked, extract the geometric intersection curves.

**Implementation:**
- Marching method traces the intersection curve within topologically bounded regions
- Marching cannot change topology — it only refines geometry
- Result fitted as NURBS curves, attached via Seam Authority Model from 6A.2

**Acceptance:**
- KV-39: Marching never changes topology; only refines geometry. Seam deviations bounded.

---

### Milestone 6B.4 — Loft Operation 🟡
**What:** Generate a NURBS surface interpolating between cross-section profiles.

**Implementation:**
- Input: two or more closed 2D profile curves at specified positions
- Output: NURBS surface(s) spanning between profiles
- Trimmed boundaries derived from profile shapes

**Test cases unlocked:**
- **TC-20 Pillow Block housing exterior:** Lofted casting shape ✓

---

### Milestone 6B.5 — Constrained NURBS Fitting 🧪🔴
**What:** The hardest remaining NURBS problem — fitting surfaces that preserve adjacency and continuity.

**Implementation:**
- Sparse local optimization: each face coupled only to adjacent faces via edge continuity constraints
- Alternating local optimization: fix all neighbors, optimize one face, repeat until convergence
- Continuity levels (selectable per edge):
  - **G0** (positional): surfaces meet at shared edges
  - **G1** (tangent): surface normals agree across shared edges
  - **G2** (curvature): surface curvatures agree across shared edges
- Two quality tiers:
  - **Standard (G1):** Edge gaps < 0.01mm, tangent deviation < 0.1°. Sufficient for general machining (90% of use cases). Must converge.
  - **Class-A (G2):** Curvature-matched at boundaries. Required for automotive/aerospace. May not converge — in which case, return best result + quality report. Never claim false success.

**Acceptance:**
- TC-20 pillow block housing: all edge gaps < 0.001mm, G1 deviation < 0.1°
- Class-A: returns quality metrics always; no silent "it's fine" on non-convergence

---

### Milestone 6B.6 — Intersection Confidence Envelope 🟡
**What:** Explainability for complex NURBS interactions.

**Implementation:**
- For every NURBS Boolean, return detailed telemetry:
  - `patch_pair_count`: Number of surface patches examined.
  - `max_subdivision_depth`: Deepest level reached before resolution.
  - `fallback_regions`: List of spatial bounds where distance-field was required.
  - `surface_continuity_report`: Max G0/G1 deviation at all newly created seams.
  - `sliver_merges`: Count of policy-driven topology simplifications triggered.

**Acceptance:**
- KV-57: Boolean results for NURBS solids contain a full continuity and complexity audit.

---

## Phase 7: Manufacturing Features
**Duration target:** 2–3 weeks
**Goal:** Features specific to manufacturing: draft, hole wizard, manufacturability analysis.
**Crate:** `forge-kernel`
**Depends on:** Phase 6B
**Unlocks:** TC-15, TC-18

---

### Milestone 7.1 — Draft (Tapered Walls) 🟡
**What:** Apply draft angle to faces for mold release.

**Implementation:**
- Planar faces → tilted planes (rotated by draft angle relative to pull direction)
- Cylindrical faces → conical surfaces
- Recompute dependent features (fillets on drafted faces update)

**Test cases unlocked:**
- **TC-18 Injection Mold:** 3° draft on all pocket walls ✓

---

### Milestone 7.2 — Hole Wizard Features 🟡
**What:** Parameterized hole types used in manufacturing.

**Implementation:**
- Counterbore: stepped cylindrical cut (large Ø → small Ø)
- Countersink: conical + cylindrical cut
- Snap ring groove: thin annular cut inside a bore (torus-plane intersection)

**Test cases unlocked:**
- **TC-15 Clevis Pin:** Cross-drilled hole (cylinder-cylinder Boolean) ✓
- **TC-20 Pillow Block:** Counterbores + snap ring grooves ✓

---

### Milestone 7.3 — Manufacturability Checker 🟡
**What:** Automated analysis of whether a solid can actually be manufactured.

**Implementation:**
- Minimum wall thickness: offset-based collision detection
- Undercut detection: ray casting from tool approach directions for CNC accessibility
- Minimum feature size: identify fillets, grooves, walls below machinable threshold
- Output: structured report with specific faces/features flagged + measurements
- Integration: tolerance decisions that affect manufacturability are flagged in the `ModelingContext`

**Test cases unlocked:**
- **TC-18 Injection Mold:** "Can this pocket be CNC-milled?" → structured report ✓
- **TC-19 Welded Structural Node:** Cope cut accessibility analysis ✓

---

## Phase 8: File I/O, Parametric Validation & Agent API
**Duration target:** 4–5 weeks
**Goal:** Industry-standard import/export, tolerant sewing for real-world files, full parametric validation of the kernel, and the token-optimized agent API that turns Forge from a library into a product.
**Crate:** `forge-io` + `forge-kernel` (extends)
**Depends on:** Phase 7
**Unlocks:** All TCs exportable + parametric, agent API, external validation

---

### Milestone 8.1 — STEP Export (AP214) 🟡
**What:** Write valid STEP files from Forge B-rep.

**Implementation:**
- Surfaces: plane, cylinder, cone, sphere, torus, B-spline
- Curves: line, circle, ellipse, B-spline
- Topology: closed shell → manifold solid B-rep

**Acceptance:**
- Export all 20 test cases. Import into SolidWorks and FreeCAD. Zero import errors, geometry matches.

---

### Milestone 8.2 — STEP Import (AP203/AP214) 🔴
**What:** Parse STEP files into Forge's internal representation.

**Implementation:**
- Parse STEP entity structure → reconstruct halfedge mesh + surface geometry
- Detect and report: missing normals, reversed face orientations, gaps between edges
- Report naked edges and gap magnitudes before sewing

---

### Milestone 8.2.1 — Import Normalization & Healing Stages 🔴
**What:** A staged pipeline for repairing imported STEP/IGES data.

**Implementation:**
- Stage 1: Orientation normalization (enforce D4).
- Stage 2: Tolerance clustering (vertex/edge coincidence).
- Stage 3: Edge sewing (Milestone 8.2.5).
- Stage 4: Face rebuilding / trimming repair.
- Stage 5: Shell stitching and solidification.
- Stage 6: Manifold validation.
- Every stage outputs a delta log and state hash.

**Acceptance:**
- KV-48: Same file yields identical stage outcomes and logs.

---

### Milestone 8.2.5 — Tolerant Edge Sewing 🔴
**What:** Close gaps in imported models by merging nearby naked edges.

**Implementation:**
- Build spatial index of all naked (unmatched) edges
- Find candidate edge pairs within `GapClosurePolicy` tolerance
- Test topological compatibility: would merging produce a valid manifold?
- Merge greedily from tightest to loosest gaps
- Stop when further merging would create non-manifold topology
- Apply Seam Authority Model: merged edges get a single authoritative curve
- Deterministic tie-breaking: when multiple candidates are equidistant, select by entity ID
- Log every sewing decision as a `ToleranceDecision`
- Report remaining naked edges with gap magnitudes

**Acceptance:**
- KV-40: Sewing reproducibility — same file imports identically every time
- Import corpus: 95%+ clean import on mechanical parts from GrabCAD/TraceParts (track honestly, report actual rate)

---

### Milestone 8.2.6 — Healing Replay Surface 🟡
**What:** Turn import healing from a "black box" into a queryable, overridable plan.

**Implementation:**
- Import process outputs a `HealingPlan`:
  - `edges_merged`: List of merged pairs with gap magnitude.
  - `faces_reoriented`: Lineage of reoriented faces.
  - `surfaces_rebuilt`: List of entities replaced during stage 4.
  - `remaining_naked_edges`: Persistent IDs + spatial coordinates of unsewn gaps.
- **AI Affordance:** Agents can tighten tolerances for a specific gap or disable a specific merge via the API.

**Acceptance:**
- KV-58: Healing plan is serializable and identifies all topological mutations performed during import.

---

### Milestone 8.3 — IGES Import/Export 🟡
**What:** Secondary format for legacy systems. Entity types 128, 186, 510, 514.

---

### Milestone 8.4 — STL/3MF Export 🟡
**What:** Triangle mesh export using watertight tessellation rules from 6A.4.

**Implementation:**
- Adaptive tessellation: finer on high-curvature faces, coarser on flats
- 3MF: includes color and metadata for 3D printing
- Uses seam-driven tessellation from 6A.4 for watertightness guarantee

**Acceptance:**
- TC-17 (lattice cube) exports as watertight STL with correct normals and no self-intersections

---

### Milestone 8.5 — Full Parametric Test Cases 🟡
**What:** End-to-end parametric validation of the kernel. Every feature in the signal graph, every selector re-evaluating, every tolerance decision traceable.

**Test cases:**
- **TC-09 Mounting Block (parametric):** Change bore diameter 20→25mm → fillets update → volume changes correctly
- **TC-16 Heat Sink (parametric):** Change fin count 10→15 → pattern updates → fin root fillets update → volume changes
- **TC-18 Injection Mold (parametric):** Change pocket width → draft angles update → runner channel updates → manufacturability checker re-runs
- **TC-20 Pillow Block (full assembly, parametric):** Change shaft diameter → bore updates → bearing updates → snap ring groove repositions → STEP export → clean import in SolidWorks

**Acceptance:**
- **Performance gate:** TC-09 interactive rebuild < 50ms
- All TCs produce correct results under parameter changes
- All tolerance decisions traceable in the `ModelingContext` log

---

### Milestone 8.6 — Token-Optimized Declarative Agent API 🟡
**What:** The API that makes Forge usable by AI agents at minimal token cost.

**Implementation:**
- Strict versioned schema (JSON)
- Maps 1:1 to the feature tree + policies
- Supported operations:
  - `create_feature` — add a feature to the tree
  - `edit_parameter` — change a feature's parameter
  - `query_decisions` — retrieve all tolerance decisions
  - `resolve_ambiguity` — pick a candidate for an ambiguous selector
  - `export` — produce STEP/STL/3MF
  - `validate` — run manufacturability checks
- Design for minimum tokens: a complete bracket (TC-01) should be expressible in < 200 tokens

**Acceptance:**
- **KV-41 Agent Gate:** TC-15 (clevis pin) generated end-to-end via declarative JSON payload in < 500 tokens
- Deterministic: same payload → identical topology hash

---

### Milestone 8.7 — Cost & Complexity Estimation Endpoint 🟡
**What:** Enables agents to plan changes before execution.

**Implementation:**
- Add `simulate_change(parameter_edit)` endpoint:
  - `will_topology_change`: Boolean prediction via firewall pre-check.
  - `recompute_scope`: List of features requiring re-evaluation.
  - `estimated_boolean_count`: Impact on geometric complexity.
  - `time_bucket`: Estimated cost (Cheap/Medium/Expensive).

**Acceptance:**
- KV-59: Estimation API returns conservative recompute bounds for parametric edits.

---

# Part 5: Reference Tables

## Dependency Map (Visual)

```
Phase 0 ──→ Phase 0.5 ──→ Phase 1 ──→ Phase 1B ──→ Phase 1C (signals)
                                                        │
                                                        ▼
                                                   Phase 2 (Booleans + Feature Tree + Serialization)
                                                        │
                                                        ▼
                                                   Phase 2B (Operation Observability)
                                                        │
                                                        ▼
                                                   Phase 2C (Representation Architecture)
                                                        │
                                                        ▼
                                                   Phase 3 (curved geometry)
                                                        │
                                                        ▼
                                                   Phase 4 (fillets/chamfers)
                                                        │
                                                        ▼
                                                   Phase 5 (sweep/pattern + selectors + undo)
                                                        │
                                                        ▼
                                                   Phase 6A (NURBS binding)
                                                        │
                                                        ▼
                                                   Phase 6B (NURBS Booleans)
                                                        │
                                                        ▼
                                                   Phase 7 (manufacturing)
                                                        │
                                                        ▼
                                                   Phase 8 (I/O + parametric TCs + agent API)
```

## Milestone Count by Phase

| Phase | Milestones | Test Cases Unlocked | Risk | Est. LOC |
|-------|-----------|--------------------|----- |----------|
| 0 — Foundation | 0.1–0.4, 0.2.1–0.2.3 (8) | — | 🔴 | ~5,000 |
| 0.5 — State Substrate | 0.5.1–0.5.2 (2) | — | 🔴 | ~2,000 |
| 1 — Topology | 1.1–1.4 (4) | — | 🟡 | ~5,000 |
| 1B — AI Affordances | 1B.1–1B.5 (5) | — | 🟡 | ~2,000 |
| 1C — Reactive Substrate | 1C.1–1C.2 (2) | — | 🟡 | ~3,000 |
| 2 — Planar Booleans | 2.0–2.8 (9) | TC-01..TC-05 | 🟡 | ~11,000 |
| 2B — Operation Observability | 2B.1–2B.6 (6) | — | 🟡 | ~3,000 |
| 2C — Representation Architecture | 2C.1–2C.6 (6) | KV-60 | 🔴 | ~4,000 |
| 3 — Curved Geometry | 3.0–3.8, 3.0.1, 3.2.5, 3.3.1, 3.4.5 (15) | TC-10,14,17,19 | 🔴 | ~11,000 |
| 4 — Fillets/Chamfers | 4.1–4.6, 4.2.1 (7) | TC-06..TC-09 | 🔴 | ~5,500 |
| 5 — Sweep, Pattern & Selectors | 5.1–5.6, 5.2.1 (7) | TC-11..13, TC-16 | 🔴 | ~7,500 |
| 6A — NURBS Binding | 6A.1–6A.4, 6A.2.1 (5) | rendering+export | 🔴 | ~5,000 |
| 6B — NURBS Booleans | 6B.1–6B.6 (6) | TC-20 | 🧪🔴 | ~8,500 |
| 7 — Manufacturing | 7.1–7.3 (3) | TC-15,18,19 | 🟡 | ~4,000 |
| 8 — I/O, Validation & Agent API | 8.1–8.7, 8.2.1, 8.2.5, 8.2.6 (10) | all exportable + parametric | 🔴 | ~10,000 |
| **Total** | **95 milestones** | **20 test cases** | | **~87,000** |

## Performance Gates

| Gate | Milestone | Target |
|------|-----------|--------|
| `orient3d` fast-path | 0.2 | < 10ns |
| BVH point-in-solid (5k faces) | 2.0 | < 1ms |
| Planar Boolean (two 1k-face solids) | 2.2 | < 100ms |
| Near-tangent cyl-cyl classification | 3.5 | < 500ms, polynomial scaling |
| Subdivision depth (99% corpus) | 3.5 | < 16 levels |
| Watertight tessellation | 6A.4 | 0 open edges |
| Interactive TC-09 rebuild | 9.6 | < 50ms |
| Agent payload (TC-15) | 9.7 | < 500 tokens |

## Kernel Validation Suites (KV-01 through KV-41)

| KV | Name | Phase | Tests |
|----|------|-------|-------|
| KV-01 | CI gate enforcement | 0.1 | CI blocks merges on failure |
| KV-02 | Zero is real | 0.2.1 | Coplanar→Zero; near-coplanar→Neg/Pos |
| KV-03 | No uncertified topo | 0.2.1 | Raw float topo decisions fail to compile |
| KV-04 | Flush cube union | 0.2.2 | Two flush cubes → single block |
| KV-05 | Coplanar overlap suite | 0.2.2 | Deterministic merge in all overlap cases |
| KV-06 | Bit-growth bounded | 0.2.3 | 10 sequential ops stay bounded |
| KV-07 | Budget triggers policy | 0.2.3 | Exceeding bit budget → logged escalation |
| KV-08 | Overconstrained apex | 0.3 | 4+ plane vertex → deterministic resolution |
| KV-09 | Determinism (100×) | 0.4 | Same op → identical hash 100 times |
| KV-10 | Failure replay | 0.4 | Every failure → repro artifact → replay works |
| KV-11 | Snapshot correctness | 0.5.1 | Old states valid after new ops |
| KV-12 | Memory growth guard | 0.5.1 | 1k edits don't grow memory linearly |
| KV-13 | Geometry-only hash stable | 0.5.2 | Geo change → topo hash unchanged |
| KV-14 | Topology hash changes | 0.5.2 | Topo change → hash changes deterministically |
| KV-15 | Topo validity checker | 1.1 | Catches broken loops, missing twins, non-manifold |
| KV-16 | Lineage determinism | 1.2 | Same op sequence → identical lineage hashes |
| KV-17 | Split lineage | 1.2 | Split face → children reflect parent + op |
| KV-18 | Cluster consistency | 1.3 | Near-degenerate planes → consistent construction |
| KV-19 | Ray degeneracy suite | 1.4 | Ray-vertex, ray-edge, ray-face all deterministic |
| KV-20 | Policy consistency | 3.0 | Different policies → different but consistent results |
| KV-21 | UV seam consistency | 3.1 | U=0 ↔ U=2π on all surface types |
| KV-22 | Dependency problem | 3.2 | Naive IA failures resolve under affine bounds |
| KV-23 | SSI curve sanity | 3.3 | Samples on curves lie on both surfaces |
| KV-24 | Cyl-cyl taxonomy | 3.3.1 | All cases produce expected topology counts |
| KV-25 | Ambiguity audit | 3.4 | Every ambiguity logged or returns PolicyRequired |
| KV-26 | No missed sign change | 3.5 | Thin-region test cases all detected |
| KV-27 | Corner chamfer | 4.1 | Multi-chamfer vertex → clean topology |
| KV-28 | Corner fillet patch | 4.2 | Convex/concave corners → valid, tangent surfaces |
| KV-29 | Consumption cascade | 4.2.1 | Fillet exceeds face → cascade or clean error |
| KV-30 | Near-tangent fillet | 4.3 | Policy outcomes match expectations |
| KV-31 | Predictive failure | 4.4 | Failure detected before corruption; rollback clean |
| KV-32 | Self-intersection detect | 5.1 | Tight-radius sweeps flagged correctly |
| KV-33 | Pattern determinism | 5.2 | IDs + lineage stable across rebuilds |
| KV-34 | Seam uniqueness | 6A.2 | No dual authoritative curves |
| KV-35 | Projection failure | 6A.2.1 | Graceful failure, no topo corruption |
| KV-36 | Thin-wall collision | 6A.3 | Collisions detected, not silently corrupted |
| KV-37 | Watertight tessellation | 6A.4 | 0 open edges on all tested models |
| KV-38 | Dependency problem (NURBS) | 6B.1 | Stalls resolved without infinite subdivision |
| KV-39 | Marching topo-safe | 6B.3 | Marching never changes topology |
| KV-40 | Sewing reproducibility | 8.2.5 | Same file → identical import every time |
| KV-41 | Agent gate | 9.7 | TC-15 in < 500 tokens, deterministic hash |
| KV-42 | Topology–Geometry Firewall | 0.5.2 | Noise injection; topology hash stable unless policy triggered |
| KV-43 | Orientation Canonicalization | 1.1 | Random booleans/imports never flip solid orientation |
| KV-44 | Sliver Creation Budget | 2.1 | Ops cannot create > N slivers without explicit policy log |
| KV-45 | Trim Loop Validity Suite | 3.2.5 | Loops close, holes work, near-closure handled |
| KV-46 | SSI Component Classification | 3.3.1 | Correct component count even with coarse extraction |
| KV-47 | Pattern Identity Stability | 5.2.1 | Changing count doesn't relabel existing instances |
| KV-48 | Healing Pipeline Determinism | 8.2.1 | Same file yields same stage outcomes and logs |
| KV-49 | Robustness Regression Guard | 9.6 | Blocks increases in fallback rate or policy decisions |
| KV-50 | Error Taxonomy | 1B.1 | All errors have structured payloads |
| KV-51 | Decision Log Query | 1B.3 | Query API returns correct empty/full logs |
| KV-52 | Topology Diff | 1B.4 | Diffs are accurate and deterministic |
| KV-53 | Boolean Introspection | 2.6 | Every Boolean result contains classification summary |
| KV-54 | Ambiguity Mapping | 3.7 | Ambiguity reports contain valid spatial/surface context |
| KV-55 | Strategy Telemetry | 3.8 | Telemetry payload present in curved results |
| KV-56 | Cascade Lineage | 4.6 | Consumption cascade produces valid lineage remaps |
| KV-57 | NURBS Continuity Audit | 6B.6 | NIRBS Boolean results contain full quality audit |
| KV-58 | Healing Metadata | 8.2.6 | Healing plan identifies all topological mutations |
| KV-59 | Estimation Accuracy | 9.8 | Estimation API returns conservative recompute bounds |
| KV-60 | Representation Firewall | 2C.5 | Geometry-only change recomputes SDF but not topology; topology change recomputes both |
| KV-61 | Envelope Reconstruction | 2B.6 | Agent reconstructs full state transition from OperationResult alone |

## NURBS Robustness Position (Canonical)

| Problem | Status | What Forge Does |
|---------|--------|----------------|
| Topo corruption from float | **Eliminated** (planar); **Policy-driven** (NURBS) | Exact predicates for planar. Certified bounds + explicit TangencyPolicy for NURBS. All decisions logged. |
| Convergence explosion | **Transformed** | Exponential subdivision → polynomial distance-field sampling. Better, not eliminated. |
| Surface fitting gaps | **Bounded** | Seam Authority Model + constrained local optimization. Sparse, well-conditioned. |
| Fillet instability | **Restructured** | SDF for topology + rolling-ball for geometry + NURBS fitting for export. Three simpler problems. |
| Tolerance dependence | **Made explicit** | Cannot be eliminated for NURBS. ModelingContext logs every decision. User can inspect, override, tighten. |

**The thesis:** Forge quarantines NURBS instability. Decisions that can be exact are exact. Decisions that can't are explicit, logged, and controllable. Geometry fitting is well-conditioned optimization, not ill-conditioned root-finding.

---

# Part 6: Robustness Scoreboard (CI Dashboard)

The build pipeline generates a JSON/Markdown report on the health of the kernel. Regressions in these metrics block merges via KV-49.

| Metric | Target | Description |
|--------|--------|-------------|
| Policy Decision Rate | < 2% | Percent of operations requiring a policy/tolerance decision. |
| Avg Subdivision Depth | < 12 | Mean depth before classification resolution. |
| Fallback Rate | < 0.5% | Percent of cases hitting distance-field fallback. |
| Sliver Production | < 5 per op | Average number of slivers produced before cleanup. |
| Mismatch Rate Trend | Flat/Down | Trend vs Open CASCADE differential mismatch. |
| Ambiguity Resolution Rate | > 95% | Percent of ambiguous cases auto-resolved by policy vs `PolicyRequired`. |
| Recompute Accuracy | > 90% | Success rate of the estimation API predicting recompute scope. |
| Healing Success Rate | > 98% | Percent of imported entities successfully sewn/healed. |

---

# Part 7: Phase Completion Checklist

A phase is **complete** only when ALL of the following are true:

- [ ] All milestone unit tests pass
- [ ] All KV suites for this phase pass
- [ ] All applicable TC parts are green (internal validation + differential comparison)
- [ ] Determinism/replay tests pass (KV-09 equivalent for this phase's operations)
- [ ] Performance gates met (or consciously deferred with tracked issue)
- [ ] Corpus fuzzer runs without crashes or unlogged ambiguity
- [ ] Code reviewed for adherence to Global Doctrines (D0, D1, D2)