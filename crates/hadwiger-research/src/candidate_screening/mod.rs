mod autocorrelation_zero_screening;
mod boundary_ownership_screening;
mod candidate_novelty_screening;
mod definitions;
mod density_cap_screening;
mod evaluation;
mod exact_arithmetic_interval_screening;
mod exact_conflict_graph_screening;
mod exact_unit_distance_conflict_screening;
mod exhaustive_local_neighborhood_screening;
mod finite_graph_view;
mod finite_patch_boundary_extension_screening;
mod forbidden_displacement_screening;
mod fractional_chromatic_screening;
mod graph_embedding_index;
mod graph_embedding_screening_support;
mod invariants;
mod known_obstruction_containment_screening;
mod local_density_window_screening;
mod lovasz_theta_screening;
mod maximum_degree_sanity_advisory;
mod minkowski_difference_screening;
mod monodromy_color_holonomy_screening;
mod numerical_margin_screening;
mod operations;
mod optimization;
mod periodic_quotient_graph_screening;
mod rectangular_screening_support;
mod rigidity_realization_screening;
mod same_color_separation_screening;
mod substitution_consistency_screening;
mod symmetry_orbit_reduction_screening;
mod tile_diameter_screening;
mod translation_rotation_closure_screening;
mod unit_distance_embeddability_screening;

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

pub use autocorrelation_zero_screening::evaluate_autocorrelation_zero_screening_checked;
pub use boundary_ownership_screening::evaluate_boundary_ownership_screening_checked;
pub use candidate_novelty_screening::evaluate_candidate_novelty_screening_checked;
pub use density_cap_screening::evaluate_density_cap_screening_checked;
pub use evaluation::{
    CandidateScreeningCertificate, CandidateScreeningEvaluation, CandidateScreeningEvaluationMode,
    CandidateScreeningEvaluationReport, CandidateScreeningVerdict,
};
pub use exact_arithmetic_interval_screening::evaluate_exact_arithmetic_interval_screening_checked;
pub use exact_conflict_graph_screening::evaluate_exact_conflict_graph_screening_checked;
pub use exact_unit_distance_conflict_screening::evaluate_exact_unit_distance_conflict_screening_checked;
pub use exhaustive_local_neighborhood_screening::evaluate_exhaustive_local_neighborhood_screening_checked;
pub use finite_patch_boundary_extension_screening::evaluate_finite_patch_boundary_extension_screening_checked;
pub use forbidden_displacement_screening::evaluate_forbidden_displacement_screening_checked;
pub use fractional_chromatic_screening::{
    evaluate_fractional_chromatic_certificate_checked,
    evaluate_fractional_chromatic_screening_checked,
};
pub(crate) use graph_embedding_index::ScreeningFiniteGraphIndex;
use invariants::{invariant_definition, ALL_SCREENING_FAMILIES};
pub use known_obstruction_containment_screening::evaluate_known_obstruction_containment_screening_checked;
pub use local_density_window_screening::evaluate_local_density_window_screening_checked;
pub use lovasz_theta_screening::{
    evaluate_lovasz_theta_certificate_checked, evaluate_lovasz_theta_screening_checked,
};
pub use maximum_degree_sanity_advisory::{
    advise_maximum_degree_sanity_checked, CandidateScreeningAdvisoryArtifact,
    CandidateScreeningAdvisoryContributionRecord, CandidateScreeningAdvisoryPosture,
};
pub use minkowski_difference_screening::evaluate_minkowski_difference_screening_checked;
pub use monodromy_color_holonomy_screening::evaluate_monodromy_color_holonomy_screening_checked;
pub use numerical_margin_screening::evaluate_numerical_margin_screening_checked;
pub use operations::{
    assemble_candidate_screening_report_checked, evaluate_certificate_screening_invariant_checked,
    evaluate_graph_screening_invariant_checked, CandidateScreeningError,
};
pub use optimization::{
    AutocorrelationOverlapCertificate, BoundaryOwnedRegion, BoundaryOwnershipCertificate,
    CandidateNoveltyCertificate, ColorPermutation, DensityCapCertificate,
    ExactArithmeticIntervalCertificate, ExactArithmeticIntervalExpectation,
    ExactConflictGraphEdgeCertificate, ExactUnitDistanceConflictCertificate,
    ExhaustiveLocalNeighborhoodCertificate, FinitePatchBoundaryExtensionCertificate,
    ForbiddenDisplacementCertificate, FractionalChromaticCertificate,
    KnownObstructionContainmentCertificate, LocalDensityWindowCertificate, LovaszThetaCertificate,
    MinkowskiUnitIntersectionCertificate, MonodromyColorHolonomyCertificate,
    NumericalMarginCertificate, PeriodicColorClassMeasureModel, PeriodicMeasureCell,
    PeriodicMeasureWindow, PeriodicQuotientConflictCertificate, PeriodicQuotientRectangleModel,
    PeriodicQuotientTile, RigidityRealizationCertificate, RigidityRealizationPosture,
    SameColorSeparationCertificate, ScreeningMatrixCertificate, ScreeningPsdWitnessCertificate,
    ScreeningRational, ScreeningRectangularRegion, ScreeningSolverTranscript,
    SubstitutionConsistencyCertificate, SubstitutionConsistencyFailureKind,
    SymmetryOrbitReductionCertificate, TileDiameterCertificate,
    TranslationRotationClosureCertificate, UnitDistanceEmbeddabilityCertificate,
};
pub use periodic_quotient_graph_screening::evaluate_periodic_quotient_graph_screening_checked;
pub use rigidity_realization_screening::evaluate_rigidity_realization_screening_checked;
pub use same_color_separation_screening::evaluate_same_color_separation_screening_checked;
pub use substitution_consistency_screening::evaluate_substitution_consistency_screening_checked;
pub use symmetry_orbit_reduction_screening::evaluate_symmetry_orbit_reduction_screening_checked;
pub use tile_diameter_screening::evaluate_tile_diameter_screening_checked;
pub use translation_rotation_closure_screening::evaluate_translation_rotation_closure_screening_checked;
pub use unit_distance_embeddability_screening::evaluate_unit_distance_embeddability_screening_checked;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CandidateScreeningInvariantFamily {
    ExactUnitDistanceConflict,
    TileDiameterSafety,
    SameColorSeparationDistanceSet,
    BoundaryOwnership,
    ExactConflictGraphConstruction,
    CliqueNumberLowerBound,
    IndependenceNumberLowerBound,
    WeightedIndependenceNumberBound,
    HallRatioSubpatchIndependenceBound,
    FractionalChromaticNumber,
    LovaszThetaBound,
    SpectralHoffmanBound,
    DegeneracyKCoreFilter,
    MaximumDegreeSanityCheck,
    PerfectGraphSanityCheck,
    SatIlpSixColorability,
    CriticalSubgraphExtraction,
    PeriodicQuotientGraph,
    ForbiddenDisplacementSet,
    MinkowskiDifferenceGeometry,
    AutocorrelationZero,
    DensityCapEachColorClass,
    LocalDensityWindow,
    UnitDistanceEmbeddability,
    RigidityRealizationConsistency,
    NumericalMargin,
    ExactArithmeticIntervalCertificate,
    MonodromyColorHolonomy,
    SymmetryOrbitReduction,
    TranslationRotationClosure,
    SubstitutionConsistency,
    FinitePatchBoundaryExtension,
    ExhaustiveLocalNeighborhood,
    KnownObstructionContainment,
    CandidateNoveltyNonIsomorphism,
}

