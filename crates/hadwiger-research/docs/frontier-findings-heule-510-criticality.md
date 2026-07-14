# Frontier Finding: Heule-510 Criticality Drill

Status: in progress. This note records a falsifiable drill over the retained
exact Heule-510 seed (510 vertices, 2504 edges, exact algebraic embedding,
retained varisat non-4-colorability certificate).

## Hypotheses

### H1: Edge slack exists

The Heule-510 seed is vertex-minimized but not proven edge-critical inside
this pipeline. H1 claims at least one unit-distance edge is removable while
preserving non-4-colorability. Every confirmed removable edge yields a
strictly sparser 5-chromatic unit-distance graph whose exact embedding is
inherited from the seed, and whose authority chain (exact unit-distance
replay, fresh UNSAT evidence) can be regenerated through the existing
Milestone 1 checker lanes.

Falsifier: every sampled edge-deletion flips the varisat lane to
`SatModelVerified` (the mutated graph becomes 4-colorable), with sampling
covering every Weisfeiler-Leman edge class.

### H2: Pressure score predicts criticality

The frontier projection's pressure score (`degree * 100 + triangles`) is
currently motif archaeology, exactly the gap Milestone 3 names. H2 claims the
score carries decision-relevant signal: low-pressure edges are
disproportionately removable (or, if H1 falsifies, low-pressure deletions are
disproportionately cheap to refute), and pressure-halo hub/spoke edges are
critical.

Falsifier: criticality outcomes are statistically indistinguishable across
the pressure ranking, in which case the pressure score must be retired as a
planning signal with this note as the retained evidence.

## Method

1. Import `FrontierGraphSeedImport::heule_510_exact()` through the admitted
   Hadwiger handle (Query declaration lane).
2. Build a local structural index: adjacency, per-vertex triangle counts,
   3-round Weisfeiler-Leman refinement classes. WL classes over-approximate
   automorphism orbits, so per-class results are sampling guidance, not
   transferred authority; every claimed outcome is individually checked.
3. Vertex sweep: one representative per WL vertex class, lowest pressure
   first; each deletion replayed through `verify_k_colorability_checked`
   (live varisat, model replay on SAT).
4. Edge sweep: one representative per WL edge class
   (`(endpoint classes, common neighbor count)`), lowest combined endpoint
   pressure first; same checker lane.
5. Posture mapping: `SatModelVerified` proves the deleted element critical;
   UNSAT postures mark removable candidates that then queue for full
   certificate generation through
   `generate_k_colorability_certificate_with_varisat_checked`.

Driver: `examples/drill_edge_criticality.rs` (budgeted via `DRILL_MODE`,
`DRILL_BUDGET`, `DRILL_ORDER`).

## Structural Results (exact, no solver required)

- 510 vertices, 2504 edges (the retained Lean-formalization variant; Heule's
  published trimming-paper graph carries 2508 edges, so this seed is already
  4 edges leaner).
- 3-round Weisfeiler-Leman refinement separates classes 316 -> 352 -> 435;
  by round 5 every vertex is a singleton class. Because automorphisms
  preserve WL classes, the automorphism group is trivial: the seed has no
  exact symmetry left to exploit, and orbit-based drill pruning is
  impossible. Pressure ranking is the only prioritization that survives.
- The six halo satellites (85, 88, 91, 94, 97, 100) nevertheless share an
  identical pressure score (degree 24, 22 triangles, score 2422): the halo
  motif is a fossil of the ancestral 6-fold symmetry that minimization
  otherwise destroyed. The hub (vertex 1) has degree 36 and 30 triangles.
- Eight vertices have degree 4 (140, 145, 317, 320, 323, 326, 329, 332) -
  the theoretical minimum for a 5-vertex-critical graph (min degree >= k-1).
  Conditional on vertex-criticality at such a vertex v, every edge at v is
  provably 4-color-critical without solving: a 4-coloring of G-v leaves at
  most 3 colors on the punctured neighborhood of v after deleting one of its
  edges, so a free color always exists. These 32 edges are theory-locked
  predictions the drill can cross-check.
- Vertices 140 and 145 are degree-4 with zero triangles (pressure 400), and
  edges 140-149 / 145-148 are the lowest-pressure edges in the graph
  (pressure 902, no common neighbors).

## Drill Results

### Vertex-criticality: verified 510/510

Every one of the 510 vertex deletions flips the varisat lane to
`SatModelVerified` (the mutated graph is 4-colorable, model independently
replayed against the CNF). The exact Heule-510 seed is therefore
**4-vertex-critical**: no single vertex can be removed while preserving
5-chromaticity. Sweep cost: 510 solves, median 0.2 s, max 7.3 s
(vertex 123). Drivers: `drill_edge_criticality.rs` (435 WL-class
representatives) plus `drill_vertex_remainder.rs` (75 WL-shared vertices).

Corollary (theory + checked witnesses): all 32 edges incident to the eight
degree-4 vertices are edge-critical, because the checked 4-coloring of
`G - v` extends across any single deleted edge at `v`. Edge 140-149 was also
confirmed directly (`SatModelVerified`, 0.1 s).

### Solve-cost anti-correlates with pressure score

Mean SAT solve seconds by pressure band over the representative sweep:

| pressure band | n | mean seconds |
| --- | --- | --- |
| 400-703 | 108 | 0.43 |
| 703-906 | 108 | 0.38 |
| 906-1206 | 108 | 0.38 |
| 1206-2422 | 108 | 0.20 |
| 2422-3630 (halo) | 3 | 0.10 |

Deleting a *low*-pressure vertex leaves the most rigid (hardest) coloring
instance; deleting the hub or a satellite relaxes the instance fastest. The
full-graph UNSAT solve costs ~231 s, and the brute-force edge probe of
145-148 (a theory-locked critical edge) exceeded 24 minutes without finding
its forced near-rigid model before being superseded.

This **falsifies the directional reading of the pressure-halo hypothesis**:
the hub is not the color-pressure reservoir - it is the slackest region of
the graph. Chromatic rigidity concentrates in the low-degree rim, exactly
where the static score is lowest. The score is decision-relevant but with
the opposite sign the hypothesis generator assumed.

### The contraction reformulation collapses edge-test cost

Because the seed is non-4-colorable, every 4-coloring of `G - e` must give
the endpoints of `e` the same color, so `G - e` is 4-colorable iff the
contraction `G / e` is. The contraction is smaller and strictly more
constrained, and varisat resolves it orders of magnitude faster than the
raw deletion: edge 145-148, which the raw `G - e` lane could not solve in
24+ minutes, resolves via contraction in 0.2 s (`drill_edge_residue.rs`).
All future edge-criticality tests in this pipeline should run through
contraction.

### Edge-criticality via witness transfer

`drill_edge_witness_transfer.rs` retained all 510 vertex-deletion colorings
and certified **633 of 2504 edges critical with zero additional solving**:
for edge `{u, v}`, when the retained coloring of `G - u` gives `v` a color
unique in `u`'s neighborhood, recoloring `u` with it constructs a 4-coloring
of `G - uv` that is exhaustively re-verified edge-by-edge. (At degree-4
vertices the neighborhood is forced rainbow, so all 32 theory-locked edges
certify automatically.)

The 1871-edge residue concentrates where neighborhoods repeat colors - all
36 hub edges are residue, as the rigidity picture predicts. The residue is
being resolved through the contraction lane.

## Interpretation

(to be filled after the edge map completes)
