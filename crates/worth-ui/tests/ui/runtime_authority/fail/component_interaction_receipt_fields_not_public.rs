use worth_ui::facade::{
    WorthUiComponentInteractionPayload, WorthUiComponentInteractionReceipt,
    WorthUiComponentInteractionStatus, WorthUiInteractionReadiness, WorthUiInteractionTarget,
};

fn main() {
    let _forged = WorthUiComponentInteractionReceipt {
        surface_id: "worth.surface.preview.button.proof".to_owned(),
        component_id: "worth.component.button".to_owned(),
        interaction_id: "worth.interaction.button.submit".to_owned(),
        kind: worth_ui::facade::WorthUiComponentInteractionKind::Submit,
        status: WorthUiComponentInteractionStatus::Emitted,
        readiness: WorthUiInteractionReadiness::Enabled,
        target: WorthUiInteractionTarget::Surface(
            "worth.surface.preview.button.proof".to_owned(),
        ),
        payload: payload(),
        receipt_digest: 1,
    };
}

fn payload() -> WorthUiComponentInteractionPayload {
    panic!("fixture only checks receipt field privacy")
}
