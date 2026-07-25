use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiMeasurementEvidenceFamily,
    UiMeasurementRequestIdentity, UiPortalAnchorRectObservation, UiPortalAnchorRectRequest,
    UiScrollContainerViewportObservation, UiScrollContainerViewportRequest,
    UiViewportExtentObservation, UiViewportExtentRequest, WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

struct ViewportAdapter;
struct ScrollAdapter;
struct PortalAdapter;

impl WorthUiMeasurementHostAdapter for ViewportAdapter {
    fn observe_measurement(
        &self,
        _: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        UiHostMeasurementObservationValue::ViewportExtent(UiViewportExtentObservation {
            width: 1024.0,
            height: 768.0,
        })
    }
}

impl WorthUiMeasurementHostAdapter for ScrollAdapter {
    fn observe_measurement(
        &self,
        _: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        UiHostMeasurementObservationValue::ScrollContainerViewport(
            UiScrollContainerViewportObservation {
                width: 320.0,
                height: 180.0,
            },
        )
    }
}

impl WorthUiMeasurementHostAdapter for PortalAdapter {
    fn observe_measurement(
        &self,
        _: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        UiHostMeasurementObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
            x: 160.0,
            y: 96.0,
            width: 240.0,
            height: 120.0,
        })
    }
}

