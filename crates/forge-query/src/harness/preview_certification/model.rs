use crate::preview::PreviewBindingCounters;

use super::super::certification::{
    digest_parts, CanonicalCertificationRow, CertificationMatrix, HostileExpectation,
    RejectionCertificationRow, RequiredAssertionClass,
};
use super::completeness::bundle_completeness_report;
use super::{PreviewCertificationLane, PreviewCertificationRejection, PreviewPerturbationClass};

pub type PreviewCertificationRow =
    CanonicalCertificationRow<PreviewPerturbationClass, PreviewCertificationLane>;
pub type PreviewRejectionRow = RejectionCertificationRow<
    PreviewPerturbationClass,
    PreviewCertificationLane,
    PreviewCertificationRejection,
>;
pub type PreviewCertificationMatrix = CertificationMatrix<
    PreviewPerturbationClass,
    PreviewCertificationLane,
    PreviewCertificationRejection,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewBundleCompletenessReport {
    pub canonical_row_count: usize,
    pub rejection_row_count: usize,
    pub supported_lane_count: usize,
    pub successful_lane_count: usize,
    pub zero_rediscovery_lane_count: usize,
    pub covered_perturbation_classes: Vec<PreviewPerturbationClass>,
    pub all_lanes_emit_required_outputs: bool,
    pub all_rows_have_hostile_coverage: bool,
    pub unmet_required_rows: Vec<&'static str>,
    pub unmet_required_assertion_classes: Vec<RequiredAssertionClass>,
    pub covers_all_currently_implemented_normative_scenarios: bool,
    pub covers_full_milestone_five_point_two_spec_matrix: bool,
    pub offline_analysis_ready: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneFivePointTwoPreviewCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub bundle_completeness_report: PreviewBundleCompletenessReport,
    pub counter_snapshot: PreviewBindingCounters,
    pub matrix: PreviewCertificationMatrix,
}

impl PreviewCertificationMatrix {
    pub fn into_milestone_five_point_two_artifact(
        self,
    ) -> MilestoneFivePointTwoPreviewCertificationArtifact {
        let bundle_completeness_report = bundle_completeness_report(&self);
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        let counter_snapshot = self.aggregate_counters();

        MilestoneFivePointTwoPreviewCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            bundle_completeness_report,
            counter_snapshot,
            matrix: self,
        }
    }

    fn aggregate_counters(&self) -> PreviewBindingCounters {
        let mut aggregate = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .fold(PreviewBindingCounters::default(), |mut aggregate, lane| {
                aggregate.absorb(&lane.counters);
                aggregate
            });

        for rejection in self
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.counters.as_ref())
        {
            aggregate.absorb(rejection);
        }

        aggregate
    }
}

impl PreviewCertificationRow {
    pub fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
    }

    pub fn has_hostile_coverage(&self) -> bool {
        match self.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                self.control_lane.query_digest == self.hostile_lane.query_digest
                    && self.control_lane.result_shape_digest
                        == self.hostile_lane.result_shape_digest
                    && self.control_lane.preview_session_identity
                        == self.hostile_lane.preview_session_identity
                    && self.control_lane.binding_digest == self.hostile_lane.binding_digest
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
                    && self.control_lane.binding_digest == self.parity_lane.binding_digest
            }
            HostileExpectation::DistinctFromControl => {
                (self.control_lane.evaluation_class != self.hostile_lane.evaluation_class
                    || self.control_lane.binding_digest != self.hostile_lane.binding_digest)
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
                    && self.control_lane.binding_digest == self.parity_lane.binding_digest
            }
        }
    }
}

impl PreviewRejectionRow {
    pub fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
    }

    pub fn has_hostile_coverage(&self) -> bool {
        self.control_lane.query_digest == self.parity_lane.query_digest
            && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
            && self.control_lane.binding_digest == self.parity_lane.binding_digest
    }
}

fn bundle_digest_parts(matrix: &PreviewCertificationMatrix) -> Vec<String> {
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
        parts.extend(rejection_digest_parts(
            &row.hostile_lane,
            "hostile_rejection",
        ));
        parts.extend(lane_digest_parts(&row.parity_lane, "parity"));
    }
    parts
}

fn coverage_digest_parts(matrix: &PreviewCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    parts.extend(
        matrix
            .rows
            .iter()
            .map(|row| format!("canonical:{}", row.row_name)),
    );
    parts.extend(
        matrix
            .rejection_rows
            .iter()
            .map(|row| format!("rejection:{}", row.row_name)),
    );
    parts
}

fn lane_digest_parts(bundle: &PreviewCertificationLane, label: &str) -> Vec<String> {
    let mut parts = vec![
        format!("{label}_query_digest:{}", bundle.query_digest),
        format!("{label}_result_shape_digest:{}", bundle.result_shape_digest),
        format!(
            "{label}_preview_session_identity:{}",
            bundle.preview_session_identity
        ),
        format!(
            "{label}_evaluation_class:{}",
            bundle.evaluation_class.as_str()
        ),
        format!(
            "{label}_lifecycle_state_kind:{}",
            bundle.lifecycle_state_kind.as_str()
        ),
        format!("{label}_binding_digest:{}", bundle.binding_digest),
    ];
    parts.extend(counter_digest_parts(&bundle.counters, label));
    parts
}

fn rejection_digest_parts(bundle: &PreviewCertificationRejection, label: &str) -> Vec<String> {
    let mut parts = vec![format!(
        "{label}_failure_class:{}",
        bundle.failure_class.as_str()
    )];
    if let Some(counters) = bundle.counters.as_ref() {
        parts.extend(counter_digest_parts(counters, label));
    }
    if let Some(case) = bundle.compile_fail_case {
        parts.push(format!("{label}_compile_fail_case:{case}"));
    }
    parts
}

fn counter_digest_parts(counters: &PreviewBindingCounters, label: &str) -> Vec<String> {
    vec![
        format!(
            "{label}_preview_session_admission_count:{}",
            counters.preview_session_admission_count()
        ),
        format!(
            "{label}_preview_basis_resolution_count:{}",
            counters.preview_basis_resolution_count()
        ),
        format!(
            "{label}_preview_lifecycle_lookup_count:{}",
            counters.preview_lifecycle_lookup_count()
        ),
        format!(
            "{label}_preview_lifecycle_rediscovery_count:{}",
            counters.preview_lifecycle_rediscovery_count()
        ),
        format!(
            "{label}_preview_invalid_basis_denial_count:{}",
            counters.preview_invalid_basis_denial_count()
        ),
        format!(
            "{label}_preview_invalid_lifecycle_denial_count:{}",
            counters.preview_invalid_lifecycle_denial_count()
        ),
        format!(
            "{label}_preview_broad_fallback_denial_count:{}",
            counters.preview_broad_fallback_denial_count()
        ),
        format!(
            "{label}_preview_executor_rediscovery_count:{}",
            counters.preview_executor_rediscovery_count()
        ),
    ]
}
