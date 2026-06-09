# Candidate Screening Invariants

This tracker is the single Markdown index for the Hadwiger-Nelson screening
invariants. Each invariant has exactly one owning Rust file under
`src/candidate_screening/invariants/`.

Status meanings:

- `native_exact`: implemented as a deterministic in-crate checker/evaluator.
- `native_bounded`: implemented for explicit bounded cases with typed budget
  stops.
- `native_special_case`: implemented for a mathematically sound subclass.
- `query_native_bounded`: implemented for explicit bounded cases through a
  real Query screening declaration plus deterministic replay.
- `query_native_special_case`: implemented for a mathematically sound subclass
  through a real Query screening declaration plus deterministic replay.
- `certificate_required`: operationalized through checked certificate intake;
  native robust math still needs implementation.
- `heuristic`: ranking/filter evidence only; never proof authority.
- `advisory`: Query-retained advisory contribution, not an invariant catalog row.

| # | Invariant | Code File | Current Status |
|---|---|---|---|
| 1 | Exact unit-distance conflict test | `src/candidate_screening/invariants/exact_unit_distance_conflict.rs` | `query_native_special_case` |
| 2 | Tile diameter safety test | `src/candidate_screening/invariants/tile_diameter_safety.rs` | `query_native_special_case` |
| 3 | Same-color separation / distance-set test | `src/candidate_screening/invariants/same_color_separation_distance_set.rs` | `query_native_special_case` |
| 4 | Boundary ownership test | `src/candidate_screening/invariants/boundary_ownership.rs` | `query_native_special_case` |
| 5 | Exact conflict graph construction | `src/candidate_screening/invariants/exact_conflict_graph_construction.rs` | `query_native_special_case` |
| 6 | Clique-number lower bound | `src/candidate_screening/invariants/clique_number_lower_bound.rs` | `native_bounded` |
| 7 | Independence-number lower bound | `src/candidate_screening/invariants/independence_number_lower_bound.rs` | `native_bounded` |
| 8 | Weighted independence-number bound | `src/candidate_screening/invariants/weighted_independence_number_bound.rs` | `native_special_case` |
| 9 | Hall-ratio / subpatch independence bound | `src/candidate_screening/invariants/hall_ratio_subpatch_independence_bound.rs` | `native_bounded` |
| 10 | Fractional chromatic number | `src/candidate_screening/invariants/fractional_chromatic_number.rs` | `query_native_bounded` |
| 11 | Lovasz theta bound | `src/candidate_screening/invariants/lovasz_theta_bound.rs` | `query_native_special_case` |
| 12 | Spectral / Hoffman bound | `src/candidate_screening/invariants/spectral_hoffman_bound.rs` | `native_special_case` |
| 13 | Degeneracy / k-core filter | `src/candidate_screening/invariants/degeneracy_k_core_filter.rs` | `native_exact` |
| 14 | Maximum-degree sanity check | `src/candidate_screening/invariants/maximum_degree_sanity_check.rs` | `advisory` |
| 15 | Perfect-graph sanity check | `src/candidate_screening/invariants/perfect_graph_sanity_check.rs` | `native_special_case` |
| 16 | SAT / ILP 6-colorability test | `src/candidate_screening/invariants/sat_ilp_six_colorability.rs` | `native_bounded` |
| 17 | Critical-subgraph extraction | `src/candidate_screening/invariants/critical_subgraph_extraction.rs` | `native_bounded` |
| 18 | Periodic quotient graph test | `src/candidate_screening/invariants/periodic_quotient_graph.rs` | `query_native_special_case` |
| 19 | Forbidden displacement set | `src/candidate_screening/invariants/forbidden_displacement_set.rs` | `query_native_special_case` |
| 20 | Minkowski-difference geometry test | `src/candidate_screening/invariants/minkowski_difference_geometry.rs` | `query_native_special_case` |
| 21 | Autocorrelation zero test | `src/candidate_screening/invariants/autocorrelation_zero.rs` | `query_native_special_case` |
| 22 | Density cap for each color class | `src/candidate_screening/invariants/density_cap_each_color_class.rs` | `query_native_special_case` |
| 23 | Local density-window test | `src/candidate_screening/invariants/local_density_window.rs` | `query_native_special_case` |
| 24 | Unit-distance embeddability test | `src/candidate_screening/invariants/unit_distance_embeddability.rs` | `query_native_special_case` |
| 25 | Rigidity / realization consistency test | `src/candidate_screening/invariants/rigidity_realization_consistency.rs` | `query_native_bounded` |
| 26 | Numerical margin test | `src/candidate_screening/invariants/numerical_margin.rs` | `query_native_special_case` |
| 27 | Exact arithmetic / interval certificate test | `src/candidate_screening/invariants/exact_arithmetic_interval_certificate.rs` | `query_native_special_case` |
| 28 | Monodromy / color-holonomy test | `src/candidate_screening/invariants/monodromy_color_holonomy.rs` | `query_native_bounded` |
| 29 | Symmetry-orbit reduction test | `src/candidate_screening/invariants/symmetry_orbit_reduction.rs` | `query_native_bounded` |
| 30 | Translation / rotation closure test | `src/candidate_screening/invariants/translation_rotation_closure.rs` | `query_native_bounded` |
| 31 | Substitution consistency test | `src/candidate_screening/invariants/substitution_consistency.rs` | `query_native_bounded` |
| 32 | Finite patch boundary-extension test | `src/candidate_screening/invariants/finite_patch_boundary_extension.rs` | `query_native_bounded` |
| 33 | Exhaustive local-neighborhood test | `src/candidate_screening/invariants/exhaustive_local_neighborhood.rs` | `query_native_bounded` |
| 34 | Known obstruction containment test | `src/candidate_screening/invariants/known_obstruction_containment.rs` | `query_native_bounded` |
| 35 | Candidate novelty / non-isomorphism test | `src/candidate_screening/invariants/candidate_novelty_non_isomorphism.rs` | `query_native_bounded` |

