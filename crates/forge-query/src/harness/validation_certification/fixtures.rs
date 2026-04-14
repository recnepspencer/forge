use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, GuidedAuthoringPath,
    IntegerComparisonPredicate, OrderingSelector, PresencePredicate, RootEntityKey,
    ScalarPredicateValue, SetMembershipPredicate, StringContainsPredicate,
};

pub(super) fn reordered_legal_detail_bundle() -> crate::facade::CanonicalQueryBundle {
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
    configure: impl FnOnce(
        crate::authoring::DetailQueryBuilder,
    ) -> crate::authoring::DetailQueryBuilder,
) -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn legal_greater_than_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_greater_than(
            IntegerComparisonPredicate::greater_than("profile", "age", 18).unwrap(),
        )
    })
}

pub(super) fn reordered_greater_than_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn strongest_greater_than_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_greater_than(
            IntegerComparisonPredicate::greater_than("profile", "age", 21).unwrap(),
        )
    })
}

pub(super) fn legal_less_than_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_less_than(IntegerComparisonPredicate::less_than("profile", "age", 65).unwrap())
    })
}

pub(super) fn reordered_less_than_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn redundant_greater_than_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query
            .where_greater_than(IntegerComparisonPredicate::greater_than("profile", "age", 18).unwrap())
            .where_greater_than(IntegerComparisonPredicate::greater_than("profile", "age", 21).unwrap())
    })
}

pub(super) fn bounded_range_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query
            .where_greater_than(IntegerComparisonPredicate::greater_than("profile", "age", 18).unwrap())
            .where_less_than(IntegerComparisonPredicate::less_than("profile", "age", 65).unwrap())
    })
}

pub(super) fn reordered_bounded_range_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn legal_contains_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_contains(StringContainsPredicate::new("profile", "display_name", "est").unwrap())
    })
}

pub(super) fn reordered_contains_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn legal_membership_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_in(
            SetMembershipPredicate::new(
                "profile",
                "age",
                [ScalarPredicateValue::Integer(21), ScalarPredicateValue::Integer(34)],
            )
            .unwrap(),
        )
    })
}

pub(super) fn reordered_membership_bundle() -> crate::facade::CanonicalQueryBundle {
    let root = RootEntityKey::new("user").unwrap();
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .where_in(
            SetMembershipPredicate::new(
                "profile",
                "age",
                [ScalarPredicateValue::Integer(34), ScalarPredicateValue::Integer(21)],
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

pub(super) fn overlapping_membership_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn intersected_membership_bundle() -> crate::facade::CanonicalQueryBundle {
    legal_membership_bundle()
}

pub(super) fn legal_presence_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_present(PresencePredicate::is_present("profile", "display_name").unwrap())
    })
}

pub(super) fn reordered_presence_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn unknown_aspect_bundle() -> crate::facade::CanonicalQueryBundle {
    canonical_bundle_with_projection("missing", "id", "value")
}

pub(super) fn structured_content_illegal_bundle() -> crate::facade::CanonicalQueryBundle {
    canonical_bundle_with_projection("content", "bio", "bio")
}

pub(super) fn workflow_context_illegal_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn non_orderable_ordering_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn forbidden_widening_bundle() -> crate::facade::CanonicalQueryBundle {
    canonical_bundle_with_projection("profile", "unknown", "value")
}

pub(super) fn illegal_traversal_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn incompatible_predicate_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn contradictory_predicate_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn empty_range_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query
            .where_greater_than(
                IntegerComparisonPredicate::greater_than("profile", "age", 65).unwrap(),
            )
            .where_less_than(IntegerComparisonPredicate::less_than("profile", "age", 65).unwrap())
    })
}

pub(super) fn text_capability_illegal_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_contains(
            StringContainsPredicate::new("profile", "private_note", "secret").unwrap(),
        )
    })
}

pub(super) fn membership_capability_illegal_bundle() -> crate::facade::CanonicalQueryBundle {
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

pub(super) fn presence_capability_illegal_bundle() -> crate::facade::CanonicalQueryBundle {
    predicate_bundle(|query| {
        query.where_present(PresencePredicate::is_present("profile", "private_note").unwrap())
    })
}

pub(super) fn invalid_result_shape_binding_bundle() -> crate::facade::CanonicalQueryBundle {
    let mut bundle = crate::harness::fixtures::schema_view::legal_detail_bundle();
    bundle
        .result_shape_mut_for_test()
        .rewrite_first_field_for_test("profile", "age", "age");
    bundle
}

fn canonical_bundle_with_projection(
    aspect: &str,
    field: &str,
    delivered_name: &str,
) -> crate::facade::CanonicalQueryBundle {
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
