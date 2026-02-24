# Boolean Pipeline Upgrade — Engineering Specification

**Forge Geometry Kernel | Aerospace-Grade Correctness Target**
**Date:** 2026-02-24 | **Status:** PLANNING

---

## 0. Purpose & Scope

This document is a complete, ordered engineering plan for integrating the
newly completed Euler operators and topology hierarchy into the boolean
pipeline. It serves as a single source of truth — every task has an
acceptance criterion, a file target, and a test command.

The boolean pipeline currently works for clean, well-separated geometry
but has structural weaknesses that will produce silent errors or
non-manifold output under aerospace-grade stress (thin features, coplanar
faces, near-tangent intersections, chained operations). This spec closes
those gaps.

### What Does NOT Change

The following algorithms are correct and must be preserved verbatim:

- BVH-accelerated AABB overlap detection (`split/eval.rs`)
- Exact 3-plane vertex intersection (`split/cut.rs`)
- Chord-gate polygon test (`split/gate.rs`)
- Sign-walk cut-point location (`split/cut.rs`)
- `VertexMatchKey` provenance dedup system (`eval.rs`, `split/schema.rs`)
- `SharedVertexRegistry` canonical position map
- Cross-solid reconciliation after independent splits (`split/reconcile.rs`)
- 4-layer vertex resolution in face copy (`assemble/copy.rs`)
- BFS coplanar group discovery (`postprocess/polygon_extract.rs`)
- EMBER coplanar override injection (`engines/planar.rs`)

### What Changes

Every site that does **direct arena insertion** to bypass the Euler
operator invariant is a defect. Every validation gap that lets
non-manifold topology reach a test assertion is a correctness hole.
Every test that passes but doesn't actually verify the invariant it
claims to verify is a regression risk.

---

## 1. Architecture Map

```
Boolean Orchestrator (execute_boolean)
│
├─ [Phase 1] SPLIT       split/eval.rs → split_all_faces()
│   ├─ BVH overlap pairs
│   ├─ Plane intersection (exact: intersect_three_planes_exact)
│   ├─ Chord gate (split/gate.rs)
│   ├─ Sign walk (split/cut.rs) — uses SplitEdge ✅, MakeEdgeFace ✅
│   └─ Reconcile shared vertices (split/reconcile.rs)
│
├─ [Phase 2] CLASSIFY    classify/
│   └─ Ray-cast point-in-solid (classify_point_in_solid from forge-topo)
│
├─ [Phase 3] COPLANAR    assemble/disjoint/ + EMBER overrides
│   └─ EmberCoplanarResolver — reassigns Inside/Outside for shared planes
│
├─ [Phase 4] SELECT      assemble/select.rs  (pure logic, no changes)
│
├─ [Phase 5] ASSEMBLE    assemble/merge/eval.rs
│   ├─ copy_faces()       ← PRIMARY DEFECT SITE (direct insertion)
│   └─ stitch_twins()     ← SECONDARY DEFECT SITE (raw radial_next writes)
│
└─ [Phase 6] POSTPROCESS postprocess/
    ├─ extract_coplanar_regions()  ← TERTIARY DEFECT SITE (direct insertion)
    └─ splice_inner_holes()        ← uses bridge_edge (non-Euler workaround)
```

---

## 2. Defect Registry

Each defect has a severity, a root cause, and a targeted fix.

