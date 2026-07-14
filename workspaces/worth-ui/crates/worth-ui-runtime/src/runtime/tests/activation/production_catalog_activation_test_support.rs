use super::activation_staging_test_support::activation_staging_inputs;

#[path = "production_catalog_activation_test_support/catalog_fixtures.rs"]
mod catalog_fixtures;
pub(crate) use catalog_fixtures::{runtime_with_portal_catalog, runtime_with_scroll_catalog};

pub(crate) fn runtime_with_production_catalog_activation() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    crate::graph::UiGraphNodeIdentity,
    crate::graph::UiGraphNodeIdentity,
    crate::runtime::UiAllocationCandidate,
) {
    let (runtime, roots, planning) = runtime_with_viewport_catalog(2);
    (runtime, roots[0], roots[1], planning)
}

pub(crate) fn runtime_with_viewport_catalog(
    count: usize,
) -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    Box<[crate::graph::UiGraphNodeIdentity]>,
    crate::runtime::UiAllocationCandidate,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (runtime, roots, planning, _, _, _, _, _, _) =
        activate_viewport_catalog(runtime, pending, count);
    (runtime, roots, planning)
}

pub(crate) fn runtime_with_durable_resize_catalog() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    crate::graph::UiGraphNodeIdentity,
    crate::runtime::WorthUiAdmittedDurableResizeInput,
) {
    let (snapshot, admissions) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_split_planning_admissions(
            "production-catalog-activation",
            2,
        );
    let (basis, selected) = &admissions[1];
    let provenance = basis
        .admit_allocation_neighborhood(&snapshot, selected)
        .expect("catalog neighborhood admits")
        .members()
        .iter()
        .find(|member| {
            matches!(
                member.role(),
                crate::evidence::UiAllocationNeighborhoodMemberRole::Root
            )
        })
        .expect("splitter catalog exposes its structural root")
        .authored_provenance_digest();
    let (runtime, pending, _) = crate::runtime::tests::durable_resize_input_boundary_tests::splitter_pending_activation_with_provenance(provenance);
    let (runtime, _, _, input, durable_root, _, _, _, _) = activate_catalog(
        runtime,
        pending,
        2,
        false,
        false,
        Some((snapshot, admissions)),
    );
    (
        runtime,
        durable_root.expect("splitter catalog owns durable target"),
        input.expect("splitter activation owns durable input"),
    )
}

fn activate_viewport_catalog(
    runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    pending: crate::runtime::WorthUiPendingActivation,
    count: usize,
) -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    Box<[crate::graph::UiGraphNodeIdentity]>,
    crate::runtime::UiAllocationCandidate,
    Option<crate::runtime::WorthUiAdmittedDurableResizeInput>,
    Option<crate::graph::UiGraphNodeIdentity>,
    crate::runtime::UiAllocationReceipt,
    crate::runtime::UiAllocationReceipt,
    Option<crate::evidence::UiProjectionFactReceipt>,
    Option<crate::runtime::UiCommittedAllocationEvidenceSet>,
) {
    activate_catalog(runtime, pending, count, false, false, None)
}

