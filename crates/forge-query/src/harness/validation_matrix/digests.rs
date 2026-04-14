use sha2::{Digest, Sha256};

use crate::facade::QueryValidationCounters;

use super::super::profiles::CertificationProfile;
use super::model::{
    ValidationCertificationBundle, ValidationCertificationMatrix, ValidationHostileExpectation,
    ValidationParityAnchor, ValidationPerturbationClass, ValidationRejectionCertificationBundle,
};

pub(crate) fn bundle_digest_parts(matrix: &ValidationCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];

    for row in &matrix.rows {
        parts.push(format!("row:{}", row.row_name));
        parts.extend(validation_bundle_digest_parts(&row.control_lane, "control"));
        parts.extend(validation_bundle_digest_parts(&row.hostile_lane, "hostile"));
        parts.extend(validation_bundle_digest_parts(&row.parity_lane, "parity"));
    }

    for row in &matrix.rejection_rows {
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

pub(crate) fn coverage_digest_parts(matrix: &ValidationCertificationMatrix) -> Vec<String> {
    let mut parts = vec![
        format!("suite:{}", matrix.suite_name),
        format!("canonical-rows:{}", matrix.rows.len()),
        format!("rejection-rows:{}", matrix.rejection_rows.len()),
    ];

    for row in &matrix.rows {
        parts.push(format!(
            "row:{}:{}:{}:{}",
            row.row_name,
            perturbation_key(row.perturbation_class),
            hostile_expectation_key(row.hostile_expectation),
            parity_anchor_key(row.parity_anchor)
        ));
    }

    for row in &matrix.rejection_rows {
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

pub(crate) fn digest_parts(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0x1f]);
    }
    format!("{:x}", hasher.finalize())
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