| ID  | Severity | Phase       | Root Cause                                                                                                                       | Fix Target                                                                                                    |
| --- | -------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| D1  | Critical | Assemble    | `copy_single_face` does raw `insert_half_edge`/`insert_face` bypassing lineage, loop invariants, and shell→region→body hierarchy | Replace with Euler-operator sequence                                                                          |
| D2  | Critical | Assemble    | `stitch_twins` writes `set_radial_next()` directly — skips `SewEdge` invariants, no Euler delta, breaks D6                       | Replace inner write with `SewEdge` operator call                                                              |
| D3  | Critical | Postprocess | `rebuild_face_from_perimeter` (polygon_extract) uses `insert_radial_pair` + direct wiring — same bypass                          | Replace with `MakeVertexFace` + N×`MakeEdgeFace` sequence                                                     |
| D4  | High     | Postprocess | `splice_inner_holes` uses `bridge_edge` which is a non-Euler workaround (no correct EulerDelta)                                  | Replace with `MakeEdgeKillLoop` (MEKL)                                                                        |
| D5  | High     | Split       | Deferred retry queue in `split_solid` abandons grazing cuts                                                                      | Use `MakeEdgeVertex` (MEV) to insert a vertex at the grazing point, converting the near-miss into a valid cut |
| D6  | High     | All         | No post-operation `ValidationLevel::Full` check in any pipeline phase                                                            | Add validation gates at phase boundaries                                                                      |
| D7  | Medium   | Assemble    | `copy.rs` creates `Region`/`Lump`/`Body` manually — does not propagate shell `kind` correctly for subtraction voids              | Audit hierarchy propagation for void shells                                                                   |
| D8  | Medium   | Classify    | `classify_point_in_solid` has no multi-direction retry if first ray is degenerate                                                | Wire existing retry logic from forge-topo (Tier 2.7 from your list)                                           |
| D9  | Low      | All         | Tests assert `assert!(result.is_ok())` without inspecting topology invariants                                                    | Add `ValidationLevel::Full` + Euler formula checks                                                            |

---

## 3. Sprint Plan & Acceptance Criteria

Tasks are ordered so each tier/sprint can be completed and verified before the next begins. Never work out of order — a lower-tier fix is always a prerequisite for a higher-tier correctness guarantee. This implementation plan is organized into four sequential sprints. Every task requires the `BooleanResult` to pass full topology validation (`ValidationLevel::Full`) and the Euler characteristic formula (`V - E + F = 2`) for all closed shells.

This implementation plan is organized into four sequential sprints. Every task requires the `BooleanResult` to pass full topology validation (`ValidationLevel::Full`) and the Euler characteristic formula (`V - E + F = 2`) for all closed shells.

---

### Sprint 1: Observability & Diagnostics (The Safety Net)

**Goal:** Prove the pipeline's current state, anchor regressions, and force the build to fail whenever topology invariants are violated mid-operation.

- **Task 1.1: Baseline & Golden Hash Anchoring**
  - Execute the full test suite and save the baseline result.
  - Implement `compute_arena_topology_hash()` ensuring strict determinism (no floating-point values, and sorted `OrderingKey` usage for any maps/sets).
  - Implement `cube_union_cube_topology_hash_is_stable()` which captures `compute_arena_topology_hash()` and asserts its stability.
- **Task 1.2: Phase-Boundary Validation Gates**
  - Add `ValidationLevel::Intermediate` to bypass strict manifold checks for temporary states during operations.
  - Inject `validate_topology(..., Intermediate)` immediately after the Phase 1 (Split), Phase 5 (Assemble), and Phase 6 (Postprocess) boundaries. Behind `cfg!(debug_assertions)`.
- **Task 1.3: Euler Formula Verification**
  - Create `assert_euler_formula_per_shell` in `test_helpers.rs` ensuring the correct generalized formula `V - E + F = 2(1 - G)`.
  - Inject it into all existing integration tests.
- **Task 1.4: Tag Defect Sites**
  - Add `// DEFECT(Dx)` comments to the 9 sites identified in the Defect Registry.

**Sprint 1 Acceptance Criteria:**

- Test suite passes.
- Any manual modification to `radial_next` in the codebase immediately triggers a validation failure at the subsequent phase-gate.
- `forge-trace-cli issues` reports 0 non-deterministic decisions on the baseline suite.

---

### Sprint 2: Structural Soundness (The Core Refactor)

**Goal:** Eliminate all direct arena insertions (Nuke-and-Pave) in the Assembly and Postprocessing phases. Replace them with identity-preserving Euler operator sequences.