Next implementation pressure should go first to exact geometry ownership
families, then finite graph optimization families, then periodic/generated
pattern families.


Context dump:
Here’s the cleaned-up robust invariant/filter list.

# Hadwiger–Nelson Candidate Screening Invariants

## 1. Exact unit-distance conflict test

The forbidden condition is **distance exactly (1)**, not merely “too close.”

For two same-color regions (A) and (B), reject if:

[
\exists a\in A,\ b\in B \quad \text{such that} \quad |a-b|=1.
]

Equivalently:

[
1\in \Delta(A,B)
]

where

[
\Delta(A,B)={|a-b|:a\in A,\ b\in B}.
]

For compact connected regions, a practical sufficient test is:

[
d_{\min}(A,B)\le 1\le d_{\max}(A,B).
]

If the distance interval crosses (1), same-color placement fails.

---

## 2. Tile diameter safety test

Every individual tile/color region must avoid internal unit distances.

For each tile (T), compute:

[
D(T)=\max_{x,y\in T}|x-y|.
]

Safe sufficient condition:

[
D(T)<1.
]

If:

[
D(T)\ge 1,
]

then the tile may contain two points at unit distance and must be checked exactly.

---

## 3. Same-color separation / distance-set test

For same-color tiles (A,B), compute:

[
d_{\min}(A,B)=\min_{a\in A,b\in B}|a-b|
]

and

[
d_{\max}(A,B)=\max_{a\in A,b\in B}|a-b|.
]

Reject if:

[
d_{\min}(A,B)\le 1\le d_{\max}(A,B).
]

Important distinction:

[
d_{\min}\le 1
]

alone is not enough to reject. The actual failure is when the same-color distance set contains exactly (1).

---

## 4. Boundary ownership test

Every boundary point must have a defined color.

Reject if any of these are ambiguous:

[
\bigcup_i C_i=\mathbb R^2
]

[
C_i\cap C_j=\varnothing \quad \text{for } i\ne j
]

unless overlapping boundary ownership is explicitly handled.

Then check boundary points too:

[
\exists x,y \text{ on boundaries},\quad |x-y|=1,\quad c(x)=c(y).
]

Many fake tiling arguments fail only on edges, vertices, or half-open boundary conventions.

---

## 5. Exact conflict graph construction

