use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, DetailAuthoredQuery, DetailAuthoredResultShape,
    GuidedAuthoringPath, RawAuthoredQuery, RawAuthoredResultShape, RootEntityKey,
    TraversalSelector,
};
use crate::canonicalization::canonicalize_request;
use crate::diagnostics::{CanonicalizationWarning, NormalizationEvent};

fn detail_query(field: &str, aspect: &str) -> DetailAuthoredQuery {
    RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new(field, aspect).unwrap())
        .build()
        .unwrap()
}

fn detail_shape(field: &str, aspect: &str, alias: &str) -> DetailAuthoredResultShape {
    RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new(field, aspect, alias).unwrap())
        .build()
        .unwrap()
}

#[test]
fn duplicate_query_clauses_collapse_with_exact_reporting() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .build()
        .unwrap();
    let request =
        GuidedAuthoringPath::pair_detail(query, detail_shape("title", "text", "title")).unwrap();

    let bundle = canonicalize_request(request).unwrap();

    assert_eq!(bundle.query().projection().len(), 1);
    assert_eq!(bundle.query().traversal().len(), 1);
    assert_eq!(bundle.counters().query_deduplication_count, 2);
    assert_eq!(bundle.counters().canonicalization_warning_count, 2);
    assert_eq!(bundle.counters().canonicalization_fallback_count, 0);
    assert!(bundle.report().events().iter().any(|event| matches!(
        event,
        NormalizationEvent::ProjectionCollapsedDuplicate { .. }
    )));
    assert!(bundle.report().events().iter().any(|event| matches!(
        event,
        NormalizationEvent::TraversalCollapsedDuplicate { .. }
    )));
    bundle.check_invariants().unwrap();
}

#[test]
fn duplicate_result_fields_collapse_with_exact_reporting() {
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();
    let request = GuidedAuthoringPath::pair_detail(detail_query("title", "text"), shape).unwrap();

    let bundle = canonicalize_request(request).unwrap();

    assert_eq!(bundle.result_shape().fields().len(), 1);
    assert_eq!(bundle.counters().result_shape_deduplication_count, 1);
    assert_eq!(bundle.counters().canonicalization_warning_count, 1);
    assert!(matches!(
        bundle.report().warnings(),
        [CanonicalizationWarning::DuplicateResultFieldCollapsed { .. }]
    ));
    bundle.check_invariants().unwrap();
}
