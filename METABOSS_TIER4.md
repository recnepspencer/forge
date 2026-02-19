# Forge MetaBoss Tier 4: "No Kernel Survives" Suite
### The Final 0.0001% — Companion to `PROOF_SYSTEM.md`

---

> This document contains the ultimate stress tests — scenarios where every commercial geometry kernel (ACIS, Parasolid, Open CASCADE) breaks. These tests combine multiple failure modes simultaneously. A green Tier 4 suite means the kernel has surpassed the state of the art.
>
> **Naming convention:** `MB-T4-*` (planar), `MB-CT4-*` (curved), `MB-FT4-*` (fillet). These extend the MB series defined in `PROOF_SYSTEM.md`.
>
> **Acceptance rule:** A Tier 4 test either (a) produces a correct manifold result, or (b) cleanly fails with a structured `ProofFailure` / `PolicyRequired` trace pointing to the exact trigger. Crashing, hanging, or producing non-manifold output is **never acceptable**.

---

# Part 1: Planar Tier 4 (MB-T4-1 through MB-T4-8)

These tests stress the planar Boolean pipeline to its absolute mathematical limits. Every test combines 3+ failure modes simultaneously.

---

### MB-T4-1 — The Coplanar Overlap Apocalypse 🔴

**Test:** Two 10k-face solids whose 800+ faces are exactly coplanar in 12 separate overlapping regions (partial overlaps, nested holes, figure-8 boundaries, one region with 50 collinear points at 1e-15 spacing). Union, then difference the result with a third solid that grazes all 12 planes at 1e-14.

**Failure modes triggered:**
- False intersection edges from coplanar classification ambiguity
- Inconsistent "which face wins" across 800+ coplanar decisions
- Sliver explosion from 1e-15 collinear points
- Orientation flip cascade from 1e-14 graze across 12 planes

**Required infrastructure:**
- Lex-tie-breaker + flush logic stress-tested at 800-face scale
- `CoincidenceGraph` (Doctrine D0) resolving 12 simultaneous coplanar regions
- Must output clean manifold with zero false edges

---

### MB-T4-2 — The Menger Sponge Graze 🧪🔴

**Test:** Level-4 Menger sponge (~20k faces, genus ~6,000) booleaned with a second identical sponge rotated 0.000001° and translated 1e-12 along each axis so that 1,200 edges graze vertices/edges of the first at near-machine-epsilon. Then chain 50 more micro-rotated unions.

**Failure modes triggered:**
- Thin-feature misclassification (sponge walls at extreme scale)
- 1,200 simultaneous edge-vertex grazes at machine epsilon
- Iterative topology shredding over 50 chained operations
- High-genus Euler validation (genus ~6,000 — most kernels only test genus 0–1)
- Sliver avalanche from accumulated micro-rotation drift

**Required infrastructure:**
- Relative-epsilon + symbolic vertices + queue decimator
- Generalized Euler (P0.2) maintaining correctness at genus ~6,000
- Per-step invariant checkpoints (P4.5) catching degradation at each of 50 steps
- Zero false slivers after 50 ops

---

### MB-T4-3 — The High-Valence Singularity Star 🔴

**Test:** 64 cubes + 32 tetrahedra + 16 dodecahedra all sharing one single vertex with edges radiating in every direction (SoS pushed to 112-way degeneracy). Subtract a tool that passes 1e-15 away from that vertex while exactly coplanar with 8 of the faces.

**Failure modes triggered:**
- SoS breakdown at 112-way degeneracy (far beyond typical 4–8 way)
- Predicate inconsistency across 100+ incident elements at one vertex
- Non-manifold star repair failure at extreme valence
- 1e-15 near-miss forcing precision escalation on every incident face

**Required infrastructure:**
- Full SoS + non-manifold post-processor at extreme valence
- Precision pipeline (P2.2) handling 100+ simultaneous near-degenerate decisions
- Euler auditor (P0.2) still holding at 112-way vertex
- Must produce valid manifold or structured `PolicyRequired`

---

### MB-T4-4 — The Self-Intersecting Thin Labyrinth 🔴

