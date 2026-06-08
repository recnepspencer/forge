mod definitions;
mod evaluation;
mod finite_graph_view;
mod fractional_chromatic_screening;
mod invariants;
mod lovasz_theta_screening;
mod operations;
mod optimization;
mod optimization_certificate_screening;

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

pub use evaluation::{
    CandidateScreeningCertificate, CandidateScreeningEvaluation, CandidateScreeningEvaluationMode,
    CandidateScreeningEvaluationReport, CandidateScreeningVerdict,
};
pub use fractional_chromatic_screening::{
    evaluate_fractional_chromatic_certificate_checked,
    evaluate_fractional_chromatic_screening_checked,
};
use invariants::{invariant_definition, ALL_SCREENING_FAMILIES};
pub use lovasz_theta_screening::{
    evaluate_lovasz_theta_certificate_checked, evaluate_lovasz_theta_screening_checked,
};
pub use operations::{
    assemble_candidate_screening_report_checked, evaluate_certificate_screening_invariant_checked,
    evaluate_graph_screening_invariant_checked, CandidateScreeningError,
};
pub use optimization::{
    AutocorrelationOverlapCertificate, DensityCapCertificate, FractionalChromaticCertificate,
    LocalDensityWindowCertificate, LovaszThetaCertificate, PeriodicColorClassMeasureModel,
    PeriodicMeasureCell, PeriodicMeasureWindow, ScreeningMatrixCertificate,
    ScreeningPsdWitnessCertificate, ScreeningRational, ScreeningSolverTranscript,
};
pub use optimization_certificate_screening::{
    evaluate_autocorrelation_zero_screening_checked, evaluate_density_cap_screening_checked,
    evaluate_local_density_window_screening_checked,
};

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

    pub fn all() -> &'static [Self; 35] {
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
