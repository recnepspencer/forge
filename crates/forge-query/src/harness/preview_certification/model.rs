use crate::preview::{
    PreviewBindingCounters, PreviewComparisonCounters, PreviewExecutionCounters,
    PreviewLiveCounters,
};

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
    pub preview_live_composition_admitted_by_design: bool,
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
    pub binding_counter_snapshot: PreviewBindingCounters,
    pub execution_counter_snapshot: PreviewExecutionCounters,
    pub comparison_counter_snapshot: PreviewComparisonCounters,
    pub preview_live_counter_snapshot: PreviewLiveCounters,
    pub matrix: PreviewCertificationMatrix,
}

impl PreviewCertificationMatrix {
    pub fn into_milestone_five_point_two_artifact(
        self,
    ) -> MilestoneFivePointTwoPreviewCertificationArtifact {
        let bundle_completeness_report = bundle_completeness_report(&self);
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        let (
            binding_counter_snapshot,
            execution_counter_snapshot,
            comparison_counter_snapshot,
            preview_live_counter_snapshot,
        ) = self.aggregate_counters();

        MilestoneFivePointTwoPreviewCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            bundle_completeness_report,
            binding_counter_snapshot,
            execution_counter_snapshot,
            comparison_counter_snapshot,
            preview_live_counter_snapshot,
            matrix: self,
        }
    }

    fn aggregate_counters(
        &self,
    ) -> (
        PreviewBindingCounters,
        PreviewExecutionCounters,
        PreviewComparisonCounters,
        PreviewLiveCounters,
    ) {
        let mut aggregate_binding = self
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
        let mut aggregate_execution = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .fold(
                PreviewExecutionCounters::default(),
                |mut aggregate, lane| {
                    aggregate.absorb(&lane.execution_counters);
                    aggregate
                },
            );
        let mut aggregate_comparison = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .filter_map(|lane| lane.comparison_counters.as_ref())
            .fold(
                PreviewComparisonCounters::default(),
                |mut aggregate, counters| {
                    aggregate.absorb(counters);
                    aggregate
                },
            );
        let mut aggregate_preview_live = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .filter_map(|lane| lane.preview_live_counters.as_ref())
            .fold(PreviewLiveCounters::default(), |mut aggregate, counters| {
                aggregate.absorb(counters);
                aggregate
            });

        for rejection in self
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.counters.as_ref())
        {
            aggregate_binding.absorb(rejection);
        }

        for rejection in self
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.execution_counters.as_ref())
        {
            aggregate_execution.absorb(rejection);
        }

        for rejection in self
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.comparison_counters.as_ref())
        {
            aggregate_comparison.absorb(rejection);
        }

        for rejection in self
            .rejection_rows
            .iter()
            .filter_map(|row| row.hostile_lane.preview_live_counters.as_ref())
        {
            aggregate_preview_live.absorb(rejection);
        }

        (
            aggregate_binding,
            aggregate_execution,
            aggregate_comparison,
            aggregate_preview_live,
        )
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
                    && self.control_lane.preview_execution_digest
                        == self.hostile_lane.preview_execution_digest
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
                    && self.control_lane.binding_digest == self.parity_lane.binding_digest
                    && self.control_lane.preview_execution_digest
                        == self.parity_lane.preview_execution_digest
                    && self.control_lane.preview_live_digest
                        == self.hostile_lane.preview_live_digest
                    && self.control_lane.preview_live_digest == self.parity_lane.preview_live_digest
                    && self.control_lane.preview_live_subscription_digest
                        == self.hostile_lane.preview_live_subscription_digest
                    && self.control_lane.preview_live_subscription_digest
                        == self.parity_lane.preview_live_subscription_digest
                    && self.control_lane.preview_live_family == self.hostile_lane.preview_live_family
                    && self.control_lane.preview_live_family == self.parity_lane.preview_live_family
            }
            HostileExpectation::DistinctFromControl => {
                ((self.control_lane.evaluation_class != self.hostile_lane.evaluation_class
                    || self.control_lane.binding_digest != self.hostile_lane.binding_digest
                    || self.control_lane.preview_execution_digest
                        != self.hostile_lane.preview_execution_digest)
                    || (self.control_lane.preview_live_digest.is_some()
                        && self.control_lane.preview_live_digest
                            != self.hostile_lane.preview_live_digest))
                    && self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
                    && self.control_lane.binding_digest == self.parity_lane.binding_digest
                    && self.control_lane.preview_execution_digest
                        == self.parity_lane.preview_execution_digest
                    && self.control_lane.preview_live_digest == self.parity_lane.preview_live_digest
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
            && self.control_lane.preview_execution_digest
                == self.parity_lane.preview_execution_digest
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
        format!(
            "{label}_preview_execution_digest:{}",
            bundle.preview_execution_digest
        ),
        format!(
            "{label}_comparison_eligibility_digest:{}",
            bundle.comparison_eligibility_digest
        ),
        format!(
            "{label}_workflow_foundation_digest:{}",
            bundle.workflow_foundation_digest
        ),
    ];
    if let Some(digest) = bundle.promotion_parity_digest.as_ref() {
        parts.push(format!("{label}_promotion_parity_digest:{digest}"));
    }
    if let Some(digest) = bundle.preview_live_digest.as_ref() {
        parts.push(format!("{label}_preview_live_digest:{digest}"));
    }
    if let Some(digest) = bundle.preview_live_subscription_digest.as_ref() {
        parts.push(format!("{label}_preview_live_subscription_digest:{digest}"));
    }
    if let Some(family) = bundle.preview_live_family.as_ref() {
        parts.push(format!("{label}_preview_live_family:{family}"));
    }
    parts.extend(counter_digest_parts(&bundle.counters, label));
    parts.extend(execution_counter_digest_parts(
        &bundle.execution_counters,
        label,
    ));
    if let Some(comparison_counters) = bundle.comparison_counters.as_ref() {
        parts.extend(comparison_counter_digest_parts(comparison_counters, label));
    }
    if let Some(preview_live_counters) = bundle.preview_live_counters.as_ref() {
        parts.extend(preview_live_counter_digest_parts(preview_live_counters, label));
    }
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
    if let Some(counters) = bundle.execution_counters.as_ref() {
        parts.extend(execution_counter_digest_parts(counters, label));
    }
    if let Some(counters) = bundle.comparison_counters.as_ref() {
        parts.extend(comparison_counter_digest_parts(counters, label));
    }
    if let Some(counters) = bundle.preview_live_counters.as_ref() {
        parts.extend(preview_live_counter_digest_parts(counters, label));
    }
    if let Some(case) = bundle.compile_fail_case {
        parts.push(format!("{label}_compile_fail_case:{case}"));
    }
    parts
}