- **Task 2.1: In-Place Target Mutation & Topological Tool Sewing (Fixes D1 & D2)**
  - Stop deep-copying the target into a new draft. Mutate the target in-place (destroying rejected faces).
  - Instantiate surviving Tool faces as independent islands using `MakeVertexFace` and `MakeEdgeFace`.
  - **Trap Avoidance (Vertex Identity):** When instantiating Tool faces along the seam, strictly retrieve identical `VertexId`s from the `SharedVertexRegistry` (do not mint new vertices), ensuring `SewEdge` operates on shared bounding vertices.
  - **Trap Avoidance (Orientation Mismatch):** For subtraction booleans, explicitly flip the loop windings of Tool faces before/during instantiation to avoid `SewEdge` encountering parallel half-edges.
  - Use `SewEdge` along the exact boundary seam (driven by `VertexWelder`) to natively zip the target and tool shells together.
- **Task 2.2: Coplanar Merge via Boundary Reduction (Fixes D3, Mitigates G8 & G9)**
  - In `polygon_extract.rs`, eliminate the nuke-and-pave strategy.
  - Traverse the coplanar group and incrementally apply `KillEdgeFace` (KEF) and `KillVertexEdge` (KVE) to dissolve internal boundaries and merge the region intact.
  - **Trap Avoidance (Lineage Loss, G8):** Apply `Lineage::merge` explicitly across all faces collapsed in the group using the appropriate `OpSignature`.
  - **Trap Avoidance (Stale Pointers, G9):** Set internal vertex `outgoing` pointers to a sentinel (`HalfEdgeId::MAX`) before the KVE pass, and assert that no live vertex holds a stale outgoing ID to prevent Validation failures.
- **Task 2.3: Hole Splicing with MEKL (Fixes D4)**
  - Replace `bridge_edge` in `hole_splice.rs` with `MakeEdgeKillLoop` (MEKL) to mathematically absorb holes into standard outer loops.
- **Task 2.4: Clean Up Non-Manifold Antennae**
  - Implement a final post-process sweep mapping `KillEdgeVertex` (KEV) to dissolve zero-area wire edges ("antennae") leftover from intermediate Euler operations.
- **Task 2.5: Multi-Lump Disconnected Shell Segregation (Mitigates G4)**
  - **Trap Avoidance (G4):** Because target mutations occur in-place, a cut may physically divide one shell into two disconnected shells. Perform a BFS reachability pass over the half-edge graph at the end of assembly.
  - For any disconnected components within the same `Solid`, assign them to new `Lump` and `Region` parents to assert strict hierarchy invariants before `ValidationLevel::Full`.

**Sprint 2 Acceptance Criteria:**

- The command `grep -rn "insert_face" crates/forge-kernel/src/operations/boolean/` returns 0 results outside of testing primitives.
- Existing tests compile and pass with `ValidationLevel::Full` enabled.
- `StitchReport::is_fully_paired()` returns true.

---

### Sprint 3: Advanced Routing & Precision Recovery

**Goal:** Route non-solid geometry mathematically correctly and implement precision rescue for near-miss operations.

- **Task 3.1: MEV Grazing-Cut Retry (Fixes D5)**
  - In `split_solid`, when a cut-attempt resolves exactly one boundary vertex, explicitly embed it using `MakeEdgeVertex` (MEV) to anchor the tangent collision and re-evaluate the split.
- **Task 3.2: Explicit Non-Solid Routing (Fixes T4.1 & T4.2)**
  - Pre-screen inputs via `detect_body_kind`.
  - Solid × Solid executes standard pipeline.
  - Solid × Sheet is formally intercepted (returning `NotSupported` pending Imprint/Trim implementation).
  - Relax `validate_solid` to permit valid halfspace approximations (`face_count` override based on shell outer orientation).
- **Task 3.3: Correct Void Shell Hierarchy (Fixes D7 & Mitigates G12)**
  - Ensure subtracted tool faces produce `Solid(Inner)` void boundaries properly nested inside the target's `Region` struct, not independent lumps or default outer shells.
  - **Trap Avoidance (G12):** Ensure `copy.rs` propagates shell kinds precisely using `?`, never defaulting missing shells to `Solid(Outer)`, which would silently invert void orientations.
- **Task 3.4: Winding Number Fast-Path Fallback (Fixes T4.3)**
  - Detect ray-parity failures (`PrecisionEscalation` flag). Only when it fires, fallback to O(N²) Generalized Winding Number classification.