impl CandidateScreeningInvariantFamily {
    pub fn as_str(self) -> &'static str {
        invariant_definition(self).key
    }

    pub fn title(self) -> &'static str {
        invariant_definition(self).title
    }

    pub fn all() -> &'static [Self; 34] {
        &ALL_SCREENING_FAMILIES
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CandidateScreeningInvariantAuthority {
    ExactCheckerReady,
    CertificateRequired,
    GraphTheoreticBound,
    HeuristicRanking,
    DiscoverySupport,
}

impl CandidateScreeningInvariantAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactCheckerReady => "exact_checker_ready",
            Self::CertificateRequired => "certificate_required",
            Self::GraphTheoreticBound => "graph_theoretic_bound",
            Self::HeuristicRanking => "heuristic_ranking",
            Self::DiscoverySupport => "discovery_support",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CandidateScreeningApplicability {
    RegionGeometry,
    TileConflictGraph,
    FiniteConflictGraph,
    PeriodicTiling,
    PointEmbedding,
    GeneratedPattern,
    DiscoveryMemory,
}

impl CandidateScreeningApplicability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RegionGeometry => "region_geometry",
            Self::TileConflictGraph => "tile_conflict_graph",
            Self::FiniteConflictGraph => "finite_conflict_graph",
            Self::PeriodicTiling => "periodic_tiling",
            Self::PointEmbedding => "point_embedding",
            Self::GeneratedPattern => "generated_pattern",
            Self::DiscoveryMemory => "discovery_memory",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateScreeningInvariantNode {
    core: HadwigerArtifactCore,
    family: CandidateScreeningInvariantFamily,
    authority: CandidateScreeningInvariantAuthority,
    applicability: CandidateScreeningApplicability,
    statement: &'static str,
    rejection_condition: &'static str,
    promotion_requirement: &'static str,
}

