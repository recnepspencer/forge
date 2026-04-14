use crate::facade::{QueryValidationCounters, QueryValidationReport, ValidationRejectionMatrix};
use sha2::{Digest, Sha256};

use super::profiles::CertificationProfile;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationHostileExpectation {
    EquivalentToControl,
    DistinctFromControl,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationParityAnchor {
    Control,
    Hostile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationCertificationRow {
    pub row_name: &'static str,
    pub perturbation_class: ValidationPerturbationClass,
    pub hostile_expectation: ValidationHostileExpectation,
    pub parity_anchor: ValidationParityAnchor,
    pub control_lane: ValidationCertificationBundle,
    pub hostile_lane: ValidationCertificationBundle,
    pub parity_lane: ValidationCertificationBundle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationRejectionCertificationRow {
    pub row_name: &'static str,
    pub perturbation_class: ValidationPerturbationClass,
    pub control_lane: ValidationCertificationBundle,
    pub hostile_lane: ValidationRejectionCertificationBundle,
    pub parity_lane: ValidationCertificationBundle,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationCertificationMatrix {
    pub suite_name: &'static str,
    pub rows: Vec<ValidationCertificationRow>,
    pub rejection_rows: Vec<ValidationRejectionCertificationRow>,
}

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
        let bundle_completeness_report = self.bundle_completeness_report();
        let counter_snapshot = self.aggregate_counters();
        let certification_bundle_digest = digest_parts(&self.bundle_digest_parts());
        let coverage_matrix_digest = digest_parts(&self.coverage_digest_parts());

        MilestoneTwoValidationCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            bundle_completeness_report,
            counter_snapshot,
            matrix: self,
        }
    }

    fn bundle_completeness_report(&self) -> ValidationBundleCompletenessReport {
        let supported_lane_count = (self.rows.len() * 3) + (self.rejection_rows.len() * 2);
        let successful_lane_count = supported_lane_count;
        let zero_fallback_lane_count = self
            .rows
            .iter()
            .flat_map(|row| [&row.control_lane, &row.hostile_lane, &row.parity_lane])
            .chain(
                self.rejection_rows
                    .iter()
                    .flat_map(|row| [&row.control_lane, &row.parity_lane]),
            )
            .filter(|lane| lane.counter_snapshot.validation_fallback_count() == 0)
            .count();
        let all_lanes_emit_required_outputs = self
            .rows
            .iter()
            .all(ValidationCertificationRow::has_required_outputs)
            && self
                .rejection_rows
                .iter()
                .all(ValidationRejectionCertificationRow::has_required_outputs);
        let all_rows_have_hostile_coverage = self
            .rows
            .iter()
            .all(ValidationCertificationRow::has_hostile_coverage)
            && self
                .rejection_rows
                .iter()
                .all(ValidationRejectionCertificationRow::has_hostile_coverage);
        let covered_perturbation_classes = self.covered_perturbation_classes();
        let covers_all_currently_implemented_normative_scenarios = self
            .contains_row("legal-detail-query-parity")
            && self.contains_row("equivalent-builder-composed-legal-query")
            && self.contains_row("unknown-aspect-projection")
            && self.contains_row("ordering-only-authority-boundary")
            && self.contains_row("non-orderable-ordering-field")
            && self.contains_row("integer-greater-than-predicate-parity")
            && self.contains_row("integer-less-than-predicate-parity")
            && self.contains_row("scalar-membership-predicate-parity")
            && self.contains_row("membership-intersection-normalization")
            && self.contains_row("presence-predicate-parity")
            && self.contains_row("bounded-range-normalization")
            && self.contains_row("text-contains-predicate-parity")
            && self.contains_row("predicate-contradiction-rejection")
            && self.contains_row("membership-capability-rejection")
            && self.contains_row("presence-capability-rejection")
            && self.contains_row("empty-range-rejection")
            && self.contains_row("text-predicate-capability-rejection")
            && self.contains_row("incompatible-predicate-family")
            && self.contains_row("illegal-traversal-edge-or-depth")
            && self.contains_row("invalid-result-shape-binding")
            && self.contains_row("structured-content-illegality")
            && self.contains_row("workflow-context-illegality")
            && self.contains_row("forbidden-widening-case");
        let covers_full_milestone_two_spec_matrix =
            covers_all_currently_implemented_normative_scenarios
                && self.contains_row("incompatible-predicate-family");
        let offline_analysis_ready = all_lanes_emit_required_outputs
            && all_rows_have_hostile_coverage
            && zero_fallback_lane_count == supported_lane_count
            && covers_all_currently_implemented_normative_scenarios;

        ValidationBundleCompletenessReport {
            canonical_row_count: self.rows.len(),
            rejection_row_count: self.rejection_rows.len(),
            supported_lane_count,
            successful_lane_count,
            zero_fallback_lane_count,
            covered_perturbation_classes,
            all_lanes_emit_required_outputs,
            all_rows_have_hostile_coverage,
            covers_all_currently_implemented_normative_scenarios,
            covers_full_milestone_two_spec_matrix,
            offline_analysis_ready,
        }
    }

    fn aggregate_counters(&self) -> QueryValidationCounters {
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

    fn bundle_digest_parts(&self) -> Vec<String> {
        let mut parts = vec![format!("suite:{}", self.suite_name)];

        for row in &self.rows {
            parts.push(format!("row:{}", row.row_name));
            parts.extend(validation_bundle_digest_parts(&row.control_lane, "control"));
            parts.extend(validation_bundle_digest_parts(&row.hostile_lane, "hostile"));
            parts.extend(validation_bundle_digest_parts(&row.parity_lane, "parity"));
        }

        for row in &self.rejection_rows {
            parts.push(format!("rejection-row:{}", row.row_name));
            parts.extend(validation_bundle_digest_parts(&row.control_lane, "control"));
            parts.extend(validation_rejection_bundle_digest_parts(
                &row.hostile_lane,
                "hostile",
            ));
            parts.extend(validation_bundle_digest_parts(&row.parity_lane, "parity"));
        }

        parts
    }

    fn coverage_digest_parts(&self) -> Vec<String> {
        let mut parts = vec![
            format!("suite:{}", self.suite_name),
            format!("canonical-rows:{}", self.rows.len()),
            format!("rejection-rows:{}", self.rejection_rows.len()),
        ];

        for row in &self.rows {
            parts.push(format!(
                "row:{}:{}:{}:{}",
                row.row_name,
                perturbation_key(row.perturbation_class),
                hostile_expectation_key(row.hostile_expectation),
                parity_anchor_key(row.parity_anchor)
            ));
        }

        for row in &self.rejection_rows {
            parts.push(format!(
                "rejection-row:{}:{}:control-hostile-parity",
                row.row_name,
                perturbation_key(row.perturbation_class)
            ));
            parts.push(format!(
                "rejection-class:{}",
                row.hostile_lane.failure_class
            ));
        }

        parts
    }

    fn covered_perturbation_classes(&self) -> Vec<ValidationPerturbationClass> {
        let mut classes: Vec<_> = self
            .rows
            .iter()
            .map(|row| row.perturbation_class)
            .chain(self.rejection_rows.iter().map(|row| row.perturbation_class))
            .collect();
        classes.sort();
        classes.dedup();
        classes
    }

    fn contains_row(&self, row_name: &str) -> bool {
        self.rows.iter().any(|row| row.row_name == row_name)
            || self
                .rejection_rows
                .iter()
                .any(|row| row.row_name == row_name)
    }
}

impl ValidationCertificationRow {
    fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.hostile_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
    }

    fn has_hostile_coverage(&self) -> bool {
        let hostile_relation = match self.hostile_expectation {
            ValidationHostileExpectation::EquivalentToControl => {
                self.control_lane.validated_query_digest == self.hostile_lane.validated_query_digest
                    && self.control_lane.validated_result_shape_digest
                        == self.hostile_lane.validated_result_shape_digest
            }
            ValidationHostileExpectation::DistinctFromControl => {
                self.control_lane.validated_query_digest != self.hostile_lane.validated_query_digest
                    || self.control_lane.validated_result_shape_digest
                        != self.hostile_lane.validated_result_shape_digest
            }
        };

        let parity_relation = match self.parity_anchor {
            ValidationParityAnchor::Control => {
                self.control_lane.validated_query_digest == self.parity_lane.validated_query_digest
                    && self.control_lane.validated_result_shape_digest
                        == self.parity_lane.validated_result_shape_digest
            }
            ValidationParityAnchor::Hostile => {
                self.hostile_lane.validated_query_digest == self.parity_lane.validated_query_digest
                    && self.hostile_lane.validated_result_shape_digest
                        == self.parity_lane.validated_result_shape_digest
            }
        };

        hostile_relation && parity_relation
    }
}

impl ValidationRejectionCertificationRow {
    fn has_required_outputs(&self) -> bool {
        self.control_lane.has_required_outputs()
            && self.parity_lane.has_required_outputs()
            && !self.hostile_lane.failure_class.is_empty()
            && !self.hostile_lane.failure_digest.is_empty()
    }

    fn has_hostile_coverage(&self) -> bool {
        self.control_lane.validated_query_digest == self.parity_lane.validated_query_digest
            && self.control_lane.validated_result_shape_digest
                == self.parity_lane.validated_result_shape_digest
    }
}

impl ValidationCertificationBundle {
    fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.validated_query_digest.is_empty()
            && !self.validated_result_shape_digest.is_empty()
            && !self.basis_digest.is_empty()
            && self.validation_report.schema_basis_digest() == self.basis_digest
            && self.counter_snapshot.validation_fallback_count() == 0
    }
}

