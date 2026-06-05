use super::{
    CandidateScreeningApplicability, CandidateScreeningInvariantAuthority,
    CandidateScreeningInvariantFamily,
};

#[derive(Clone, Copy)]
pub(crate) struct CandidateScreeningInvariantDefinition {
    pub(crate) family: CandidateScreeningInvariantFamily,
    pub(crate) key: &'static str,
    pub(crate) title: &'static str,
    pub(crate) authority: CandidateScreeningInvariantAuthority,
    pub(crate) applicability: CandidateScreeningApplicability,
    pub(crate) statement: &'static str,
    pub(crate) rejection_condition: &'static str,
    pub(crate) promotion_requirement: &'static str,
}

pub(crate) const ALL_SCREENING_FAMILIES: [CandidateScreeningInvariantFamily; 35] = [
    CandidateScreeningInvariantFamily::ExactUnitDistanceConflict,
    CandidateScreeningInvariantFamily::TileDiameterSafety,
    CandidateScreeningInvariantFamily::SameColorSeparationDistanceSet,
    CandidateScreeningInvariantFamily::BoundaryOwnership,
    CandidateScreeningInvariantFamily::ExactConflictGraphConstruction,
    CandidateScreeningInvariantFamily::CliqueNumberLowerBound,
    CandidateScreeningInvariantFamily::IndependenceNumberLowerBound,
    CandidateScreeningInvariantFamily::WeightedIndependenceNumberBound,
    CandidateScreeningInvariantFamily::HallRatioSubpatchIndependenceBound,
    CandidateScreeningInvariantFamily::FractionalChromaticNumber,
    CandidateScreeningInvariantFamily::LovaszThetaBound,
    CandidateScreeningInvariantFamily::SpectralHoffmanBound,
    CandidateScreeningInvariantFamily::DegeneracyKCoreFilter,
    CandidateScreeningInvariantFamily::MaximumDegreeSanityCheck,
    CandidateScreeningInvariantFamily::PerfectGraphSanityCheck,
    CandidateScreeningInvariantFamily::SatIlpSixColorability,
    CandidateScreeningInvariantFamily::CriticalSubgraphExtraction,
    CandidateScreeningInvariantFamily::PeriodicQuotientGraph,
    CandidateScreeningInvariantFamily::ForbiddenDisplacementSet,
    CandidateScreeningInvariantFamily::MinkowskiDifferenceGeometry,
    CandidateScreeningInvariantFamily::AutocorrelationZero,
    CandidateScreeningInvariantFamily::DensityCapEachColorClass,
    CandidateScreeningInvariantFamily::LocalDensityWindow,
    CandidateScreeningInvariantFamily::UnitDistanceEmbeddability,
    CandidateScreeningInvariantFamily::RigidityRealizationConsistency,
    CandidateScreeningInvariantFamily::NumericalMargin,
    CandidateScreeningInvariantFamily::ExactArithmeticIntervalCertificate,
    CandidateScreeningInvariantFamily::MonodromyColorHolonomy,
    CandidateScreeningInvariantFamily::SymmetryOrbitReduction,
    CandidateScreeningInvariantFamily::TranslationRotationClosure,
    CandidateScreeningInvariantFamily::SubstitutionConsistency,
    CandidateScreeningInvariantFamily::FinitePatchBoundaryExtension,
    CandidateScreeningInvariantFamily::ExhaustiveLocalNeighborhood,
    CandidateScreeningInvariantFamily::KnownObstructionContainment,
    CandidateScreeningInvariantFamily::CandidateNoveltyNonIsomorphism,
];

