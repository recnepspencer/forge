use worth_ui_host_contract::{
    UiMountedAccessibilityProjection, UiMountedAllocationProjection, UiMountedDiagnosticProjection,
    UiMountedDiagnosticReference, UiMountedEffectFamily, UiMountedFrameIdentity,
    UiMountedMechanicalRole, UiMountedMotionProjection, UiMountedNodeProjectionView,
    UiMountedNodeProjectionViewInput, UiMountedNodeReceiptIssuer, UiMountedOmissionReason,
    UiMountedPaintBatchTable, UiMountedPaintProjection, UiMountedParticipation,
    UiMountedParticipationFact, UiMountedParticipationInput, UiMountedParticipationStatus,
    UiMountedPreviewProjection, UiMountedProjectionView, UiMountedProjectionViewInput,
    UiMountedRealtimeBatchTable, UiMountedResourceTable, UiMountedSpatialBatchTable,
    UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
};

#[test]
fn native_classifier_denies_motion_then_diagnostic_before_adapter_effects() {
    assert_eq!(
        super::mounted_effect_support::unsupported_projection_effect(&effect_projection(
            true, true,
        )),
        Some(UiMountedEffectFamily::Motion)
    );
    assert_eq!(
        super::mounted_effect_support::unsupported_projection_effect(&effect_projection(
            false, true,
        )),
        Some(UiMountedEffectFamily::Diagnostic)
    );
}

fn effect_projection(motion: bool, diagnostic: bool) -> UiMountedProjectionView {
    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let mounted_instance =
        worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap();
    let receipt = UiMountedNodeReceiptIssuer::mint_for(frame)
        .unwrap()
        .receipt_for(mounted_instance);
    let admitted = UiMountedParticipationFact::new(UiMountedParticipationStatus::Admitted);
    let withheld = UiMountedParticipationFact::new(UiMountedParticipationStatus::Withheld);
    let omitted = UiMountedOmissionReason::NotDefinedByCurrentRuntime;
    let motion_projection = if motion {
        UiMountedMotionProjection::Admitted
    } else {
        UiMountedMotionProjection::Omitted(omitted)
    };
    let diagnostic_projection = if diagnostic {
        UiMountedDiagnosticProjection::Reference(UiMountedDiagnosticReference::new(5))
    } else {
        UiMountedDiagnosticProjection::Omitted(omitted)
    };
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
                    motion: if motion { admitted } else { withheld },
                    diagnostic: if diagnostic { admitted } else { withheld },
                }),
                allocation: UiMountedAllocationProjection::Omitted(omitted),
                preview: UiMountedPreviewProjection::Omitted(omitted),
                paint: UiMountedPaintProjection::Omitted(omitted),
                accessibility: UiMountedAccessibilityProjection::Omitted(omitted),
                motion: motion_projection,
                diagnostic: diagnostic_projection,
            },
        )],
        clips: worth_ui_host_contract::UiMountedClipTable::produced(Vec::new()),
        layers: worth_ui_host_contract::UiMountedLayerTable::produced(Vec::new()),
        paint_batches: UiMountedPaintBatchTable::new(Vec::new()),
        spatial_batches: UiMountedSpatialBatchTable::new(Vec::new()),
        realtime_batches: UiMountedRealtimeBatchTable::new(Vec::new()),
        resources: UiMountedResourceTable::new(Vec::new()),
    })
}
