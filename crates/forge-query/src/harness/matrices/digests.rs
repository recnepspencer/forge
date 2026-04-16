use super::model::{
    CanonicalCertificationBundle, CertificationMatrix, CertificationPerturbationClass,
    HostileLaneExpectation, ParityAnchor, RejectionCertificationBundle,
};
use crate::facade::{
    CanonicalizationCounters, CanonicalizationReport, CanonicalizationWarning,
    CompatibilityEvidence, NormalizationEvent,
};

pub(super) fn canonical_bundle_digest_parts(
    bundle: &CanonicalCertificationBundle,
    lane_name: &str,
) -> Vec<String> {
    let mut parts = vec![
        format!("lane:{lane_name}"),
        format!("profile:{}", profile_key(&bundle.profile)),
        format!("query:{}", bundle.query_digest),
        format!("result-shape:{}", bundle.result_shape_digest),
        format!("warning-count:{}", bundle.warning_count),
        format!("event-count:{}", bundle.event_count),
    ];
    parts.extend(report_digest_parts(&bundle.canonicalization_report));
    parts.extend(counter_digest_parts(&bundle.counter_snapshot));
    parts
}

pub(super) fn rejection_bundle_digest_parts(
    bundle: &RejectionCertificationBundle,
    lane_name: &str,
) -> Vec<String> {
    vec![
        format!("lane:{lane_name}"),
        format!("profile:{}", profile_key(&bundle.profile)),
        format!("failure-class:{}", bundle.failure_class),
        format!("failure-digest:{}", bundle.failure_digest),
    ]
}

pub(super) fn bundle_digest_parts(matrix: &CertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];

    for row in &matrix.rows {
        parts.push(format!("row:{}", row.row_name));
        parts.extend(canonical_bundle_digest_parts(&row.control_lane, "control"));
        parts.extend(canonical_bundle_digest_parts(&row.hostile_lane, "hostile"));
        parts.extend(canonical_bundle_digest_parts(&row.parity_lane, "parity"));
    }

    for row in &matrix.rejection_rows {
        parts.push(format!("rejection-row:{}", row.row_name));
        parts.extend(canonical_bundle_digest_parts(&row.control_lane, "control"));
        parts.extend(rejection_bundle_digest_parts(&row.hostile_lane, "hostile"));
        parts.extend(canonical_bundle_digest_parts(&row.parity_lane, "parity"));
    }

    parts
}

pub(super) fn coverage_digest_parts(matrix: &CertificationMatrix) -> Vec<String> {
    let mut parts = vec![
        format!("suite:{}", matrix.suite_name),
        format!("canonical-rows:{}", matrix.rows.len()),
        format!("rejection-rows:{}", matrix.rejection_rows.len()),
    ];

    for row in &matrix.rows {
        parts.push(format!(
            "row:{}:{}:{}:{}",
            row.row_name,
            perturbation_class_key(row.perturbation_class),
            hostile_expectation_key(row.hostile_expectation),
            parity_anchor_key(row.parity_anchor)
        ));
    }

    for row in &matrix.rejection_rows {
        parts.push(format!(
            "rejection-row:{}:{}:control-hostile-parity",
            row.row_name,
            perturbation_class_key(row.perturbation_class)
        ));
        parts.push(format!(
            "rejection-class:{}",
            row.hostile_lane.failure_class
        ));
    }

    parts
}

pub(super) fn perturbation_class_key(class: CertificationPerturbationClass) -> &'static str {
    match class {
        CertificationPerturbationClass::ConstructionPath => "construction-path",
        CertificationPerturbationClass::ResultShapeComposition => "result-shape-composition",
        CertificationPerturbationClass::BindingDescriptorVariation => {
            "binding-descriptor-variation"
        }
        CertificationPerturbationClass::Deduplication => "deduplication",
        CertificationPerturbationClass::MeaningChange => "meaning-change",
        CertificationPerturbationClass::UnsupportedAuthoredForm => "unsupported-authored-form",
        CertificationPerturbationClass::ForbiddenFallback => "forbidden-fallback",
    }
}

pub(super) fn hostile_expectation_key(expectation: HostileLaneExpectation) -> &'static str {
    match expectation {
        HostileLaneExpectation::EquivalentToControl => "equivalent-to-control",
        HostileLaneExpectation::DistinctFromControl => "distinct-from-control",
    }
}

pub(super) fn parity_anchor_key(anchor: ParityAnchor) -> &'static str {
    match anchor {
        ParityAnchor::Control => "control",
        ParityAnchor::Hostile => "hostile",
    }
}