fn activate_catalog(
    mut runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    pending: crate::runtime::WorthUiPendingActivation,
    count: usize,
    scroll: bool,
    portal: bool,
    admitted_inputs: Option<(
        crate::graph::UiGraphSnapshot,
        Vec<(
            crate::evidence::UiMeasurementBasis,
            crate::obligations::selection::UiSelectedObligationSet,
        )>,
    )>,
) -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    Box<[crate::graph::UiGraphNodeIdentity]>,
    crate::runtime::UiAllocationCandidate,
    Option<crate::runtime::WorthUiAdmittedDurableResizeInput>,
    Option<crate::graph::UiGraphNodeIdentity>,
    crate::runtime::UiAllocationReceipt,
    crate::runtime::UiAllocationReceipt,
    Option<crate::evidence::UiProjectionFactReceipt>,
    Option<crate::runtime::UiCommittedAllocationEvidenceSet>,
) {
    let durable_resize = pending
        .staged_replacement()
        .reconciliation_plan()
        .admitted_durable_resize_input("surface:main")
        .cloned();
    let (snapshot, admissions) = if let Some(admitted_inputs) = admitted_inputs {
        admitted_inputs
    } else if scroll {
        crate::runtime::tests::allocation_catalog_test_support::admitted_scroll_planning_admissions(
            "production-scroll-catalog-activation",
            count,
        )
    } else if portal {
        crate::runtime::tests::allocation_catalog_test_support::admitted_portal_planning_admissions(
            "production-portal-catalog-activation",
            count,
        )
    } else if durable_resize.is_some() {
        crate::runtime::tests::allocation_catalog_test_support::admitted_split_planning_admissions(
            "production-catalog-activation",
            count,
        )
    } else {
        crate::runtime::tests::allocation_catalog_test_support::admitted_viewport_planning_admissions(
            "production-catalog-activation",
            count,
        )
    };
    let admitted_catalog = snapshot
        .admit_allocation_catalog_basis_set(admissions)
        .expect("graph admits complete catalog basis");
    let candidates = admitted_catalog
        .entries
        .iter()
        .map(|(basis, selected)| {
            let input = runtime
                .admit_planning_lane_input(&pending, &snapshot, basis.clone(), selected)
                .expect("admitted catalog retains each canonical planning input");
            runtime.plan_allocation(input)
        })
        .collect::<Vec<_>>();
    let planning = if scroll {
        candidates
            .iter()
            .find(|candidate| {
                candidate
                    .measurement_basis()
                    .evidence_inputs()
                    .iter()
                    .any(|input| input.as_query_projection_fact().is_some())
            })
            .expect("scroll catalog exposes its structural scroll-owned candidate")
            .clone()
    } else if portal {
        candidates
            .iter()
            .find(|candidate| {
                candidate
                    .allocation_constraint_set()
                    .and_then(
                        crate::evidence::UiAllocationConstraintSet::portal_anchor_planning_input,
                    )
                    .is_some()
            })
            .expect("portal catalog exposes its structural portal-owned candidate")
            .clone()
    } else {
        candidates[0].clone()
    };
    let query = candidates.iter().find_map(|candidate| {
        candidate
            .measurement_basis()
            .evidence_inputs()
            .iter()
            .find_map(crate::evidence::MeasurementEvidenceInput::as_query_projection_fact)
            .cloned()
    });
    let durable_root = durable_resize.as_ref().and_then(|input| {
        candidates.iter().find_map(|candidate| {
            candidate
                .measurement_basis()
                .durable_resize_support(input.identity_digest())
                .map(|_| candidate.measurement_basis().graph_node_identity())
        })
    });
    if let Some(input) = durable_resize.as_ref() {
        let root_provenance = candidates
            .iter()
            .map(|candidate| {
                candidate
                    .allocation_neighborhood()
                    .members()
                    .iter()
                    .find(|member| {
                        matches!(
                            member.role(),
                            crate::evidence::UiAllocationNeighborhoodMemberRole::Root
                        )
                    })
                    .expect("candidate neighborhood has a root")
                    .authored_provenance_digest()
            })
            .collect::<Vec<_>>();
        assert!(
            candidates.iter().any(|candidate| candidate
                .measurement_basis()
                .durable_resize_inputs()
                .any(|identity| identity == input.identity_digest())),
            "canonical planning must attach pending durable-resize provenance {:?} to one of its structural split roots {root_provenance:?}",
            input.authored_provenance_digest()
        );
    }
    let roots = candidates
        .iter()
        .map(|candidate| candidate.measurement_basis().graph_node_identity())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let boundary = runtime.safe_frame_boundary();
    let swap_receipt = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(
            pending,
            admitted_catalog,
            boundary,
            None,
        )
        .expect("production catalog activation owns the atomic swap");
    let committed_evidence = swap_receipt
        .committed_allocation()
        .committed_evidence()
        .clone();
    assert_eq!(swap_receipt.committed_allocation().receipts().len(), count);
    let planning_basis_identity = planning.measurement_basis().identity_digest();
    let receipt = swap_receipt
        .committed_allocation()
        .catalog_bindings()
        .rows()
        .iter()
        .find(|binding| binding.measurement_basis_identity_digest() == planning_basis_identity)
        .expect("canonical catalog binding identifies the planning basis")
        .receipt()
        .clone();
    let unrelated_receipt = swap_receipt
        .committed_allocation()
        .catalog_bindings()
        .rows()
        .iter()
        .find(|binding| binding.measurement_basis_identity_digest() != planning_basis_identity)
        .expect("catalog includes a structurally unrelated receipt")
        .receipt()
        .clone();
    let committed_binding_identity = swap_receipt
        .committed_allocation()
        .catalog_bindings()
        .identity_digest();
    let scroll_catalog = swap_receipt
        .scroll_owner_catalog()
        .expect("atomic swap seals scroll-owner catalog evidence");
    assert!(matches!(
        swap_receipt.scroll_catalog_evidence(),
        crate::runtime::UiScrollCatalogSwapEvidence::Prepared(receipt)
            if receipt == scroll_catalog
    ));
    assert_eq!(scroll_catalog.counters().context_reads(), count as u16);
    assert_eq!(
        scroll_catalog.identity().activation_keys().len(),
        scroll_catalog.owner_count() as usize
    );
    assert_ne!(scroll_catalog.catalog_identity_digest(), 0);
    assert_ne!(scroll_catalog.successor_identity_digest(), 0);
    assert_ne!(
        scroll_catalog.predecessor_identity_digest(),
        scroll_catalog.successor_identity_digest()
    );
    assert_eq!(
        scroll_catalog.committed_binding_identity_digest(),
        committed_binding_identity
    );
    assert_eq!(
        scroll_catalog.virtualization(),
        crate::runtime::UiScrollVirtualizationPosture::NonVirtualized
    );
    assert_eq!(
        scroll_catalog.offset_allocation(),
        crate::runtime::UiScrollOffsetAllocationPosture::ProjectedInteractionOnly
    );
    assert_eq!(runtime.committed_allocation_scope_count_for_test(), count);
    (
        runtime,
        roots,
        planning,
        durable_resize,
        durable_root,
        receipt,
        unrelated_receipt,
        query,
        Some(committed_evidence),
    )
}

