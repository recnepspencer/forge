use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, GuidedAuthoringPath,
    IntegerComparisonPredicate, OrderingSelector, PresencePredicate, RootEntityKey,
    ScalarPredicateValue, SetMembershipPredicate, StringContainsPredicate,
};
use crate::facade::CanonicalQueryBundle;

pub fn runtime_detail_bundle() -> CanonicalQueryBundle {
    let request = GuidedAuthoringPath::pair_detail(
        super::authored_requests::runtime_detail_query(),
        super::authored_requests::runtime_detail_result_shape(),
    )
    .unwrap();
    crate::facade::canonicalize_request(request).unwrap()
}

pub fn runtime_bound_detail_bundle() -> CanonicalQueryBundle {
    let bindings = crate::facade::QueryBindingDescriptor::new().with_identity(
        crate::facade::IdentityBindingDescriptor::new(
            crate::facade::QueryBindingSlot::new("root").unwrap(),
            crate::facade::QueryBindingSubject::RootEntity,
        ),
    );
    let request = GuidedAuthoringPath::pair_detail_with_bindings(
        super::authored_requests::runtime_detail_query(),
        super::authored_requests::runtime_detail_result_shape(),
        bindings,
    )
    .unwrap();
    crate::facade::canonicalize_request(request).unwrap()
}

pub fn legal_detail_bundle() -> CanonicalQueryBundle {
    super::schema_view::legal_detail_bundle()
}

pub fn legal_structured_content_bundle() -> CanonicalQueryBundle {
    super::schema_view::legal_structured_content_bundle()
}

pub fn legal_workflow_predicate_bundle() -> CanonicalQueryBundle {
    super::schema_view::legal_workflow_predicate_bundle()
}

pub fn reordered_legal_detail_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .traverse(crate::authoring::TraversalSelector::bounded("manager", 1).unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("profile", "display_name", "name").unwrap())
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn predicate_bundle(
    configure: impl FnOnce(crate::authoring::DetailQueryBuilder) -> crate::authoring::DetailQueryBuilder,
) -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = configure(
        crate::authoring::DetailQueryBuilder::new(root)
            .project(AspectFieldSelector::new("identity", "id").unwrap()),
    )
    .build()
    .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn legal_greater_than_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_greater_than(
            IntegerComparisonPredicate::greater_than("profile", "age", 18).unwrap(),
        )
    })
}

pub fn reordered_greater_than_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .where_greater_than(IntegerComparisonPredicate::greater_than("profile", "age", 18).unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn strongest_greater_than_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_greater_than(
            IntegerComparisonPredicate::greater_than("profile", "age", 21).unwrap(),
        )
    })
}

pub fn legal_less_than_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_less_than(IntegerComparisonPredicate::less_than("profile", "age", 65).unwrap())
    })
}

pub fn reordered_less_than_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .where_less_than(IntegerComparisonPredicate::less_than("profile", "age", 65).unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn redundant_greater_than_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query
            .where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 18).unwrap(),
            )
            .where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 21).unwrap(),
            )
    })
}

pub fn bounded_range_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query
            .where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 18).unwrap(),
            )
            .where_less_than(IntegerComparisonPredicate::less_than("profile", "age", 65).unwrap())
    })
}

pub fn reordered_bounded_range_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .where_less_than(IntegerComparisonPredicate::less_than("profile", "age", 65).unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .where_greater_than(IntegerComparisonPredicate::greater_than("profile", "age", 18).unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn legal_contains_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_contains(
            StringContainsPredicate::new("profile", "display_name", "est").unwrap(),
        )
    })
}

pub fn reordered_contains_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .where_contains(StringContainsPredicate::new("profile", "display_name", "est").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn legal_membership_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_in(
            SetMembershipPredicate::new(
                "profile",
                "age",
                [
                    ScalarPredicateValue::Integer(21),
                    ScalarPredicateValue::Integer(34),
                ],
            )
            .unwrap(),
        )
    })
}