**Test:** Input a deliberately corrupted solid (self-intersecting faces + non-manifold wire edges + 5,000 walls of thickness 1e-10) inside a clean outer cube. Intersect with a complex tool that creates 200 new self-intersections inside the labyrinth.

**Failure modes triggered:**
- Self-intersection recovery on 200+ simultaneous violations
- Thin-feature misclassification at 1e-10 wall thickness
- Non-manifold wire edges in the input
- Orientation inconsistency from crossed/self-intersecting faces

**Required infrastructure:**
- Transaction-rollback (Doctrine D6) on failed intersection recovery
- Self-intersect cleaner + relative epsilon (P2.4)
- Must heal everything into one valid manifold shell, or correctly report empty/unfixable
- `OperationResult<T>` must carry full diagnostic payload

---

### MB-T4-5 — The 500-Step Exact Cancellation Chain 🔴

**Test:** `(((A ∪ B) − C) ∩ D) …` repeated 500 times using identical geometry with exact 180° rotations and translations that should periodically cancel back to the original solid or perfect voids. Insert one 1e-14 graze at step 237.

**Failure modes triggered:**
- Deep path-dependent accumulated error over 500 operations
- Numerical drift altering cancellation outcome
- Exact-flush empty result at intermediate steps
- Orientation flip hidden until step ~498 by accumulated topology
- Single 1e-14 graze at step 237 butterfly-effecting the chain

**Required infrastructure:**
- Per-step invariant checkpoints (P4.5) — must detect graze effect at step 237
- Signed-volume accounting (P4.3) verifying cancellation at each period
- Global re-normalizer preventing drift accumulation
- Bit-identical final topology to the no-graze case (graze should be resolved by precision pipeline, not alter the result)

---

### MB-T4-6 — The Unbounded Half-Space Storm 🔴

**Test:** Boolean a closed solid against 200 infinite planar half-spaces (open sheets) arranged so 80 are exactly coplanar in groups, 60 graze at 1e-14, and 60 create sliver volumes thinner than 1e-12. Convert final result back to closed manifold.

**Failure modes triggered:**
- Open-sheet trimming logic at scale
- Coplanar storm with 80 exactly-aligned half-spaces
- Thin-volume slivers at 1e-12 thickness
- Unbounded → bounded classification failure

**Required infrastructure:**
- Winding-number classifier (P1.2) handling unbounded domains
- Open-sheet handler converting infinite half-space results to finite manifold
- Manifold repair post-processing
- Dual-path cross-check (P1.1) verifying the bounded conversion

---

### MB-T4-7 — The Scale-Invariant Micro-Feature Avalanche 🔴

**Test:** A 1e12-unit cube containing 10,000 micro-cubes (1e-9 size) arranged in a 3D grid with 1e-12 gaps. Subtract a tool that grazes every micro-cube at 1e-14 while being exactly flush with the large cube faces.

**Failure modes triggered:**
- 21 orders of magnitude scale separation (1e12 to 1e-9)
- 10,000 simultaneous thin-feature decisions
- Collinear storm from grid arrangement
- Performance death on 10,000+ BVH queries

**Required infrastructure:**
- Local coordinate spaces (P2.4, DZ-1) — mandatory, not optional
- BVH acceleration for 10,000-face intersection
- Lazy evaluation on micro-feature boolean
- Relative epsilon scaled per micro-cube
- **Performance gate:** Must complete in < 10 seconds with zero slivers

---

### MB-T4-8 — The Ultimate Planar Degeneracy Avalanche (True Final Boss) 🧪🔴

**Test:** Start with MB-T4-3 star, add MB-T4-1 coplanar overlaps on 12 planes, wrap in MB-T4-2 Menger-level genus, inject MB-T4-4 self-intersections, then run MB-T4-5 200-step chain with MB-T4-7 micro-features and one orientation flip at step 100. Fuzz against CGAL Nef at every step.

**Failure modes triggered:** Literally every planar failure mode simultaneously.

