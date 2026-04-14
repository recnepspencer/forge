use crate::facade::{CanonicalizationCounters, CanonicalizationReport};

use super::super::profiles::CertificationProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalCertificationBundle {
    pub profile: CertificationProfile,
    pub query_digest: String,
    pub result_shape_digest: String,
    pub canonicalization_report: CanonicalizationReport,
    pub warning_count: usize,
    pub event_count: usize,
    pub counter_snapshot: CanonicalizationCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectionCertificationBundle {
    pub profile: CertificationProfile,
    pub failure_class: String,
    pub failure_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CertificationPerturbationClass {
    ConstructionPath,
    ResultShapeComposition,
    BindingDescriptorVariation,
    Deduplication,
    MeaningChange,
    UnsupportedAuthoredForm,
    ForbiddenFallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostileLaneExpectation {
    EquivalentToControl,
    DistinctFromControl,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParityAnchor {
    Control,
    Hostile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificationRow {
    pub row_name: &'static str,
    pub perturbation_class: CertificationPerturbationClass,
    pub hostile_expectation: HostileLaneExpectation,
    pub parity_anchor: ParityAnchor,
    pub control_lane: CanonicalCertificationBundle,
    pub hostile_lane: CanonicalCertificationBundle,
    pub parity_lane: CanonicalCertificationBundle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectionCertificationRow {
    pub row_name: &'static str,
    pub perturbation_class: CertificationPerturbationClass,
    pub control_lane: CanonicalCertificationBundle,
    pub hostile_lane: RejectionCertificationBundle,
    pub parity_lane: CanonicalCertificationBundle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificationMatrix {
    pub suite_name: &'static str,
    pub rows: Vec<CertificationRow>,
    pub rejection_rows: Vec<RejectionCertificationRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificationBundleCompletenessReport {
    pub canonical_row_count: usize,
    pub rejection_row_count: usize,
    pub supported_lane_count: usize,
    pub successful_lane_count: usize,
    pub zero_fallback_lane_count: usize,
    pub covered_perturbation_classes: Vec<CertificationPerturbationClass>,
    pub all_lanes_emit_required_outputs: bool,
    pub all_rows_have_hostile_coverage: bool,
    pub covers_all_mutation_sensitivity_classes: bool,
    pub covers_all_milestone_one_normative_scenarios: bool,
    pub offline_analysis_ready: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneOneCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub bundle_completeness_report: CertificationBundleCompletenessReport,
    pub counter_snapshot: CanonicalizationCounters,
    pub matrix: CertificationMatrix,
}
