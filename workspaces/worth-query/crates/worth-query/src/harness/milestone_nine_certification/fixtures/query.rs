use crate::authoring::AspectFieldSelector;
use crate::authoring::AuthoredResultShapeField;
use crate::authoring::GuidedAuthoringPath;
use crate::authoring::RawAuthoredQuery;
use crate::authoring::RawAuthoredResultShape;
use crate::authoring::RootEntityKey;
use crate::authoring::TraversalSelector;

pub(in crate::harness::milestone_nine_certification) fn canonical_query(
) -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub(in crate::harness::milestone_nine_certification) fn canonical_query_with_secret_projection(
) -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("secret", "salary").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub(in crate::harness::milestone_nine_certification) fn canonical_query_with_manager_traversal(
) -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("secret", "salary").unwrap())
        .traverse(TraversalSelector::bounded("manager", 1).unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub(in crate::harness::milestone_nine_certification) fn canonical_query_with_secret_predicate(
) -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .where_equal(
            crate::authoring::EqualityPredicate::new(
                "secret",
                "salary",
                crate::authoring::WorthQueryPredicateOperand::int64(7),
            )
            .unwrap(),
        )
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub(in crate::harness::milestone_nine_certification) fn canonical_query_with_secret_ordering(
) -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .order_by(crate::authoring::OrderingSelector::ascending("secret", "salary").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}