#[test]
fn hostile_workbench_combines_every_allocation_source_without_bypass() {
    let first = run_hostile_workbench();
    let replay = run_hostile_workbench();
    assert_eq!(
        first, replay,
        "the same hostile event batch must replay deterministically"
    );
    assert_eq!(
        first.maximum_receipts, 1,
        "every hostile transition must remain a one-receipt local commit"
    );
    assert_eq!(
        first.maximum_neighborhoods, 1,
        "every hostile transition must remain a one-neighborhood local replan"
    );
    assert_eq!(
        first.root_widen_attempts, 0,
        "workbench silently widened to root"
    );
    assert_eq!(
        first.verified_source_transitions, 7,
        "every allocation source must prove its own terminal transition"
    );
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkbenchSummary {
    maximum_receipts: u16,
    maximum_neighborhoods: u16,
    root_widen_attempts: u16,
    verified_source_transitions: u8,
}

#[derive(Clone, Copy)]
enum ExpectedCompletion {
    Narrowed {
        stream: crate::runtime::UiAllocationStreamFamily,
        invalidation: crate::runtime::UiAllocationInvalidationFamily,
    },
    Viewport,
    Durable,
}

fn run_hostile_workbench() -> WorkbenchSummary {
    let (mut runtime, roots, durable_input, mut query) =
        crate::runtime::tests::production_catalog_activation_test_support::runtime_with_hostile_workbench_catalog();
    let capability =
        crate::evidence::measurement::projection::fact_test_support::capability_report(77);
    let profile = crate::host::UiHostMeasurementAssumptionProfile::from_capability_report(
        &capability,
        11,
        22,
        33,
        44,
    );
    let mut maximum_receipts = 0u16;
    let mut maximum_neighborhoods = 0u16;
    let mut root_widen_attempts = 0u16;
    let mut verified_source_transitions = 0u8;

    let typing = runtime.execute_framework_turn(|turn| {
        turn.interaction(|source| {
            source
                .admit_and_submit(
                    roots[0],
                    crate::runtime::WorthUiTransientInteractionState::TextInput,
                )
                .expect("typing source admits");
        });
    });
    observe_completion(
        &typing,
        &mut maximum_receipts,
        &mut maximum_neighborhoods,
        &mut root_widen_attempts,
        &mut verified_source_transitions,
        ExpectedCompletion::Narrowed {
            stream: crate::runtime::UiAllocationStreamFamily::TextInput,
            invalidation: crate::runtime::UiAllocationInvalidationFamily::TextContentChange,
        },
    );

    let growth = query.settle_snapshot();
    let fact_link = runtime.query_fact_link_for_test("inspector.measurements");
    let query_growth = runtime.execute_framework_turn(|turn| {
        turn.query_projection(|source| {
            source
                .refresh_settled(growth)
                .expect("settled Query growth atomically refreshes exact authority");
            source
                .submit_settled(&fact_link)
                .expect("Query growth source admits");
        });
    });
    observe_completion(
        &query_growth,
        &mut maximum_receipts,
        &mut maximum_neighborhoods,
        &mut root_widen_attempts,
        &mut verified_source_transitions,
        ExpectedCompletion::Narrowed {
            stream: crate::runtime::UiAllocationStreamFamily::QueryProjection,
            invalidation: crate::runtime::UiAllocationInvalidationFamily::ContentExtentChange,
        },
    );

    let preview = runtime.execute_framework_turn(|turn| {
        turn.resize_preview(|source| {
            for pixels in [280.0, 300.0, 320.0] {
                source
                    .admit_and_submit(crate::runtime::UiResizePreviewSample::new(
                        roots[3],
                        crate::runtime::UiResizeLogicalExtent::try_from_logical_pixels(pixels)
                            .expect("positive extent admits"),
                    ))
                    .expect("preview source admits");
            }
        });
    });
    let (transition, _) = preview
        .into_pending_mounted_preview()
        .unwrap_or_else(|other| panic!("preview remains isolated: {other:?}"));
    assert!(transition.preview().all_candidates_admitted());
    let before = transition.preview().capture_isolation_basis();
    let resolved_preview = transition.finish(before);
    assert!(matches!(
        resolved_preview.isolation,
        crate::runtime::UiPreviewPaintIsolationOutcome::Verified(_)
    ));
    assert!(matches!(
        resolved_preview.follow_on,
        crate::runtime::WorthUiMountedPreviewFollowOn::PreviewOnly
    ));
    verified_source_transitions += 1;

    let durable = runtime.execute_framework_turn(|turn| {
        turn.durable_resize(|source| {
            source
                .admit_and_submit(crate::runtime::UiDurableResizeCommitIntent::terminal(
                    durable_input.clone(),
                    crate::runtime::UiResizeLogicalExtent::try_from_logical_pixels(320.0)
                        .expect("positive extent admits"),
                ))
                .expect("durable resize source admits");
        });
    });
    observe_completion(
        &durable,
        &mut maximum_receipts,
        &mut maximum_neighborhoods,
        &mut root_widen_attempts,
        &mut verified_source_transitions,
        ExpectedCompletion::Durable,
    );

    let viewport = runtime.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            source
                .collect_and_submit(
                    &ViewportAdapter,
                    crate::host::UiHostMeasurementCollectionInput {
                        identity: UiMeasurementRequestIdentity::new(998),
                        evidence_family: UiMeasurementEvidenceFamily::ViewportExtent,
                        need: crate::host::UiHostMeasurementNeed::ViewportExtent(
                            UiViewportExtentRequest,
                        ),
                        capability_report: &capability,
                        evidence_generation: UiEvidenceAuthorityGeneration::new(17),
                        normalization_context:
                            crate::host::UiHostMeasurementNormalizationContext::viewport_logical_exact(
                                profile,
                            ),
                    },
                )
                .expect("viewport source admits");
        });
    });
    observe_completion(
        &viewport,
        &mut maximum_receipts,
        &mut maximum_neighborhoods,
        &mut root_widen_attempts,
        &mut verified_source_transitions,
        ExpectedCompletion::Viewport,
    );

    let scroll = runtime.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            source.collect_and_submit(
                &ScrollAdapter,
                crate::host::UiHostMeasurementCollectionInput {
                    identity: UiMeasurementRequestIdentity::new(995),
                    evidence_family: UiMeasurementEvidenceFamily::ScrollContainerViewport,
                    need: crate::host::UiHostMeasurementNeed::ScrollContainerViewport(
                        UiScrollContainerViewportRequest::new(55),
                    ),
                    capability_report: &capability,
                    evidence_generation: UiEvidenceAuthorityGeneration::new(17),
                    normalization_context:
                        crate::host::UiHostMeasurementNormalizationContext::scroll_container_logical_exact(profile),
                },
            ).expect("scroll source admits");
        });
    });
    observe_completion(
        &scroll,
        &mut maximum_receipts,
        &mut maximum_neighborhoods,
        &mut root_widen_attempts,
        &mut verified_source_transitions,
        ExpectedCompletion::Narrowed {
            stream: crate::runtime::UiAllocationStreamFamily::ScrollExtentObservation,
            invalidation: crate::runtime::UiAllocationInvalidationFamily::ScrollExtentObservation,
        },
    );

    let portal = runtime.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            source.collect_and_submit(
                &PortalAdapter,
                crate::host::UiHostMeasurementCollectionInput {
                    identity: UiMeasurementRequestIdentity::new(997),
                    evidence_family: UiMeasurementEvidenceFamily::PortalAnchorRect,
                    need: crate::host::UiHostMeasurementNeed::PortalAnchorRect(
                        UiPortalAnchorRectRequest::new(55),
                    ),
                    capability_report: &capability,
                    evidence_generation: UiEvidenceAuthorityGeneration::new(18),
                    normalization_context:
                        crate::host::UiHostMeasurementNormalizationContext::portal_anchor_logical_exact_in(
                            crate::host::UiPortalAnchorCoordinateSpacePosture::PortalLayer,
                            profile,
                        ),
                },
            ).expect("portal source admits");
        });
    });
    observe_completion(
        &portal,
        &mut maximum_receipts,
        &mut maximum_neighborhoods,
        &mut root_widen_attempts,
        &mut verified_source_transitions,
        ExpectedCompletion::Narrowed {
            stream: crate::runtime::UiAllocationStreamFamily::PortalAnchorObservation,
            invalidation: crate::runtime::UiAllocationInvalidationFamily::PortalAnchorMovement,
        },
    );

    WorkbenchSummary {
        maximum_receipts,
        maximum_neighborhoods,
        root_widen_attempts,
        verified_source_transitions,
    }
}