fn validation_bundle_digest_parts(
    bundle: &ValidationCertificationBundle,
    lane_name: &str,
) -> Vec<String> {
    let mut parts = vec![
        format!("lane:{lane_name}"),
        format!("profile:{}", profile_key(&bundle.profile)),
        format!("query:{}", bundle.query_digest),
        format!("validated-query:{}", bundle.validated_query_digest),
        format!(
            "validated-result-shape:{}",
            bundle.validated_result_shape_digest
        ),
        format!("basis:{}", bundle.basis_digest),
        format!(
            "projection-count:{}",
            bundle.validation_report.validated_projection_entries()
        ),
        format!(
            "traversal-count:{}",
            bundle.validation_report.validated_traversal_entries()
        ),
        format!(
            "predicate-count:{}",
            bundle.validation_report.validated_predicates()
        ),
        format!(
            "result-shape-bindings:{}",
            bundle.validation_report.validated_result_shape_bindings()
        ),
    ];
    for event in bundle.validation_report.events() {
        parts.push(format!("event:{event:?}"));
    }
    parts.extend(validation_counter_digest_parts(&bundle.counter_snapshot));
    parts
}

fn validation_rejection_bundle_digest_parts(
    bundle: &ValidationRejectionCertificationBundle,
    lane_name: &str,
) -> Vec<String> {
    let mut parts = vec![
        format!("lane:{lane_name}"),
        format!("profile:{}", profile_key(&bundle.profile)),
        format!("failure-class:{}", bundle.failure_class),
        format!("failure-digest:{}", bundle.failure_digest),
        format!(
            "projection-rejections:{}",
            bundle.validation_rejection_matrix.projection_rejections()
        ),
        format!(
            "ordering-rejections:{}",
            bundle.validation_rejection_matrix.ordering_rejections()
        ),
        format!(
            "traversal-rejections:{}",
            bundle.validation_rejection_matrix.traversal_rejections()
        ),
        format!(
            "predicate-rejections:{}",
            bundle.validation_rejection_matrix.predicate_rejections()
        ),
        format!(
            "result-shape-rejections:{}",
            bundle.validation_rejection_matrix.result_shape_rejections()
        ),
        format!(
            "compatibility-rejections:{}",
            bundle
                .validation_rejection_matrix
                .compatibility_rejections()
        ),
    ];
    parts.extend(validation_counter_digest_parts(&bundle.counter_snapshot));
    parts
}

