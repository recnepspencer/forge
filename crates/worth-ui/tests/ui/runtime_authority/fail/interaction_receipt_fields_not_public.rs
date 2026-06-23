use worth_ui::facade::{
    WorthUiInteractionKind, WorthUiInteractionPayload, WorthUiInteractionReadiness,
    WorthUiInteractionReceipt, WorthUiInteractionStatus, WorthUiInteractionTarget,
};

fn main() {
    let _forged = WorthUiInteractionReceipt {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        component_id: "worth.component.primitive.proof".to_owned(),
        interaction_id: "worth.interaction.primitive.submit".to_owned(),
        kind: WorthUiInteractionKind::Submit,
        status: WorthUiInteractionStatus::Emitted,
        readiness: WorthUiInteractionReadiness::Enabled,
        target: WorthUiInteractionTarget::Surface(
            "worth.surface.preview.primitive.proof".to_owned(),
        ),
        payload: payload(),
        receipt_digest: 1,
    };
}

fn payload() -> WorthUiInteractionPayload {
    panic!("fixture only checks receipt field privacy")
}
