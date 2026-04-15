use crate::facade::{QueryValidationCounters, QueryValidationReport, ValidationRejectionMatrix};

use super::super::certification::{
    CanonicalCertificationRow, CertificationMatrix, HostileExpectation, ParityAnchor,
    RejectionCertificationRow, RequiredAssertionClass, digest_parts,
};
use super::super::profiles::CertificationProfile;
use super::completeness::bundle_completeness_report;
use super::digests::{bundle_digest_parts, coverage_digest_parts};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationCertificationBundle {
    pub profile: CertificationProfile,
    pub query_digest: String,
    pub validated_query_digest: String,
    pub validated_result_shape_digest: String,
    pub basis_digest: String,
    pub validation_report: QueryValidationReport,
    pub counter_snapshot: QueryValidationCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationRejectionCertificationBundle {
    pub profile: CertificationProfile,
    pub failure_class: String,
    pub failure_digest: String,
    pub validation_rejection_matrix: ValidationRejectionMatrix,
    pub counter_snapshot: QueryValidationCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ValidationPerturbationClass {
    ConstructionPath,
    SchemaBasisVariation,
    ProjectionLegality,
    OrderingLegality,
    PredicateLegality,
    TraversalLegality,
    ResultShapeBindingLegality,
    StructuredContentLegality,
    WorkflowContextLegality,
    ForbiddenWidening,
}

pub type ValidationHostileExpectation = HostileExpectation;
pub type ValidationParityAnchor = ParityAnchor;
pub type ValidationCertificationRow =
    CanonicalCertificationRow<ValidationPerturbationClass, ValidationCertificationBundle>;
pub type ValidationRejectionCertificationRow = RejectionCertificationRow<
    ValidationPerturbationClass,
    ValidationCertificationBundle,
    ValidationRejectionCertificationBundle,
>;
pub type ValidationCertificationMatrix = CertificationMatrix<
    ValidationPerturbationClass,
    ValidationCertificationBundle,
    ValidationRejectionCertificationBundle,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationBundleCompletenessReport {
    pub canonical_row_count: usize,
    pub rejection_row_count: usize,
    pub supported_lane_count: usize,
    pub successful_lane_count: usize,
    pub zero_fallback_lane_count: usize,
    pub covered_perturbation_classes: Vec<ValidationPerturbationClass>,
    pub all_lanes_emit_required_outputs: bool,
    pub all_rows_have_hostile_coverage: bool,
    pub unmet_required_rows: Vec<&'static str>,
    pub unmet_required_assertion_classes: Vec<RequiredAssertionClass>,
    pub covers_all_currently_implemented_normative_scenarios: bool,
    pub covers_full_milestone_two_spec_matrix: bool,
    pub offline_analysis_ready: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneTwoValidationCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub bundle_completeness_report: ValidationBundleCompletenessReport,
    pub counter_snapshot: QueryValidationCounters,
    pub matrix: ValidationCertificationMatrix,
}

impl ValidationCertificationMatrix {
    pub fn into_milestone_two_artifact(self) -> MilestoneTwoValidationCertificationArtifact {
        let bundle_completeness_report = bundle_completeness_report(&self);
        let counter_snapshot = self.aggregate_counters();
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));

        MilestoneTwoValidationCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            bundle_completeness_report,
            counter_snapshot,
            matrix: self,
        }
    }

    pub(crate) fn aggregate_counters(&self) -> QueryValidationCounters {
        self.rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .fold(QueryValidationCounters::default(), |mut aggregate, lane| {
                aggregate.absorb(&lane.counter_snapshot);
                aggregate
            })
    }
}

impl ValidationCertificationRow {
    pub(crate) fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
    }

    pub(crate) fn has_hostile_coverage(&self) -> bool {
        let hostile_relation = match self.hostile_expectation {
            HostileExpectation::EquivalentToControl => {
                self.control_lane.validated_query_digest == self.hostile_lane.validated_query_digest
                    && self.control_lane.validated_result_shape_digest
                        == self.hostile_lane.validated_result_shape_digest
            }
            HostileExpectation::DistinctFromControl => {
                self.control_lane.validated_query_digest != self.hostile_lane.validated_query_digest
                    || self.control_lane.validated_result_shape_digest
                        != self.hostile_lane.validated_result_shape_digest
            }
        };

        let parity_relation = match self.parity_anchor {
            ParityAnchor::Control => {
                self.control_lane.validated_query_digest == self.parity_lane.validated_query_digest
                    && self.control_lane.validated_result_shape_digest
                        == self.parity_lane.validated_result_shape_digest
            }
            ParityAnchor::Hostile => {
                self.hostile_lane.validated_query_digest == self.parity_lane.validated_query_digest
                    && self.hostile_lane.validated_result_shape_digest
                        == self.parity_lane.validated_result_shape_digest
            }
        };

        hostile_relation && parity_relation
    }
}

impl ValidationRejectionCertificationRow {
    pub(crate) fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
            && !self.hostile_lane.failure_class.is_empty()
            && !self.hostile_lane.failure_digest.is_empty()
    }

    pub(crate) fn has_hostile_coverage(&self) -> bool {
        self.control_lane.validated_query_digest == self.parity_lane.validated_query_digest
            && self.control_lane.validated_result_shape_digest
                == self.parity_lane.validated_result_shape_digest
    }
}

impl ValidationCertificationBundle {
    pub(crate) fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.validated_query_digest.is_empty()
            && !self.validated_result_shape_digest.is_empty()
            && !self.basis_digest.is_empty()
            && self.validation_report.schema_basis_digest() == self.basis_digest
            && self.counter_snapshot.validation_fallback_count() == 0
    }
}