**Sprint 3 Acceptance Criteria:**

- The pipeline correctly rejects a Solid × Sheet boolean with an explicit error instead of a panic/topological rip.
- Nested voids (a sphere subtracted from inside a cube) yield exactly `1 Body -> 1 Lump -> 1 Region -> 1 Outer Shell + 1 Inner Shell`.

---

### Sprint 4: Adversarial Aerospace Integration

**Goal:** Prove the pipeline against the most mathematically hostile configurations found in real-world aerospace components (e.g., turbine blade cooling passages, sheet metal tolerancing, staggered interference).

- **Task 4.1: The Nested Toroidal Epsilon Gap**
  - _Setup:_ Two identical toruses inside each other, but the inner torus is offset by `1e-13` (below standard tolerance but above machine epsilon).
  - _Validation:_ The boolean must cleanly merge them into a single solid without leaving thousands of sliver faces or invalidating the generalized Euler characteristic (genus).
- **Task 4.2: The Fractal Coplanar Storm (MB1 Upgrade)**
  - _Setup:_ A target cube intersected with 10 overlapping tools, exactly coplanar to the target's top face, but staggered in X/Y to create a fractal checkerboard of intersecting edges.
  - _Validation:_ Post-processing Boundary Reduction (Sprint 2.2) cleanly reduces the checkerboard back to a single maximal top face without winding-order corruption.
- **Task 4.3: The Bowtie Singularity Pinch**
  - _Setup:_ Two pyramids meeting exactly at their tips (vertex-on-vertex) + a small offset perturbation causing them to merge into a single non-manifold vertex.
  - _Validation:_ The pipeline correctly preserves the manifold, or cleanly aborts with `TopologyError` prior to creating an invalid umbrella.
- **Task 4.4: 10,000-Step Precision Drift Test**
  - _Setup:_ N=10,000 randomized micro-cuts on a single sphere using `MakeEdgeFace` + `MakeVertexFace`.
  - _Validation:_ Prove `Cumulative EulerDelta == Current State - Base State` exactly, and `ValidationLevel::Full` holds through loop 10,000.
- **Task 4.5: The Cylinder-Tangent Graze**
  - _Setup:_ A flat plate subtracted by a cylinder whose radius _exactly_ touches the bottom plane of the plate (grazing singularity).
  - _Validation:_ Sprint 3.1’s MEV retry correctly embeds the vertex, completing the boolean without abandoning the cut or dropping the face.
- **Task 4.6: Dual-Pipeline Parity Validation**
  - _Setup:_ Run every test in the suite on both the `planar_engine()` and `planar_engine_legacy()`.
  - _Validation:_ Both return valid topology, and the Euler formula holds, establishing an unshakable math proxy over both architectures.

**Sprint 4 Acceptance Criteria:**

- All 6 adversarial models successfully yield valid closed B-Reps.
- Output from adversarial stress tests loads flawlessly into the `forge-trace-cli` viewer for deterministic path inspection.

---

## 4. Architectural Feature Routing (T4.1–T4.4)

These feature routing tasks ensure correct pipeline branching or graceful rejection when dealing with inputs outside the standard watertight Solid paradigm.

### The Core Assumption Violation

Every phase of the current pipeline assumes:

- The input has a **well-defined interior** (needed for ray-parity classify)
- Every edge has **exactly 2 radial halfedges** (needed for stitch, validate, polygon extract)
- The result satisfies **V−E+F=2** per shell (Euler formula for closed manifolds)

Sheets, wires, and non-manifold surfaces violate all three.

### The Operation Matrix

| Target \ Tool | Solid               | Sheet                | Wire                 | Halfspace          |
| ------------- | ------------------- | -------------------- | -------------------- | ------------------ |
| **Solid**     | Current pipeline ✅ | _Trim_ ❌            | _Wire clip_ ❌       | _Clip by plane_ 🔶 |
| **Sheet**     | _Trim_ ❌           | _Sheet intersect_ ❌ | —                    | —                  |
| **Wire**      | _Wire clip_ ❌      | —                    | _Curve intersect_ ❌ | —                  |