**Required infrastructure:**
- Full pipeline: predicates → symbolic vertices → SoS → decimator → Euler auditor → transaction rollback
- Must survive or cleanly fail with a debug trace (P3.3) pointing to the exact trigger
- When this test goes green, the planar layer is harder than any commercial kernel

---

# Part 2: Curved Geometry Tier 4 (MB-CT4-1 through MB-CT4-8)

These tests extend the planar Tier 4 into analytic curved geometry — circles, helices, ellipses. They prove the curve layer has inherited full planar robustness.

---

### MB-CT4-1 — The Coplanar Curve Overlap Apocalypse 🧪🔴

**Test:** 12 exactly coplanar circular arcs (radii from 1e-6 to 1e6) with 800+ partial overlaps, nested holes, figure-8 touching points, and 50 collinear points spaced at 1e-15 along one arc. Boolean union with a planar tool that grazes all 12 at 1e-14, then difference the result.

**Failure modes triggered:**
- False intersection segments from coplanar curve ambiguity
- "Which arc wins?" classification across 800+ overlapping curves
- Tiny dangling curve fragments from intersection resolution
- Orientation flip on split edges at parametric boundaries

**Required infrastructure:**
- Coplanar curve classifier + lex-tie-breaker extended to parametric space
- Exact 1D sorter on curve parameter for collinear resolution
- Zero dangling fragments in final result

---

### MB-CT4-2 — The Micro-Curve Avalanche 🔴

**Test:** 1e12-unit cylinder containing 10,000 tiny circular holes (radius 1e-9) arranged in a grid with 1e-12 gaps. Subtract a curved tool (helical groove) that grazes every hole at 1e-14 while flush with the big cylinder. Chain 20 more micro-rotated booleans.

**Failure modes triggered:**
- Relative epsilon failure on tiny radii (1e-9 vs 1e12 container)
- Intersection point drift over 20 chained operations
- Zero-length curved edges from micro-hole grazes

**Required infrastructure:**
- Symbolic curve representation (`curve = intersection of 2 surfaces`)
- Local coordinate normalization mandatory before every curve-surface intersect (P2.4)
- Per-step curve-length checkpoints

---

### MB-CT4-3 — The Tangent Graze Storm 🔴

**Test:** 200 analytic curves (circles + helices) that are pairwise exactly tangent or 1e-14 away from each other or from vertices/edges. Perform 50 chained booleans with tiny rotations.

**Failure modes triggered:**
- Inconsistent In/On/Out classification at tangency points
- NaN in curvature computation at exact tangent contact
- Zero-length edges from tangent resolution
- Self-intersecting trim curves from micro-rotation accumulation

**Required infrastructure:**
- Full SoS extended to curve parameters (EdgeID + parameter value for tie-breaking)
- Adaptive precision curve-curve / curve-surface predicates
- Per-step invariant checkpoints

---

### MB-CT4-4 — The High-Valence Curve Singularity Star 🧪🔴

**Test:** 64 circular arcs + 32 helical edges + 16 elliptical arcs all meeting at one single vertex with every possible degeneracy (tangent, cusp, zero curvature). Subtract a tool that passes 1e-15 away while exactly tangent to 12 of them.

**Failure modes triggered:**
- Predicate explosion at 112-way curve vertex
- Non-manifold curve star at extreme valence
- Euler violation on the vertex from inconsistent curve classification

**Required infrastructure:**
- Non-manifold post-processor handling curve valence > 20
- Symbolic re-evaluation of entire star at high precision
- Euler auditor extended to curved topology

---

### MB-CT4-5 — The 500-Step Curved Cancellation Chain 🔴

**Test:** `(((A ∪ cylinder) − torus) ∩ helix) …` repeated 500 times with exact 180° rotations and translations that should cancel back to original or perfect voids. Inject one 1e-14 graze at step 237.

**Failure modes triggered:**
- Tiny curve fragment "cancer" growing silently until step ~480
- Hidden orientation flip in curved shell genus
- Wrong genus on curved shells from accumulated classification errors

**Required infrastructure:**
- Per-step curve-length + winding-number + signed-volume checkpoints
- Transaction rollback with full curve trace dump
- Must detect graze effect at step 237, not step 480

---

