use worth_ui::facade::{
    WorthUiPrimitiveActivationPosture, WorthUiPrimitiveEventContainment,
    WorthUiPrimitiveEventRegionOrder, WorthUiPrimitiveEventRegionReceipt, WorthUiPrimitiveFrame,
    WorthUiPrimitiveHitFrameDerivationReceipt, WorthUiPrimitivePointerCapture,
    WorthUiPrimitiveResolvedCursorPosture,
};

fn main() {
    let _forged = WorthUiPrimitiveEventRegionReceipt {
        surface_id: "worth.surface.preview.primitive.inner".to_owned(),
        interaction_id: "worth.interaction.primitive.submit".to_owned(),
        parent_surface_id: Some("worth.surface.preview.primitive.proof".to_owned()),
        order: WorthUiPrimitiveEventRegionOrder::new(1, 0),
        visual_frame: frame(),
        hit_frame: frame(),
        hit_frame_derivation: derivation(),
        cursor: WorthUiPrimitiveResolvedCursorPosture::Pointer,
        activation_posture: activation_posture(),
        containment: WorthUiPrimitiveEventContainment::Contain,
        capture: WorthUiPrimitivePointerCapture::None,
        receipt_digest: 1,
    };
}

fn frame() -> WorthUiPrimitiveFrame {
    panic!("fixture only checks event region field privacy")
}

fn derivation() -> WorthUiPrimitiveHitFrameDerivationReceipt {
    panic!("fixture only checks event region field privacy")
}

fn activation_posture() -> WorthUiPrimitiveActivationPosture {
    panic!("fixture only checks event region field privacy")
}