Build a graph (G) where each tile/region is a vertex.

Add edge (ij) if tiles (T_i,T_j) cannot share a color:

[
ij\in E(G)
\quad\Longleftrightarrow\quad
1\in \Delta(T_i,T_j).
]

Do **not** build the conflict graph from vague adjacency. Build it from certified unit-distance possibility.

---

## 6. Clique-number lower bound

Compute the clique number:

[
\omega(G).
]

Since:

[
\chi(G)\ge \omega(G),
]

reject any 6-color candidate if:

[
\omega(G)>6.
]

A (K_7) in the tile-conflict graph immediately kills 6-colorability.

---

## 7. Independence-number lower bound

Compute the independence number:

[
\alpha(G).
]

Then:

[
\chi(G)\ge \frac{|V(G)|}{\alpha(G)}.
]

Reject if:

[
\frac{|V(G)|}{\alpha(G)}>6.
]

Example:

[
|V|=60,\quad \alpha=9
]

[
\chi(G)\ge \frac{60}{9}=6.67
]

so at least (7) colors are required.

---

## 8. Weighted independence-number bound

Use this when tiles have unequal area or importance.

Assign each tile a weight:

[
w_i=\operatorname{area}(T_i).
]

Let:

[
W=\sum_i w_i.
]

Let (\alpha_w(G)) be the maximum total weight of an independent set.

Then:

[
\chi(G)\ge \frac{W}{\alpha_w(G)}.
]

Reject if:

[
\frac{W}{\alpha_w(G)}>6.
]

This prevents weird unequal tile patterns from hiding density problems behind small tile counts.

---

## 9. Hall-ratio / subpatch independence bound

Do not only test the whole graph. Test subgraphs too.

For every relevant subgraph (H\subseteq G):

[
\rho(H)=\frac{|V(H)|}{\alpha(H)}.
]

Since:

[
\chi(G)\ge \max_{H\subseteq G}\rho(H),
]

reject if:

[
\max_H \frac{|V(H)|}{\alpha(H)}>6.
]

Weighted version:

[
\rho_w(H)=\frac{W(H)}{\alpha_w(H)}.
]

Reject if:

[
\max_H \rho_w(H)>6.
]

This catches small dense obstructions inside large mostly harmless patches.

---

## 10. Fractional chromatic number

Compute:

[
\chi_f(G).
]

Since:

[
\chi(G)\ge \chi_f(G),
]

reject any 6-color candidate if:

[
\chi_f(G)>6.
]

This is one of the best early kill-switches because it catches impossible color-density structure before full coloring search.

---

## 11. Lovász theta bound

Compute a semidefinite bound using Lovász theta.

A useful lower-bound form is:

[
\chi(G)\ge \vartheta(\overline{G}).
]

Reject if:

[
\vartheta(\overline{G})>6.
]

This can catch cases where independence number and fractional coloring are not sharp enough.

---

## 12. Spectral / Hoffman bound

For regular or nearly regular conflict graphs, use eigenvalues.

For a (d)-regular graph with smallest adjacency eigenvalue (\lambda_{\min}<0):

[
\chi(G)\ge 1-\frac{d}{\lambda_{\min}}.
]

Reject if:

[
1-\frac{d}{\lambda_{\min}}>6.
]

This is especially useful for highly symmetric candidates.

---

## 13. Degeneracy / (k)-core filter

If a graph is (5)-degenerate, then it is greedily 6-colorable.

Algorithm:

1. Repeatedly remove vertices of degree (\le 5).
2. If the whole graph disappears, the graph cannot force more than 6 colors.
3. If a nonempty (6)-core remains, only then spend serious effort.

A true 7-critical obstruction must have minimum degree at least (6).

---

## 14. Maximum-degree sanity check

Use Brooks-style intuition.

If:

[
\Delta(G)\le 6
]

and the graph has no (K_7)-type obstruction, it is unlikely to force (7) colors.

This is not a universal proof filter by itself, but it is a strong priority/ranking signal.

Low-degree graphs are usually not where a 7-color obstruction will hide.

---

## 15. Perfect-graph sanity check

If the conflict graph is perfect, then:

[
\chi(G)=\omega(G).
]

So if (G) is perfect and:

[
\omega(G)\le 6,
]

then:

[
\chi(G)\le 6.
]

That means the graph cannot prove a 7-color lower bound.

---

## 16. SAT / ILP 6-colorability test

Encode 6-colorability directly.

Variables:

[
x_{v,c}\in{0,1}
]

meaning vertex (v) receives color (c).

Each vertex gets one color:

[
\sum_{c=1}^{6}x_{v,c}=1.
]

Adjacent vertices cannot share a color:

[
x_{u,c}+x_{v,c}\le 1
]

for every edge (uv) and color (c).

If the SAT/ILP problem is UNSAT, the graph is not 6-colorable.

Then extract a small UNSAT core.

---

## 17. Critical-subgraph extraction

If a graph is not 6-colorable, minimize it.

Find a subgraph (H\subseteq G) such that:

[
\chi(H)>6
]

but removing any edge or vertex makes it 6-colorable.

A strong obstruction should be close to 7-critical.

This gives the AI a small reusable proof object instead of a giant noisy failed candidate.

---

## 18. Periodic quotient graph test

For periodic tilings, build the quotient graph on one fundamental domain.

Include edges not only inside the domain, but also across lattice translations.

For tile (T_i) and translated tile (T_j+\lambda), add an edge if:

[
1\in \Delta(T_i,T_j+\lambda).
]

Then solve coloring on the quotient graph with wraparound constraints.

This catches patterns that look valid in one cell but fail across periodic boundaries.

---

## 19. Forbidden displacement set

For repeated tile shape (P), two translated copies (P+x) and (P+y) conflict when:

[
\exists p,q\in P
\quad\text{such that}\quad
|(x-y)+(p-q)|=1.
]

Let:

[
v=x-y.
]

Precompute the forbidden displacement set:

[
F_P={v:\exists r\in P-P,\ |v+r|=1}.
]

Same-color displacement vectors must avoid (F_P).

Reject if:

[
v\in F_P.
]

This is much stronger than checking center-to-center distance.

---

## 20. Minkowski-difference geometry test

For two regions (A,B), define:

[
A-B={a-b:a\in A,b\in B}.
]

Then (A) and (B) have a unit-distance conflict iff:

[
(A-B)\cap S^1\ne \varnothing
]

where (S^1) is the unit circle.

Reject same-color pairs if:

[
(A-B)\cap S^1\ne \varnothing.
]

This is a clean geometric formulation for exact computation.

---

## 21. Autocorrelation zero test

For each color class (C_i), define autocorrelation:

[
A_i(u)=\operatorname{area}(C_i\cap(C_i+u)).
]

A valid coloring requires:

[
A_i(u)=0
]

for every color (i) and every vector (u) with:

[
|u|=1.
]

Reject if:

[
\exists i,\exists u,\quad |u|=1,\quad A_i(u)>0.
]

This is especially useful for rasterized, generated, or approximate periodic candidates.

---

## 22. Density cap for each color class

Each color class is a measurable 1-avoiding set.

So no color class can exceed known upper-density bounds for 1-avoiding measurable sets.

For each color (C_i), compute its density:

[
d_i.
]

Reject if:

[
d_i>D_{\max}
]

where (D_{\max}) is the best known upper density bound being used.

This mostly catches lopsided colorings.

---

## 23. Local density-window test

Global density can pass while local density is impossible.

For windows (W_R(x)), compute:

[
d_i(W_R(x))=
\frac{\operatorname{area}(C_i\cap W_R(x))}
{\operatorname{area}(W_R(x))}.
]

Reject if the local density exceeds the maximum possible density of a 1-avoiding set in that window size.

This catches local overcrowding.

---

## 24. Unit-distance embeddability test

If the AI proposes a graph first, verify that it can actually be represented as a planar unit-distance graph.

For each edge (ij):

[
|p_i-p_j|^2=1.
]

For non-edges, optionally enforce:

[
|p_i-p_j|^2\ne 1.
]

Reject graph candidates that are chromatically interesting but geometrically impossible.