### MB-CT4-6 — The Self-Intersecting Curve Labyrinth 🔴

**Test:** Feed a deliberately dirty solid (self-intersecting circular loops + 5,000 micro-arc walls at 1e-10 thickness) and intersect with a complex curved tool that creates 300 new self-intersections inside the labyrinth.

**Failure modes triggered:**
- Self-intersection recovery failure on curved segments
- Thin curved walls misclassified due to relative epsilon failure
- Non-manifold wire-curves from intersection resolution

**Required infrastructure:**
- Self-intersect pre-pass extended to parametric curves
- Relative epsilon handling for 1e-10 thickness curved walls
- Auto-refine stage for near-self-intersecting curved loops

---

### MB-CT4-7 — The Unbounded Curve Trimming Storm 🔴

**Test:** Boolean a closed solid against 200 bounded analytic curves arranged as "infinite" trimming paths (very long helices, large-radius arcs) where 80 are exactly coplanar in groups, 60 graze at 1e-14, 60 create sliver volumes thinner than 1e-12. Convert back to closed manifold.

**Failure modes triggered:**
- Open-curve trimming logic at extreme scale
- Unbounded classification failure on large-radius arcs
- Sliver curved faces from 1e-12 thickness volumes

**Required infrastructure:**
- Winding-number classifier extended to curved edges (P1.4)
- Manifold repair for open→closed conversion on curves
- Fuel-bounded quadrature (Doctrine P5) on integration over long curves

---

### MB-CT4-8 — The Ultimate Curve Degeneracy Avalanche (True Final Boss) 🧪🔴

**Test:** MB-CT4-4 star + MB-CT4-1 coplanar curves on 12 surfaces + MB-CT4-2 micro-curves + MB-CT4-6 dirty input + MB-CT4-5 200-step chain + one orientation flip at step 100 + MB-CT4-3 grazes everywhere. Fuzz against OpenCascade + Parasolid reference at every checkpoint.

**Failure modes triggered:** Every curved geometry failure mode simultaneously.

**Required infrastructure:**
- Full integration of curve pipeline with all proof layers
- When this test goes green, the curve layer has inherited full planar god-tier robustness
- Reference comparison against Parasolid/OpenCascade at every checkpoint — must match or cleanly exceed

---

# Part 3: Fillet Tier 4 (MB-FT4-1 through MB-FT4-8)

These tests target the hardest problem in B-rep modeling: fillet operations. Filleting combines surface generation, topology mutation, and precision management in ways that no other operation does.

---

### MB-FT4-1 — The Multi-Edge Junction Apocalypse 🧪🔴

**Test:** 16 edges (mix of straight + circular + helical) all meeting at one vertex. Apply constant-radius fillet to all, then override 6 of them with variable-radius (shrinking from 5mm to 0.0001mm). Add a 13th "crossing" edge that grazes the junction at 1e-14.

**Failure modes triggered:**
- Non-manifold pinch at the 16-way junction
- Lost G1 continuity at 16 simultaneous blend transitions
- Zero-radius collapse creating wire edges (0.0001mm region)
- Orientation flip on new blend faces from 1e-14 graze

**Required infrastructure:**
- Junction topology solver for 16-way vertex
- Symbolic fillet-surface representation (not just sampled points)
- Non-manifold post-processor extended to blend vertices

---

### MB-FT4-2 — The Variable-Radius Collapse & Overflow Storm 🔴

**Test:** Long thin wall (thickness 1e-9) with 200 edges. Apply variable-radius fillet that starts at 2mm, linearly drops to 0mm, then "overflows" past the opposite side. Chain 30 more operations with micro-rotations.

**Failure modes triggered:**
- Radius hits zero → dangling curve fragments in blend surface
- Overflow → self-intersecting blend surface crossing the wall
- Thin-wall misclassification at 1e-9 thickness

**Required infrastructure:**
- Relative-epsilon radius clamping (auto-clamp at feature-size-aware minimum)
- Automatic "stop & split" logic where radius exceeds available material
- Queue decimator for collapsing blend regions
- Per-step invariant checkpoints over 30 chained operations