🔶 = faked today via large-box approximation in mb6 (works for convex cases only)

### T4.1 — Add Input-Type Detection Before Pipeline Entry

**File:** `execute_boolean` orchestrator

Before running any pipeline phase, detect the shell kinds of both inputs:

```rust
fn detect_body_kind(topo: &TopologyState) -> BodyKind {
    // Collect distinct ShellKind values across all shells.
    // If any shell is Sheet → BodyKind::Sheet
    // If any shell is Wire  → BodyKind::Wire
    // If all shells are Solid → BodyKind::Solid
}

match (detect_body_kind(&input.target_topology()),
       detect_body_kind(&input.tool_topology())) {
    (BodyKind::Solid, BodyKind::Solid) => run_solid_pipeline(input, engine, ctx),
    (BodyKind::Solid, BodyKind::Sheet) |
    (BodyKind::Sheet, BodyKind::Solid) => Err(KernelError::NotSupported {
        message: "Sheet trim operation not implemented".into(), context: None,
    }),
    (_, BodyKind::Wire) | (BodyKind::Wire, _) => Err(KernelError::NotSupported {
        message: "Wire clip operation not implemented".into(), context: None,
    }),
    _ => Err(KernelError::NotSupported {
        message: "Operation type not implemented".into(), context: None,
    }),
}
```

This ensures unsupported paths return a **named error** instead of silently
producing garbage topology or panicking.

**Acceptance:** `execute_boolean(sheet_input)` returns `Err(NotSupported)`,
not a topology violation.

### T4.2 — Relax `validate_solid` for Halfspace / Large-Box Inputs

**File:** `schema.rs::validate_solid`

The current check `face_count < 4` is the right guard for manifold solids but
blocks valid large-box halfspace approximations at odd face counts.

More precise check (replace the face-count minimum):

```rust
// A valid solid input must have at least one shell marked Solid(Outer).
let has_outer_solid_shell = arena.iter_shells()
    .any(|(_, s)| matches!(s.kind(), ShellKind::Solid(ShellOrientation::Outer)));
if !has_outer_solid_shell {
    return Err(KernelError::InvalidInput { message:
        "Boolean tool must have at least one closed outer shell".into(), context: None,
    });
}
```

**Acceptance:** A 6-face cube (valid) passes. A single-face sheet (invalid for
solid boolean) is caught and named.

### T4.3 — Replace Ray-Parity Classify with Winding Number (Solids Only)

**File:** `classify/eval.rs`, `forge-topo::classify`

Ray-parity fails for non-convex, genus>0, and nested-shell inputs because an
even number of crossings in a re-entrant shell can cancel. Winding number is
the correct algorithm for the general case.

**Crucial Math Constraint:** Winding number is mathematically undefined for open boundaries (`ShellKind::Sheet`). A sheet encapsulates 0 volume. Therefore:

- Solid × Solid: Use Winding Number as the fallback classifier.
- Solid × Sheet: This is an **Imprint & Trim** operation. The Sheet can be classified against the Solid, but **the Solid must never be classified against the Sheet**. The classify phase must skip Solid-against-Sheet entirely.

**However:** Winding number is O(N²) naively. The right strategy is:

1. Keep ray-parity as the **fast path** for the common case (convex solid inputs)
2. Detect when ray-parity is unreliable (escalation flag already on `PointClassification`)
3. Fall back to generalized winding number only when the parity test escalates

This is what `mb6_halfspace_storm.rs` already lists as a `KERNEL REQUIREMENT`.
The `PrecisionEscalation` field on `PointClassification::Inside`/`Outside` is
your hook — populate it when crossing count is suspicious (odd number but
alternating sign pattern) and add a retry path.

**Acceptance:** mb6's `halfspace_storm_80_coplanar` passes with χ=2.

### T4.4 — Add Tests That Assert `NotSupported` for Unimplemented Combinations

**File:** `tests/edge_case_tests.rs`

For every unsupported input combination, add a test that expects the named error:

```rust
#[test]
fn sheet_solid_boolean_returns_not_supported() {
    let sheet = build_sheet_quad(/* ... */); // single face, ShellKind::Sheet
    let cube = make_cube([0.0;3], 10.0).unwrap();
    let input = BooleanInput::new(sheet.0, sheet.1, cube.0, cube.1, BooleanOp::Union);
    let result = execute_boolean(input, &mut ctx, &planar_engine());
    assert!(matches!(result, Err(KernelError::NotSupported { .. })),
        "Expected NotSupported, got {:?}", result);
}
```

**Acceptance:** All unimplemented paths return `NotSupported`. No panics.

---

## 5. Latent Topological Traps (G1-G12)

These are the **architectural traps** most likely to remain latent even after
Tiers 1–6 are complete. Each will appear only under specific geometric conditions
and will fail silently or produce subtly wrong output.

### G1 — The Zombie Edge (Silent Non-Manifold Insertion)

**Where it appears:** After assemble stitch, when `classify_faces` misclassifies
one face adjacent to the intersection boundary. The result: one face is included
that should be excluded, its edges now have 3 radial halfedges in the result arena
(one from target, one from tool reversed, one from the wrongly-included face).
Validation won't catch this at `Quick` level. `Full` validation will, which is why
T1.1 is critical.
**Mitigation:** T1.1 (Full validation gate), plus a dedicated post-stitch check:
`all edges in result must have radial ring size == 2`.

### G2 — The Ray Co-axis Failure

**Where it appears:** `classify_point_in_solid` fires a ray in the +X direction
but the face being classified has its centroid exactly on the X-axis of the
other solid's symmetric geometry. The parity count becomes dependent on how
near-tangent crossings are resolved. The existing multi-direction retry (Tier 2.7
from your list, also D8 in this spec) is the fix — **but it is not yet wired**.
**Mitigation:** Wire the retry logic referenced in your Tier 2.7 task.

### G3 — The Polygon Extract Concave Winding Error

**Where it appears:** `walk_boundary_perimeter` in `polygon_extract.rs` assumes
the perimeter is simply connected (one winding). If a boolean produces a face
whose coplanar group has a T-shaped boundary (two boundary loops meeting at a
point), the perimeter walk can traverse the wrong direction at the junction.
**Mitigation:** After `walk_boundary_perimeter`, verify that the returned vertices
are in a consistent winding order relative to the face plane before calling
`rebuild_face_from_perimeter`.

### G4 — The Disconnected Boolean Result (Multi-Lump)

**Where it appears:** When `Union` of two non-overlapping solids is requested
(user error, but valid input). The result should have two disconnected Lump
entities. The current `copy_faces` creates one `Lump` per call. If both target
and tool faces are copied into one Lump, hierarchy validation will fail after T3.4.
**Mitigation:** After assembly, detect connected components of the result arena
and assign each to a separate Lump. This is a graph connectivity problem (BFS
over the halfedge adjacency graph).

### G5 — The Halfspace Large-Box AABB Explosion

**Where it appears:** `mb6_halfspace_storm` uses boxes with `extent=1e6`. The BVH
for these inputs will have AABB sizes of ~2×10⁶ on each axis. The BVH
`query_overlapping_pairs` will match every face of the large box against every
face of the target cube, producing N×6 BVH pairs when most of them are irrelevant
(the far faces of the large box never actually intersect the cube).
For 200 chained operations, this is 200 × 6 × F_cube pairs, not just F_cube.
**Mitigation:** Add a geometry-scale pre-filter: if an AABB pair's overlap volume
is less than `(characteristic_scale * tolerance)^3`, skip it before the chord gate.

### G6 — The Chord-Gate False Positive on Distant Planes

**Where it appears:** `supplement_cuts_exhaustive` adds cuts to faces that are
"already being cut" (have at least one BVH-proposed cut) against all opposing
planes. If the opposing solid is very large (halfspace box), every plane of the
large box crosses through the target cube's plane-extension. The chord gate fires
for geometrically irrelevant planes.
**Mitigation:** Restrict `supplement_one_direction` to planes whose AABB intersects
the face AABB (a second geometric gate, cheaper than the chord compute).

### G7 — The Select Phase Binary-Decision on Non-Manifold Input