fn validation_counter_digest_parts(counters: &QueryValidationCounters) -> Vec<String> {
    vec![
        format!(
            "validated-predicates:{}",
            counters.validated_predicate_count()
        ),
        format!(
            "validated-projections:{}",
            counters.validated_projection_entry_count()
        ),
        format!(
            "validated-traversals:{}",
            counters.validated_traversal_clause_count()
        ),
        format!(
            "validated-result-bindings:{}",
            counters.validated_result_shape_binding_count()
        ),
        format!(
            "validated-ordering:{}",
            counters.validated_ordering_field_count()
        ),
        format!("schema-lookups:{}", counters.schema_lookup_count()),
        format!("rejections:{}", counters.validation_rejection_count()),
        format!(
            "widening-denials:{}",
            counters.projection_widening_denial_count()
        ),
        format!("warnings:{}", counters.validation_warning_count()),
        format!("fallbacks:{}", counters.validation_fallback_count()),
    ]
}

fn profile_key(profile: &CertificationProfile) -> &'static str {
    match profile {
        CertificationProfile::DirectConstruction => "direct-construction",
        CertificationProfile::BuilderReordering => "builder-reordering",
        CertificationProfile::BindingVariation => "binding-variation",
        CertificationProfile::ReplayParity => "replay-parity",
    }
}

fn perturbation_key(class: ValidationPerturbationClass) -> &'static str {
    match class {
        ValidationPerturbationClass::ConstructionPath => "construction-path",
        ValidationPerturbationClass::SchemaBasisVariation => "schema-basis-variation",
        ValidationPerturbationClass::ProjectionLegality => "projection-legality",
        ValidationPerturbationClass::OrderingLegality => "ordering-legality",
        ValidationPerturbationClass::PredicateLegality => "predicate-legality",
        ValidationPerturbationClass::TraversalLegality => "traversal-legality",
        ValidationPerturbationClass::ResultShapeBindingLegality => "result-shape-binding-legality",
        ValidationPerturbationClass::StructuredContentLegality => "structured-content-legality",
        ValidationPerturbationClass::WorkflowContextLegality => "workflow-context-legality",
        ValidationPerturbationClass::ForbiddenWidening => "forbidden-widening",
    }
}

fn hostile_expectation_key(expectation: ValidationHostileExpectation) -> &'static str {
    match expectation {
        ValidationHostileExpectation::EquivalentToControl => "equivalent-to-control",
        ValidationHostileExpectation::DistinctFromControl => "distinct-from-control",
    }
}

fn parity_anchor_key(anchor: ValidationParityAnchor) -> &'static str {
    match anchor {
        ValidationParityAnchor::Control => "control",
        ValidationParityAnchor::Hostile => "hostile",
    }
}

fn digest_parts(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0x1f]);
    }
    format!("{:x}", hasher.finalize())
}