pub fn reordered_membership_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .where_in(
            SetMembershipPredicate::new(
                "profile",
                "age",
                [
                    ScalarPredicateValue::Integer(34),
                    ScalarPredicateValue::Integer(21),
                ],
            )
            .unwrap(),
        )
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn overlapping_membership_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query
            .where_in(
                SetMembershipPredicate::new(
                    "profile",
                    "age",
                    [
                        ScalarPredicateValue::Integer(18),
                        ScalarPredicateValue::Integer(21),
                        ScalarPredicateValue::Integer(34),
                    ],
                )
                .unwrap(),
            )
            .where_in(
                SetMembershipPredicate::new(
                    "profile",
                    "age",
                    [
                        ScalarPredicateValue::Integer(21),
                        ScalarPredicateValue::Integer(34),
                        ScalarPredicateValue::Integer(55),
                    ],
                )
                .unwrap(),
            )
    })
}

pub fn intersected_membership_bundle() -> CanonicalQueryBundle {
    legal_membership_bundle()
}

pub fn legal_presence_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_present(PresencePredicate::is_present("profile", "display_name").unwrap())
    })
}

pub fn reordered_presence_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .where_present(PresencePredicate::is_present("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn reordered_legal_structured_content_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("content", "bio").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("content", "bio", "bio").unwrap())
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn reordered_legal_workflow_predicate_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .where_equal(
            EqualityPredicate::new(
                "workflow",
                "status",
                ScalarPredicateValue::String("done".to_string()),
            )
            .unwrap(),
        )
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn unknown_aspect_bundle() -> CanonicalQueryBundle {
    canonical_bundle_with_projection("missing", "id", "value")
}

pub fn structured_content_illegal_bundle() -> CanonicalQueryBundle {
    canonical_bundle_with_projection("content", "bio", "bio")
}

pub fn workflow_context_illegal_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_equal(
            EqualityPredicate::new(
                "workflow",
                "status",
                ScalarPredicateValue::String("done".to_string()),
            )
            .unwrap(),
        )
    })
}

pub fn non_orderable_ordering_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .order_by(OrderingSelector::ascending("profile", "private_note").unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn forbidden_widening_bundle() -> CanonicalQueryBundle {
    canonical_bundle_with_projection("profile", "unknown", "value")
}

pub fn illegal_traversal_bundle() -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .traverse(crate::authoring::TraversalSelector::bounded("manager", 2).unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

pub fn incompatible_predicate_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_equal(
            EqualityPredicate::new(
                "profile",
                "age",
                ScalarPredicateValue::String("too-old".to_string()),
            )
            .unwrap(),
        )
    })
}

pub fn contradictory_predicate_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query
            .where_equal(
                EqualityPredicate::new("profile", "age", ScalarPredicateValue::Integer(18))
                    .unwrap(),
            )
            .where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 18).unwrap(),
            )
    })
}

pub fn empty_range_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query
            .where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 65).unwrap(),
            )
            .where_less_than(IntegerComparisonPredicate::less_than("profile", "age", 65).unwrap())
    })
}

pub fn text_capability_illegal_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_contains(
            StringContainsPredicate::new("profile", "private_note", "secret").unwrap(),
        )
    })
}

pub fn membership_capability_illegal_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_in(
            SetMembershipPredicate::new(
                "profile",
                "private_note",
                [ScalarPredicateValue::String("secret".to_string())],
            )
            .unwrap(),
        )
    })
}

pub fn presence_capability_illegal_bundle() -> CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_present(PresencePredicate::is_present("profile", "private_note").unwrap())
    })
}

pub fn invalid_result_shape_binding_bundle() -> CanonicalQueryBundle {
    let mut bundle = super::schema_view::legal_detail_bundle();
    bundle
        .result_shape_mut_for_test()
        .rewrite_first_field_for_test("profile", "age", "age");
    bundle
}

fn canonical_bundle_with_projection(
    aspect: &str,
    field: &str,
    delivered_name: &str,
) -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new(aspect, field).unwrap())
        .build()
        .unwrap();
    let result_shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new(aspect, field, delivered_name).unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}