---

### MB-FT4-3 — The Grazing Tangent Fillet Nightmare 🔴

**Test:** 120 curved edges where 80 are exactly tangent to the fillet path and 40 are 1e-14 away. Fillet all at once with G2 continuity requested. Then difference the result with a tool that grazes every new blend edge.

**Failure modes triggered:**
- Inconsistent tangent classification across 120 edges
- NaN in curvature continuity computation at exact tangent
- Zero-length fillet segments from tangent collapse
- SoS failure on blend rail curves

**Required infrastructure:**
- Full SoS + symbolic vertices extended to fillet-rail curves
- Adaptive precision for rolling-ball vs. edge tangency determination
- G2 continuity enforcement with certified error bounds

---

### MB-FT4-4 — The Thin-Feature Fillet Labyrinth 🔴

**Test:** Deliberately dirty input with 8,000 micro-edges (thickness 1e-10) and self-intersecting loops. Fillet every edge with 0.05mm radius (larger than some features). Intersect the filleted body with a complex curved tool.

**Failure modes triggered:**
- Fillet swallows entire thin features (radius > feature size)
- Non-manifold "swallow" regions where features disappear
- Self-intersection recovery failure on blend surfaces

**Required infrastructure:**
- Pre-fillet auto-refine: detect features smaller than radius, report via `PolicyRequired` (DZ-4)
- Self-intersect cleaner extended to fillet-generated faces
- Relative epsilon on feature size vs. radius (never fillet a feature smaller than the radius without explicit policy)

---

### MB-FT4-5 — The 500-Step Fillet Chain Cancellation 🔴

**Test:** `(((base ∪ fillet-set1) − tool) + fillet-set2) …` repeated 500 times. Radii and positions designed to periodically cancel back to perfect sharp edges or voids. Inject one 1e-14 graze on a fillet rail at step 243.

**Failure modes triggered:**
- Tiny blend fragment "cancer" growing silently until step ~470
- Hidden orientation flip in fillet junction
- Wrong genus on filleted shells from accumulated errors

**Required infrastructure:**
- Per-step signed-volume + fillet-continuity + Euler checkpoints
- Full transaction rollback with blend-surface trace dumps (P3.4)
- Must detect graze at step 243, not step 470

---

### MB-FT4-6 — The High-Valence Fillet Singularity Star 🧪🔴

**Test:** 48 edges radiating from one vertex (all possible curvatures). Fillet with varying radii so 12 junctions meet at a single point with G2 continuity. Subtract a tool that passes 1e-15 away from the star while tangent to 9 blend surfaces.

**Failure modes triggered:**
- Predicate explosion at 48-way blend vertex
- Non-manifold "black-hole" vertex from junction collapse
- Curvature discontinuity cascade across 12 junctions

**Required infrastructure:**
- Non-manifold repair + SoS pushed to 48-way blend valence
- Symbolic re-evaluation of entire fillet star at full precision
- Euler auditor handling extreme-valence fillet topology

---

### MB-FT4-7 — The Scale-Invariant Micro-Fillet Avalanche 🔴

**Test:** 1e12-unit block containing 15,000 tiny holes/ribs. Fillet every single edge with 1e-9 radius (micro-fillets). Then apply a large-radius fillet on the outer block that grazes all micro-fillets at 1e-14.

**Failure modes triggered:**
- 21 orders of magnitude scale separation killing precision
- Micro-fillets creating 15,000 sliver blend surfaces
- Performance death on 15,000 blend surface evaluations
- Large-radius fillet interfering with all micro-fillets at 1e-14

**Required infrastructure:**
- Local coordinate normalization before every fillet (P2.4, DZ-1)
- BVH acceleration for 15,000 blend surfaces
- Lazy evaluation on blend surface creation (only create when intersected)
- **Performance gate:** Must complete with zero slivers

---

### MB-FT4-8 — The Ultimate Fillet Degeneracy Avalanche (True Final Boss) 🧪🔴

