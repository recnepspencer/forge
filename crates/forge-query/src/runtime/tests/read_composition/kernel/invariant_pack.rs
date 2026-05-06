use super::super::support::*;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField, TraversalSelector};
use crate::runtime::{
    ForgeQueryReadGraphFamily, ForgeQueryReadInvariantPackViolation, ForgeQueryReadOperatorFamily,
    ForgeQueryReadScopeClass, ForgeQueryRuntimeError,
};

#[test]
fn compose_read_with_invariant_pack_executes_when_pack_admits_read_graph() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.invariant-pack-admitted")
        .expect("read-backed runtime should open a workspace");

    let result = workspace
        .compose_read_with_invariant_pack(
            |read| {
                read.local_detail(
                    "user",
                    manager_schema(),
                    |query| {
                        query
                            .project(
                                AspectFieldSelector::new("identity", "id")
                                    .expect("identity projection should build"),
                            )
                            .traverse(
                                TraversalSelector::bounded("manager", 1)
                                    .expect("bounded traversal should build"),
                            )
                    },
                    |shape| {
                        shape.field(
                            AuthoredResultShapeField::new("identity", "id", "id")
                                .expect("identity result-shape field should build"),
                        )
                    },
                )
            },
            |context| {
                let summary = context.read_domain_invariant_summary();
                assert_eq!(summary.scope_class(), "local_neighborhood");
                assert!(summary
                    .operator_families()
                    .contains(&ForgeQueryReadOperatorFamily::Traversal));
                assert!(summary.built_in_operator_coverage().is_empty());
                Ok(())
            },
        )
        .expect("admitted invariant packs should allow read execution");

    assert!(!result.payload().is_empty());
    assert_eq!(
        result.receipt().scope_class(),
        &ForgeQueryReadScopeClass::LocalNeighborhood
    );
}

#[test]
fn compose_read_with_invariant_pack_denies_through_typed_domain_invariant_lane() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.invariant-pack-denied")
        .expect("read-backed runtime should open a workspace");

    let error = workspace
        .compose_read_with_invariant_pack(
            |read| {
                read.local_detail(
                    "user",
                    manager_schema(),
                    |query| {
                        query
                            .project(
                                AspectFieldSelector::new("identity", "id")
                                    .expect("identity projection should build"),
                            )
                            .traverse(
                                TraversalSelector::bounded("manager", 1)
                                    .expect("bounded traversal should build"),
                            )
                    },
                    |shape| {
                        shape.field(
                            AuthoredResultShapeField::new("identity", "id", "id")
                                .expect("identity result-shape field should build"),
                        )
                    },
                )
            },
            |context| {
                let summary = context.read_domain_invariant_summary();
                if summary.declared_traversal_clause_count() > 0 {
                    Err(ForgeQueryReadInvariantPackViolation::new(
                        "no_traversal_reads",
                        "this domain hook denies traversal-bearing reads",
                    ))
                } else {
                    Ok(())
                }
            },
        )
        .expect_err("denied invariant packs should reject before execution");

    match error {
        ForgeQueryRuntimeError::ReadCompositionDomainInvariantDenied(denial) => {
            assert_eq!(denial.hook_family(), "domain_invariant_pack_hook");
            assert_eq!(denial.invariant_family(), "no_traversal_reads");
            assert_eq!(
                denial.message(),
                "this domain hook denies traversal-bearing reads"
            );
            assert_eq!(
                denial.domain_invariant_summary().graph_family(),
                &ForgeQueryReadGraphFamily::Detail
            );
            assert_eq!(
                denial.domain_invariant_summary().scope_class(),
                "local_neighborhood"
            );
            assert_eq!(
                denial
                    .domain_invariant_summary()
                    .declared_traversal_clause_count(),
                1
            );
            assert_eq!(
                denial
                    .domain_invariant_summary()
                    .declared_traversal_depth_limit(),
                1
            );
            assert!(denial
                .domain_invariant_summary()
                .built_in_operator_coverage()
                .is_empty());
            assert!(!denial.denial_digest().is_empty());
            assert!(!denial
                .domain_invariant_summary()
                .summary_digest()
                .is_empty());
        }
        other => panic!("expected typed read domain invariant denial, got {other:?}"),
    }
}
