use super::super::certification::{
    digest_parts, CanonicalCertificationRow, CertificationMatrix, RejectionCertificationRow,
};
use crate::facade::foundation::CorrespondenceHistoricalParityBundle;
use crate::harness::certification::RequiredAssertionClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CorrespondenceHistoryPerturbationClass {
    LineageAuthoritativeParity,
    StructuralAdvisoryBoundary,
    LineageStructuralDisagreement,
    StructuralAmbiguityBoundary,
    HistoricalRetainedPathParity,
    HistoricalReplayPathParity,
    HistoricalReconstructionPathParity,
    PredictionDriftExplicitness,
    StructuralAuthorityPromotionForbidden,
    AmbiguityCollapseForbidden,
    UnsupportedCorrespondenceFamily,
    UnsupportedHistoricalMaterializationPath,
    HiddenMaterializationSubstitutionForbidden,
    BroadCandidateScanForbidden,
    ExecutorPathMutationForbidden,
    HostCacheHistoryAuthorityForbidden,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CorrespondenceHistoryFailureClass {
    CorrespondenceDenied,
    HistoricalPathDenied,
    CompileFail,
}

impl CorrespondenceHistoryFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CorrespondenceDenied => "correspondence_denied",
            Self::HistoricalPathDenied => "historical_path_denied",
            Self::CompileFail => "compile_fail",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoryCertificationLane {
    pub parity_bundle: CorrespondenceHistoricalParityBundle,
}

impl CorrespondenceHistoryCertificationLane {
    pub fn has_required_outputs(&self) -> bool {
        !self.parity_bundle.query_digest().as_str().is_empty()
            && !self.parity_bundle.lineage_digest().as_str().is_empty()
            && !self.parity_bundle.basis_digest().as_str().is_empty()
            && !self
                .parity_bundle
                .counter_snapshot_digest()
                .as_str()
                .is_empty()
            && self
                .parity_bundle
                .result_digest()
                .map(|digest| !digest.as_str().is_empty())
                .unwrap_or(false)
            && self.parity_bundle.failure_digest().is_none()
    }

    pub fn has_zero_rediscovery(&self) -> bool {
        self.parity_bundle
            .performance_prediction_drift_outcome()
            .as_str()
            != "executor_rediscovery"
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoryCertificationRejection {
    pub failure_class: CorrespondenceHistoryFailureClass,
    pub failure_digest: String,
    pub counter_snapshot_digest: Option<String>,
    pub compile_fail_case: Option<&'static str>,
}

impl CorrespondenceHistoryCertificationRejection {
    pub fn has_required_outputs(&self) -> bool {
        !self.failure_digest.is_empty()
            && (!self
                .counter_snapshot_digest
                .as_ref()
                .map(|digest| digest.is_empty())
                .unwrap_or(true)
                || self.compile_fail_case.is_some())
    }
}

pub type CorrespondenceHistoryCertificationRow = CanonicalCertificationRow<
    CorrespondenceHistoryPerturbationClass,
    CorrespondenceHistoryCertificationLane,
>;
pub type CorrespondenceHistoryRejectionRow = RejectionCertificationRow<
    CorrespondenceHistoryPerturbationClass,
    CorrespondenceHistoryCertificationLane,
    CorrespondenceHistoryCertificationRejection,
>;
pub type CorrespondenceHistoryCertificationMatrix = CertificationMatrix<
    CorrespondenceHistoryPerturbationClass,
    CorrespondenceHistoryCertificationLane,
    CorrespondenceHistoryCertificationRejection,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoryBundleCompletenessReport {
    pub canonical_row_count: usize,
    pub rejection_row_count: usize,
    pub all_lanes_emit_required_outputs: bool,
    pub zero_rediscovery_lane_count: usize,
    pub unmet_required_rows: Vec<&'static str>,
    pub unmet_required_assertion_classes: Vec<RequiredAssertionClass>,
    pub offline_analysis_ready: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneFivePointFourCorrespondenceHistoryCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub bundle_completeness_report: CorrespondenceHistoryBundleCompletenessReport,
    pub matrix: CorrespondenceHistoryCertificationMatrix,
}

impl CorrespondenceHistoryCertificationMatrix {
    pub fn into_milestone_five_point_four_artifact(
        self,
        bundle_completeness_report: CorrespondenceHistoryBundleCompletenessReport,
    ) -> MilestoneFivePointFourCorrespondenceHistoryCertificationArtifact {
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));

        MilestoneFivePointFourCorrespondenceHistoryCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            bundle_completeness_report,
            matrix: self,
        }
    }
}