impl CandidateScreeningInvariantNode {
    fn new(
        definition: definitions::CandidateScreeningInvariantDefinition,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let core = artifact_core(
            HadwigerArtifactKind::CandidateScreeningInvariantNode,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "candidate_screening_invariant_node".to_string(),
            },
            Vec::new(),
            node_payload(definition),
        )?;
        Ok(Self {
            core,
            family: definition.family,
            authority: definition.authority,
            applicability: definition.applicability,
            statement: definition.statement,
            rejection_condition: definition.rejection_condition,
            promotion_requirement: definition.promotion_requirement,
        })
    }

    pub fn family(&self) -> CandidateScreeningInvariantFamily {
        self.family
    }

    pub fn authority(&self) -> CandidateScreeningInvariantAuthority {
        self.authority
    }

    pub fn applicability(&self) -> CandidateScreeningApplicability {
        self.applicability
    }

    pub fn statement(&self) -> &'static str {
        self.statement
    }

    pub fn rejection_condition(&self) -> &'static str {
        self.rejection_condition
    }

    pub fn promotion_requirement(&self) -> &'static str {
        self.promotion_requirement
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(CandidateScreeningInvariantNode, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateScreeningInvariantCatalog {
    core: HadwigerArtifactCore,
    nodes: Vec<CandidateScreeningInvariantNode>,
}

impl CandidateScreeningInvariantCatalog {
    fn new(
        handle_digest: &str,
        mut nodes: Vec<CandidateScreeningInvariantNode>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        nodes.sort_by_key(|node| node.family());
        let mut parents = nodes
            .iter()
            .map(CandidateScreeningInvariantNode::reference)
            .collect::<Vec<HadwigerArtifactReference>>();
        parents.sort_by_key(HadwigerArtifactReference::stable_token);
        let core = artifact_core(
            HadwigerArtifactKind::CandidateScreeningInvariantCatalog,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "candidate_screening_invariant_catalog".to_string(),
            },
            parents,
            catalog_payload(handle_digest, &nodes),
        )?;
        Ok(Self { core, nodes })
    }

    pub fn nodes(&self) -> &[CandidateScreeningInvariantNode] {
        &self.nodes
    }

    pub fn has_family(&self, family: CandidateScreeningInvariantFamily) -> bool {
        self.nodes.iter().any(|node| node.family() == family)
    }

    pub fn families(&self) -> Vec<CandidateScreeningInvariantFamily> {
        self.nodes
            .iter()
            .map(CandidateScreeningInvariantNode::family)
            .collect()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(CandidateScreeningInvariantCatalog, core);

pub fn draft_candidate_screening_invariant_catalog_checked(
    handle: &HadwigerResearchHandle,
) -> Result<CandidateScreeningInvariantCatalog, HadwigerArtifactShapeError> {
    let nodes = CandidateScreeningInvariantFamily::all()
        .iter()
        .copied()
        .map(|family| CandidateScreeningInvariantNode::new(invariant_definition(family)))
        .collect::<Result<Vec<_>, _>>()?;
    CandidateScreeningInvariantCatalog::new(handle.handle_identity_digest(), nodes)
}

fn node_payload(
    definition: definitions::CandidateScreeningInvariantDefinition,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.candidate_screening.v1"),
        HadwigerArtifactPayloadEntry::text("family", definition.key),
        HadwigerArtifactPayloadEntry::text("title", definition.title),
        HadwigerArtifactPayloadEntry::text("authority", definition.authority.as_str()),
        HadwigerArtifactPayloadEntry::text("applicability", definition.applicability.as_str()),
        HadwigerArtifactPayloadEntry::text("statement", definition.statement),
        HadwigerArtifactPayloadEntry::text("reject_if", definition.rejection_condition),
        HadwigerArtifactPayloadEntry::text("promotion_requires", definition.promotion_requirement),
    ]
}

fn catalog_payload(
    handle_digest: &str,
    nodes: &[CandidateScreeningInvariantNode],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.screening_catalog.v1"),
        HadwigerArtifactPayloadEntry::text("handle_digest", handle_digest),
        HadwigerArtifactPayloadEntry::unsigned("node_count", nodes.len() as u128),
    ];
    for node in nodes {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "node",
            node.reference().stable_token(),
        ));
    }
    payload
}
