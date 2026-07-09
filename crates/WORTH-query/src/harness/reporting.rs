use crate::authoring::{RawAuthoredQuery, RawAuthoredResultShape};
use crate::facade::{
    AspectFieldSelector, AuthoredResultShapeField, IdentityBindingDescriptor,
    IdentityFreezeEvidence, NonIdentityBindingMetadata, QueryBindingDescriptor, QueryBindingSlot,
    QueryBindingSubject, RootEntityKey, TraversalSelector,
};

#[test]
fn duplicate_projection_collapses_with_warning_and_counter() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request = crate::facade::GuidedAuthoringPath::pair_detail(query, shape).unwrap();
    let bundle = crate::facade::canonicalize_request(request).unwrap();

    assert_eq!(bundle.query().projection().len(), 1);
    assert_eq!(bundle.counters().query_deduplication_count, 1);
    assert_eq!(bundle.counters().canonicalization_warning_count, 1);
    assert_eq!(bundle.counters().normalized_clause_count, 1);
    assert_eq!(bundle.counters().canonicalization_fallback_count, 0);
    assert!(bundle.report().events().iter().any(|event| matches!(
        event,
        crate::facade::NormalizationEvent::ProjectionCollapsedDuplicate { .. }
    )));
    bundle.check_invariants().unwrap();
}

#[test]
fn duplicate_result_shape_field_collapses_with_warning_and_counter() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request = crate::facade::GuidedAuthoringPath::pair_detail(query, shape).unwrap();
    let bundle = crate::facade::canonicalize_request(request).unwrap();

    assert_eq!(bundle.result_shape().fields().len(), 1);
    assert_eq!(bundle.counters().result_shape_deduplication_count, 1);
    assert_eq!(bundle.counters().canonicalization_warning_count, 1);
    assert_eq!(bundle.counters().canonicalization_fallback_count, 0);
    assert!(bundle.report().events().iter().any(|event| matches!(
        event,
        crate::facade::NormalizationEvent::ResultFieldCollapsedDuplicate { .. }
    )));
    bundle.check_invariants().unwrap();
}

#[test]
fn duplicate_traversal_collapses_with_warning_and_counter() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request = crate::facade::GuidedAuthoringPath::pair_detail(query, shape).unwrap();
    let bundle = crate::facade::canonicalize_request(request).unwrap();

    assert_eq!(bundle.query().traversal().len(), 1);
    assert_eq!(bundle.counters().query_deduplication_count, 1);
    assert_eq!(bundle.counters().canonicalization_warning_count, 1);
    assert_eq!(bundle.counters().canonicalization_fallback_count, 0);
    assert!(bundle.report().events().iter().any(|event| matches!(
        event,
        crate::facade::NormalizationEvent::TraversalCollapsedDuplicate { .. }
    )));
    bundle.check_invariants().unwrap();
}

#[test]
fn report_trace_and_counter_integrity_hold_under_mixed_normalization_pressure() {
    let query = RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .build()
        .unwrap();
    let bindings = QueryBindingDescriptor::new()
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ))
        .with_non_identity(NonIdentityBindingMetadata::new("route", "tasks.index").unwrap())
        .with_non_identity(
            NonIdentityBindingMetadata::new("debug_label", "collection-index").unwrap(),
        );

    let request =
        crate::facade::GuidedAuthoringPath::pair_collection_with_bindings(query, shape, bindings)
            .unwrap();
    let bundle = crate::facade::canonicalize_request(request).unwrap();

    assert_eq!(bundle.counters().query_deduplication_count, 2);
    assert_eq!(bundle.counters().result_shape_deduplication_count, 1);
    assert_eq!(bundle.counters().canonicalization_warning_count, 5);
    assert_eq!(bundle.counters().canonicalization_fallback_count, 0);
    assert_eq!(
        bundle
            .report()
            .warnings()
            .iter()
            .filter(|warning| matches!(
                warning,
                crate::facade::CanonicalizationWarning::DuplicateProjectionCollapsed { .. }
            ))
            .count(),
        1
    );
    assert_eq!(
        bundle
            .report()
            .warnings()
            .iter()
            .filter(|warning| matches!(
                warning,
                crate::facade::CanonicalizationWarning::DuplicateTraversalCollapsed { .. }
            ))
            .count(),
        1
    );
    assert_eq!(
        bundle
            .report()
            .warnings()
            .iter()
            .filter(|warning| matches!(
                warning,
                crate::facade::CanonicalizationWarning::DuplicateResultFieldCollapsed { .. }
            ))
            .count(),
        1
    );
    assert_eq!(
        bundle
            .report()
            .warnings()
            .iter()
            .filter(|warning| matches!(
                warning,
                crate::facade::CanonicalizationWarning::NonIdentityBindingMetadataIgnored { .. }
            ))
            .count(),
        2
    );
    bundle.check_invariants().unwrap();
}

