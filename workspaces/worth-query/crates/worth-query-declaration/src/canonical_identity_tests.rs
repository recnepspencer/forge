use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, DetailAuthoredQuery, DetailAuthoredResultShape,
    GuidedAuthoringPath, RawAuthoredQuery, RawAuthoredResultShape, RootEntityKey,
};
use crate::canonicalization::canonicalize_request;
use crate::identity::CanonicalEquivalence;

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
fn distinct_projection_meaning_has_distinct_identity() {
    let title = canonicalize_request(
        GuidedAuthoringPath::pair_detail(
            detail_query("title", "text"),
            detail_shape("title", "text", "title"),
        )
        .unwrap(),
    )
    .unwrap();
    let status = canonicalize_request(
        GuidedAuthoringPath::pair_detail(
            detail_query("status", "kind"),
            detail_shape("status", "kind", "status"),
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        title.query().equivalence_to(status.query()),
        CanonicalEquivalence::Distinct
    );
    assert_eq!(
        title.equivalence_to(&status),
        CanonicalEquivalence::Distinct
    );
}

#[test]
fn result_alias_changes_only_result_shape_identity() {
    let query = detail_query("title", "text");
    let title = canonicalize_request(
        GuidedAuthoringPath::pair_detail(query.clone(), detail_shape("title", "text", "title"))
            .unwrap(),
    )
    .unwrap();
    let label = canonicalize_request(
        GuidedAuthoringPath::pair_detail(query, detail_shape("title", "text", "label")).unwrap(),
    )
    .unwrap();

    assert_eq!(title.query().digest(), label.query().digest());
    assert_eq!(
        title.query().equivalence_to(label.query()),
        CanonicalEquivalence::Equivalent
    );
    assert_ne!(title.result_shape().digest(), label.result_shape().digest());
    assert_eq!(title.equivalence_to(&label), CanonicalEquivalence::Distinct);
}

#[test]
fn omitted_result_field_does_not_rewrite_query_identity() {
    let query = RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .build()
        .unwrap();
    let full_shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .build()
        .unwrap();
    let omitted_shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();
    let full = canonicalize_request(
        GuidedAuthoringPath::pair_collection(query.clone(), full_shape).unwrap(),
    )
    .unwrap();
    let omitted =
        canonicalize_request(GuidedAuthoringPath::pair_collection(query, omitted_shape).unwrap())
            .unwrap();

    assert_eq!(full.query().digest(), omitted.query().digest());
    assert_ne!(
        full.result_shape().digest(),
        omitted.result_shape().digest()
    );
    assert_eq!(
        full.equivalence_to(&omitted),
        CanonicalEquivalence::Distinct
    );
}