fn profile_key(profile: &super::super::profiles::CertificationProfile) -> &'static str {
    match profile {
        super::super::profiles::CertificationProfile::DirectConstruction => "direct-construction",
        super::super::profiles::CertificationProfile::BuilderReordering => "builder-reordering",
        super::super::profiles::CertificationProfile::BindingVariation => "binding-variation",
        super::super::profiles::CertificationProfile::ReplayParity => "replay-parity",
    }
}

fn report_digest_parts(report: &CanonicalizationReport) -> Vec<String> {
    let mut parts = vec![
        format!(
            "compatibility:{}",
            compatibility_key(report.compatibility())
        ),
        format!(
            "normalized-projections:{}",
            report.normalized_projection_entries()
        ),
        format!(
            "normalized-traversals:{}",
            report.normalized_traversal_entries()
        ),
        format!("normalized-fields:{}", report.normalized_result_fields()),
        format!(
            "identity-freeze-query:{}",
            report.identity_freeze().query_digest
        ),
        format!(
            "identity-freeze-shape:{}",
            report.identity_freeze().result_shape_digest
        ),
    ];

    for warning in report.warnings() {
        parts.push(format!("warning:{}", warning_key(warning)));
    }

    for event in report.events() {
        parts.push(format!("event:{}", event_key(event)));
    }

    parts
}

fn counter_digest_parts(counters: &CanonicalizationCounters) -> Vec<String> {
    vec![
        format!("raw-clauses:{}", counters.raw_clause_count),
        format!("normalized-clauses:{}", counters.normalized_clause_count),
        format!("projections:{}", counters.projection_entry_count),
        format!("traversals:{}", counters.traversal_clause_count),
        format!("result-fields:{}", counters.result_shape_field_count),
        format!("bindings:{}", counters.binding_descriptor_count),
        format!("query-dedup:{}", counters.query_deduplication_count),
        format!("shape-dedup:{}", counters.result_shape_deduplication_count),
        format!("warnings:{}", counters.canonicalization_warning_count),
        format!("fallbacks:{}", counters.canonicalization_fallback_count),
    ]
}

fn compatibility_key(evidence: &CompatibilityEvidence) -> &'static str {
    match evidence {
        CompatibilityEvidence::Compatible => "compatible",
    }
}

fn warning_key(warning: &CanonicalizationWarning) -> String {
    match warning {
        CanonicalizationWarning::DuplicateProjectionCollapsed { aspect, field } => {
            format!("duplicate-projection:{aspect}:{field}")
        }
        CanonicalizationWarning::DuplicateTraversalCollapsed { relation, depth } => {
            format!("duplicate-traversal:{relation}:{depth}")
        }
        CanonicalizationWarning::DuplicateResultFieldCollapsed { delivered_name } => {
            format!("duplicate-result-field:{delivered_name}")
        }
        CanonicalizationWarning::NonIdentityBindingMetadataIgnored { key } => {
            format!("ignored-binding-metadata:{key}")
        }
    }
}

fn event_key(event: &NormalizationEvent) -> String {
    match event {
        NormalizationEvent::ProjectionRetained { aspect, field } => {
            format!("projection-retained:{aspect}:{field}")
        }
        NormalizationEvent::ProjectionCollapsedDuplicate { aspect, field } => {
            format!("projection-collapsed:{aspect}:{field}")
        }
        NormalizationEvent::TraversalRetained { relation, depth } => {
            format!("traversal-retained:{relation}:{depth}")
        }
        NormalizationEvent::TraversalCollapsedDuplicate { relation, depth } => {
            format!("traversal-collapsed:{relation}:{depth}")
        }
        NormalizationEvent::ResultFieldRetained {
            source_aspect,
            source_field,
            delivered_name,
        } => format!("result-field-retained:{source_aspect}:{source_field}:{delivered_name}"),
        NormalizationEvent::ResultFieldCollapsedDuplicate { delivered_name } => {
            format!("result-field-collapsed:{delivered_name}")
        }
        NormalizationEvent::IdentityBindingRetained { slot } => {
            format!("identity-binding-retained:{slot}")
        }
        NormalizationEvent::IdentityBindingCollapsedDuplicate { slot } => {
            format!("identity-binding-collapsed:{slot}")
        }
        NormalizationEvent::NonIdentityBindingIgnored { key } => {
            format!("non-identity-binding-ignored:{key}")
        }
        NormalizationEvent::CompatibilityEstablished => "compatibility-established".to_string(),
        NormalizationEvent::IdentityFrozen {
            query_digest,
            result_shape_digest,
        } => format!("identity-frozen:{query_digest}:{result_shape_digest}"),
    }
}