fn bundle_digest_parts(matrix: &CorrespondenceHistoryCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    for row in &matrix.rows {
        parts.push(format!("canonical:{}", row.row_name));
        parts.extend(lane_digest_parts(&row.control_lane, "control"));
        parts.extend(lane_digest_parts(&row.hostile_lane, "hostile"));
        parts.extend(lane_digest_parts(&row.parity_lane, "parity"));
    }
    for row in &matrix.rejection_rows {
        parts.push(format!("rejection:{}", row.row_name));
        parts.extend(lane_digest_parts(&row.control_lane, "control"));
        parts.push(format!(
            "hostile.failure_class:{}",
            row.hostile_lane.failure_class.as_str()
        ));
        parts.push(format!(
            "hostile.failure_digest:{}",
            row.hostile_lane.failure_digest
        ));
        if let Some(counter_snapshot_digest) = &row.hostile_lane.counter_snapshot_digest {
            parts.push(format!(
                "hostile.counter_snapshot_digest:{counter_snapshot_digest}"
            ));
        }
        if let Some(case) = row.hostile_lane.compile_fail_case {
            parts.push(format!("hostile.compile_fail_case:{case}"));
        }
        parts.extend(lane_digest_parts(&row.parity_lane, "parity"));
    }
    parts
}

fn coverage_digest_parts(matrix: &CorrespondenceHistoryCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    parts.push(format!("canonical_count:{}", matrix.rows.len()));
    parts.push(format!("rejection_count:{}", matrix.rejection_rows.len()));
    for row in &matrix.rows {
        parts.push(format!("canonical_row:{}", row.row_name));
    }
    for row in &matrix.rejection_rows {
        parts.push(format!("rejection_row:{}", row.row_name));
    }
    parts
}

fn lane_digest_parts(lane: &CorrespondenceHistoryCertificationLane, prefix: &str) -> Vec<String> {
    let bundle = &lane.parity_bundle;
    let mut parts = vec![
        format!("{prefix}.variant:{}", bundle.parity_variant().as_str()),
        format!("{prefix}.query_digest:{}", bundle.query_digest().as_str()),
        format!(
            "{prefix}.lineage_digest:{}",
            bundle.lineage_digest().as_str()
        ),
        format!("{prefix}.basis_digest:{}", bundle.basis_digest().as_str()),
        format!(
            "{prefix}.correspondence_outcome_digest:{}",
            bundle.correspondence_outcome_digest().as_str()
        ),
        format!(
            "{prefix}.correspondence_cost_posture_digest:{}",
            bundle.correspondence_cost_posture_digest().as_str()
        ),
        format!(
            "{prefix}.counter_snapshot_digest:{}",
            bundle.counter_snapshot_digest().as_str()
        ),
        format!(
            "{prefix}.prediction_drift:{}",
            bundle.performance_prediction_drift_outcome().as_str()
        ),
    ];
    if let Some(result_digest) = bundle.result_digest() {
        parts.push(format!("{prefix}.result_digest:{}", result_digest.as_str()));
    }
    if let Some(failure_digest) = bundle.failure_digest() {
        parts.push(format!(
            "{prefix}.failure_digest:{}",
            failure_digest.as_str()
        ));
    }
    if let Some(requested_path_digest) = bundle.requested_path_digest() {
        parts.push(format!(
            "{prefix}.requested_path_digest:{}",
            requested_path_digest.as_str()
        ));
    }
    if let Some(admitted_path_digest) = bundle.admitted_path_digest() {
        parts.push(format!(
            "{prefix}.admitted_path_digest:{}",
            admitted_path_digest.as_str()
        ));
    }
    if let Some(resolved_path_digest) = bundle.resolved_path_digest() {
        parts.push(format!(
            "{prefix}.resolved_path_digest:{}",
            resolved_path_digest.as_str()
        ));
    }
    if let Some(historical_cost_posture_digest) = bundle.historical_cost_posture_digest() {
        parts.push(format!(
            "{prefix}.historical_cost_posture_digest:{}",
            historical_cost_posture_digest.as_str()
        ));
    }
    parts
}
