use worth_ui_host_contract::{
    UiHostSurfacePresentationMode, UiMountedEffectFamily, UiMountedPaintPrimitiveKind,
    UiMountedPaintProjection, UiMountedProjectionView,
};

pub(super) fn required_effects(
    mode: UiHostSurfacePresentationMode,
    projection: &UiMountedProjectionView,
) -> Vec<UiMountedEffectFamily> {
    if mode == UiHostSurfacePresentationMode::RecordOnly {
        return vec![UiMountedEffectFamily::RecordedProjection];
    }

    let mut effects = Vec::new();
    if has_canvas_effect(projection) {
        effects.push(UiMountedEffectFamily::CanvasSpatial);
    }
    if has_realtime_effect(projection) {
        effects.push(UiMountedEffectFamily::Realtime);
    }
    if projection.nodes().iter().any(|node| {
        matches!(node.paint(), UiMountedPaintProjection::FilledRect(_))
            || matches!(
                node.preview(),
                worth_ui_host_contract::UiMountedPreviewProjection::Resize { .. }
            )
    }) {
        effects.push(UiMountedEffectFamily::NativePaint);
    }
    if projection.nodes().iter().any(|node| {
        matches!(
            node.accessibility(),
            worth_ui_host_contract::UiMountedAccessibilityProjection::Admitted(_)
        )
    }) {
        effects.push(UiMountedEffectFamily::Accessibility);
    }
    if projection.nodes().iter().any(|node| {
        node.participation().focus().status()
            == worth_ui_host_contract::UiMountedParticipationStatus::Admitted
    }) {
        effects.push(UiMountedEffectFamily::Focus);
    }
    if projection.nodes().iter().any(|node| {
        matches!(
            node.motion(),
            worth_ui_host_contract::UiMountedMotionProjection::Admitted
        )
    }) {
        effects.push(UiMountedEffectFamily::Motion);
    }
    if projection.nodes().iter().any(|node| {
        matches!(
            node.diagnostic(),
            worth_ui_host_contract::UiMountedDiagnosticProjection::Reference(_)
        )
    }) {
        effects.push(UiMountedEffectFamily::Diagnostic);
    }
    effects
}

fn has_canvas_effect(projection: &UiMountedProjectionView) -> bool {
    !projection.spatial_batches().rows().is_empty()
        || projection
            .paint_batches()
            .rows()
            .iter()
            .any(|row| row.primitive_kind() == UiMountedPaintPrimitiveKind::CanvasSpatialBatch)
}

fn has_realtime_effect(projection: &UiMountedProjectionView) -> bool {
    !projection.realtime_batches().rows().is_empty()
        || projection
            .paint_batches()
            .rows()
            .iter()
            .any(|row| row.primitive_kind() == UiMountedPaintPrimitiveKind::RealtimeBatch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_ui_host_contract::{
        UiMountedAccessibilityProjection, UiMountedAllocationProjection,
        UiMountedDiagnosticProjection, UiMountedDiagnosticReference, UiMountedFrameIdentity,
        UiMountedMechanicalRole, UiMountedMotionProjection, UiMountedNodeProjectionView,
        UiMountedNodeProjectionViewInput, UiMountedNodeReceiptIssuer, UiMountedOmissionReason,
        UiMountedPaintBatchTable, UiMountedParticipation, UiMountedParticipationFact,
        UiMountedParticipationInput, UiMountedParticipationStatus, UiMountedPreviewProjection,
        UiMountedProjectionViewInput, UiMountedRealtimeBatchTable, UiMountedResourceTable,
        UiMountedSpatialBatchTable, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
    };

    #[test]
    fn projection_derived_requirements_include_motion_and_diagnostic() {
        let projection = admitted_effect_projection();
        assert_eq!(
            required_effects(UiHostSurfacePresentationMode::NativeDisplay, &projection),
            vec![
                UiMountedEffectFamily::Motion,
                UiMountedEffectFamily::Diagnostic,
            ]
        );
        assert_eq!(
            required_effects(UiHostSurfacePresentationMode::RecordOnly, &projection),
            vec![UiMountedEffectFamily::RecordedProjection]
        );
    }

    fn admitted_effect_projection() -> UiMountedProjectionView {
        let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
        let instance = worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap();
        let receipt = UiMountedNodeReceiptIssuer::mint_for(frame)
            .unwrap()
            .receipt_for(instance);
        let admitted = UiMountedParticipationFact::new(UiMountedParticipationStatus::Admitted);
        let withheld = UiMountedParticipationFact::new(UiMountedParticipationStatus::Withheld);
        let omitted = UiMountedOmissionReason::NotDefinedByCurrentRuntime;
        UiMountedProjectionView::new(UiMountedProjectionViewInput {
            frame,
            surface: UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
            binding: UiSurfaceBindingGeneration::mint_unbound().unwrap(),
            nodes: vec![UiMountedNodeProjectionView::new(
                UiMountedNodeProjectionViewInput {
                    mounted_instance: instance,
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
                        UiMountedDiagnosticReference::new(8),
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
}
