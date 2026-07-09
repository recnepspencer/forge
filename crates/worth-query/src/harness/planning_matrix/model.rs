use crate::execution::ExecutionCounters;

use super::super::certification::{
    digest_parts, CanonicalCertificationRow, CertificationMatrix, HostileExpectation,
    RejectionCertificationRow, RequiredAssertionClass,
};
use super::super::profiles::CertificationProfile;
use super::completeness::bundle_completeness_report;
use super::digests::{bundle_digest_parts, coverage_digest_parts};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningCertificationBundle {
    pub profile: CertificationProfile,
    pub query_digest: String,
    pub plan_digest: String,
    pub result_digest: String,
    pub basis_digest: String,
    pub counter_snapshot: ExecutionCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningRejectionBundle {
    pub profile: CertificationProfile,
    pub failure_class: String,
    pub failure_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlanningPerturbationClass {
    DirectRuntimeParity,
    ReplayParity,
    BindingParity,
    BasisRepeatability,
    BasisDifference,
    RouteSemanticDifference,
    BindingRejection,
    FallbackRejection,
    BasisResolutionFailure,
}

pub type PlanningHostileExpectation = HostileExpectation;
pub type PlanningCertificationRow =
    CanonicalCertificationRow<PlanningPerturbationClass, PlanningCertificationBundle>;
pub type PlanningRejectionRow = RejectionCertificationRow<
    PlanningPerturbationClass,
    PlanningCertificationBundle,
    PlanningRejectionBundle,
>;
pub type PlanningCertificationMatrix = CertificationMatrix<
    PlanningPerturbationClass,
    PlanningCertificationBundle,
    PlanningRejectionBundle,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningBundleCompletenessReport {
    pub canonical_row_count: usize,
    pub rejection_row_count: usize,
    pub supported_lane_count: usize,
    pub successful_lane_count: usize,
    pub zero_fallback_lane_count: usize,
    pub zero_rediscovery_lane_count: usize,
    pub covered_perturbation_classes: Vec<PlanningPerturbationClass>,
    pub all_lanes_emit_required_outputs: bool,
    pub all_rows_have_hostile_coverage: bool,
    pub unmet_required_rows: Vec<&'static str>,
    pub unmet_required_assertion_classes: Vec<RequiredAssertionClass>,
    pub covers_all_currently_implemented_normative_scenarios: bool,
    pub covers_full_milestone_three_spec_matrix: bool,
    pub offline_analysis_ready: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneThreePlanningCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub bundle_completeness_report: PlanningBundleCompletenessReport,
    pub counter_snapshot: ExecutionCounters,
    pub matrix: PlanningCertificationMatrix,
}

impl PlanningCertificationMatrix {
    pub fn into_milestone_three_artifact(self) -> MilestoneThreePlanningCertificationArtifact {
        let bundle_completeness_report = bundle_completeness_report(&self);
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        let counter_snapshot = self.aggregate_counters();

        MilestoneThreePlanningCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            bundle_completeness_report,
            counter_snapshot,
            matrix: self,
        }
    }

    fn aggregate_counters(&self) -> ExecutionCounters {
        self.rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .fold(ExecutionCounters::default(), |mut aggregate, lane| {
                aggregate.absorb(&lane.counter_snapshot);
                aggregate
            })
    }
}

impl PlanningCertificationBundle {
    pub fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.plan_digest.is_empty()
            && !self.result_digest.is_empty()
            && !self.basis_digest.is_empty()
            && self.counter_snapshot.executor_semantic_rediscovery_count() == 0
    }
}

impl PlanningCertificationRow {
    pub fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
    }

    pub fn has_hostile_coverage(&self) -> bool {
        match self.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                self.control_lane.plan_digest == self.hostile_lane.plan_digest
                    && self.control_lane.result_digest == self.hostile_lane.result_digest
                    && self.control_lane.basis_digest == self.hostile_lane.basis_digest
                    && self.control_lane.plan_digest == self.parity_lane.plan_digest
                    && self.control_lane.result_digest == self.parity_lane.result_digest
            }
            HostileExpectation::DistinctFromControl => {
                (self.control_lane.plan_digest != self.hostile_lane.plan_digest
                    || self.control_lane.result_digest != self.hostile_lane.result_digest
                    || self.control_lane.basis_digest != self.hostile_lane.basis_digest)
                    && self.control_lane.plan_digest == self.parity_lane.plan_digest
                    && self.control_lane.result_digest == self.parity_lane.result_digest
            }
        }
    }
}

impl PlanningRejectionRow {
    pub fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
            && !self.hostile_lane.failure_class.is_empty()
            && !self.hostile_lane.failure_digest.is_empty()
    }

    pub fn has_hostile_coverage(&self) -> bool {
        self.control_lane.plan_digest == self.parity_lane.plan_digest
            && self.control_lane.result_digest == self.parity_lane.result_digest
    }
}
