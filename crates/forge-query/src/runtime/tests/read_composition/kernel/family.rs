use super::super::support::*;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField, TraversalSelector};
use crate::runtime::{
    ForgeQueryReadFamily, ForgeQueryReadFamilyAdmission, ForgeQueryReadGraphFamily,
    ForgeQueryReadInvariantPackViolation, ForgeQueryReadScopeClass, ForgeQueryRuntimeError,
};

#[test]
fn define_read_family_freezes_reusable_read_graph_artifact() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.family-define")
        .expect("read-backed runtime should open a workspace");

    let family = workspace
        .define_read_family("manager-chain", |read| {
            read.anchored_detail(
                "user",
                expanded_manager_schema(),
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id")
                                .expect("identity projection should build"),
                        )
                        .traverse(
                            TraversalSelector::bounded("manager", 2)
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
        })
        .expect("read family should define");

    assert_eq!(family.family_name(), "manager-chain");
    assert!(!family.family_digest().is_empty());
    assert_eq!(
        family.read_graph().scope_class(),
        &ForgeQueryReadScopeClass::AnchoredExpansion
    );
    assert_eq!(
        family.read_graph().family(),
        &ForgeQueryReadGraphFamily::Detail
    );
    assert_eq!(
        family.admission(),
        &ForgeQueryReadFamilyAdmission::KernelOnly
    );
}

#[test]
fn execute_read_family_reuses_same_canonical_read_graph_across_runs() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.family-execute")
        .expect("read-backed runtime should open a workspace");

    let family = workspace
        .define_read_family("manager-chain", |read| {
            read.anchored_detail(
                "user",
                expanded_manager_schema(),
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
                        .traverse(
                            TraversalSelector::bounded("manager", 2)
                                .expect("bounded traversal should build"),
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
        .expect("read family should define");

    let first = workspace
        .execute_read_family(&family)
        .expect("first read-family execution should succeed");
    let second = workspace
        .execute_read_family(&family)
        .expect("second read-family execution should succeed");

    assert_eq!(
        first.receipt().read_graph_digest(),
        family.read_graph().digest()
    );
    assert_eq!(
        second.receipt().read_graph_digest(),
        family.read_graph().digest()
    );
    assert_eq!(
        first.receipt().query_digest(),
        second.receipt().query_digest()
    );
    assert_eq!(
        first.receipt().scope_class(),
        &ForgeQueryReadScopeClass::AnchoredExpansion
    );
}

#[test]
fn define_read_family_with_invariant_pack_denies_through_typed_domain_lane() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.family-invariant-denial")
        .expect("read-backed runtime should open a workspace");

    let error = workspace
        .define_read_family_with_invariant_pack(
            "manager-chain",
            "single_hop_family_forbidden",
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
                if summary.declared_traversal_clause_count() == 1 {
                    Err(ForgeQueryReadInvariantPackViolation::new(
                        "single_hop_family_forbidden",
                        "this reusable family forbids single-hop traversal",
                    ))
                } else {
                    Ok(())
                }
            },
        )
        .expect_err("denied invariant packs should reject reusable family definition");

    match error {
        ForgeQueryRuntimeError::ReadCompositionDomainInvariantDenied(denial) => {
            assert_eq!(denial.invariant_family(), "single_hop_family_forbidden");
            assert_eq!(
                denial.domain_invariant_summary().graph_family(),
                &ForgeQueryReadGraphFamily::Detail
            );
            assert_eq!(
                denial.domain_invariant_summary().scope_class(),
                "local_neighborhood"
            );
        }
        other => panic!("expected typed read domain invariant denial, got {other:?}"),
    }
}

#[test]
fn invariant_admitted_family_carries_distinct_admission_evidence() {
    let mut workspace = read_runtime()
        .workspace("runtime.read-composition.family-invariant-evidence")
        .expect("read-backed runtime should open a workspace");

    let plain = workspace
        .define_read_family("manager-chain", |read| {
            read.anchored_detail(
                "user",
                expanded_manager_schema(),
                |query| {
                    query
                        .project(
                            AspectFieldSelector::new("identity", "id")
                                .expect("identity projection should build"),
                        )
                        .traverse(
                            TraversalSelector::bounded("manager", 2)
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
        })
        .expect("plain read family should define");

    let invariant_admitted = workspace
        .define_read_family_with_invariant_pack(
            "manager-chain",
            "manager_depth_budget",
            |read| {
                read.anchored_detail(
                    "user",
                    expanded_manager_schema(),
                    |query| {
                        query
                            .project(
                                AspectFieldSelector::new("identity", "id")
                                    .expect("identity projection should build"),
                            )
                            .traverse(
                                TraversalSelector::bounded("manager", 2)
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
            |_context| Ok(()),
        )
        .expect("invariant-admitted read family should define");

    assert_ne!(plain.family_digest(), invariant_admitted.family_digest());
    match invariant_admitted.admission() {
        ForgeQueryReadFamilyAdmission::KernelOnly => {
            panic!("expected invariant-admitted reusable family evidence")
        }
        ForgeQueryReadFamilyAdmission::DomainInvariantAdmitted(evidence) => {
            assert_eq!(evidence.invariant_family(), "manager_depth_budget");
            assert_eq!(
                evidence.domain_invariant_summary().scope_class(),
                "anchored_expansion"
            );
        }
    }
}

fn _read_family_type_is_public(_family: &ForgeQueryReadFamily) {}