pub(crate) fn invariant_definition(
    family: CandidateScreeningInvariantFamily,
) -> CandidateScreeningInvariantDefinition {
    use CandidateScreeningApplicability as A;
    use CandidateScreeningInvariantAuthority as T;
    use CandidateScreeningInvariantFamily as F;
    match family {
        F::ExactUnitDistanceConflict => def(family, "exact_unit_distance_conflict", "Exact unit-distance conflict test", T::ExactCheckerReady, A::RegionGeometry, "Reject same-color regions exactly when their distance set contains 1.", "1 in Delta(A,B), or a certified compact interval crosses 1.", "exact geometry or interval certificate"),
        F::TileDiameterSafety => def(family, "tile_diameter_safety", "Tile diameter safety test", T::CertificateRequired, A::RegionGeometry, "Every individual color region must avoid internal unit distances.", "tile diameter is at least 1 without an exact internal-distance clearance certificate", "exact diameter or internal-distance certificate"),
        F::SameColorSeparationDistanceSet => def(family, "same_color_separation_distance_set", "Same-color separation distance-set test", T::ExactCheckerReady, A::RegionGeometry, "Same-color rejection requires the exact distance set to contain 1, not only minimum distance <= 1.", "d_min <= 1 <= d_max for a certified connected compact pair", "exact distance-set certificate"),
        F::BoundaryOwnership => def(family, "boundary_ownership", "Boundary ownership test", T::CertificateRequired, A::RegionGeometry, "Every boundary point must have exactly one defined color unless overlap ownership is explicitly modeled.", "uncovered, ambiguously owned, or same-color unit-distance boundary points exist", "boundary ownership and boundary conflict certificate"),
        F::ExactConflictGraphConstruction => def(family, "exact_conflict_graph_construction", "Exact conflict graph construction", T::ExactCheckerReady, A::TileConflictGraph, "Conflict graph edges are certified by unit-distance possibility, not vague adjacency.", "an edge is missing or present contrary to 1 in Delta(T_i,T_j)", "exact tile-pair conflict certificate"),
        F::CliqueNumberLowerBound => def(family, "clique_number_lower_bound", "Clique-number lower bound", T::GraphTheoreticBound, A::FiniteConflictGraph, "Clique number lower-bounds chromatic number.", "omega(G) > 6 for a six-color candidate", "certified clique witness"),
        F::IndependenceNumberLowerBound => def(family, "independence_number_lower_bound", "Independence-number lower bound", T::GraphTheoreticBound, A::FiniteConflictGraph, "|V| / alpha(G) lower-bounds chromatic number.", "|V| / alpha(G) > 6", "certified maximum independent-set bound"),
        F::WeightedIndependenceNumberBound => def(family, "weighted_independence_number_bound", "Weighted independence-number bound", T::GraphTheoreticBound, A::TileConflictGraph, "Weighted independence catches unequal tile-density hiding.", "total weight / alpha_w(G) > 6", "certified weighted independent-set bound"),
        F::HallRatioSubpatchIndependenceBound => def(family, "hall_ratio_subpatch_independence_bound", "Hall-ratio subpatch independence bound", T::GraphTheoreticBound, A::FiniteConflictGraph, "Dense subgraphs can force more colors even when the whole graph looks mild.", "max_H |V(H)| / alpha(H) > 6, or weighted analogue", "certified dense subpatch witness"),
        F::FractionalChromaticNumber => def(family, "fractional_chromatic_number", "Fractional chromatic number", T::GraphTheoreticBound, A::FiniteConflictGraph, "Fractional chromatic number lower-bounds chromatic number.", "chi_f(G) > 6", "certified fractional-coloring lower bound"),
        F::LovaszThetaBound => def(family, "lovasz_theta_bound", "Lovasz theta bound", T::CertificateRequired, A::FiniteConflictGraph, "Lovasz theta of the complement can lower-bound chromatic number.", "theta(complement(G)) > 6", "semidefinite certificate or independently checked bound"),
        F::SpectralHoffmanBound => def(family, "spectral_hoffman_bound", "Spectral / Hoffman bound", T::GraphTheoreticBound, A::FiniteConflictGraph, "For regular or near-regular graphs, spectral bounds can certify color pressure.", "1 - d / lambda_min > 6 in the valid Hoffman regime", "checked eigenvalue and regularity certificate"),
        F::DegeneracyKCoreFilter => def(family, "degeneracy_k_core_filter", "Degeneracy / k-core filter", T::GraphTheoreticBound, A::FiniteConflictGraph, "A 5-degenerate graph is greedily 6-colorable, so only nonempty 6-cores deserve serious lower-bound work.", "the 6-core is empty for a claimed 7-obstruction priority lane", "deterministic k-core peel record"),
        F::MaximumDegreeSanityCheck => def(family, "maximum_degree_sanity_check", "Maximum-degree sanity check", T::HeuristicRanking, A::FiniteConflictGraph, "Low maximum degree without K7-type pressure is usually a poor 7-obstruction priority.", "Delta(G) <= 6 and no stronger obstruction witness is present", "ranking only; never proof authority"),
        F::PerfectGraphSanityCheck => def(family, "perfect_graph_sanity_check", "Perfect-graph sanity check", T::GraphTheoreticBound, A::FiniteConflictGraph, "Perfect graphs satisfy chi(G)=omega(G).", "G is perfect and omega(G) <= 6 for a claimed 7-lower-bound witness", "perfectness and clique certificate"),
        F::SatIlpSixColorability => def(family, "sat_ilp_six_colorability", "SAT / ILP 6-colorability test", T::ExactCheckerReady, A::FiniteConflictGraph, "Direct six-colorability encoding decides whether the finite conflict graph is 6-colorable.", "the checked SAT/ILP six-color instance is UNSAT", "model replay for SAT or checked refutation certificate for UNSAT"),
        F::CriticalSubgraphExtraction => def(family, "critical_subgraph_extraction", "Critical-subgraph extraction", T::DiscoverySupport, A::FiniteConflictGraph, "Non-6-colorable graphs should be minimized into reusable obstruction evidence.", "a smaller non-6-colorable subgraph exists or criticality is untested", "checked minimality or obstruction extraction record"),
        F::PeriodicQuotientGraph => def(family, "periodic_quotient_graph", "Periodic quotient graph test", T::ExactCheckerReady, A::PeriodicTiling, "Periodic tilings must include wraparound conflicts across lattice translations.", "the quotient graph with translated tile conflicts is not 6-colorable or contradicts the proposed coloring", "exact translated-pair conflict and quotient-coloring certificate"),
        F::ForbiddenDisplacementSet => def(family, "forbidden_displacement_set", "Forbidden displacement set", T::ExactCheckerReady, A::PeriodicTiling, "Repeated tile copies conflict by forbidden displacement, not center distance alone.", "a same-color displacement vector lies in F_P", "exact Minkowski/difference displacement certificate"),
        F::MinkowskiDifferenceGeometry => def(family, "minkowski_difference_geometry", "Minkowski-difference geometry test", T::ExactCheckerReady, A::RegionGeometry, "Two regions conflict iff their Minkowski difference intersects the unit circle.", "(A-B) intersects S^1", "exact region-difference intersection certificate"),
        F::AutocorrelationZero => def(family, "autocorrelation_zero", "Autocorrelation zero test", T::CertificateRequired, A::GeneratedPattern, "Each color class must have zero autocorrelation on every unit vector.", "area(C_i intersect (C_i+u)) > 0 for some |u|=1", "measure/raster certificate with exact or interval replay"),
        F::DensityCapEachColorClass => def(family, "density_cap_each_color_class", "Density cap for each color class", T::CertificateRequired, A::GeneratedPattern, "Each measurable 1-avoiding color class must respect the selected density upper bound.", "a color density exceeds the selected D_max", "named density theorem/bound and checked density estimate"),
        F::LocalDensityWindow => def(family, "local_density_window", "Local density-window test", T::CertificateRequired, A::GeneratedPattern, "Local windows can violate 1-avoiding density even when global density looks plausible.", "a window density exceeds the selected finite-window bound", "window bound provenance and checked local-density estimate"),
        F::UnitDistanceEmbeddability => def(family, "unit_distance_embeddability", "Unit-distance embeddability test", T::ExactCheckerReady, A::PointEmbedding, "Chromatic graphs matter only if they realize actual unit distances in the plane.", "any edge fails |p_i-p_j|^2=1, or optional non-edge exclusions fail", "exact coordinate and edge-distance certificate"),
        F::RigidityRealizationConsistency => def(family, "rigidity_realization_consistency", "Rigidity / realization consistency test", T::CertificateRequired, A::PointEmbedding, "Point candidates should classify realization as impossible, flexible, locally rigid, or globally rigid.", "distance constraints are impossible or certification status is too weak for proof use", "rigidity matrix, interval solving, or realization certificate"),
        F::NumericalMargin => def(family, "numerical_margin", "Numerical margin test", T::CertificateRequired, A::RegionGeometry, "Floating near misses are not proof; unresolved near-unit distances must be quarantined.", "same-color safety relies on floating distance without interval/exact clearance", "interval arithmetic or exact geometry margin certificate"),
        F::ExactArithmeticIntervalCertificate => def(family, "exact_arithmetic_interval_certificate", "Exact arithmetic / interval certificate test", T::ExactCheckerReady, A::RegionGeometry, "Final conflict and safety claims must replay without trusting floating point.", "a claimed safe or conflict pair lacks an exact/interval certificate", "exact arithmetic or interval certificate for every relevant pair"),
        F::MonodromyColorHolonomy => def(family, "monodromy_color_holonomy", "Monodromy / color-holonomy test", T::ExactCheckerReady, A::GeneratedPattern, "Closed loops of transformations must return compatible color permutations.", "a closed loop forces a tile/color to return with an incompatible permutation", "checked loop generator and permutation certificate"),
        F::SymmetryOrbitReduction => def(family, "symmetry_orbit_reduction", "Symmetry-orbit reduction test", T::DiscoverySupport, A::FiniteConflictGraph, "Symmetry quotients reduce search and expose hidden constraints.", "symmetry-reduced constraints are inconsistent or contradict full graph constraints", "group action, orbit, and stabilizer certificate"),
        F::TranslationRotationClosure => def(family, "translation_rotation_closure", "Translation / rotation closure test", T::ExactCheckerReady, A::GeneratedPattern, "Infinite extension generators must preserve all unit-distance constraints.", "a generated transform creates a same-color unit-distance conflict", "checked generator-closure certificate"),
        F::SubstitutionConsistency => def(family, "substitution_consistency", "Substitution consistency test", T::CertificateRequired, A::GeneratedPattern, "Recursive substitutions must preserve internal, boundary, and cross-level legality.", "legality holds at one level but fails at the next or parent-child colors are incompatible", "substitution-level replay certificate"),
        F::FinitePatchBoundaryExtension => def(family, "finite_patch_boundary_extension", "Finite patch boundary-extension test", T::DiscoverySupport, A::GeneratedPattern, "Finite colorable patches may fail to extend to forced neighborhoods.", "boundary colorings do not extend to required larger patches", "bounded extension search certificate"),
        F::ExhaustiveLocalNeighborhood => def(family, "exhaustive_local_neighborhood", "Exhaustive local-neighborhood test", T::ExactCheckerReady, A::PointEmbedding, "Visible neighbors are not enough; all bounded-radius unit-distance interactions must be checked.", "a generated local unit-distance neighbor has the same color", "bounded neighborhood generation certificate"),
        F::KnownObstructionContainment => def(family, "known_obstruction_containment", "Known obstruction containment test", T::DiscoverySupport, A::DiscoveryMemory, "Known non-6-colorable or high-pressure subgraphs should kill repeats early.", "a retained known obstruction embeds in the candidate", "typed obstruction library embedding certificate"),
        F::CandidateNoveltyNonIsomorphism => def(family, "candidate_novelty_non_isomorphism", "Candidate novelty / non-isomorphism test", T::DiscoverySupport, A::DiscoveryMemory, "Relabeled or near-isomorphic candidates should be rejected or deprioritized before expensive compute.", "canonical graph, WL, spectral, symmetry, or geometric signatures match retained work", "canonicalization/fingerprint comparison record"),
    }
}

fn def(
    family: CandidateScreeningInvariantFamily,
    key: &'static str,
    title: &'static str,
    authority: CandidateScreeningInvariantAuthority,
    applicability: CandidateScreeningApplicability,
    statement: &'static str,
    rejection_condition: &'static str,
    promotion_requirement: &'static str,
) -> CandidateScreeningInvariantDefinition {
    CandidateScreeningInvariantDefinition {
        family,
        key,
        title,
        authority,
        applicability,
        statement,
        rejection_condition,
        promotion_requirement,
    }
}
