use worth_ui_host_contract::{
    UiHostObservationValue, UiMeasurementEvidenceFamily, UiMeasurementRequest,
    UiMeasurementRequestIdentity, UiScrollContainerViewportObservation,
    UiScrollContainerViewportRequest, WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

struct ScrollViewportAdapter {
    width: f32,
    height: f32,
}

impl WorthUiMeasurementHostAdapter for ScrollViewportAdapter {
    fn observe_measurement(&self, _request: &UiMeasurementRequest) -> UiHostObservationValue {
        UiHostObservationValue::ScrollContainerViewport(UiScrollContainerViewportObservation {
            width: self.width,
            height: self.height,
        })
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
    let (mut runtime, _, _, _query, active_receipt, _, _, mut query) =
        super::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let scroll_root = active_receipt.identity().graph_node_identity();
    let projection = query.settle_snapshot();
    let fact_link = runtime.query_fact_link_for_test("inspector.measurements");
    let completion = runtime.execute_framework_turn(|turn| {
        turn.query_projection(|source| {
            source
                .refresh_settled(projection)
                .expect("ordinary Query settlement atomically refreshes exact authority");
            source
                .submit_settled(&fact_link)
                .expect("ordinary Query settlement enters the allocation stream");
        });
    });
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        selection,
        transaction,
        ..
    } = completion
    else {
        panic!("Query content extent must resolve through ordinary locality: {completion:?}")
    };
    assert_eq!(selection.ordered_neighborhoods().len(), 1);
    assert_eq!(
        selection.primary().identity().root_graph_node_identity(),
        scroll_root
    );
    let crate::runtime::UiAllocationReplanTransactionOutcome::Committed(committed) = transaction
    else {
        panic!("Query content extent must commit the selected neighborhood")
    };
    let evidence = committed
        .scroll_owned_evidence()
        .first()
        .expect("Query content replan retains source lineage");
    assert_eq!(
        evidence.cause(),
        crate::evidence::UiScrollOwnedExtentCause::QueryContentExtent
    );
    assert_eq!(evidence.committed_receipts(), 1);
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
