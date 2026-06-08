# Candidate Screening Invariants

This tracker is the single Markdown index for the Hadwiger-Nelson screening
invariants. Each invariant has exactly one owning Rust file under
`src/candidate_screening/invariants/`.

Status meanings:

- `native_exact`: implemented as a deterministic in-crate checker/evaluator.
- `native_bounded`: implemented for explicit bounded cases with typed budget
  stops.
- `native_special_case`: implemented for a mathematically sound subclass.
- `certificate_required`: operationalized through checked certificate intake;
  native robust math still needs implementation.
- `heuristic`: ranking/filter evidence only; never proof authority.

| # | Invariant | Code File | Current Status |
|---|---|---|---|
| 1 | Exact unit-distance conflict test | `src/candidate_screening/invariants/exact_unit_distance_conflict.rs` | `certificate_required` |
| 2 | Tile diameter safety test | `src/candidate_screening/invariants/tile_diameter_safety.rs` | `certificate_required` |
| 3 | Same-color separation / distance-set test | `src/candidate_screening/invariants/same_color_separation_distance_set.rs` | `certificate_required` |
| 4 | Boundary ownership test | `src/candidate_screening/invariants/boundary_ownership.rs` | `certificate_required` |
| 5 | Exact conflict graph construction | `src/candidate_screening/invariants/exact_conflict_graph_construction.rs` | `certificate_required` |
| 6 | Clique-number lower bound | `src/candidate_screening/invariants/clique_number_lower_bound.rs` | `native_bounded` |
| 7 | Independence-number lower bound | `src/candidate_screening/invariants/independence_number_lower_bound.rs` | `native_bounded` |
| 8 | Weighted independence-number bound | `src/candidate_screening/invariants/weighted_independence_number_bound.rs` | `native_special_case` |
| 9 | Hall-ratio / subpatch independence bound | `src/candidate_screening/invariants/hall_ratio_subpatch_independence_bound.rs` | `native_bounded` |
| 10 | Fractional chromatic number | `src/candidate_screening/invariants/fractional_chromatic_number.rs` | `certificate_required` |
| 11 | Lovasz theta bound | `src/candidate_screening/invariants/lovasz_theta_bound.rs` | `certificate_required` |
| 12 | Spectral / Hoffman bound | `src/candidate_screening/invariants/spectral_hoffman_bound.rs` | `native_special_case` |
| 13 | Degeneracy / k-core filter | `src/candidate_screening/invariants/degeneracy_k_core_filter.rs` | `native_exact` |
| 14 | Maximum-degree sanity check | `src/candidate_screening/invariants/maximum_degree_sanity_check.rs` | `heuristic` |
| 15 | Perfect-graph sanity check | `src/candidate_screening/invariants/perfect_graph_sanity_check.rs` | `native_special_case` |
| 16 | SAT / ILP 6-colorability test | `src/candidate_screening/invariants/sat_ilp_six_colorability.rs` | `native_bounded` |
| 17 | Critical-subgraph extraction | `src/candidate_screening/invariants/critical_subgraph_extraction.rs` | `native_bounded` |
| 18 | Periodic quotient graph test | `src/candidate_screening/invariants/periodic_quotient_graph.rs` | `certificate_required` |
| 19 | Forbidden displacement set | `src/candidate_screening/invariants/forbidden_displacement_set.rs` | `certificate_required` |
| 20 | Minkowski-difference geometry test | `src/candidate_screening/invariants/minkowski_difference_geometry.rs` | `certificate_required` |
| 21 | Autocorrelation zero test | `src/candidate_screening/invariants/autocorrelation_zero.rs` | `certificate_required` |
| 22 | Density cap for each color class | `src/candidate_screening/invariants/density_cap_each_color_class.rs` | `certificate_required` |
| 23 | Local density-window test | `src/candidate_screening/invariants/local_density_window.rs` | `certificate_required` |
| 24 | Unit-distance embeddability test | `src/candidate_screening/invariants/unit_distance_embeddability.rs` | `certificate_required` |
| 25 | Rigidity / realization consistency test | `src/candidate_screening/invariants/rigidity_realization_consistency.rs` | `certificate_required` |
| 26 | Numerical margin test | `src/candidate_screening/invariants/numerical_margin.rs` | `certificate_required` |
| 27 | Exact arithmetic / interval certificate test | `src/candidate_screening/invariants/exact_arithmetic_interval_certificate.rs` | `certificate_required` |
| 28 | Monodromy / color-holonomy test | `src/candidate_screening/invariants/monodromy_color_holonomy.rs` | `certificate_required` |
| 29 | Symmetry-orbit reduction test | `src/candidate_screening/invariants/symmetry_orbit_reduction.rs` | `certificate_required` |
| 30 | Translation / rotation closure test | `src/candidate_screening/invariants/translation_rotation_closure.rs` | `certificate_required` |
| 31 | Substitution consistency test | `src/candidate_screening/invariants/substitution_consistency.rs` | `certificate_required` |
| 32 | Finite patch boundary-extension test | `src/candidate_screening/invariants/finite_patch_boundary_extension.rs` | `certificate_required` |
| 33 | Exhaustive local-neighborhood test | `src/candidate_screening/invariants/exhaustive_local_neighborhood.rs` | `certificate_required` |
| 34 | Known obstruction containment test | `src/candidate_screening/invariants/known_obstruction_containment.rs` | `certificate_required` |
| 35 | Candidate novelty / non-isomorphism test | `src/candidate_screening/invariants/candidate_novelty_non_isomorphism.rs` | `certificate_required` |

Next implementation pressure should go first to exact geometry ownership
families, then finite graph optimization families, then periodic/generated
pattern families.
