use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiMeasurementEvidenceFamily,
    UiMeasurementRequestIdentity, UiScrollContainerViewportObservation,
    UiScrollContainerViewportRequest, WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

struct ScrollViewportAdapter {
    width: f32,
    height: f32,
}

#[test]
fn committed_scroll_truth_enters_observation_without_runtime_effects() {
    let (mut runtime, _, _, _, _, _, _, _) =
        super::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let truth = runtime.allocation_truth_revision();
    let catalog_len = runtime.allocation_invalidation_index.borrow().catalog.len();
    let dispatcher = runtime.allocation_frame_dispatcher_state();
    let counters = runtime.allocation_frame_dispatcher_counters();
    let session =
        crate::facade::WorthUiActiveApplicationSessionIdentity::from_host_session_value(91);

    let mut turn = runtime.begin_observation_turn(session, 77).unwrap();
    let receipt = turn.admit_committed_runtime_state().unwrap();
    assert_eq!(receipt.admitted().len(), 1);
    let admitted = turn.seal().unwrap();
    let scroll = admitted.observations()[0]
        .committed_scroll_extent()
        .expect("scroll catalog emits only committed scroll truth");
    assert_eq!(scroll.allocation_truth_revision(), truth);
    assert!(!scroll.source_identity_digests().is_empty());
    assert_eq!(runtime.allocation_truth_revision(), truth);
    assert_eq!(
        runtime.allocation_invalidation_index.borrow().catalog.len(),
        catalog_len
    );
    assert_eq!(runtime.allocation_frame_dispatcher_state(), dispatcher);
    assert_eq!(runtime.allocation_frame_dispatcher_counters(), counters);

    let mut repeated = runtime.begin_observation_turn(session, 77).unwrap();
    assert_eq!(
        repeated.admit_committed_runtime_state(),
        Err(crate::runtime::observation::UiObservationAdmissionDenial::HistoricalOwnerOrder)
    );
}

impl WorthUiMeasurementHostAdapter for ScrollViewportAdapter {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        UiHostMeasurementObservationValue::ScrollContainerViewport(
            UiScrollContainerViewportObservation {
                width: self.width,
                height: self.height,
            },
        )
    }
}

#[test]
fn phase_10_scroll_viewport_extent_replans_only_the_owned_neighborhood() {
    let (mut runtime, roots, _, _, active_receipt, _, _, _) =
        super::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let scroll_root = active_receipt.identity().graph_node_identity();
    let unrelated_roots = roots
        .iter()
        .copied()
        .filter(|root| *root != scroll_root)
        .collect::<Vec<_>>();
    let report = crate::evidence::measurement::projection::fact_test_support::capability_report(77);
    let profile = crate::host::UiHostMeasurementAssumptionProfile::from_capability_report(
        &report, 11, 22, 33, 44,
    );
    let completion = runtime.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            source
                .collect_and_submit(
                    &ScrollViewportAdapter { width: 180.0, height: 90.0 },
                    crate::host::UiHostMeasurementCollectionInput {
                        identity: UiMeasurementRequestIdentity::new(951),
                        evidence_family: UiMeasurementEvidenceFamily::ScrollContainerViewport,
                        need: crate::host::UiHostMeasurementNeed::ScrollContainerViewport(
                            UiScrollContainerViewportRequest::new(55),
                        ),
                        capability_report: &report,
                        evidence_generation: UiEvidenceAuthorityGeneration::new(17),
                        normalization_context:
                            crate::host::UiHostMeasurementNormalizationContext::scroll_container_logical_exact(profile),
                    },
                )
                .expect("ordinary host observation enters the allocation stream");
        });
    });
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        selection,
        transaction,
        ..
    } = completion
    else {
        panic!("scroll viewport extent must resolve through the ordinary locality path: {completion:?}")
    };
    assert_eq!(selection.ordered_neighborhoods().len(), 1);
    assert_eq!(
        selection.primary().identity().root_graph_node_identity(),
        scroll_root
    );
    assert!(unrelated_roots.iter().all(|root| {
        selection
            .ordered_neighborhoods()
            .iter()
            .all(|selected| selected.identity().root_graph_node_identity() != *root)
    }));
    let crate::runtime::UiAllocationReplanTransactionOutcome::Committed(committed) = transaction
    else {
        panic!("scroll viewport locality must commit one canonical replan")
    };
    assert_eq!(committed.receipts().len(), 1);
    let evidence = committed
        .scroll_owned_evidence()
        .first()
        .expect("committed scroll replan retains its local why evidence");
    assert_eq!(
        committed.committed_evidence().scroll_owned(),
        committed.scroll_owned_evidence()
    );
    assert_eq!(
        evidence.cause(),
        crate::evidence::UiScrollOwnedExtentCause::HostContainerViewport
    );
    assert_eq!(evidence.actual_invalidations(), 1);
    assert_eq!(evidence.committed_receipts(), 1);
}