**Test:** Combine MB-FT4-6 star + MB-FT4-1 multi-junctions on 12 surfaces + MB-FT4-2 variable-radius collapses + MB-FT4-4 dirty thin labyrinth + MB-FT4-5 200-step chain + MB-FT4-3 grazes + one intentional orientation flip at step 112 + MB-FT4-7 micro-fillets everywhere. Fuzz against Parasolid + OpenCascade at every checkpoint.

**Failure modes triggered:** Every fillet failure mode simultaneously — the test that proves the entire pre-NURBS stack (planar + curves + fillets) is harder than any commercial kernel.

**Required infrastructure:**
- Full integration + automatic test-case mutation
- Reference comparison against Parasolid/OpenCascade at every checkpoint
- When this test goes green, the fillet layer is untouchable

---

# Part 4: Summary Tables

## Test Count by Domain

| Domain | Tests | Risk Profile | Key Challenge |
|--------|-------|-------|---------------|
| Planar (MB-T4) | 8 | 🔴🧪 | Coplanar storms + extreme valence + scale separation |
| Curved (MB-CT4) | 8 | 🔴🧪 | Parametric precision + tangent classification + symbolic curves |
| Fillet (MB-FT4) | 8 | 🔴🧪 | Junction topology + radius overflow + G2 continuity |
| **Total** | **24** | | |

## Failure Mode Coverage Matrix

| Failure Mode | Planar Tests | Curve Tests | Fillet Tests |
|---|---|---|---|
| Coplanar/flush ambiguity | T4-1, T4-6 | CT4-1, CT4-7 | — |
| High-valence degeneracy | T4-3 | CT4-4 | FT4-1, FT4-6 |
| Scale separation (21 OoM) | T4-7 | CT4-2 | FT4-7 |
| Self-intersection recovery | T4-4 | CT4-6 | FT4-4 |
| 500-step chain + graze | T4-5 | CT4-5 | FT4-5 |
| Thin-feature misclassification | T4-4, T4-7 | CT4-2, CT4-6 | FT4-2, FT4-4 |
| Orientation flip cascade | T4-1, T4-5 | CT4-1, CT4-5 | FT4-1, FT4-5 |
| Unbounded domain handling | T4-6 | CT4-7 | — |
| High-genus Euler | T4-2 | CT4-5 | FT4-5 |
| Tangent classification | — | CT4-3, CT4-4 | FT4-3, FT4-6 |
| Radius overflow/collapse | — | — | FT4-2, FT4-4 |
| G2 continuity enforcement | — | — | FT4-3, FT4-6 |
| **All-at-once final boss** | T4-8 | CT4-8 | FT4-8 |

## Prerequisites from PROOF_SYSTEM.md

Each Tier 4 test depends on proof infrastructure from the parent spec:

| Proof System Component | Tier 4 Tests That Depend On It |
|---|---|
| Layer 1: Topological Invariants (P0) | All 24 tests (Euler validation at every step) |
| Layer 2: Dual-Path Verification (P1) | T4-6, T4-8, CT4-7, CT4-8 (unbounded conversion) |
| Layer 3: Precision Pipeline (P2) | T4-3, T4-5, T4-7, CT4-2, CT4-3, FT4-7 (scale + tangent) |
| Layer 4: Causal Replay (P3) | T4-5, T4-8, CT4-5, CT4-8, FT4-5, FT4-8 (chain debugging) |
| Layer 5: Self-Consistency (P4) | T4-5, CT4-5, FT4-5 (cancellation identity) |
| Doctrine P5 (Det. Fuel) | CT4-7, FT4-7 (quadrature on curved surfaces) |
| Doctrine P6 (Gen. Handles) | All 24 tests (extreme mutation sequences) |
| DZ-1 (Local Coordinates) | T4-7, CT4-2, FT4-7 (scale separation) |
| DZ-4 (Fillet Cascade) | FT4-1, FT4-2, FT4-4 (radius vs. feature size) |

---

# Part 5: The Thesis

> When all three final boss tests (MB-T4-8, MB-CT4-8, MB-FT4-8) go green — or cleanly fail with structured traces pointing to the exact trigger — the Forge kernel has surpassed every commercial B-rep engine in provable robustness. Not by a margin. By a category.
