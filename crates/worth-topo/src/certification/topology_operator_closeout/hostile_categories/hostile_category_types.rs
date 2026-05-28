use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MilestoneThreeHostileCertificationCategory {
    MutationPipelineIntegrity,
    PrimitiveTopologyFamilyClosure,
    OperatorBrutality,
    QueryTraversalBrutality,
    NonManifoldRadialBrutality,
    DegeneracyCorruptionLocalization,
    DeterminismOrderAssault,
    DiagnosticsFailureTaxonomy,
    ScaleDepthSustainedPressure,
}

impl MilestoneThreeHostileCertificationCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MutationPipelineIntegrity => "mutation_pipeline_integrity",
            Self::PrimitiveTopologyFamilyClosure => "primitive_topology_family_closure",
            Self::OperatorBrutality => "operator_brutality",
            Self::QueryTraversalBrutality => "query_traversal_brutality",
            Self::NonManifoldRadialBrutality => "non_manifold_radial_brutality",
            Self::DegeneracyCorruptionLocalization => "degeneracy_corruption_localization",
            Self::DeterminismOrderAssault => "determinism_order_assault",
            Self::DiagnosticsFailureTaxonomy => "diagnostics_failure_taxonomy",
            Self::ScaleDepthSustainedPressure => "scale_depth_sustained_pressure",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MilestoneThreeHostileCertificationStatus {
    Certified,
    Partial,
}

impl MilestoneThreeHostileCertificationStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Partial => "partial",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileCertificationCategoryRow {
    pub(crate) category: MilestoneThreeHostileCertificationCategory,
    pub(crate) status: MilestoneThreeHostileCertificationStatus,
    pub(crate) scenario_count: usize,
    pub(crate) evidence_count: usize,
    pub(crate) replay_verified_count: usize,
    pub(crate) diagnostic_locality_count: usize,
    pub(crate) evidence_labels: Vec<String>,
    pub(crate) gap_labels: Vec<String>,
    pub(crate) row_digest: String,
}