#[test]
fn successful_catalog_activation_publishes_every_receipt_once() {
    let (runtime, _, _, _) = runtime_with_production_catalog_activation();
    assert_eq!(runtime.committed_allocation_scope_count_for_test(), 2);
}

#[test]
fn denied_catalog_activation_never_publishes_prepared_receipts() {
    let inputs = activation_staging_inputs();
    let (mut runtime, pending) = inputs.into_runtime_and_pending();
    assert!(runtime.durable_semantic_state().is_none());
    let (snapshot, first, second) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_disjoint_planning_admissions(
            "catalog-rollback",
        );
    let admitted = snapshot
        .admit_allocation_catalog_basis_set(vec![first, second])
        .expect("graph admits complete catalog basis");
    assert_eq!(runtime.committed_allocation_scope_count_for_test(), 0);
    let denied_boundary = runtime.traversal_frame_boundary_for_test();
    let denial = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(
            pending,
            admitted,
            denied_boundary,
            None,
        )
        .expect_err("unsafe boundary denies the complete activation attempt");
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("post-mint denial carries canonical attempt evidence")
    };
    let crate::runtime::UiCommittedAllocationActivationDenialReason::FrameBoundary(gate_denial) =
        denial.reason()
    else {
        panic!("unsafe boundary denies during immutable transaction preflight")
    };
    assert_eq!(
        gate_denial.reason(),
        crate::runtime::WorthUiActivationGateDenialReason::UnsafeFrameBoundary
    );
    assert!(denial.evidence().live_state_unchanged());
    assert_eq!(denial.evidence().committed_row_count(), 2);
    assert_eq!(denial.evidence().counters().denial_count(), 1);
    assert_ne!(denial.attempt_identity_digest(), 0);
    assert_eq!(runtime.committed_allocation_scope_count_for_test(), 0);
    assert!(runtime.durable_semantic_state().is_none());
}

#[test]
fn catalog_activation_denies_multi_receipt_and_durable_replacement_atomically() {
    for remaining in [0, 1, 2] {
        let inputs = activation_staging_inputs();
        let (runtime, pending) = inputs.into_runtime_and_pending();
        let predecessor = runtime
            .allocation_receipt_ledger
            .position_truth_revision_for_test(remaining);
        let (snapshot, admissions) =
            crate::runtime::tests::allocation_catalog_test_support::admitted_viewport_planning_admissions(
                "catalog-authority-exhaustion",
                2,
            );
        let admitted = snapshot
            .admit_allocation_catalog_basis_set(admissions)
            .expect("two-neighborhood catalog admits");
        let denial = runtime
            .prepare_allocation_catalog_activation(&pending, admitted)
            .expect_err("exhausted catalog publication denies before activation");
        let crate::runtime::launch::UiAllocationCatalogPreparationDenial::ReceiptCommit(
            crate::runtime::UiAllocationReceiptCommitOutcome::Denied(
                crate::runtime::UiAllocationReceiptCommitDenial::AuthorityCounterExhausted(
                    exhaustion,
                ),
            ),
        ) = denial
        else {
            panic!("catalog exhaustion remains typed")
        };
        assert_eq!(
            exhaustion.counter(),
            crate::runtime::UiAllocationAuthorityCounter::TruthRevision
        );
        assert_eq!(exhaustion.increment(), 3);
        assert_eq!(
            runtime.allocation_receipt_ledger.ledger_baseline_for_test(),
            predecessor
        );
        assert_eq!(runtime.committed_allocation_scope_count_for_test(), 0);
        assert!(runtime.durable_semantic_state().is_none());
    }
}