#[test]
fn invariant_check_rejects_duplicate_compatibility_event() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request = crate::facade::GuidedAuthoringPath::pair_detail(query, shape).unwrap();
    let mut bundle = crate::facade::canonicalize_request(request).unwrap();
    bundle
        .report_mut_for_test()
        .events_mut_for_test()
        .push(crate::facade::NormalizationEvent::CompatibilityEstablished);

    let error = bundle.check_invariants().unwrap_err();
    assert!(matches!(
        error,
        crate::facade::QueryCanonicalizationError::BundleInvariantViolation {
            message: "compatibility must be established exactly once"
        }
    ));
}

#[test]
fn invariant_check_rejects_warning_counter_drift() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request = crate::facade::GuidedAuthoringPath::pair_detail(query, shape).unwrap();
    let mut bundle = crate::facade::canonicalize_request(request).unwrap();
    bundle
        .counters_mut_for_test()
        .canonicalization_warning_count += 1;

    let error = bundle.check_invariants().unwrap_err();
    assert!(matches!(
        error,
        crate::facade::QueryCanonicalizationError::BundleInvariantViolation {
            message: "warning count does not match warning list length"
        }
    ));
}

#[test]
fn invariant_check_rejects_normalized_projection_count_drift() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request = crate::facade::GuidedAuthoringPath::pair_detail(query, shape).unwrap();
    let mut bundle = crate::facade::canonicalize_request(request).unwrap();
    bundle
        .report_mut_for_test()
        .set_normalized_projection_entries_for_test(2);

    let error = bundle.check_invariants().unwrap_err();
    assert!(matches!(
        error,
        crate::facade::QueryCanonicalizationError::BundleInvariantViolation {
            message: "normalized projection count does not match canonical query projection length"
        }
    ));
}

#[test]
fn invariant_check_rejects_identity_freeze_digest_drift() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request = crate::facade::GuidedAuthoringPath::pair_detail(query, shape).unwrap();
    let mut bundle = crate::facade::canonicalize_request(request).unwrap();
    let result_shape_digest = bundle.result_shape().digest().as_str().to_string();
    bundle
        .report_mut_for_test()
        .set_identity_freeze_for_test(IdentityFreezeEvidence {
            query_digest: "worthd-query-digest".to_string(),
            result_shape_digest,
        });

    let error = bundle.check_invariants().unwrap_err();
    assert!(matches!(
        error,
        crate::facade::QueryCanonicalizationError::BundleInvariantViolation {
            message: "query digest mismatch between bundle and identity freeze evidence"
        }
    ));
}

#[test]
fn invariant_check_rejects_normalized_traversal_count_drift() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request = crate::facade::GuidedAuthoringPath::pair_detail(query, shape).unwrap();
    let mut bundle = crate::facade::canonicalize_request(request).unwrap();
    bundle
        .report_mut_for_test()
        .set_normalized_traversal_entries_for_test(0);

    let error = bundle.check_invariants().unwrap_err();
    assert!(matches!(
        error,
        crate::facade::QueryCanonicalizationError::BundleInvariantViolation {
            message: "normalized traversal count does not match canonical query traversal length"
        }
    ));
}

#[test]
fn invariant_check_rejects_normalized_result_field_count_drift() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request = crate::facade::GuidedAuthoringPath::pair_detail(query, shape).unwrap();
    let mut bundle = crate::facade::canonicalize_request(request).unwrap();
    bundle
        .report_mut_for_test()
        .set_normalized_result_fields_for_test(0);

    let error = bundle.check_invariants().unwrap_err();
    assert!(matches!(
        error,
        crate::facade::QueryCanonicalizationError::BundleInvariantViolation {
            message:
                "normalized result field count does not match canonical result-shape field length"
        }
    ));
}

#[test]
fn invariant_check_rejects_ignored_binding_warning_event_drift() {
    let query = RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();
    let bindings = QueryBindingDescriptor::new()
        .with_non_identity(NonIdentityBindingMetadata::new("route", "tasks.index").unwrap());

    let request =
        crate::facade::GuidedAuthoringPath::pair_collection_with_bindings(query, shape, bindings)
            .unwrap();
    let mut bundle = crate::facade::canonicalize_request(request).unwrap();
    bundle.report_mut_for_test().warnings_mut_for_test().clear();
    bundle
        .counters_mut_for_test()
        .canonicalization_warning_count = 0;

    let error = bundle.check_invariants().unwrap_err();
    assert!(matches!(
        error,
        crate::facade::QueryCanonicalizationError::BundleInvariantViolation {
            message: "ignored binding event count does not match ignored binding warning count"
        }
    ));
}
