use super::super::support::*;
use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, OrderingSelector,
    ScalarPredicateValue,
};
use crate::runtime::{
    ForgeQueryReadBuiltInOperator, ForgeQueryReadBuiltInOperatorDenialReason,
    ForgeQueryReadScopeClass, ForgeQueryRuntimeError,
};

#[test]
fn compose_read_executes_operator_owned_frontier_search_detail() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.operator-frontier-search-detail")
        .expect("read-backed runtime should open a workspace");

    let result = workspace
        .compose_read(|read| {
            read.explicit_broad_search_frontier_detail(
                "user",
                searchable_expanded_frontier_manager_schema(),
                [manager_relation_name(), mentor_relation_name()],
                2,
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id")
                                .expect("identity projection should build"),
                        )
                        .project(
                            AspectFieldSelector::new("profile", "display_name")
                                .expect("name projection should build"),
                        )
                        .where_equal(
                            EqualityPredicate::new(
                                "profile",
                                "display_name",
                                ScalarPredicateValue::String("Ada".to_string()),
                            )
                            .expect("equality predicate should build"),
                        )
                },
                |shape| {
                    shape
                        .field(
                            AuthoredResultShapeField::new("identity", "id", "id")
                                .expect("identity result-shape field should build"),
                        )
                        .field(
                            AuthoredResultShapeField::new(
                                "profile",
                                "display_name",
                                "display_name",
                            )
                            .expect("name result-shape field should build"),
                        )
                },
            )
        })
        .expect("operator-owned frontier search detail should execute");

    assert!(!result.payload().is_empty());
    assert_eq!(
        result.receipt().scope_class(),
        &ForgeQueryReadScopeClass::ExplicitBroadSearch
    );
    assert_eq!(
        result.receipt().built_in_operator_coverage(),
        [ForgeQueryReadBuiltInOperator::FrontierSearch]
    );
    assert_descriptor_admitted_synthetic_runtime_relationship_proof(
        &result,
        3,
        &bounded_ancestor_synthetic_runtime_surfaces(),
    );
}

#[test]
fn compose_read_executes_operator_owned_frontier_search_collection() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.operator-frontier-search-collection")
        .expect("read-backed runtime should open a workspace");

    let result = workspace
        .compose_read(|read| {
            read.explicit_broad_search_frontier_collection(
                "user",
                searchable_expanded_frontier_manager_schema(),
                [manager_relation_name(), mentor_relation_name()],
                2,
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id")
                                .expect("identity projection should build"),
                        )
                        .project(
                            AspectFieldSelector::new("profile", "display_name")
                                .expect("name projection should build"),
                        )
                        .where_equal(
                            EqualityPredicate::new(
                                "profile",
                                "display_name",
                                ScalarPredicateValue::String("Ada".to_string()),
                            )
                            .expect("equality predicate should build"),
                        )
                        .order_by(
                            OrderingSelector::ascending("profile", "display_name")
                                .expect("ordering should build"),
                        )
                },
                |shape| {
                    shape
                        .field(
                            AuthoredResultShapeField::new("identity", "id", "id")
                                .expect("identity result-shape field should build"),
                        )
                        .field(
                            AuthoredResultShapeField::new(
                                "profile",
                                "display_name",
                                "display_name",
                            )
                            .expect("name result-shape field should build"),
                        )
                },
            )
        })
        .expect("operator-owned frontier search collection should execute");

    assert!(!result.payload().is_empty());
    assert_collection_receipt(&result, ForgeQueryReadScopeClass::ExplicitBroadSearch);
    assert_eq!(
        result.receipt().built_in_operator_coverage(),
        [ForgeQueryReadBuiltInOperator::FrontierSearch]
    );
}

#[test]
fn compose_read_denies_frontier_search_without_predicate() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.operator-frontier-search-no-predicate")
        .expect("read-backed runtime should open a workspace");

    let error = workspace
        .compose_read(|read| {
            read.explicit_broad_search_frontier_detail(
                "user",
                expanded_frontier_manager_schema(),
                [manager_relation_name(), mentor_relation_name()],
                2,
                |query| {
                    query.project(
                        AspectFieldSelector::new("identity", "id")
                            .expect("identity projection should build"),
                    )
                },
                |shape| {
                    shape.field(
                        AuthoredResultShapeField::new("identity", "id", "id")
                            .expect("identity result-shape field should build"),
                    )
                },
            )
        })
        .expect_err("frontier search should deny when no predicate is declared");

    match error {
        ForgeQueryRuntimeError::ReadCompositionDenied(denial) => {
            assert_built_in_operator_denial(
                &denial,
                ForgeQueryReadBuiltInOperator::FrontierSearch,
                ForgeQueryReadBuiltInOperatorDenialReason::MissingBroadSearchPredicate,
            );
            assert!(denial.message().contains("requires at least one predicate"));
        }
        other => panic!("expected typed read-composition denial, got {other:?}"),
    }
}