A graph with (\chi>6) is useless for Hadwiger–Nelson unless it can be realized by actual unit distances in the plane.

---

## 25. Rigidity / realization consistency test

For point-based candidates, check whether the distance constraints are rigid, underdetermined, or inconsistent.

Use the rigidity matrix or interval solving to classify:

1. Impossible realization.
2. Flexible realization.
3. Locally rigid realization.
4. Globally rigid realization.

A proof-grade candidate should have certified realization, not just floating-point coordinates.

---

## 26. Numerical margin test

Floating-point near misses are not proof.

For every same-color pair, certify one of:

[
d_{\max}<1-\varepsilon
]

or

[
d_{\min}>1+\varepsilon
]

or more generally:

[
1\notin \Delta(A,B)
]

using interval arithmetic or exact geometry.

Reject or quarantine candidates with many unresolved distances near (1).

---

## 27. Exact arithmetic / interval certificate test

Any final candidate must be reproducible without trusting floating-point computation.

For every claimed safe pair, produce a certificate that:

[
1\notin \Delta(A,B).
]

For every claimed conflict edge, produce a certificate that:

[
1\in \Delta(A,B).
]

No final result should depend on approximate coordinates alone.

---

## 28. Monodromy / color-holonomy test

For patterns built by repeating patches with transformations or color permutations, track color changes around closed loops.

For every closed loop, the resulting color permutation must be compatible with the starting assignment.

Required:

[
\pi_{\text{loop}}=\mathrm{id}
]

or at least the loop permutation must fix the relevant colors.

Reject if a tile returns to itself with a forced different color.

This catches globally inconsistent patterns that look locally valid.

---

## 29. Symmetry-orbit reduction test

If the candidate has symmetry, quotient by the symmetry group.

Group vertices or tiles into orbits.

Then test coloring constraints on orbit representatives plus stabilizer constraints.

This reduces search size and also exposes hidden contradictions caused by symmetry.

Reject if the symmetry-reduced constraints are inconsistent.

---

## 30. Translation / rotation closure test

If the pattern claims to extend infinitely by translations, rotations, substitutions, or inflation rules, verify that the generation rule preserves all constraints.

For every generator (g) of the pattern:

[
c(gx)
]

must remain compatible with unit-distance constraints.

Reject if the finite seed is valid but the generated infinite pattern creates a same-color unit-distance conflict after repeated application.

---

## 31. Substitution consistency test

For recursive or substitution tilings, check that the substitution rule preserves legality at every level.

If a supertile is replaced by smaller tiles, verify:

1. Internal legality inside each substituted patch.
2. Boundary legality between adjacent substituted patches.
3. Legality across all substitution levels.
4. Color compatibility between parent and child structures.

Reject if legality holds at level (n) but not level (n+1).

---

## 32. Finite patch boundary-extension test

A finite patch that is 6-colorable may not extend to the infinite plane.

For a finite patch (P), test whether every boundary coloring extends to larger patches.

Reject or quarantine if:

[
P \text{ is colorable}
]

but no compatible coloring exists for a larger forced neighborhood.

This is useful for detecting locally pretty but globally impossible patterns.

---

## 33. Exhaustive local-neighborhood test

For each tile or point, generate its full unit-distance neighborhood within a bounded radius.

Check whether the proposed color assignment survives all forced local constraints.

This catches cases where the AI only checked visible neighbors but missed non-adjacent unit-distance interactions.

---

## 34. Known obstruction containment test

Maintain a library of known non-6-colorable or high-pressure subgraphs/subpatches.

For each new candidate, search for embeddings of known obstructions.

Reject immediately if a known obstruction appears.

Over time, this becomes one of the most valuable runtime accelerators.

---

## 35. Candidate novelty / non-isomorphism test

Before spending expensive compute, check whether the candidate conflict graph is isomorphic or nearly isomorphic to something already tested.

Reject or deprioritize if it is just a relabeled version of a known failure.

Use:

1. Graph canonicalization.
2. Weisfeiler–Lehman fingerprints.
3. Spectral fingerprints.
4. Symmetry signatures.
5. Geometric hash signatures.

This prevents the AI from rediscovering the same dead pattern repeatedly.

---
