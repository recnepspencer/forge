# Forge Proof & Validation System v1
### The Internal Certainty Specification

---

# Part 1: How to Read This Document

This specification defines **5 proof layers, 29 milestones, 61 proof validation suites (PV), 50 MetaBoss tests across 7 series (MB), 8 performance gates, and 8 Danger Zones.** It is the companion specification to the `DEVELOPMENT_BLUEPRINT.MD` — that document builds the kernel; this one proves the kernel is correct.

> **See also:** [`METABOSS_TIER4.md`](file:///Users/spenstar/Documents/programming/Forge/METABOSS_TIER4.md) — 24 additional "No Kernel Survives" tests that combine multiple failure modes simultaneously. These are the final 0.0001% scenarios that no commercial kernel handles.

**Proof Layers** are the five independent verification mechanisms. Each layer catches a distinct class of defect. A bug that evades one layer is caught by another. The system achieves certainty through redundancy, not through the perfection of any single layer.

**Milestones** are the atomic work units. Each is scoped for an AI coding agent to implement and validate in isolation. Milestones within a layer are sequential unless noted otherwise.

**PV suites** (Proof Validation) are internal correctness tests specific to the proof system itself. They validate that the proof infrastructure works correctly — they are tests of the tests. Named `PV-01` through `PV-40+`.

**MB series** (MetaBoss) are extreme-condition test batteries designed to push each proof layer beyond the capability of any commercial kernel. Named `MB-T` (topological), `MB-D` (dual-path), `MB-N` (numerical), `MB-R` (replay), `MB-S` (self-consistency), `MB-C` (chains), and `MB-F` (fillets/curves). A green MB series means absolute certainty for that class of defect.

**Performance gates** are hard numeric thresholds. Proof mechanisms must not degrade kernel performance beyond acceptable bounds.

**Risk markers** on each milestone:
- ✅ = Well-understood engineering, low risk
- 🟡 = Moderate complexity, known approaches exist
- 🔴 = Hard problem, requires careful implementation
- 🧪 = Research-grade, may require iteration on approach

**Relationship to the Development Blueprint:**
- Blueprint KV suites validate kernel *features*
- Proof System PV suites validate *verification infrastructure*
- Blueprint doctrines (D0–D9) are the rules; PV suites prove the rules are enforced
- MB series are the ultimate stress tests that no individual KV suite can provide

---

# Part 2: Proof Doctrines

These five principles govern every component of the proof system. They are meta-invariants — invariants about the invariant system itself.

### P0 — Independence of Proof Layers
Each proof layer must operate independently. A failure in Layer 2 (dual representation) must not compromise Layer 1 (topological invariants). Proof layers share no mutable state. They communicate only through the immutable `TopologyState` and `OperationResult<T>` envelope.

### P1 — Proof Before Feature
No modeling feature ships without a corresponding proof mechanism at every applicable layer. The proof system is not a post-hoc audit — it is a prerequisite for feature acceptance.

### P2 — Monotonic Corpus Growth
Test corpus and regression cases grow monotonically. Cases are never discarded. Every bug found by any layer is pinned as a permanent regression test across all layers.

### P3 — Quantifiable Certainty
Every proof mechanism produces a numeric confidence metric. "The test passed" is insufficient. The system reports: margin of closest decision, percentage of entities verified, divergence magnitude, and statistical coverage. Certainty is measured, not asserted.

### P4 — Proof Observability
Every proof mechanism emits structured, machine-readable results that an AI agent can consume within a bounded token budget. An agent must be able to identify: what was proven, what was not proven, and where the closest margin to failure was — from the proof output alone.

### P5 — Deterministic Fuel (No Wall-Clock Dependencies)
No iterative algorithm in the proof or kernel pipeline may use wall-clock time as a termination condition. All iterative processes (adaptive quadrature, rational arithmetic fallback, subdivision refinement) consume **fuel** — a strict counter of iterations or operation cycles. When fuel is exhausted, the algorithm returns `ProofFailure::FuelExhausted` or `KernelError::FuelExhausted` with a structured report of progress made. This guarantees that the exact same logic aborts at the exact same point regardless of CPU load, scheduler jitter, or hardware speed. Performance gates in this document use wall-clock time for CI reporting, but the proof system itself never branches on time.

### P6 — Generational Handle Integrity
All entity references (`FaceId`, `VertexId`, `HalfEdgeId`, `LoopId`) use **generational indices** — a slot index plus a generation counter. When an entity is deleted and the slot is reused, any stale handle from a previous generation is detected immediately via generation mismatch. This is non-negotiable: the ABA problem (reading a new entity's data through a stale handle) produces topology that is structurally valid but semantically corrupt — invisible to Layer 1 invariants. Forge already uses typed generational handles via `thunderdome`; this doctrine mandates that no alternative indexing scheme is ever introduced.

---

# Part 3: The Five Proof Layers

```
Layer 1: Topological Invariants       — "Is the structure legal?"
Layer 2: Dual-Path Verification       — "Do independent algorithms agree?"
Layer 3: Redundant Numerical Modes    — "Do different precisions agree?"
Layer 4: Causal Replay & Witnesses    — "Can we explain and reproduce every decision?"
Layer 5: Self-Consistency Fuzzing     — "Does the kernel agree with itself under algebraic identities?"
```

### Why Five Layers?

| Defect Class | Caught By | Missed By |
|---|---|---|
| Broken halfedge pointers | Layer 1 | Layers 2–5 |
| Wrong face kept in Boolean | Layer 2 | Layer 1 (topology is locally valid) |
| Float sign flip near boundary | Layer 3 | Layers 1–2 (topology valid, classification consistent at float precision) |
| Non-deterministic operation | Layer 4 | Layers 1–3 (each individual run may be correct) |
| Boolean operator breaks identity law | Layer 5 | Layers 1–4 (each operation correct in isolation; only composition reveals the bug) |

No single layer is sufficient. A kernel that passes only topological invariants can still produce geometrically wrong results. A kernel that passes only self-fuzzing can still have non-deterministic edge cases. The five layers form a closed net.

---

# Part 4: Proof Phases

## Phase Sequence

```
Phase P0 → P1 → P2 → P3 → P4
```

Each phase builds one proof layer, bottom-up. Later phases depend on earlier ones for infrastructure but not for correctness — each layer stands alone.

---

## Phase P0: Topological Invariant Fortress

**Duration target:** 2–3 weeks
**Goal:** Extend the existing topological validation into a comprehensive invariant system that catches every structural defect a geometry kernel can produce. This layer produces binary verdicts — pass/fail, no ambiguity, no thresholds.
**Crate:** `forge-topo` + `forge-kernel`
**Depends on:** Existing `validate_topology()` in `forge-topo/src/topology/integrity/validate.rs`
**Unlocks:** Phase P1 (dual-path needs invariants as ground truth)

**Existing foundation:** `validate_topology()` already checks twin reciprocity, previous consistency, vertex continuity, loop closure, and per-shell Euler formula. This is ahead of most commercial kernels. The following milestones extend this into a fortress.

---

### Milestone P0.1 — Geometric Invariant Extensions 🟡
**What:** Add geometric checks that catch degenerate entities invisible to purely topological validation.

**Implementation:**
- Zero-area face detection: compute signed area via cross-product summation over loop edges. Flag faces below `area_threshold` (passed from `ToleranceConfig`).
- Zero-length edge detection: measure 3D distance between edge endpoints. Flag edges below `edge_length_threshold`.
- Signed volume consistency: compute signed volume of each shell. All closed shells of a manifold solid must have positive signed volume (outward normals). Inner shells (voids) must have negative signed volume.
- Degenerate loop detection: loops with fewer than 3 distinct vertices cannot bound a valid face.

**Acceptance:**
- PV-01: Zero-area face injected into a valid cube → validator detects it
- PV-02: Zero-length edge injected → validator detects it
- PV-03: Inverted shell (inward normals) → validator detects via negative signed volume
- PV-04: Degenerate 2-vertex loop → validator rejects

> [!IMPORTANT]
> **NURBS Readiness — Trait-Dispatched Invariants.**
> All geometric invariant computations (area, volume, edge length) must go through a geometry-dispatched evaluator trait — never inline the formula. For planar geometry, `Plane` is the only implementor. When curved surfaces arrive, new implementors slot in without changing the invariant checker code itself. Concretely: the checker calls `surface.compute_area(parameter_domain)`, not `cross_product_area(vertices)`.

> [!NOTE]
> **Future: Pcurve Consistency Invariant (PV-56).**
> When edge geometry is introduced (curved geometry phase), Layer 1 must verify that for every edge, the 3D curve `C(t)` agrees with each adjacent face's surface `S` evaluated at the corresponding parameter curve `p(t)`, within the edge's certified tolerance: `max_t ‖C(t) − S(p(t))‖ < ε`. For planar geometry this invariant is trivially satisfied (edges are plane-plane intersection lines) and is not checked.

> [!NOTE]
> **Future: Parametric Domain Invariants (PV-57..PV-60).**
> When parameterized surfaces are introduced, Layer 1 must additionally validate: (a) trim curve closure in parameter space, (b) non-self-intersection of trim curves, (c) non-degenerate surface Jacobian `∂S/∂u × ∂S/∂v ≠ 0` within each face's parameter domain. These have no planar equivalent and will be implemented when the parametric surface infrastructure is built.

---

### Milestone P0.2 — Euler Characteristic Hardening 🟡
**What:** Extend Euler formula validation to handle multi-shell solids, voids, and non-genus-0 topology.

**Implementation:**
- Generalized Euler: `V - E + F = 2(S - G) + R` where S = shells, G = genus (handles), R = rings (inner loops/holes per face).
- Per-solid validation: decompose the arena into connected solids, validate each independently.
- Handle count extraction: detect genus by computing first Betti number from connectivity.
- Inner loop accounting: count rings (holes in faces) separately from outer loops.

**Why this matters:** A torus has genus 1. A solid with a through-hole has genus 1. Without genus-aware validation, these legitimate topologies would be flagged as errors.

**Acceptance:**
- PV-05: Torus (genus-1) passes generalized Euler validation
- PV-06: Cube with through-hole passes
- PV-07: Multi-shell solid (cube with internal void) passes
- PV-08: Same topologies with one edge removed → fails validation

---

### Milestone P0.3 — Orientation Canonicalization Proof 🟡
**What:** Prove that every solid in the kernel has outward-facing normals and consistent winding at all times.

**Implementation:**
- Post-operation orientation check: after every `MutableDraft::commit()`, verify face normals via signed-volume test.
- Import orientation healing: after STEP/IGES import, canonicalize orientation and log every flip as a `TracedDecision`.
- Orientation inversion detection: if any Boolean operation produces an inverted face (normal pointing inward), flag as a topology error.

**Acceptance:**
- PV-09: 1,000 random Boolean operations → every result has outward normals
- PV-10: Random import files with mixed orientations → healing canonicalizes all, logged deterministically

---

### Milestone P0.4 — Non-Manifold Edge Detection 🟡
**What:** Guarantee that the kernel never produces non-manifold topology (edges shared by more than 2 faces).

**Implementation:**
- Edge valence check: every edge must have exactly 2 adjacent faces in a manifold solid.
- T-junction detection: vertices where 3+ edges meet at a non-manifold junction.
- Post-Boolean manifold gate: after assembly, verify manifoldness before committing.

**Acceptance:**
- PV-11: Constructed non-manifold T-junction → validator rejects
- PV-12: Boolean result that would create a non-manifold edge → operation returns `KernelError::NonManifold` before commit

---

### Milestone P0.5 — Invariant Checkpoint System 🟡
**What:** Wire invariant validation into the operation pipeline so that invariants are checked automatically, not just on demand.

**Implementation:**
```rust
pub enum ValidationCheckpoint {
    /// After MutableDraft::commit()
    PostCommit,
    /// After every Boolean operation
    PostBoolean,
    /// After every feature evaluation
    PostFeature,
    /// After STEP/IGES import healing
    PostImport,
    /// On explicit request
    OnDemand,
}

pub struct ValidationConfig {
    /// Which checkpoints are enabled (default: all in debug, PostBoolean + PostImport in release)
    pub checkpoints: Vec<ValidationCheckpoint>,
    /// Whether to include geometric invariants (more expensive)
    pub include_geometric: bool,
    /// Maximum entities before skipping (perf safety valve)
    pub entity_limit: usize,
}
```
- Integrate with `ModelingContext`: validation config stored alongside tolerance config.
- ValidationResult logged in `OperationResult<T>` envelope.
- Cost-bounded: geometric invariants skip entities beyond `entity_limit` but log the skip.

**Acceptance:**
- PV-13: Boolean with broken result triggers automatic validation failure at `PostBoolean` checkpoint
- PV-14: Validation of 50,000-entity solid completes in < 100ms (non-geometric mode)
- **Performance gate:** Non-geometric validation adds < 5% overhead to operations

---

### Milestone P0.6 — MB-T: MetaBoss Topological Torture Suite 🔴
**What:** Extreme topological stress tests that no commercial kernel survives cleanly.

**Test Series:**
```
MB-T1: 500-step Boolean chain — Euler invariants verified at every step
MB-T2: Near-degenerate face injection — 1,000 faces with area approaching zero from both sides
         of threshold — validator draws correct binary line
MB-T3: Genus-10 solid — 10 through-holes — generalized Euler correct
MB-T4: Multi-shell solid with 20 internal voids — per-shell validation correct
MB-T5: Orientation chaos — 1,000 randomly oriented imported faces — healing produces
         canonical orientation deterministically in 100 consecutive runs
MB-T6: Non-manifold edge stress — 50 Boolean operations designed to create near-non-manifold
         configurations — all caught or all clean
MB-T7: Scale-extreme validation — solid with 1e12 extent + 1e-9 feature size — invariants
         still function (no float overflow in area/volume computation)
```

**Acceptance:**
- All MB-T series green
- Zero false positives (valid topology never flagged)
- Zero false negatives (invalid topology never passes)

---

## Phase P1: Dual-Path Verification Engine

**Duration target:** 3–4 weeks
**Goal:** Build an independent geometric verification path that validates Boolean results by re-classifying geometry through a completely separate algorithm. This catches the most dangerous class of bug: topologically valid but geometrically wrong results.
**Crate:** `forge-kernel` + `forge-topo`
**Depends on:** Phase P0 (needs invariants for ground truth), existing `classify_point_in_solid`
**Unlocks:** Phase P2, MB-D series

**Core principle:** Two independent algorithms are unlikely to have the same bug. If both agree, the result is almost certainly correct. If they disagree, we have found a bug.

---

### Milestone P1.1 — Post-Boolean Cross-Check Wiring 🟡
**What:** After every Boolean operation, re-classify a sample of face centroids through `classify_point_in_solid` on the pre-Boolean operands and compare against the assembled result.

**Implementation:**
- Extract centroids of all result faces.
- For each centroid, classify against the original operands A and B independently using ray-based `classify_point_in_solid`.
- Determine expected inclusion based on Boolean type:
  - **Union:** centroid should be Inside(A) OR Inside(B)
  - **Intersection:** centroid should be Inside(A) AND Inside(B)
  - **Subtraction:** centroid should be Inside(A) AND NOT Inside(B)
- Compare classification against which faces were actually kept.
- Disagreement → `ProofFailure::DualPathMismatch` with centroid coordinates + classifications.

**Acceptance:**
- PV-15: Correct Boolean → cross-check passes (zero false positives on 1,000 random cases)
- PV-16: Deliberately wrong Boolean (wrong face kept) → cross-check catches it

---

### Milestone P1.2 — Winding Number Classifier 🔴
**What:** Build a second, independent point-in-solid classifier based on generalized winding numbers instead of ray casting. This eliminates shared-bug risk between the two paths.

**Implementation:**
- Solid angle summation: for each face, compute the signed solid angle subtended at the query point.
- Sum over all faces: winding number = total solid angle / 4π.
- Result: > 0.5 → inside, < 0.5 → outside, ≈ 0.5 → on boundary.
- The algorithm is fundamentally different from ray casting — no ray-face intersections, no parity counting, no degenerate ray handling.
- Handle open shells (incomplete solids) gracefully: winding number degrades gracefully rather than crashing.
- **BVH-accelerated early rejection:** Build a bounding volume hierarchy over face solid-angle contributions. For faces whose solid-angle upper bound is below a threshold at the query point, skip exact computation. This is critical for meeting the performance gate on high-face-count solids.

**Why a second classifier:** Ray casting and winding-number computation share almost zero code paths. A bug in ray-face intersection logic cannot manifest in solid-angle computation, and vice versa. This is true independence.

**Acceptance:**
- PV-17: Winding-number classifier agrees with ray-casting classifier on 10,000 random points against 100 random solids
- PV-18: Winding-number classifier handles degenerate cases (point on face, point on edge, point on vertex) deterministically
- **Performance gate:** Winding-number query < 10ms for a 5,000-face planar solid (BVH-accelerated); < 50ms for 5,000-face curved solid with polynomial approximation early-outs

---

### Milestone P1.3 — Dual-Path Disagreement Protocol 🟡
**What:** When the two classifiers disagree, execute a structured diagnostic protocol instead of silently accepting either result.

**Implementation:**
```rust
pub struct DualPathResult {
    /// The primary classification (ray casting)
    pub primary: Classification,
    /// The secondary classification (winding number)
    pub secondary: Classification,
    /// Agreement status
    pub agreement: PathAgreement,
    /// Detailed context when disagreement occurs
    pub disagreement_context: Option<DisagreementContext>,
}

pub enum PathAgreement {
    /// Both paths agree — high confidence
    FullAgreement,
    /// Paths agree on inside/outside but differ on boundary
    BoundaryDisagreement,
    /// Fundamental disagreement — one says inside, other says outside
    FundamentalDisagreement,
}

pub struct DisagreementContext {
    pub query_point: [f64; 3],
    pub ray_cast_detail: RayCastDetail,
    pub winding_number_detail: WindingDetail,
    pub nearest_face: FaceId,
    pub distance_to_boundary: f64,
    pub entity_lineage: Lineage,
}
```
- `FundamentalDisagreement` triggers: re-run both classifiers with higher-precision arithmetic (Layer 3 integration). If still disagreeing, flag as `ProofFailure::IrreconcilableDualPath` and abort the operation.
- `BoundaryDisagreement` triggers: log as `TracedDecision::NearBoundary` with the distance-to-boundary metric. Apply `ModelingContext` policy.

**Acceptance:**
- PV-19: Constructed case at exact boundary → both classifiers return boundary-aware results, disagreement protocol logs appropriately
- PV-20: Constructed wrong-classification → `FundamentalDisagreement` raised, operation aborted

---

### Milestone P1.4 — Curved Geometry Dual-Path Extension 🔴
**What:** Extend the dual-path system to work with curved (analytic and NURBS) geometry, not just planar solids.

> [!WARNING]
> **Quadrature Performance Reality:** Adaptive quadrature over curved surfaces is computationally violent. Near boundaries and high-curvature regions, convergence requires deep subdivision trees. The < 10ms planar gate will not hold for complex NURBS. Mitigation: polynomial approximation for early-out rejection (compute degree-4 polynomial bound on solid-angle contribution; if bound resolves sign, skip full quadrature) + aggressive BVH caching of per-face contribution bounds.

**Implementation:**
- Winding-number computation for curved faces: requires numerical integration of solid angle over curved surface patches.
- Adaptive quadrature: subdivide curved faces until solid-angle contribution converges. **Fuel-bounded** (P5 doctrine): maximum quadrature depth is a fuel parameter, not unbounded recursion.
- Polynomial approximation early-out: for each curved face, compute a cheap polynomial upper bound on solid-angle magnitude at the query point. If the bound proves the contribution is negligible, skip full integration.
- BVH-cached contribution bounds: pre-compute per-face solid-angle bounds in the BVH. For distant faces, use cached bounds instead of exact computation.
- Certified bounds: winding-number sum has bounded error from quadrature — if error bound crosses 0.5 threshold, escalate to higher resolution.
- Ray-curved-surface intersection: extends `classify_point_in_solid` with ray-analytic and ray-NURBS intersection.

**Acceptance:**
- PV-21: Dual-path cross-check on cylinder-cylinder Boolean produces zero false positives
- PV-22: Dual-path cross-check on near-tangent cylinder pair correctly identifies boundary region
- PV-53: Fuel-bounded quadrature returns `FuelExhausted` rather than hanging on pathological high-curvature surfaces

---

### Milestone P1.5 — MB-D: MetaBoss Dual-Path Torture Suite 🔴
**What:** Extreme dual-path verification tests.

**Test Series:**
```
MB-D1: 100 random planar Booleans — dual-path must agree on every face centroid
MB-D2: 100 random curved Booleans — dual-path must agree or produce structured disagreement
MB-D3: Near-tangent cylinder union (1e-14 gap) — dual-path catches classification ambiguity
MB-D4: Coplanar face Boolean — face lies exactly on boundary of both operands —
        dual-path handles without false positives
MB-D5: 50-entity solid → Boolean → dual-path cross-check in < 50ms
MB-D6: Scale-extreme: 1e12 block ∪ 1e-6 cylinder — dual-path functions across 18 orders
        of magnitude
```

**Acceptance:**
- All MB-D series green
- Zero false positives on valid Booleans
- 100% detection rate on injected wrong-face errors

---

## Phase P2: Redundant Numerical Modes

**Duration target:** 3–4 weeks
**Goal:** Build the infrastructure to run the same geometric decision through multiple numerical precision modes and detect when float-precision results diverge from exact results. This catches the most insidious class of bug: numerically plausible but mathematically wrong decisions.
**Crate:** `forge-math` + `forge-geom` + `forge-kernel`
**Depends on:** Phase P0, existing `CertifiedTriSign` + exact predicates
**Unlocks:** Phase P3, MB-N series

**Core principle:** A decision that produces the same answer at float precision and at exact (rational) precision is almost certainly correct. A decision that flips between precisions is definitively a near-degenerate case requiring policy intervention.

---

### Milestone P2.1 — Interval Arithmetic Core 🔴
**What:** Add interval arithmetic to `forge-math` as a runtime precision mode between float and rational.

**Implementation:**
- `Interval` type: lower and upper bounds as `f64` values, tracking accumulated error.
- Interval versions of basic operations: add, sub, mul, div, sqrt.
- Sign determination: if interval contains zero → inconclusive, otherwise → certified sign.
- Integration with `CertifiedTriSign`: interval evaluation can produce `CertifiedTriSign` when the interval doesn't contain zero.

**Why interval before rational:** Interval arithmetic is 10–100× faster than rational arithmetic and resolves 99%+ of near-degenerate cases. Rational is the fallback for the remaining 1%.

**Acceptance:**
- PV-23: `orient3d` via interval arithmetic matches exact predicate on 100,000 random inputs
- PV-24: Near-degenerate `orient3d` — interval correctly reports "inconclusive" where float would silently choose
- **Performance gate:** Interval `orient3d` < 100ns (vs. < 10ns for float fast-path)

---

### Milestone P2.2 — Precision Escalation Pipeline 🟡
**What:** Automatic three-stage precision escalation: float → interval → rational.

**Implementation:**
```rust
pub enum PrecisionMode {
    /// Standard IEEE 754 double — fast, sufficient for 95%+ of decisions
    Float64,
    /// Interval arithmetic — catches 99%+ of remaining cases
    Interval,
    /// Exact rational — resolves everything, expensive
    Rational,
}

pub struct PrecisionEscalation {
    /// The mode that produced the final answer
    pub resolved_at: PrecisionMode,
    /// Whether the float result agreed with the final result
    pub float_agreed: bool,
    /// The interval width at the point of escalation (if applicable)
    pub interval_width: Option<f64>,
}
```
- Every predicate call passes through the pipeline automatically.
- `PrecisionEscalation` is attached to the corresponding `TracedDecision`.
- When `float_agreed == false`, the decision is flagged for review — this is a near-degenerate case that float alone would have gotten wrong.

**Acceptance:**
- PV-25: Pipeline resolves all standard cases at Float64 level (no unnecessary escalation)
- PV-26: Crafted near-degenerate case escalates to Interval → still resolves → logs escalation
- PV-27: Crafted exactly-degenerate case escalates to Rational → resolves → logs full escalation chain

---

### Milestone P2.3 — Divergence Detection & Reporting 🟡
**What:** When float and higher-precision modes disagree, produce a structured report identifying the root cause.

**Implementation:**
- After every operation, scan `DecisionLog` for decisions where `float_agreed == false`.
- For each divergent decision, compute:
  - The float-precision answer and the exact answer.
  - The margin (how close to the threshold the float answer was).
  - The entity scope (which face/edge/vertex the decision affected).
  - The topological consequence (would the float answer have changed the result?).
- Aggregate into a `DivergenceReport`:
  ```rust
  pub struct DivergenceReport {
      pub total_decisions: usize,
      pub divergent_decisions: usize,
      pub divergence_rate: f64,
      pub topology_affecting_divergences: usize,
      pub min_margin: f64,
      pub details: Vec<DivergenceDetail>,
  }
  ```
- If `topology_affecting_divergences > 0`, this is a **critical finding** — float precision would have produced a different topology.

**Acceptance:**
- PV-28: Clean operation → divergence rate = 0.0
- PV-29: Near-degenerate operation → non-zero divergence rate, correct classification of topology impact
- PV-30: Report is serializable and parseable by AI agents

---

### Milestone P2.4 — Scale-Invariant Precision Guards 🔴
**What:** Ensure precision escalation works correctly across extreme scale differences — the key challenge for MB-C7 and MB-F4 scenarios.

> [!CAUTION]
> **The f64 Scale Trap:** IEEE 754 `f64` provides ~15–17 decimal digits. At coordinate magnitude 1e12, machine epsilon is ~1e-4. A 1e-9 micro-fillet at 1e12 coordinates **cannot be represented** in f64 without total loss of significance. Interval bounds will instantly widen to encompass zero, forcing every operation into exact rational arithmetic. The fix is **mandatory local coordinate spaces** — not optional optimization.

**Implementation:**
- **Local coordinate space transform (critical):** Before any Boolean or fillet operation involving mixed scales, translate operands to the origin and normalize scale. Compute in local space. Transform results back. This is not an optimization — without it, MB-F4 and MB-C7 are mathematically impossible at f64 precision.
- Condition number computation for geometric operations: before solving a system, estimate condition number.
- Scale-adaptive thresholds: escalation triggers adjusted based on the scale of input coordinates. A 1e-14 residual in a 1e12-scale system is well-conditioned; in a 1e-3-scale system it's ill-conditioned.
- Relative-error intervals: interval arithmetic uses relative error bounds, not absolute, for scale-invariant behavior.
- Explicit underflow/overflow guards: detect when float operations would lose precision due to extreme magnitudes.
- PV-52: Local coordinate transform round-trip: transform to local → compute → transform back → position error < 1 ULP at original scale.

**Acceptance:**
- PV-31: Same geometric configuration at scale 1.0, 1e6, 1e-6, 1e12, 1e-12 → identical topological result at every scale
- PV-32: Mixed-scale operation (1e12 block + 1e-9 feature) → correct precision escalation, no overflow
- PV-52: Local coordinate transform preserves precision across 21 orders of magnitude

---

### Milestone P2.5 — MB-N: MetaBoss Numerical Torture Suite 🔴
**What:** Extreme numerical precision tests.

**Test Series:**
```
MB-N1: 10,000 random orient3d calls near the degenerate plane — float vs. interval vs. rational
        comparison — every divergence caught and classified
MB-N2: Boolean of near-coincident faces (gap = 1e-14) — precision pipeline resolves correctly
MB-N3: 500-step Boolean chain — no accumulated float drift alters any topological decision
MB-N4: Scale-sweep: same Boolean at 20 different scales from 1e-12 to 1e12 — identical
        topology at all scales
MB-N5: Condition-number stress: two nearly-parallel planes (angle = 1e-15 rad) as Boolean
        cutting planes — correct intersection computed or PolicyRequired raised
MB-N6: Bit-growth budget: 100 chained exact rational operations — bit length stays bounded
        per Milestone 0.2.3 of the Blueprint
```

**Acceptance:**
- All MB-N series green
- Zero topology-affecting divergences that are not caught and reported
- Float fast-path resolves > 95% of decisions in the MB-N corpus (precision escalation is rare)

---

## Phase P3: Causal Replay & Witness System

**Duration target:** 3–4 weeks
**Goal:** Build a system where every kernel decision is not just logged, but causally traceable — you can replay any decision, mutate its inputs, and observe the downstream effect. This transforms debugging from "find the bug" into "replay the decision chain and identify the first wrong step."
**Crate:** `forge-core` + `forge-topo` + `forge-kernel`
**Depends on:** Phase P0, existing `DecisionLog` + `ReplayLog` + `Lineage`
**Unlocks:** Phase P4, MB-R series

**Existing foundation:** `ReplayLog` records operations with pre/post hashes. `DecisionLog` captures span-based decision traces with margins. `Lineage` provides Merkle DAG ancestry. This phase connects these into a causal debugging system.

---

### Milestone P3.1 — Checkpoint Diffing 🟡
**What:** Diff `DecisionLog` snapshots between operation steps to identify exactly when a divergence was introduced.

**Implementation:**
- After each operation step in a chain, snapshot the `DecisionLog`.
- `diff_decision_logs(before, after)`: returns new decisions, modified decisions, removed decisions.
- `DecisionDelta` struct: captures what changed between two checkpoints.
- Integration with `ReplayLog`: each `ReplayEntry` carries a `DecisionDelta` relative to the previous step.
- Temporal query: "show me all decisions that changed between step 47 and step 48 of this 500-step chain."

**Acceptance:**
- PV-33: 10-step Boolean chain → diffs correctly identify new decisions at each step
- PV-34: Identical operation re-run → diff is empty (no spurious changes)
- PV-34.5: 10-step Boolean chain → diff_decision_logs produces exact zero false positives and zero false negatives against a full sequential trace. The union of the diffs exactly reconstructs the final DecisionLog
---

### Milestone P3.2 — Minimal Region Extractor 🔴
**What:** Given a problematic entity (face, edge, vertex), extract the minimal topological sub-region needed to reproduce the problem.

**Implementation:**
- N-ring extraction: given a face, extract the face + its N-ring neighborhood (faces sharing edges).
- Boundary sealing: the extracted sub-region must be a valid, closed mesh (add boundary faces as needed).
- Geometry extraction: extract only the planes/surfaces referenced by the extracted entities.
- Serialization: serialize the extracted region as a standalone test case that can be loaded independently.
- Delta-debug integration: binary search over the operation chain to find the minimal prefix that produces the failure.

**Why this matters:** When a 500-step Boolean chain fails at step 487, you don't want to debug all 487 steps. You want the minimal region around the failure, and the minimal sequence of operations that triggers it. This is what makes extreme test cases (MB-C, MB-F) debuggable.

**Acceptance:**
- PV-35: Extract 3-ring neighborhood of a face → produces valid, serializable sub-mesh
- PV-36: Delta-debug on a 100-step chain with injected failure at step 73 → finds step 73 automatically
- PV-36.5: Extract 3-ring neighborhood around a failing entity → the serialized sub-mesh must independently reproduce the exact same ProofFailure or DivergenceReport when the failing operation is applied to it in isolation.
---

### Milestone P3.3 — Causal Decision Chain Reconstruction 🔴
**What:** Given a topological entity in the final result, reconstruct the complete chain of decisions that led to its creation.

> [!WARNING]
> **Token Budget Reality:** Topological changes cascade — a single face split can alter half-edge pointers across an entire shell. Dumping raw `CausalStep` arrays with full `TracedDecision` payloads will exceed any reasonable LLM context. The causal chain must support **semantic summarization** — compressing "vertex V7 moved, edges E12/E13 updated, loop L3 re-linked" into "face F4 was split by plane P2, creating two child faces."

**Implementation:**
- Walk `Lineage` ancestry from final entity back to origin feature.
- For each ancestor entity, query the `DecisionLog` for all decisions that affected it.
- Produce a `CausalChain`:
  ```rust
  pub struct CausalChain {
      /// The entity whose history we're tracing
      pub target: EntityRef,
      /// Ordered list of causal steps, from origin to present
      pub steps: Vec<CausalStep>,
      /// Semantic summary of the chain (agent-consumable, < 200 tokens)
      pub summary: ChainSummary,
  }

  pub struct CausalStep {
      /// The operation that occurred
      pub operation: OpSignature,
      /// The entity at this stage of its life
      pub entity_state: EntityRef,
      /// Decisions made during this operation that affected this entity
      pub decisions: Vec<TracedDecision>,
      /// Pre/post topology hash for this operation
      pub topology_hashes: (u128, u128),
      /// Human/agent-readable one-line summary of what this step did to this entity
      pub semantic_summary: String,
  }

  pub struct ChainSummary {
      /// Total steps in the chain
      pub total_steps: usize,
      /// Steps containing NearBoundary or Ambiguous decisions (the interesting ones)
      pub decision_steps: usize,
      /// The tightest margin across all decisions in the chain
      pub min_margin: f64,
      /// One-line narrative: "Face created by Extrude-1, split by Boolean-3, classified Inside"
      pub narrative: String,
  }
  ```
- **Semantic summarization layer:** Each `CausalStep` generates a `semantic_summary` from its `OpSignature` + entity delta (e.g., "split by plane intersection" or "classified as Inside by ray-cast"). The `ChainSummary::narrative` is a concatenation of the most significant summaries.
- Agent API: `query_causal_chain(entity_id)` → returns the full chain. `query_causal_summary(entity_id)` → returns only `ChainSummary` in < 200 tokens.

**Acceptance:**
- PV-37: Face created by Boolean → causal chain traces back through split → classification → assembly → origin feature
- PV-38: Causal chain for a face in a 50-step chain has < 10 relevant steps (not all 50)
- PV-54: `ChainSummary` for a 50-step entity is < 200 tokens and contains the tightest-margin decision
- PV-54.5: Causal chain for an entity in a 50-step chain excludes all operations that did not mutate the entity or its direct topological N-ring, but retains 100% of operations that altered its bounding vertices/edges. No missing ancestors.

---

### Milestone P3.4 — Witness-Based Replay 🔴
**What:** Replay a specific decision with mutated inputs to test counterfactuals — "what would have happened if this decision went the other way?"

**Implementation:**
- `replay_decision(decision_id, override_value)`: re-execute the operation containing this decision, but force the specified decision to the override value.
- Produce a `CounterfactualResult`:
  - The original topology hash.
  - The counterfactual topology hash.
  - The delta: which entities differ.
  - Whether the counterfactual result passes topological validation.
- This is the most powerful debugging tool for tolerance-sensitive decisions: "if I had classified this face as Inside instead of OnBoundary, would the result still be manifold?"

**Acceptance:**
- PV-39: Override a NearBoundary decision → counterfactual produces valid alternative topology
- PV-40: Override a decision that was correct → counterfactual produces broken topology (proving the original was necessary)
- PV-40.5: Override a NearBoundary decision → counterfactual executes deterministically, completely bypassing the original divergence, and the resulting alternative topology passes Layer 1 (Invariants) and Layer 2 (Dual-Path) checks without returning NonManifold

---

### Milestone P3.5 — MB-R: MetaBoss Replay Torture Suite 🔴
**What:** Extreme replay and causal analysis tests.

> [!CAUTION]
> **The FMA Cross-Architecture Ghost (MB-R6):** IEEE 754 math is standard, but hardware execution is not. Fused Multiply-Add (FMA) computes `(a * b) + c` with a single rounding instead of two. This 15th-decimal-place difference can flip a `NearBoundary` decision and butterfly-effect the entire topology. Cross-session replay across x86 and ARM (or even different compiler flags on the same machine) will silently produce different topologies unless FMA is controlled. **Mitigation:** Enforce `-C target-feature=-fma` for all exact predicate and interval arithmetic modules (already required by Blueprint Doctrine D8). The `ReplayLog` must record the compilation target triple; mismatched triples produce an explicit `ReplayError::ArchitectureMismatch`.

**Test Series:**
```
MB-R1: 500-step Boolean chain — full causal chain for every face in final result computable
        in < 5 seconds total
MB-R2: Deterministic replay — same ReplayLog replayed 100× produces identical DecisionLog
        every time (extending KV-09)
MB-R3: Minimal region extraction on a 10,000-face solid — extracts in < 100ms
MB-R4: Delta-debug on a 200-step chain with failure at step 167 — finds the failure step
        in < 30 seconds (< 8 bisection steps)
MB-R5: Counterfactual replay of every NearBoundary decision in a complex Boolean —
        all counterfactuals produce valid topology or identified as topology-breaking
MB-R6: Cross-session replay — serialize ReplayLog, deserialize in fresh process, replay
        produces identical result. ReplayLog records target triple; mismatched-triple
        replay returns structured ArchitectureMismatch error.
MB-R7: FMA sensitivity test — identify all NearBoundary decisions in a complex Boolean,
        compute each with and without FMA — any that flip are flagged as
        architecture-sensitive and escalated to interval/rational
```

**Acceptance:**
- All MB-R series green
- Replay is bit-exact (not approximately correct — exactly identical)
- Causal chains are complete (no missing ancestors)
- MB-R7: Zero architecture-sensitive decisions left unescalated

---

## Phase P4: Self-Consistency Fuzzing Engine

**Duration target:** 4–5 weeks
**Goal:** Build a fuzzing engine that tests the kernel against itself using mathematical identities. No external oracle needed. The kernel is its own proof system. This is the ultimate layer — it catches bugs that are invisible to every other layer because they only manifest in composition.
**Crate:** `forge-test` + `forge-kernel`
**Depends on:** Phases P0–P3 (uses all previous layers for diagnostics when a fuzzing failure is found)
**Unlocks:** MB-S series, MB-C series, MB-F series

**Core principle:** Boolean algebra has identities. If `(A ∪ B) − B ≠ A`, the kernel has a bug. The kernel doesn't need an external oracle — it needs to agree with itself.

---

### Milestone P4.1 — Boolean Identity Combinators 🟡
**What:** Reusable test combinators that assert Boolean algebraic identities.

**Implementation:**
```rust
/// Tests: (A ∪ B) − B ≈ A  (cancellation law)
pub fn assert_union_cancellation(a: &TopologyState, b: &TopologyState, ctx: &ModelingContext)
    -> ProofResult;

/// Tests: A ∪ B == B ∪ A  (commutativity)
pub fn assert_commutative(a: &TopologyState, b: &TopologyState, op: BooleanOp, ctx: &ModelingContext)
    -> ProofResult;

/// Tests: (A ∪ B) ∪ C == A ∪ (B ∪ C)  (associativity)
pub fn assert_associative(a: &TopologyState, b: &TopologyState, c: &TopologyState,
    op: BooleanOp, ctx: &ModelingContext) -> ProofResult;

/// Tests: A ∪ A == A  (idempotence)
pub fn assert_idempotent(a: &TopologyState, op: BooleanOp, ctx: &ModelingContext)
    -> ProofResult;

/// Tests: A − A == ∅  (self-subtraction)
pub fn assert_self_subtraction_empty(a: &TopologyState, ctx: &ModelingContext)
    -> ProofResult;

/// Tests: A ∩ ∅ == ∅ and A ∪ ∅ == A  (identity elements)
pub fn assert_identity_element(a: &TopologyState, op: BooleanOp, ctx: &ModelingContext)
    -> ProofResult;

/// Tests: Volume(A ∪ B) ≥ max(Volume(A), Volume(B))
pub fn assert_volume_monotonicity(a: &TopologyState, b: &TopologyState, ctx: &ModelingContext)
    -> ProofResult;

/// Tests: Volume(A ∩ B) ≤ min(Volume(A), Volume(B))
pub fn assert_volume_boundedness(a: &TopologyState, b: &TopologyState, ctx: &ModelingContext)
    -> ProofResult;

/// Tests: Volume(A ∪ B) + Volume(A ∩ B) == Volume(A) + Volume(B)  (inclusion-exclusion)
pub fn assert_inclusion_exclusion(a: &TopologyState, b: &TopologyState, ctx: &ModelingContext)
    -> ProofResult;
```
- Each combinator returns `ProofResult` with confidence metrics, not just pass/fail.
- Volume comparison uses signed-volume computation from validated topology (not SDF approximation).
- "Approximately equal" for topology: same number of shells, same Euler characteristic. For volume: within `volume_tolerance` (computed from entity scale).

**Acceptance:**
- PV-41: All combinators pass on two unit cubes with known overlap
- PV-42: All combinators produce meaningful error messages on injected failures

---

### Milestone P4.2 — Transform Invariance Proofs 🟡
**What:** Prove that Boolean results are invariant under rigid transformations — rotation, translation, and uniform scaling.

**Implementation:**
```rust
/// Tests: rotate(A ∪ B) == rotate(A) ∪ rotate(B)
pub fn assert_transform_invariance(
    a: &TopologyState, b: &TopologyState,
    op: BooleanOp, transform: &Matrix4,
    ctx: &ModelingContext,
) -> ProofResult;
```
- Apply transform to both operands, compute Boolean on transformed inputs.
- Apply transform to the original Boolean result.
- Compare: topological structure must be identical, geometric positions within transform-scaled tolerance.
- Test with random rotations, translations, and scale factors.

**Why this matters:** If `rotate(A ∪ B) ≠ rotate(A) ∪ rotate(B)`, the Boolean result depends on absolute orientation. This catches subtly broken coordinate-dependent logic (e.g., hardcoded axis assumptions, non-deterministic sorting by coordinate value).

**Acceptance:**
- PV-43: 100 random rotations → all produce identical topology
- PV-44: 10 scale factors from 1e-6 to 1e6 → all produce identical topology

---

### Milestone P4.3 — Volume Oracle System 🔴
**What:** High-precision volume computation that serves as the ground truth for all identity checks.

**Implementation:**
- Signed-volume computation from divergence theorem: `V = (1/6) Σ |det([v1, v2, v3])|` over all triangulated faces.
- For curved faces: adaptive quadrature of the divergence-theorem integral over surface patches.
- Certified bounds: volume computation returns `(lower_bound, upper_bound)` — identity checks use these bounds for comparison.
- Volume-change tracking: every `OperationResult<T>` carries `volume_before` and `volume_after` as certified intervals.

**Acceptance:**
- PV-45: Unit cube volume = 1.0 ± 1e-15
- PV-46: Analytical sphere (30-face approximation) volume within 1% of 4πr³/3
- PV-47: Volume intervals for Boolean results satisfy inclusion-exclusion within bounds

> [!IMPORTANT]
> **NURBS Readiness — Trait-Dispatched Volume.**
> Volume computation must be dispatched through the surface evaluator trait. The divergence-theorem formula `V = (1/6) Σ |det([v1, v2, v3])|` for planar faces is the first implementation; curved faces will use `∫∫ (1/3) S · n dA` via adaptive surface quadrature with fuel-bounded subdivision (Doctrine P5). The `VolumeOracle` API must accept `&dyn SurfaceEvaluator`, not raw vertex arrays.

---

### Milestone P4.4 — Continuous Fuzz Corpus Engine 🔴
**What:** Automated, monotonically growing test corpus that runs identity checks on randomly generated solids.

> [!IMPORTANT]
> **Adversarial Fuzzing vs. Random Fuzzing:** Random generation is surprisingly uniform — it rarely produces the truly nightmarish scenarios: planes intersecting at 1e-15 radians, or vertices offset from an edge by a single ULP. The corpus must evolve from random to **adversarial**: guided mutation that actively hunts for maximum divergence between float and interval paths.

**Implementation:**
- **Tier 1 — Random solid generators:**
  - Random convex polyhedron: N random planes → BSP construction
  - Random concave solid: union of K random convex polyhedra
  - Random analytic solid: random cylinders, spheres, cones
  - Random positioning: two solids at random relative positions + orientations
- **Tier 2 — Adversarial mutation engine (the moat):**
  - Objective function: maximize `|float_result - interval_result|` — actively hunt for the worst possible condition numbers
  - Mutation strategies: perturb vertex coordinates by ±1 ULP, rotate planes to near-parallel, position solids at near-tangent contact
  - Genetic algorithm: evolve a population of test cases; fitness = divergence magnitude
  - Seeded adversarial cases: start from known-clean cases and systematically degrade them toward the degenerate boundary
  - Coverage metric: track the minimum condition number encountered across the corpus — this number should decrease monotonically as the adversarial fuzzer finds harder cases
- Per case, run all applicable combinators from P4.1 and P4.2.
- Track per case:
  - Identity check results (all must pass)
  - Divergence report from Layer 3
  - Dual-path agreement from Layer 2
  - Invariant validation from Layer 1
  - Condition number (for adversarial cases)
- **Equivalence for curved solids:** Two results are "approximately equal" if they have identical topological structure (same Euler characteristic, same shell/face/edge/vertex counts) AND volume agreement within certified bounds. Surface-level comparison uses sampled point deviation: for N random `(u,v)` samples on each corresponding face pair, `‖S₁(u,v) − S₂(u,v)‖ < surface_tolerance`. This metric is invariant to surface re-parameterization.
- Corpus management:
  - Cases that expose bugs → pinned as permanent regression tests
  - Corpus grows monotonically — 100+ new cases per CI run
  - Total corpus target: 10,000+ random + 1,000+ adversarial cases within first month
- Statistical tracking:
  - Identity failure rate (target: 0.0%)
  - Divergence rate trend (target: monotonically decreasing)
  - Volume error distribution
  - Minimum condition number in corpus (target: monotonically decreasing)

**Acceptance:**
- PV-48: 1,000 random planar Boolean cases → 0% identity failure rate
- PV-49: 100 random curved Boolean cases → 0% identity failure rate (or all failures are NearBoundary with logged policy decisions)
- PV-55: 100 adversarial cases with condition number < 1e-10 → all correctly escalated to interval/rational, zero topology errors
- **Performance gate:** 100 random cases/minute for planar, 10 cases/minute for curved; adversarial mutation: 10 generations/minute

---

### Milestone P4.5 — Per-Step Euler Checkpoint in Chains 🟡
**What:** For long operation chains, verify Euler invariants after every single step — not just at the end.

**Implementation:**
- Extend `ReplayLog` to support per-step validation hooks.
- After each operation in a chain, run Layer 1 invariants (fast, < 5% overhead).
- After every Nth operation (configurable, default N=10), run Layer 2 dual-path cross-check (slower, more thorough).
- On first invariant failure, stop the chain and produce:
  - The step number where the failure occurred.
  - The minimal region around the failure (via P3.2).
  - The causal chain of the failing entity (via P3.3).
  - A serialized test case for the single failing step.

**Acceptance:**
- PV-50: 100-step chain with injected invariant violation at step 42 → detected at step 42, not step 100
- PV-51: 500-step clean chain → all checkpoints pass, overhead < 15%

---

### Milestone P4.6 — MB-S: MetaBoss Self-Consistency Torture Suite 🔴
**What:** The ultimate self-consistency tests.

**Test Series:**
```
MB-S1: Cancellation law:  (A ∪ B) − B ≈ A  for 500 random solid pairs — 0% failure
MB-S2: Commutativity:  A ∪ B == B ∪ A  and  A ∩ B == B ∩ A  for 500 pairs — 0% failure
MB-S3: Associativity:  (A ∪ B) ∪ C == A ∪ (B ∪ C)  for 200 triples — 0% failure
MB-S4: Idempotence:  A ∪ A == A  for 100 solids — exact topological match
MB-S5: Inclusion-exclusion: Vol(A∪B) + Vol(A∩B) == Vol(A) + Vol(B) for 500 pairs —
        volume agreement within certified bounds
MB-S6: Transform invariance: 100 random rigid transforms — identical topology every time
MB-S7: Self-subtraction: A − A produces empty solid for 100 random solids
MB-S8: De Morgan: (A ∪ B)' == A' ∩ B' for 200 pairs (complement via universal bounding box)
```

---

### Milestone P4.7 — MB-C: MetaBoss Chain Torture Suite 🔴
**What:** Long operation chain stress tests with per-step proof.

**Test Series:**
```
MB-C1: 100-step union chain — start with cube, union 99 random cubes — per-step Euler valid
MB-C2: 100-step alternating union/subtract — no accumulated topology corruption
MB-C3: 50-step union chain, then subtract all 50 operands one by one — result ≈ original cube
MB-C4: 200-step chain with 5% near-degenerate operations — all resolved by precision pipeline
MB-C5: 500-step chain — per-step Euler + every 10th step dual-path cross-check
MB-C6: Determinism: 500-step chain replayed 10× — identical topology hash at every step
MB-C7: Scale-sweep chain: 20 operations at scales from 1e-3 to 1e9, interleaved — no
        precision accumulation breaks topology
MB-C8: 100-step chain where each step adds a feature that references previous features
        via selectors — no selector resolution failures
```

**Acceptance:**
- All MB-C series green
- Every chain step passes at least Layer 1 invariants
- Every 10th step passes Layer 2 dual-path
- Total chain time < 30 seconds for 500-step chains (planar)

---

### Milestone P4.8 — MB-F: MetaBoss Fillet & Curve Torture Suite 🧪🔴
**What:** The ultimate curved geometry stress tests. These are research-grade — some may require iteration on approach.

**Test Series:**
```
MB-F1: Constant-radius fillet on all 12 edges of a cube simultaneously — no corner-patch failures
MB-F2: Variable-radius fillet (R=5mm to R=0.5mm) along a single edge — smooth transition,
        no self-intersection
MB-F3: Fillet radius exceeding adjacent face width — cascade produces clean result or
        structured PolicyRequired error with: consumed face ID, candidate reconstruction
        strategies, manifoldness prediction for each strategy. The kernel MUST NOT attempt
        to guess topological intent — it kicks the decision to the user/agent layer.
MB-F4: Micro-fillet (R=1e-9) on a macro-solid (extent=1e12) — requires local coordinate
        space transform (P2.4) before fillet computation. Scale-invariant precision, correct
        topology across 21 orders of magnitude.
MB-F5: Multi-edge fillet junction — 5 fillets meeting at a single vertex — corner patch valid,
        G1 continuity at all seams
MB-F6: Fillet on near-tangent edges (dihedral angle < 1° ) — policy correctly handles
        near-tangency, result is manifold or PolicyRequired returned
MB-F7: 50-fillet chain on a complex solid — per-step Layer 1 validation, no accumulated
        topology corruption
MB-F8: Fillet followed by Boolean — fillet surfaces participate correctly in Boolean
        classification, dual-path agrees
```

**Acceptance:**
- MB-F1 through MB-F3: Must pass (these are hard but well-understood). MB-F3 specifically must return a structured `PolicyRequired` with reconstruction options — never a generic error.
- MB-F4 through MB-F8: Must either pass or return `PolicyRequired` / structured error — never crash, never produce non-manifold output
- MB-F4 specifically: must use local coordinate space transform; direct computation at 1e12 coordinates is forbidden by P2.4

---

# Part 5: Reference Tables

## Proof Layer Summary

| Layer | Purpose | Key Mechanism | Catches | Misses |
|-------|---------|---------------|---------|--------|
| 1 | Topological Invariants | Euler formula, twin reciprocity, manifoldness | Structural corruption | Geometrically wrong but topologically valid |
| 2 | Dual-Path Verification | Independent classifiers (ray + winding number) | Wrong face selection, misclassification | Both classifiers share a numerical bug |
| 3 | Redundant Numerical Modes | Float vs. interval vs. rational comparison | Precision-dependent decisions | Exactly coincident cases (algebraic, not numerical) |
| 4 | Causal Replay & Witnesses | Decision chain reconstruction + counterfactuals | Non-determinism, unreproducible failures | Consistent but wrong logic |
| 5 | Self-Consistency Fuzzing | Boolean identity laws + volume conservation | Composition errors, accumulation | Individual operation correctness |

## Milestone Count by Phase

| Phase | Milestones | PV Suites | MB Series | Risk | Est. LOC |
|-------|-----------|-----------|-----------|------|----------|
| P0 — Topological Fortress | P0.1–P0.6 (6) | PV-01..PV-14 | MB-T (7 tests) | 🟡 | ~3,000 |
| P1 — Dual-Path Verification | P1.1–P1.5 (5) | PV-15..PV-22, PV-53 | MB-D (6 tests) | 🔴 | ~5,000 |
| P2 — Redundant Numerical | P2.1–P2.5 (5) | PV-23..PV-32, PV-52 | MB-N (6 tests) | 🔴 | ~6,000 |
| P3 — Causal Replay | P3.1–P3.5 (5) | PV-33..PV-40, PV-54 | MB-R (7 tests) | 🔴 | ~5,000 |
| P4 — Self-Consistency Fuzzing | P4.1–P4.8 (8) | PV-41..PV-51, PV-55 | MB-S/C/F (24 tests) | 🧪🔴 | ~8,000 |
| **Total** | **29 milestones** | **61 PV suites** | **50 MB tests** | | **~27,000** |

## Performance Gates

| Gate | Milestone | Target |
|------|-----------|--------|
| Non-geometric validation overhead | P0.5 | < 5% of operation time |
| Winding-number query, planar (5k faces) | P1.2 | < 10ms (BVH-accelerated) |
| Winding-number query, curved (5k faces) | P1.2 | < 50ms (polynomial early-out) |
| Interval `orient3d` | P2.1 | < 100ns |
| Minimal region extraction (10k faces) | P3.2 | < 100ms |
| Fuzz corpus throughput (planar) | P4.4 | 100 random cases/minute |
| Adversarial mutation throughput | P4.4 | 10 generations/minute |
| 500-step chain with per-step proof | P4.7 | < 30 seconds total |

## Proof Validation Suites (PV-01 through PV-55)

| PV | Name | Phase | Tests |
|----|------|-------|-------|
| PV-01 | Zero-area face detection | P0.1 | Injected degenerate → caught |
| PV-02 | Zero-length edge detection | P0.1 | Injected degenerate → caught |
| PV-03 | Inverted shell detection | P0.1 | Negative signed volume → caught |
| PV-04 | Degenerate loop detection | P0.1 | 2-vertex loop → rejected |
| PV-05 | Torus Euler validation | P0.2 | Genus-1 passes generalized Euler |
| PV-06 | Through-hole Euler | P0.2 | Genus-1 hole passes |
| PV-07 | Multi-shell Euler | P0.2 | Internal void passes |
| PV-08 | Broken genus detection | P0.2 | Removed edge → fails |
| PV-09 | Orientation post-Boolean | P0.3 | 1,000 ops → all outward normals |
| PV-10 | Import orientation healing | P0.3 | Random orientations → canonical |
| PV-11 | T-junction detection | P0.4 | Non-manifold → rejected |
| PV-12 | Post-Boolean manifold gate | P0.4 | Non-manifold result → error before commit |
| PV-13 | Auto-validation at checkpoint | P0.5 | Broken result → auto-detected |
| PV-14 | Validation performance | P0.5 | 50k entities < 100ms |
| PV-15 | Cross-check no false positives | P1.1 | 1,000 random correct Booleans |
| PV-16 | Cross-check catches wrong face | P1.1 | Wrong face kept → detected |
| PV-17 | Winding-number vs. ray-cast | P1.2 | 10,000 points, 100 solids agree |
| PV-18 | Winding-number degeneracy | P1.2 | Point on face/edge/vertex handled |
| PV-19 | Boundary disagreement protocol | P1.3 | Near-boundary → structured log |
| PV-20 | Fundamental disagreement | P1.3 | Wrong classification → abort |
| PV-21 | Curved dual-path | P1.4 | Cylinder Boolean → zero false positives |
| PV-22 | Near-tangent dual-path | P1.4 | Tangent cylinder → boundary identified |
| PV-23 | Interval orient3d correctness | P2.1 | 100,000 random inputs match exact |
| PV-24 | Interval inconclusive detection | P2.1 | Near-degenerate → inconclusive |
| PV-25 | Fast-path dominance | P2.2 | Standard cases resolve at Float64 |
| PV-26 | Interval escalation | P2.2 | Near-degenerate → escalates + logs |
| PV-27 | Rational fallback | P2.2 | Exactly degenerate → rational resolves |
| PV-28 | Clean divergence report | P2.3 | Clean op → rate = 0.0 |
| PV-29 | Degenerate divergence report | P2.3 | Degenerate → correct classification |
| PV-30 | Report serializability | P2.3 | Agent-parseable output |
| PV-31 | Scale-invariant precision | P2.4 | 5 scales → identical topology |
| PV-32 | Mixed-scale precision | P2.4 | 1e12 + 1e-9 → correct escalation |
| PV-33 | Checkpoint diffing correctness | P3.1 | 10-step chain → diffs correct |
| PV-34 | Checkpoint diffing determinism | P3.1 | Identical re-run → empty diff |
| PV-35 | Region extraction validity | P3.2 | Extracted sub-mesh is valid |
| PV-36 | Delta-debug convergence | P3.2 | 100-step chain → finds step 73 |
| PV-37 | Causal chain completeness | P3.3 | Boolean face → full ancestry |
| PV-38 | Causal chain conciseness | P3.3 | 50-step chain → < 10 relevant steps |
| PV-39 | Counterfactual valid alternative | P3.4 | Override NearBoundary → valid topology |
| PV-40 | Counterfactual proves necessity | P3.4 | Override correct decision → broken result |
| PV-41 | Identity combinators correct | P4.1 | Known cubes → all pass |
| PV-42 | Identity combinator error messages | P4.1 | Injected failure → meaningful errors |
| PV-43 | Rotation invariance | P4.2 | 100 rotations → identical topology |
| PV-44 | Scale invariance | P4.2 | 10 scales → identical topology |
| PV-45 | Volume oracle precision | P4.3 | Unit cube = 1.0 ± 1e-15 |
| PV-46 | Curved volume accuracy | P4.3 | Sphere within 1% of analytical |
| PV-47 | Volume inclusion-exclusion | P4.3 | Certified bounds satisfy identity |
| PV-48 | Planar fuzz corpus | P4.4 | 1,000 cases → 0% failure |
| PV-49 | Curved fuzz corpus | P4.4 | 100 cases → 0% structural failure |
| PV-50 | Per-step chain detection | P4.5 | Injected failure at step 42 → caught at 42 |
| PV-51 | Chain checkpoint overhead | P4.5 | 500-step chain overhead < 15% |
| PV-52 | Local coord transform precision | P2.4 | Round-trip preserves precision across 21 orders of magnitude |
| PV-53 | Fuel-bounded quadrature | P1.4 | Pathological surfaces return `FuelExhausted`, never hang |
| PV-54 | ChainSummary token budget | P3.3 | 50-step entity summary < 200 tokens |
| PV-55 | Adversarial escalation | P4.4 | Condition number < 1e-10 → correct precision escalation |
| PV-56 | Pcurve consistency (future) | P0.1 | Edge 3D curve matches surface evaluation at parameter curve |
| PV-57 | Trim curve closure (future) | P0.1 | Trim curves form closed loops in (u,v) space |
| PV-58 | Trim curve non-intersection (future) | P0.1 | Trim curves do not self-intersect in parameter space |
| PV-59 | Surface Jacobian non-degeneracy (future) | P0.1 | `∂S/∂u × ∂S/∂v ≠ 0` within parameter domain |
| PV-60 | Curved volume oracle (future) | P4.3 | Adaptive quadrature volume matches analytical for known curved solids |
| PV-61 | Curved solid equivalence (future) | P4.4 | Sampled point deviation metric validates curved identity combinators |

## MetaBoss Test Series (MB-T, MB-D, MB-N, MB-R, MB-S, MB-C, MB-F)

| Series | Count | Phase | Class |
|--------|-------|-------|-------|
| MB-T (Topological) | 7 | P0.6 | Extreme topological stress |
| MB-D (Dual-Path) | 6 | P1.5 | Classification independence |
| MB-N (Numerical) | 6 | P2.5 | Precision boundary stress |
| MB-R (Replay) | 7 | P3.5 | Causal replay under scale |
| MB-S (Self-Consistency) | 8 | P4.6 | Boolean algebraic identities |
| MB-C (Chains) | 8 | P4.7 | Long operation chains with proof |
| MB-F (Fillets/Curves) | 8 | P4.8 | Curved geometry extremes |
| **Total** | **50** | | |

---

# Part 6: Proof Scoreboard (CI Dashboard)

The CI pipeline generates a proof health report alongside the existing robustness scoreboard. Regressions in these metrics block merges.

| Metric | Target | Description |
|--------|--------|-------------|
| Layer 1 Pass Rate | 100% | Topological invariants must never fail on valid operations |
| Dual-Path Agreement Rate | > 99.9% | Classifier agreement (remaining 0.1% = logged NearBoundary) |
| Float Divergence Rate | < 0.1% | Percentage of decisions where float disagrees with exact |
| Topology-Affecting Divergences | 0 | Float decisions that would change topology if exact — must be zero |
| Replay Bit-Exactness | 100% | Replayed operations produce identical hashes |
| Identity Failure Rate | 0% | Boolean identities must never fail |
| Volume Conservation Error | < 1e-10 | Inclusion-exclusion volume identity error |
| Fuzz Corpus Size | > 10,000 | Total regression corpus (monotonically growing) |
| MB Series Green Rate | 100% | All MetaBoss tests must pass (or return structured PolicyRequired) |
| Causal Chain Coverage | > 95% | Percentage of result entities with complete causal chains |

---

# Part 7: Phase Completion Checklist

A proof phase is **complete** only when ALL of the following are true:

- [ ] All milestone unit tests pass
- [ ] All PV suites for this phase pass
- [ ] All applicable MB series are green (or return structured errors for research-grade tests)
- [ ] Performance gates met (or consciously deferred with tracked issue)
- [ ] Proof mechanisms do not degrade kernel operation performance beyond stated thresholds
- [ ] Proof output is machine-readable and consumable by AI agents within token budget
- [ ] Integration with `OperationResult<T>` envelope verified (proof results flow through standard channels)
- [ ] Documentation updated: proof mechanism described in crate-level docs
- [ ] Corpus fuzzer runs without crashes or unlogged proof failures

---

# Part 8: Integration with Development Blueprint

The proof system cross-references the Development Blueprint at specific points:

| Blueprint Phase | Proof Integration |
|----------------|-------------------|
| Phase 0 (Foundation) | Proof Phase P2 extends exact predicates with interval arithmetic |
| Phase 0.5 (State) | Proof Phase P3 extends topology hashing with checkpoint diffing |
| Phase 1 (Topology) | Proof Phase P0 extends `validate_topology()` with geometric invariants |
| Phase 1B (AI Affordances) | Proof Phase P3 extends `OperationResult<T>` with proof results |
| Phase 2 (Booleans) | Proof Phase P1 adds post-Boolean dual-path cross-check |
| Phase 2B (Observability) | Proof Phase P3 extends `DecisionLog` with causal chains |
| Phase 3 (Curved) | Proof Phase P1.4 extends dual-path to curved geometry |
| Phase 4 (Fillets) | MB-F series validates fillet correctness across extreme conditions |
| Phase 5+ | All proof layers apply to every subsequent operation automatically |

**Key constraint:** Proof infrastructure must be built incrementally. Phase P0 can be started immediately (extends existing code). Phases P1–P2 should be built alongside Blueprint Phase 2–3. Phase P3–P4 should be built alongside Blueprint Phase 3–4. The proof system and the kernel it validates grow together.

---

# Part 9: Danger Zones & Reality Checks

These are the mathematical and engineering realities that will kill you if you don't account for them. Each is addressed by specific milestones, but they deserve explicit acknowledgment as **existential risks** to the proof system's integrity.

---

### DZ-1: The f64 Scale Trap ⚠️
**Affects:** P2.4, MB-F4, MB-C7, MB-T7

**The math:** IEEE 754 `f64` has ~15–17 significant decimal digits. At coordinate magnitude 1e12, machine epsilon is ~1e-4. You **physically cannot represent** a 1e-9 feature at 1e12 coordinates — the feature is 100,000× smaller than the smallest representable difference.

**The consequence:** Interval arithmetic bounds instantly widen to encompass zero. The precision pipeline escalates to rational arithmetic for *every single operation*. Performance collapses.

**The fix:** Local coordinate space transforms (implemented in P2.4). All operations at extreme scale must first translate to origin + normalize. This is not an optimization — it is a mathematical prerequisite.

---

### DZ-2: Adaptive Quadrature is Computationally Violent ⚠️
**Affects:** P1.4, P4.3

**The math:** Computing winding numbers (Layer 2) and exact volumes (Layer 5) for curved surfaces requires numerical integration over surface patches. Near boundaries and high-curvature regions, quadrature convergence requires deep subdivision trees.

**The consequence:** The < 10ms planar performance gate is unrealistic for complex NURBS surfaces. Without mitigation, curved dual-path verification becomes a bottleneck that makes proof checking slower than the operation itself.

**The fix:** Three-tier acceleration (implemented in P1.2 and P1.4):
1. **BVH-cached contribution bounds:** Pre-compute per-face solid-angle upper bounds. Skip distant/negligible faces.
2. **Polynomial approximation early-out:** Degree-4 polynomial bound on solid-angle contribution resolves most faces without full integration.
3. **Fuel-bounded recursion (Doctrine P5):** Quadrature depth is a fuel parameter, not unbounded recursion. Exhausted fuel → `FuelExhausted`, never infinite hang.

---

### DZ-3: The Token Budget vs. Causal Cascade ⚠️
**Affects:** P3.3

**The problem:** A single face split in a Boolean can alter half-edge pointers across an entire shell. Walking the Lineage Merkle DAG produces causal chains with dozens of raw vertex-edge-loop updates that are irrelevant to *why* the entity exists.

**The consequence:** Dumping raw `CausalStep` arrays into an LLM context wastes tokens on structural bookkeeping. The agent needs to know "face F4 was split by plane P2" — not the 15 pointer updates that implement that split.

**The fix:** Semantic summarization layer (implemented in P3.3). Every `CausalStep` generates a human-readable `semantic_summary`. The `ChainSummary` provides a < 200 token narrative of the entity's life. Raw details available on drill-down.

---

### DZ-4: The Fillet Cascade Abyss ⚠️
**Affects:** MB-F3, Blueprint Phase 4

**The problem:** When a fillet radius exceeds an adjacent face width, the fillet doesn't just fail — it **consumes the face entirely**. The kernel must dynamically reconstruct topology of faces that were never explicitly selected. This cascade can propagate through multiple faces.

**The consequence:** Without an explicit abort-and-report mechanism, the kernel will attempt to guess the user's topological intent. These guesses are wrong 50%+ of the time and produce non-manifold output.

**The fix:** `OperationResult<T>` must support a robust `PolicyRequired` variant for fillet cascades that includes: consumed face IDs, candidate reconstruction strategies with predicted manifoldness, and estimated topology impact. The kernel **never guesses** — it reports options and waits.

---

### DZ-5: The FMA Cross-Architecture Ghost ⚠️
**Affects:** MB-R6, MB-R7, Blueprint Doctrine D8

**The problem:** Fused Multiply-Add (FMA) computes `(a * b) + c` with a single rounding error instead of two. This changes the 15th decimal place. A `NearBoundary` decision that resolves to `Pos` on x86 may resolve to `Neg` on ARM (or even on the same machine with different compiler flags). The topology butterflies from there.

**The consequence:** Cross-session, cross-architecture replay silently produces different topologies. The user sees "replay succeeded" but the result is wrong.

**The fix:** Enforce `-C target-feature=-fma` for all exact predicate and interval arithmetic modules. `ReplayLog` records the compilation target triple. Mismatched-triple replay returns `ReplayError::ArchitectureMismatch`. MB-R7 specifically tests for FMA-sensitive decisions and escalates them.

---

### DZ-6: Wall-Clock Time is Non-Deterministic ⚠️
**Affects:** All performance gates, Doctrine P5

**The problem:** If your CI server is under load, a test that normally takes 50ms takes 150ms and "fails." Worse: if adaptive quadrature has a time-based bail-out, a loaded CPU produces a *different winding number* than a quiet one.

**The consequence:** Performance gates become flaky. Time-bounded algorithms become non-deterministic.

**The fix:** Doctrine P5 (Deterministic Fuel). All iterative algorithms consume **fuel** (iteration counter), not time. CI performance gates use wall-clock for reporting only — the proof system itself never branches on `Instant::now()`. The fuel budget is a first-class parameter in `ValidationConfig`.

---

### DZ-7: The ABA Arena Problem ⚠️
**Affects:** All layers, P0 invariants specifically

**The problem:** Arena allocators reuse slots. If face `FaceId(12)` is deleted and a new face is created, the arena may reuse slot 12. Any stale reference to the old `FaceId(12)` now silently reads the new face's data. Layer 1 invariants won't catch this — the graph looks structurally valid.

**The consequence:** Lineage corruption. Causal chains connect to the wrong entities. Dual-path verification uses the wrong geometry. Everything looks correct but is semantically wrong.

**The fix:** Doctrine P6 (Generational Handle Integrity). All entity IDs are generational indices: 32-bit slot + 32-bit generation. Generation mismatch → immediate panic (not silent corruption). Forge already uses `thunderdome` for this — the doctrine ensures no alternative indexing is ever introduced.

---

### DZ-8: Random ≠ Adversarial ⚠️
**Affects:** P4.4, all MB series

**The problem:** Random polyhedra generators produce geometrically "nice" configurations. They rarely generate planes at 1e-15 radian angles, or vertices offset from edges by a single ULP. The hardest bugs live at the boundary of representable precision.

**The consequence:** A green corpus of 10,000 random cases provides false confidence. The real bugs are in the 0.001% of configuration space that random generation never visits.

**The fix:** Adversarial mutation engine (implemented in P4.4, Tier 2). Fitness function = `|float_result - interval_result|`. The fuzzer actively evolves test cases toward maximum divergence. Minimum condition number in corpus is a tracked CI metric — it must decrease monotonically.

---

# Part 10: Why This Is Sufficient

Commercial geometry kernels (ACIS, Parasolid, Open CASCADE) rely primarily on:
1. Topological validation (incomplete — genus-aware Euler is rare)
2. External oracle comparison (expensive, fragile, not self-contained)
3. Manual regression suites (non-monotonic, coverage unknown)

Forge's five-layer system surpasses this because:

| Property | Commercial | Forge |
|----------|-----------|-------|
| **Independence** | 1–2 verification paths | 5 independent layers |
| **Self-contained** | Requires external oracle | Self-consistent (Layer 5) |
| **Precision-aware** | Float-only decisions | Float → Interval → Rational escalation |
| **Deterministic** | Not guaranteed | Bit-exact replay (D1 + Layer 4) |
| **Observable** | Black box | Every decision carries proof metadata (D9 + Layer 4) |
| **Compositional** | Individual op tests only | Algebraic identity proofs over compositions (Layer 5) |
| **Scale-invariant** | Fixes for specific tolerances | Local coordinate spaces + adaptive precision (Layer 3) |
| **Monotonic** | Test suites may shrink | Corpus grows forever (P2) |
| **Adversarial** | Only tests what developers think of | Guided mutation hunts for worst-case inputs (Layer 5) |
| **Fuel-bounded** | Time-dependent iteration limits | Deterministic fuel ensures identical behavior regardless of hardware (P5) |

**The thesis:** No single proof mechanism provides certainty. Five independent mechanisms, each catching a distinct defect class, with monotonically growing adversarial coverage, deterministic fuel-bounded iteration, generational handle safety, and machine-readable output for AI-driven improvement — that is certainty.

---

# Part 11: NURBS Readiness Protocol

This section consolidates the design constraints that ensure the proof system transitions to curved (NURBS/analytic) geometry without refactoring the verification infrastructure. These constraints cost zero code during the planar phase — they govern API signatures and abstraction boundaries only.

### NR-1: Geometry-Dispatched Invariant Checks
**Affects:** P0.1, P4.3

All geometric invariant computations (face area, edge length, shell volume, signed distance) must be dispatched through the `SurfaceEvaluator` trait, never inlined as planar-specific formulas. `Plane` is the first and only implementor during the planar phase. When curved surfaces arrive, new implementors slot in without modifying any invariant checker code.

**Concrete rule:** If a proof function computes a geometric property, its signature must accept `&dyn SurfaceEvaluator` (or the face-level equivalent), not `&[[f64; 3]]` vertices.

### NR-2: Reserved Parametric Invariant Slots
**Affects:** P0.1 (Layer 1)

NURBS introduces an invariant class with no planar equivalent: parametric domain integrity. PV-56 through PV-59 are reserved for these checks. During the planar phase, these are trivially satisfied and not executed. When parameterized surfaces land, the invariant framework already expects them.

| Invariant | Check |
|-----------|-------|
| PV-56: Pcurve consistency | `max_t ‖C(t) − S(p(t))‖ < ε` |
| PV-57: Trim curve closure | Closed loops in (u,v) |
| PV-58: Trim non-intersection | No self-crossing in parameter space |
| PV-59: Jacobian non-degeneracy | `∂S/∂u × ∂S/∂v ≠ 0` over domain |

### NR-3: Volume Oracle Trait Dispatch
**Affects:** P4.3

The `VolumeOracle` must accept geometry through the evaluator trait, not raw vertex arrays. Planar path: divergence-theorem determinants. Curved path: fuel-bounded adaptive quadrature of `∫∫ (1/3) S · n dA`. Same API, different implementation.

### NR-4: Curved Solid Equivalence Metric
**Affects:** P4.4 (Layer 5)

Boolean identity combinators compare results for "approximate equality." For planar solids, this is topological isomorphism + volume agreement. For curved solids, add sampled surface deviation: N random `(u,v)` samples per face pair, `‖S₁(u,v) − S₂(u,v)‖ < surface_tolerance`. This metric is re-parameterization invariant.

### NR-5: Anti-Pattern — No Per-Entity Tolerance Scalars
**Permanent rule**

Do NOT store mutable tolerance radii on `Vertex` or `Edge` structs (the Open CASCADE model). Per-entity tolerances cause tolerance creep — widening propagates through adjacent entities until tolerance spheres consume geometric features. Forge's precision escalation pipeline (Float → Interval → Rational) is the correct mechanism: escalate precision at query time rather than storing widened tolerance at creation time. If per-entity tolerance metadata is needed for NURBS interchange (e.g., STEP import), store it as **read-only provenance** in the `AttributeStore`, never as a mutable field that downstream code can widen.