fn execution_counter_digest_parts(counters: &PreviewExecutionCounters, label: &str) -> Vec<String> {
    vec![
        format!(
            "{label}_preview_execution_envelope_count:{}",
            counters.preview_execution_envelope_count()
        ),
        format!(
            "{label}_preview_execution_count:{}",
            counters.preview_execution_count()
        ),
        format!(
            "{label}_preview_promotable_execution_count:{}",
            counters.preview_promotable_execution_count()
        ),
        format!(
            "{label}_preview_read_only_execution_count:{}",
            counters.preview_read_only_execution_count()
        ),
        format!(
            "{label}_preview_comparison_eligibility_proof_count:{}",
            counters.preview_comparison_eligibility_proof_count()
        ),
        format!(
            "{label}_preview_comparison_shape_check_width:{}",
            counters.preview_comparison_shape_check_width()
        ),
        format!(
            "{label}_preview_workflow_foundation_artifact_lookup_count:{}",
            counters.preview_workflow_foundation_artifact_lookup_count()
        ),
        format!(
            "{label}_preview_workflow_foundation_admission_count:{}",
            counters.preview_workflow_foundation_admission_count()
        ),
        format!(
            "{label}_preview_workflow_foundation_denial_count:{}",
            counters.preview_workflow_foundation_denial_count()
        ),
        format!(
            "{label}_preview_work_avoided_by_explicit_basis_count:{}",
            counters.preview_work_avoided_by_explicit_basis_count()
        ),
    ]
}

fn comparison_counter_digest_parts(
    counters: &PreviewComparisonCounters,
    label: &str,
) -> Vec<String> {
    vec![
        format!(
            "{label}_preview_promotion_comparison_count:{}",
            counters.preview_promotion_comparison_count()
        ),
        format!(
            "{label}_preview_promotion_comparison_denial_count:{}",
            counters.preview_promotion_comparison_denial_count()
        ),
        format!(
            "{label}_preview_comparison_eligibility_proof_count:{}",
            counters.preview_comparison_eligibility_proof_count()
        ),
        format!(
            "{label}_preview_comparison_shape_check_width:{}",
            counters.preview_comparison_shape_check_width()
        ),
        format!(
            "{label}_preview_basis_pair_width:{}",
            counters.preview_basis_pair_width()
        ),
    ]
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
        format!(
            "{label}_preview_replay_bundle_lookup_count:{}",
            counters.preview_replay_bundle_lookup_count()
        ),
        format!(
            "{label}_preview_bridge_promotion_linkage_count:{}",
            counters.preview_bridge_promotion_linkage_count()
        ),
    ]
}

fn preview_live_counter_digest_parts(counters: &PreviewLiveCounters, label: &str) -> Vec<String> {
    vec![
        format!(
            "{label}_preview_live_admission_count:{}",
            counters.preview_live_admission_count()
        ),
        format!(
            "{label}_preview_live_execution_count:{}",
            counters.preview_live_execution_count()
        ),
        format!(
            "{label}_preview_live_lifecycle_check_count:{}",
            counters.preview_live_lifecycle_check_count()
        ),
        format!(
            "{label}_preview_live_drift_denial_count:{}",
            counters.preview_live_drift_denial_count()
        ),
        format!(
            "{label}_preview_live_rebind_available_count:{}",
            counters.preview_live_rebind_available_count()
        ),
        format!(
            "{label}_preview_live_broad_fallback_denial_count:{}",
            counters.preview_live_broad_fallback_denial_count()
        ),
    ]
}
