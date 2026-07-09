use crate::facade::{CanonicalizationCounters, CanonicalizationReport};

use super::super::certification::{
    digest_parts, CanonicalCertificationRow as SharedCanonicalCertificationRow,
    CertificationMatrix as SharedCertificationMatrix, HostileExpectation,
    ParityAnchor as SharedParityAnchor,
    RejectionCertificationRow as SharedRejectionCertificationRow,
};
use super::super::profiles::CertificationProfile;
use super::completeness::bundle_completeness_report;
use super::digests::{bundle_digest_parts, coverage_digest_parts};

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

pub type HostileLaneExpectation = HostileExpectation;
pub type CertificationRow =
    SharedCanonicalCertificationRow<CertificationPerturbationClass, CanonicalCertificationBundle>;
pub type ParityAnchor = SharedParityAnchor;
pub type RejectionCertificationRow = SharedRejectionCertificationRow<
    CertificationPerturbationClass,
    CanonicalCertificationBundle,
    RejectionCertificationBundle,
>;
pub type CertificationMatrix = SharedCertificationMatrix<
    CertificationPerturbationClass,
    CanonicalCertificationBundle,
    RejectionCertificationBundle,
>;

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
    pub unmet_required_rows: Vec<&'static str>,
    pub unmet_required_assertion_classes: Vec<super::super::certification::RequiredAssertionClass>,
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

impl CertificationMatrix {
    pub fn into_milestone_one_artifact(self) -> MilestoneOneCertificationArtifact {
        let bundle_completeness_report = bundle_completeness_report(&self);
        let counter_snapshot = self.aggregate_counters();
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));

        MilestoneOneCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            bundle_completeness_report,
            counter_snapshot,
            matrix: self,
        }
    }

    pub(crate) fn aggregate_counters(&self) -> CanonicalizationCounters {
        self.rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .fold(
                CanonicalizationCounters::default(),
                |mut aggregate, lane| {
                    aggregate.raw_clause_count += lane.counter_snapshot.raw_clause_count;
                    aggregate.normalized_clause_count +=
                        lane.counter_snapshot.normalized_clause_count;
                    aggregate.projection_entry_count +=
                        lane.counter_snapshot.projection_entry_count;
                    aggregate.traversal_clause_count +=
                        lane.counter_snapshot.traversal_clause_count;
                    aggregate.result_shape_field_count +=
                        lane.counter_snapshot.result_shape_field_count;
                    aggregate.binding_descriptor_count +=
                        lane.counter_snapshot.binding_descriptor_count;
                    aggregate.query_deduplication_count +=
                        lane.counter_snapshot.query_deduplication_count;
                    aggregate.result_shape_deduplication_count +=
                        lane.counter_snapshot.result_shape_deduplication_count;
                    aggregate.canonicalization_warning_count +=
                        lane.counter_snapshot.canonicalization_warning_count;
                    aggregate.canonicalization_fallback_count +=
                        lane.counter_snapshot.canonicalization_fallback_count;
                    aggregate
                },
            )
    }
}

impl CanonicalCertificationBundle {
    pub(crate) fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.result_shape_digest.is_empty()
            && self.warning_count == self.canonicalization_report.warnings().len()
            && self.event_count == self.canonicalization_report.events().len()
            && self.canonicalization_report.identity_freeze().query_digest == self.query_digest
            && self
                .canonicalization_report
                .identity_freeze()
                .result_shape_digest
                == self.result_shape_digest
    }
}

impl CertificationRow {
    pub(crate) fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
    }

    pub(crate) fn has_hostile_coverage(&self) -> bool {
        let hostile_relation = match self.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                self.control_lane.query_digest == self.hostile_lane.query_digest
                    && self.control_lane.result_shape_digest
                        == self.hostile_lane.result_shape_digest
            }
            HostileExpectation::DistinctFromControl => {
                self.control_lane.query_digest != self.hostile_lane.query_digest
                    || self.control_lane.result_shape_digest
                        != self.hostile_lane.result_shape_digest
            }
        };

        let parity_relation = match self.parity_anchor {
            SharedParityAnchor::Control => {
                self.control_lane.query_digest == self.parity_lane.query_digest
                    && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
            }
            SharedParityAnchor::Hostile => {
                self.hostile_lane.query_digest == self.parity_lane.query_digest
                    && self.hostile_lane.result_shape_digest == self.parity_lane.result_shape_digest
            }
        };

        hostile_relation && parity_relation
    }
}

impl RejectionCertificationRow {
    pub(crate) fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
            && !self.hostile_lane.failure_class.is_empty()
            && !self.hostile_lane.failure_digest.is_empty()
    }

    pub(crate) fn has_hostile_coverage(&self) -> bool {
        self.control_lane.query_digest == self.parity_lane.query_digest
            && self.control_lane.result_shape_digest == self.parity_lane.result_shape_digest
    }
}