fn observe_completion(
    completion: &crate::runtime::WorthUiFrameworkTurnCompletion<'_>,
    maximum_receipts: &mut u16,
    maximum_neighborhoods: &mut u16,
    root_widen_attempts: &mut u16,
    verified_source_transitions: &mut u8,
    expected: ExpectedCompletion,
) {
    match (expected, completion) {
        (
            ExpectedCompletion::Narrowed {
                stream,
                invalidation,
            },
            crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
                plan,
                selection,
                transaction,
                ..
            },
        ) => {
            assert_eq!(plan.families(), &[stream]);
            assert_eq!(
                plan.invalidations()
                    .iter()
                    .map(|value| value.family())
                    .collect::<Vec<_>>(),
                vec![invalidation]
            );
            observe_selection(selection, maximum_neighborhoods, root_widen_attempts);
            match transaction {
                crate::runtime::UiAllocationReplanTransactionOutcome::Committed(committed)
                | crate::runtime::UiAllocationReplanTransactionOutcome::Replayed(committed) => {
                    *maximum_receipts =
                        (*maximum_receipts).max(committed.counters().committed_receipts());
                }
                crate::runtime::UiAllocationReplanTransactionOutcome::Denied(denial) => {
                    panic!(
                        "hostile workbench {stream:?}/{invalidation:?} transaction denied: {:?}",
                        denial.evidence(),
                    )
                }
            }
        }
        (
            ExpectedCompletion::Viewport,
            crate::runtime::WorthUiFrameworkTurnCompletion::ViewportResizeResolved {
                outcome, ..
            },
        ) => {
            assert_eq!(outcome.counters().selected_neighborhoods(), 1);
            assert!(!matches!(
                outcome.root_posture(),
                crate::graph::UiReplanRootPosture::CountedRootWiden { .. }
            ));
            *maximum_receipts = (*maximum_receipts).max(outcome.counters().committed_receipts());
            *maximum_neighborhoods = 1.max(*maximum_neighborhoods);
        }
        (
            ExpectedCompletion::Durable,
            crate::runtime::WorthUiFrameworkTurnCompletion::DurableResizeCommitted {
                outcome,
                selection,
                ..
            },
        ) => {
            observe_selection(selection, maximum_neighborhoods, root_widen_attempts);
            *maximum_receipts = (*maximum_receipts).max(outcome.counters().committed_receipts());
        }
        _ => panic!("allocation source reached the wrong terminal completion: {completion:?}"),
    }
    *verified_source_transitions += 1;
}

fn observe_selection(
    selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    maximum_neighborhoods: &mut u16,
    root_widen_attempts: &mut u16,
) {
    assert_eq!(selection.counters().set_cardinality(), 1);
    assert_eq!(selection.counters().root_widen_attempts(), 0);
    *maximum_neighborhoods = 1.max(*maximum_neighborhoods);
    *root_widen_attempts =
        root_widen_attempts.saturating_add(selection.counters().root_widen_attempts());
}