#[test]
fn phase_10_query_content_extent_replans_only_the_bound_scroll_neighborhood() {
    let (mut runtime, _, _, predecessor_query, active_receipt, _, _, mut query) =
        super::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let fact_link = runtime.query_fact_link_for_test("inspector.measurements");
    let (successor_fact, committed) =
        commit_query_refresh(&mut runtime, &mut query, &fact_link, &active_receipt);
    let (successor_query, successor_allocation) = assert_query_commit_lineage(
        &committed,
        &predecessor_query,
        &active_receipt,
        &successor_fact,
    );
    assert_query_owner_succession(
        &mut runtime,
        &predecessor_query,
        &active_receipt,
        &successor_query,
        &successor_allocation,
    );

    let (second_fact, second_commit) =
        commit_query_refresh(&mut runtime, &mut query, &fact_link, &successor_allocation);
    let (second_query, second_allocation) = assert_query_commit_lineage(
        &second_commit,
        &successor_query,
        &successor_allocation,
        &second_fact,
    );
    assert_query_owner_succession(
        &mut runtime,
        &successor_query,
        &successor_allocation,
        &second_query,
        &second_allocation,
    );
}

fn commit_query_refresh(
    runtime: &mut crate::runtime::WorthUiRuntimeFrameworkLoop,
    query: &mut worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture,
    fact_link: &crate::runtime::WorthUiQueryLaneFactLink,
    predecessor: &crate::runtime::UiAllocationReceipt,
) -> (
    std::sync::Arc<worth_ui_query_binding::WorthUiSettledSnapshotFact>,
    crate::runtime::UiCommittedAllocationReplan,
) {
    let projection = query.settle_snapshot();
    let mut successor_fact = None;
    let completion = runtime.execute_framework_turn(|turn| {
        turn.query_projection(|source| {
            successor_fact = Some(
                source
                    .refresh_settled(projection)
                    .expect("the exact binding atomically replaces its settlement"),
            );
            source
                .submit_settled(fact_link)
                .expect("the stable plan link resolves the current consequence");
        });
    });
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        selection,
        transaction,
        ..
    } = completion
    else {
        panic!("Query refresh must remain on the ordinary allocation path: {completion:?}")
    };
    assert_eq!(selection.ordered_neighborhoods().len(), 1);
    assert_eq!(
        selection.primary().identity().root_graph_node_identity(),
        predecessor.identity().graph_node_identity()
    );
    let crate::runtime::UiAllocationReplanTransactionOutcome::Committed(committed) = transaction
    else {
        panic!("Query refresh must commit the selected neighborhood: {transaction:?}")
    };
    (
        successor_fact.expect("refresh returns the current UI consequence"),
        committed,
    )
}

fn assert_query_commit_lineage(
    committed: &crate::runtime::UiCommittedAllocationReplan,
    predecessor_query: &crate::evidence::UiSettledQueryFactReceipt,
    predecessor_allocation: &crate::runtime::UiAllocationReceipt,
    fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
) -> (
    crate::evidence::UiSettledQueryFactReceipt,
    crate::runtime::UiAllocationReceipt,
) {
    let evidence = committed
        .scroll_owned_evidence()
        .first()
        .expect("Query content replan retains source lineage");
    assert_eq!(
        evidence.cause(),
        crate::evidence::UiScrollOwnedExtentCause::QueryContentExtent
    );
    assert_eq!(evidence.committed_receipts(), 1);
    assert!(committed.scroll_binding_succession().is_some());
    let successor_query = successor_query_receipt(
        predecessor_query,
        predecessor_allocation
            .committed_allocation()
            .measurement_basis(),
        fact,
    );
    let successor_allocation = committed.receipts()[0].clone();
    assert!(successor_allocation
        .committed_allocation()
        .measurement_basis()
        .evidence_inputs()
        .iter()
        .any(|input| input.as_settled_query_fact() == Some(&successor_query)));
    (successor_query, successor_allocation)
}

fn assert_query_owner_succession(
    runtime: &mut crate::runtime::WorthUiRuntimeFrameworkLoop,
    predecessor_query: &crate::evidence::UiSettledQueryFactReceipt,
    predecessor_allocation: &crate::runtime::UiAllocationReceipt,
    successor_query: &crate::evidence::UiSettledQueryFactReceipt,
    successor_allocation: &crate::runtime::UiAllocationReceipt,
) {
    let mut first_successor_denial = None;
    let mut second_successor_owner = None;
    let completion = runtime.execute_framework_turn(|turn| {
        turn.scroll_offset(|source| {
            first_successor_denial = source
                .acquire_settled_query_owner(predecessor_query, predecessor_allocation)
                .err();
            second_successor_owner = source
                .acquire_settled_query_owner(successor_query, successor_allocation)
                .ok();
        });
    });
    drop(completion);
    assert_eq!(
        first_successor_denial,
        Some(crate::runtime::UiScrollOwnerAcquisitionDenial::ContradictorySource)
    );
    assert!(second_successor_owner.is_some());
}