#[test]
fn compose_read_denies_frontier_search_without_relations() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.operator-frontier-search-empty")
        .expect("read-backed runtime should open a workspace");

    let error = workspace
        .compose_read(|read| {
            read.explicit_broad_search_frontier_detail(
                "user",
                searchable_expanded_frontier_manager_schema(),
                Vec::new(),
                2,
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id")
                                .expect("identity projection should build"),
                        )
                        .where_equal(
                            EqualityPredicate::new(
                                "profile",
                                "display_name",
                                ScalarPredicateValue::String("Ada".to_string()),
                            )
                            .expect("equality predicate should build"),
                        )
                },
                |shape| {
                    shape.field(
                        AuthoredResultShapeField::new("identity", "id", "id")
                            .expect("identity result-shape field should build"),
                    )
                },
            )
        })
        .expect_err("frontier search should deny empty frontier relations");

    match error {
        ForgeQueryRuntimeError::ReadCompositionDenied(denial) => {
            assert_built_in_operator_denial(
                &denial,
                ForgeQueryReadBuiltInOperator::FrontierSearch,
                ForgeQueryReadBuiltInOperatorDenialReason::EmptyFrontier,
            );
            assert!(denial.message().contains("frontier search requires"));
        }
        other => panic!("expected typed read-composition denial, got {other:?}"),
    }
}

#[test]
fn compose_read_denies_frontier_search_with_zero_depth() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.operator-frontier-search-zero-depth")
        .expect("read-backed runtime should open a workspace");

    let error = workspace
        .compose_read(|read| {
            read.explicit_broad_search_frontier_detail(
                "user",
                searchable_expanded_frontier_manager_schema(),
                [manager_relation_name(), mentor_relation_name()],
                0,
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id")
                                .expect("identity projection should build"),
                        )
                        .where_equal(
                            EqualityPredicate::new(
                                "profile",
                                "display_name",
                                ScalarPredicateValue::String("Ada".to_string()),
                            )
                            .expect("equality predicate should build"),
                        )
                },
                |shape| {
                    shape.field(
                        AuthoredResultShapeField::new("identity", "id", "id")
                            .expect("identity result-shape field should build"),
                    )
                },
            )
        })
        .expect_err("frontier search should deny zero-depth frontier relations");

    match error {
        ForgeQueryRuntimeError::ReadCompositionDenied(denial) => {
            assert_built_in_operator_denial(
                &denial,
                ForgeQueryReadBuiltInOperator::FrontierSearch,
                ForgeQueryReadBuiltInOperatorDenialReason::ZeroDepth,
            );
            assert!(denial
                .message()
                .contains("frontier search requires max_depth >= 1"));
        }
        other => panic!("expected typed read-composition denial, got {other:?}"),
    }
}

#[test]
fn compose_read_denies_frontier_search_with_duplicate_relations() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.operator-frontier-search-duplicate")
        .expect("read-backed runtime should open a workspace");

    let error = workspace
        .compose_read(|read| {
            read.explicit_broad_search_frontier_collection(
                "user",
                searchable_expanded_frontier_manager_schema(),
                [manager_relation_name(), manager_relation_name()],
                2,
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id")
                                .expect("identity projection should build"),
                        )
                        .where_equal(
                            EqualityPredicate::new(
                                "profile",
                                "display_name",
                                ScalarPredicateValue::String("Ada".to_string()),
                            )
                            .expect("equality predicate should build"),
                        )
                },
                |shape| {
                    shape.field(
                        AuthoredResultShapeField::new("identity", "id", "id")
                            .expect("identity result-shape field should build"),
                    )
                },
            )
        })
        .expect_err("frontier search should deny duplicate frontier relations");

    match error {
        ForgeQueryRuntimeError::ReadCompositionDenied(denial) => {
            assert_built_in_operator_denial(
                &denial,
                ForgeQueryReadBuiltInOperator::FrontierSearch,
                ForgeQueryReadBuiltInOperatorDenialReason::DuplicateFrontierRelation,
            );
            assert!(denial
                .message()
                .contains("frontier search forbids duplicate frontier relations"));
        }
        other => panic!("expected typed read-composition denial, got {other:?}"),
    }
}

#[test]
fn compose_read_denies_frontier_search_with_degenerate_frontier_shape() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.operator-frontier-search-degenerate")
        .expect("read-backed runtime should open a workspace");

    let error = workspace
        .compose_read(|read| {
            read.explicit_broad_search_frontier_detail(
                "user",
                searchable_expanded_frontier_manager_schema(),
                [manager_relation_name(), mentor_relation_name()],
                1,
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id")
                                .expect("identity projection should build"),
                        )
                        .where_equal(
                            EqualityPredicate::new(
                                "profile",
                                "display_name",
                                ScalarPredicateValue::String("Ada".to_string()),
                            )
                            .expect("equality predicate should build"),
                        )
                },
                |shape| {
                    shape.field(
                        AuthoredResultShapeField::new("identity", "id", "id")
                            .expect("identity result-shape field should build"),
                    )
                },
            )
        })
        .expect_err("frontier search should deny one-hop frontier broad-search shapes");

    match error {
        ForgeQueryRuntimeError::ReadCompositionDenied(denial) => {
            assert_built_in_operator_denial(
                &denial,
                ForgeQueryReadBuiltInOperator::FrontierSearch,
                ForgeQueryReadBuiltInOperatorDenialReason::DegenerateFrontierShape,
            );
            assert!(denial
                .message()
                .contains("frontier search requires max_depth > 1"));
        }
        other => panic!("expected typed read-composition denial, got {other:?}"),
    }
}
