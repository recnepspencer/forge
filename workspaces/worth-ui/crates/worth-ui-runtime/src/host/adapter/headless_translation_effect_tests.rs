use worth_ui_host_contract::{
    UiHostProtocolContract, UiHostProtocolNegotiation, UiHostSurfaceIdentity,
    UiHostSurfacePresentationMode, UiMountedAccessibilityProjection, UiMountedAllocationProjection,
    UiMountedDiagnosticProjection, UiMountedDiagnosticReference, UiMountedFrameConsumptionInput,
    UiMountedFrameIdentity, UiMountedMechanicalRole, UiMountedMotionProjection,
    UiMountedNodeProjectionView, UiMountedNodeProjectionViewInput, UiMountedNodeReceiptIssuer,
    UiMountedOmissionReason, UiMountedPaintBatchTable, UiMountedPaintProjection,
    UiMountedParticipation, UiMountedParticipationFact, UiMountedParticipationInput,
    UiMountedParticipationStatus, UiMountedPresentationAttemptIdentity,
    UiMountedPresentationLeaseGate, UiMountedPreviewProjection, UiMountedProjectionView,
    UiMountedProjectionViewInput, UiMountedRealtimeBatchTable, UiMountedResourceTable,
    UiMountedSpatialBatchTable, UiMountedSurfaceBindingRequirement, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration, WorthUiHostCapabilityObservationGeneration,
};

use super::{UiHeadlessRecorderCapacity, UiHeadlessUnperformedEffect};

#[test]
fn headless_translation_records_motion_and_diagnostic_mechanics_independently() {
    let projection = admitted_effect_projection();
    let protocol = match UiHostProtocolContract::current().negotiate() {
        UiHostProtocolNegotiation::Compatible(protocol) => protocol,
        UiHostProtocolNegotiation::Incompatible(_) => panic!("current protocol is compatible"),
    };
    let capability_generation = WorthUiHostCapabilityObservationGeneration::new(7);
    let requirement = UiMountedSurfaceBindingRequirement::new(
        projection.surface(),
        UiHostSurfaceIdentity::mint_unbound().unwrap(),
        projection.binding(),
        capability_generation,
        11,
        UiHostSurfacePresentationMode::RecordOnly,
    );
    let lease = UiMountedPresentationLeaseGate::default()
        .claim()
        .expect("isolated test lease is available");
    let view = lease.open(UiMountedFrameConsumptionInput {
        host_session_identity: 13,
        protocol,
        capability_generation,
        capability_profile_digest: 11,
        attempt: UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
        deadline: worth_ui_host_contract::UiPresentationDeadline::at_tick(20),
        requirement,
        projection: &projection,
    });

    let transcript = super::headless_translation::translate_headless_frame(
        &view,
        UiHeadlessRecorderCapacity::production_default(),
    )
    .expect("record-only translation accepts admitted external mechanics");
    assert_eq!(transcript.nodes().len(), 1);
    assert_eq!(
        transcript.nodes()[0].motion(),
        UiMountedMotionProjection::Admitted
    );
    assert_eq!(
        transcript.nodes()[0].diagnostic(),
        UiMountedDiagnosticProjection::Reference(UiMountedDiagnosticReference::new(3))
    );
    assert!(transcript
        .unperformed_effects()
        .contains(&UiHeadlessUnperformedEffect::Motion { node_count: 1 }));
    assert!(transcript
        .unperformed_effects()
        .contains(&UiHeadlessUnperformedEffect::Diagnostic { node_count: 1 }));
}

fn admitted_effect_projection() -> UiMountedProjectionView {
    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let mounted_instance =
        worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap();
    let receipt = UiMountedNodeReceiptIssuer::mint_for(frame)
        .unwrap()
        .receipt_for(mounted_instance);
    let admitted = UiMountedParticipationFact::new(UiMountedParticipationStatus::Admitted);
    let withheld = UiMountedParticipationFact::new(UiMountedParticipationStatus::Withheld);
    let omitted = UiMountedOmissionReason::NotDefinedByCurrentRuntime;
    UiMountedProjectionView::new(UiMountedProjectionViewInput {
        frame,
        surface: UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
        binding: UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        nodes: vec![UiMountedNodeProjectionView::new(
            UiMountedNodeProjectionViewInput {
                mounted_instance,
                node_receipt: receipt,
                role: UiMountedMechanicalRole::Diagnostic,
                participation: UiMountedParticipation::new(UiMountedParticipationInput {
                    paint: withheld,
                    clip: withheld,
                    input: withheld,
                    focus: withheld,
                    hit_test: withheld,
                    accessibility: withheld,
                    motion: admitted,
                    diagnostic: admitted,
                }),
                allocation: UiMountedAllocationProjection::Omitted(omitted),
                preview: UiMountedPreviewProjection::Omitted(omitted),
                paint: UiMountedPaintProjection::Omitted(omitted),
                accessibility: UiMountedAccessibilityProjection::Omitted(omitted),
                motion: UiMountedMotionProjection::Admitted,
                diagnostic: UiMountedDiagnosticProjection::Reference(
                    UiMountedDiagnosticReference::new(3),
                ),
            },
        )],
        clips: worth_ui_host_contract::UiMountedClipTable::produced(Vec::new()),
        layers: worth_ui_host_contract::UiMountedLayerTable::produced(Vec::new()),
        filled_rects: worth_ui_host_contract::UiMountedFilledRectTable::empty(),
        paint_batches: UiMountedPaintBatchTable::new(Vec::new()),
        spatial_batches: UiMountedSpatialBatchTable::new(Vec::new()),
        realtime_batches: UiMountedRealtimeBatchTable::new(Vec::new()),
        resources: UiMountedResourceTable::new(Vec::new()),
    })
}