fn successor_query_receipt(
    predecessor: &crate::evidence::UiSettledQueryFactReceipt,
    basis: &crate::evidence::UiMeasurementBasis,
    fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
) -> crate::evidence::UiSettledQueryFactReceipt {
    crate::evidence::consume_settled_query_measurement_fact(
        predecessor.declaration_identity().clone(),
        predecessor.declaration_support_authority_generation(),
        basis.declared_measurement_policy(),
        predecessor.view_binding_id().clone(),
        fact,
    )
    .expect("the current UI consequence satisfies the same declared measurement contract")
}

#[test]
fn panicking_source_collection_discards_a_valid_partial_scroll_replan() {
    let (mut runtime, _, _, _, _, _, _, _) =
        super::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let report = crate::evidence::measurement::projection::fact_test_support::capability_report(77);
    let profile = crate::host::UiHostMeasurementAssumptionProfile::from_capability_report(
        &report, 11, 22, 33, 44,
    );
    let truth_before = runtime.allocation_receipt_ledger.truth_revision();
    let committed_scopes_before = runtime.committed_allocation_scope_count_for_test();

    let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = runtime.execute_framework_turn(|turn| {
            turn.host_measurement(|source| {
                source
                    .collect_and_submit(
                        &ScrollViewportAdapter {
                            width: 180.0,
                            height: 90.0,
                        },
                        crate::host::UiHostMeasurementCollectionInput {
                            identity: UiMeasurementRequestIdentity::new(952),
                            evidence_family:
                                UiMeasurementEvidenceFamily::ScrollContainerViewport,
                            need: crate::host::UiHostMeasurementNeed::ScrollContainerViewport(
                                UiScrollContainerViewportRequest::new(55),
                            ),
                            capability_report: &report,
                            evidence_generation: UiEvidenceAuthorityGeneration::new(18),
                            normalization_context:
                                crate::host::UiHostMeasurementNormalizationContext::scroll_container_logical_exact(profile),
                        },
                    )
                    .expect("valid host observation enters the partial turn");
            });
            panic!("collection failed after valid ingress");
        });
    }));

    assert!(unwind.is_err());
    assert_eq!(
        runtime.allocation_receipt_ledger.truth_revision(),
        truth_before
    );
    assert_eq!(
        runtime.committed_allocation_scope_count_for_test(),
        committed_scopes_before
    );
    assert!(runtime
        .execute_framework_turn(|_| {})
        .into_execution()
        .is_ok());
}

#[test]
fn phase_10_unadmitted_scroll_ownership_denies_without_host_fallback() {
    let (mut runtime, _, _) =
        super::production_catalog_activation_test_support::runtime_with_viewport_catalog(2);
    let committed_before = runtime.committed_allocation_scope_count_for_test();
    let report = crate::evidence::measurement::projection::fact_test_support::capability_report(77);
    let profile = crate::host::UiHostMeasurementAssumptionProfile::from_capability_report(
        &report, 11, 22, 33, 44,
    );
    let completion = runtime.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            source
                .collect_and_submit(
                    &ScrollViewportAdapter { width: 180.0, height: 90.0 },
                    crate::host::UiHostMeasurementCollectionInput {
                        identity: UiMeasurementRequestIdentity::new(951),
                        evidence_family: UiMeasurementEvidenceFamily::ScrollContainerViewport,
                        need: crate::host::UiHostMeasurementNeed::ScrollContainerViewport(
                            UiScrollContainerViewportRequest::new(55),
                        ),
                        capability_report: &report,
                        evidence_generation: UiEvidenceAuthorityGeneration::new(17),
                        normalization_context:
                            crate::host::UiHostMeasurementNormalizationContext::scroll_container_logical_exact(profile),
                    },
                )
                .expect("host observation is valid evidence even when allocation ownership is absent");
        });
    });
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationNarrowingDenied {
        rejection,
    } = completion
    else {
        panic!("unadmitted scroll ownership must deny before publication: {completion:?}")
    };
    assert_eq!(
        rejection.denial(),
        crate::runtime::UiAllocationInvalidationNarrowingDenial::ScrollOwnershipNotAdmitted {
            ordinal: 0,
        }
    );
    assert_eq!(
        runtime.committed_allocation_scope_count_for_test(),
        committed_before
    );
}
