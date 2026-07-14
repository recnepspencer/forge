mod autocorrelation_zero;
mod boundary_ownership;
mod candidate_novelty_non_isomorphism;
mod clique_number_lower_bound;
mod critical_subgraph_extraction;
mod degeneracy_k_core_filter;
mod density_cap_each_color_class;
mod exact_arithmetic_interval_certificate;
mod exact_conflict_graph_construction;
mod exact_unit_distance_conflict;
mod exhaustive_local_neighborhood;
mod finite_patch_boundary_extension;
mod forbidden_displacement_set;
mod fractional_chromatic_number;
mod geometric_fractional_chromatic_number;
mod hall_ratio_subpatch_independence_bound;
mod independence_number_lower_bound;
mod known_obstruction_containment;
mod local_density_window;
mod lovasz_theta_bound;
mod maximum_degree_sanity_check;
mod minkowski_difference_geometry;
mod monodromy_color_holonomy;
mod numerical_margin;
mod perfect_graph_sanity_check;
mod periodic_quotient_graph;
mod rigidity_realization_consistency;
mod same_color_separation_distance_set;
mod sat_ilp_six_colorability;
mod spectral_hoffman_bound;
mod substitution_consistency;
mod symmetry_orbit_reduction;
mod tile_diameter_safety;
mod translation_rotation_closure;
mod unit_distance_embeddability;
mod weighted_independence_number_bound;

use super::definitions::CandidateScreeningInvariantDefinition;
pub(super) use super::{
    CandidateScreeningApplicability as A, CandidateScreeningInvariantAuthority as T,
    CandidateScreeningInvariantFamily as F,
};

pub(crate) const ALL_SCREENING_FAMILIES: [F; 35] = [
    F::ExactUnitDistanceConflict,
    F::TileDiameterSafety,
    F::SameColorSeparationDistanceSet,
    F::BoundaryOwnership,
    F::ExactConflictGraphConstruction,
    F::CliqueNumberLowerBound,
    F::IndependenceNumberLowerBound,
    F::WeightedIndependenceNumberBound,
    F::HallRatioSubpatchIndependenceBound,
    F::FractionalChromaticNumber,
    F::GeometricFractionalChromaticNumber,
    F::LovaszThetaBound,
    F::SpectralHoffmanBound,
    F::DegeneracyKCoreFilter,
    F::PerfectGraphSanityCheck,
    F::SatIlpSixColorability,
    F::CriticalSubgraphExtraction,
    F::PeriodicQuotientGraph,
    F::ForbiddenDisplacementSet,
    F::MinkowskiDifferenceGeometry,
    F::AutocorrelationZero,
    F::DensityCapEachColorClass,
    F::LocalDensityWindow,
    F::UnitDistanceEmbeddability,
    F::RigidityRealizationConsistency,
    F::NumericalMargin,
    F::ExactArithmeticIntervalCertificate,
    F::MonodromyColorHolonomy,
    F::SymmetryOrbitReduction,
    F::TranslationRotationClosure,
    F::SubstitutionConsistency,
    F::FinitePatchBoundaryExtension,
    F::ExhaustiveLocalNeighborhood,
    F::KnownObstructionContainment,
    F::CandidateNoveltyNonIsomorphism,
];

pub(crate) fn invariant_definition(family: F) -> CandidateScreeningInvariantDefinition {
    match family {
        F::ExactUnitDistanceConflict => exact_unit_distance_conflict::definition(),
        F::TileDiameterSafety => tile_diameter_safety::definition(),
        F::SameColorSeparationDistanceSet => same_color_separation_distance_set::definition(),
        F::BoundaryOwnership => boundary_ownership::definition(),
        F::ExactConflictGraphConstruction => exact_conflict_graph_construction::definition(),
        F::CliqueNumberLowerBound => clique_number_lower_bound::definition(),
        F::IndependenceNumberLowerBound => independence_number_lower_bound::definition(),
        F::WeightedIndependenceNumberBound => weighted_independence_number_bound::definition(),
        F::HallRatioSubpatchIndependenceBound => {
            hall_ratio_subpatch_independence_bound::definition()
        }
        F::FractionalChromaticNumber => fractional_chromatic_number::definition(),
        F::GeometricFractionalChromaticNumber => {
            geometric_fractional_chromatic_number::definition()
        }
        F::LovaszThetaBound => lovasz_theta_bound::definition(),
        F::SpectralHoffmanBound => spectral_hoffman_bound::definition(),
        F::DegeneracyKCoreFilter => degeneracy_k_core_filter::definition(),
        F::MaximumDegreeSanityCheck => maximum_degree_sanity_check::definition(),
        F::PerfectGraphSanityCheck => perfect_graph_sanity_check::definition(),
        F::SatIlpSixColorability => sat_ilp_six_colorability::definition(),
        F::CriticalSubgraphExtraction => critical_subgraph_extraction::definition(),
        F::PeriodicQuotientGraph => periodic_quotient_graph::definition(),
        F::ForbiddenDisplacementSet => forbidden_displacement_set::definition(),
        F::MinkowskiDifferenceGeometry => minkowski_difference_geometry::definition(),
        F::AutocorrelationZero => autocorrelation_zero::definition(),
        F::DensityCapEachColorClass => density_cap_each_color_class::definition(),
        F::LocalDensityWindow => local_density_window::definition(),
        F::UnitDistanceEmbeddability => unit_distance_embeddability::definition(),
        F::RigidityRealizationConsistency => rigidity_realization_consistency::definition(),
        F::NumericalMargin => numerical_margin::definition(),
        F::ExactArithmeticIntervalCertificate => {
            exact_arithmetic_interval_certificate::definition()
        }
        F::MonodromyColorHolonomy => monodromy_color_holonomy::definition(),
        F::SymmetryOrbitReduction => symmetry_orbit_reduction::definition(),
        F::TranslationRotationClosure => translation_rotation_closure::definition(),
        F::SubstitutionConsistency => substitution_consistency::definition(),
        F::FinitePatchBoundaryExtension => finite_patch_boundary_extension::definition(),
        F::ExhaustiveLocalNeighborhood => exhaustive_local_neighborhood::definition(),
        F::KnownObstructionContainment => known_obstruction_containment::definition(),
        F::CandidateNoveltyNonIsomorphism => candidate_novelty_non_isomorphism::definition(),
    }
}

pub(super) fn definition_row(
    family: F,
    key: &'static str,
    title: &'static str,
    authority: T,
    applicability: A,
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