**Where it appears:** `assemble/select.rs` decides Keep/Drop for each
`ClassifiedFace`. For non-manifold edges, a face may simultaneously be `Inside`
relative to one radial neighbor and `Outside` relative to another. The binary
Keep/Drop decision is undefined for this case.
**Mitigation:** Detect faces adjacent to non-manifold edges before select. Escalate
to `PolicyRequired` if any such face exists. This is not yet wired.

### G8 — The Coplanar-Merge Lineage Collapse

**Where it appears:** When `extract_coplanar_regions` merges N fragments into
one face, it picks `sample_face` as the lineage donor. If that face came from the
tool solid but the merged result is semantically a target face (e.g., in a union),
the lineage attribution is wrong. Subsequent feature operations that trace
lineage back to the tool may operate on the wrong face.
**Mitigation:** After T2.3 (Euler-op rebuild), merge lineages explicitly:
`Lineage::merge` across all faces in the group using the appropriate `OpSignature`.

### G9 — The Vertex Outgoing Pointer Corruption After Polygon Extract

**Where it appears:** `rebuild_face_from_perimeter` sets vertex outgoing pointers
to new halfedge IDs. But internal vertices (those exclusively inside the coplanar
group, deleted at the end) still have their old outgoing pointer set. If any code
checks `arena.get_vertex(internal_v).outgoing()` before the remove, it will
get a stale ID pointing to a now-deleted halfedge.
**Mitigation:** Set internal vertex outgoing pointers to a sentinel (`HalfEdgeId::MAX`)
before the delete pass and add an assertion that no live vertex has a stale outgoing.

### G10 — The Euler Delta Drift in Chained Operations

**Where it appears:** The `EulerDelta` struct tracks incremental changes per
operator invocation. For chained booleans (mb5, mb6), the delta is fresh per
operation, not cumulative. But `OperationResult` reports the delta for the most
recent operation only. If a Proof System check verifies the cumulative delta
after N operations, it will see only the last delta, not the sum.
**Mitigation:** Make `BooleanResult` carry a `cumulative_delta` field that
aggregates across all phases of one `execute_boolean` call.

### G11 — The Reconcile Search Tolerance Overshoot

**Where it appears:** In `split_solid`, `reconcile_search_tol` is computed as:
`base_tol.max(gap_closure_max * 256.0)`. For inputs with a large gap closure
policy (e.g., importing CAD files with mm-level gaps), this can set the
reconcile tolerance to 256× the gap, which may weld vertices that are actually
distinct. This silently changes topology.
**Mitigation:** Cap `reconcile_search_tol` at `min(calculated, characteristic_scale * 1e-4)`.  
Document the cap as a `ToleranceDecision`.

### G12 — The Shell-Kind Defaulting in `copy.rs`

**Where it appears:** Line 136 of `copy.rs`:

```rust
.unwrap_or(forge_topo::arena::ShellKind::Solid(ShellOrientation::Outer))
```

If the source shell can't be read (stale after draft mutation), the copy silently
marks it as an outer solid shell. This turns a void shell (inner) into an outer
shell in the result, inverted orientation silently.
**Mitigation:** Replace `unwrap_or` with `?` — propagate the error. A missing
shell is always a real topology defect, not a recoverable default.

---

## 6. Key Test Commands Reference

```bash
# Full boolean test suite
cargo test -p forge-kernel 2>&1 | grep -E "ok|FAILED|error"

# Specific phase tests
cargo test -p forge-kernel copy_faces
cargo test -p forge-kernel stitch
cargo test -p forge-kernel postprocess
cargo test -p forge-kernel brutality

# With tracing
FORGE_TRACE_DIR=/tmp/forge_traces cargo test -p forge-kernel boolean
cargo run -p forge-view --bin forge-trace-cli -- issues /tmp/forge_traces
cargo run -p forge-view --bin forge-trace-cli -- list /tmp/forge_traces

# Validate a single failing test with full stderr
FORGE_LOG=full cargo test -p forge-kernel <test_name> -- --nocapture 2>&1

# Both pipelines for a given test
FORGE_SKIP_COPLANAR_POSTPROCESS=1 cargo test -p forge-kernel <test_name>
```
