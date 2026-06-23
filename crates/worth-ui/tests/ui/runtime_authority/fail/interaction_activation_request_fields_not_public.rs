use worth_ui::facade::{
    SurfaceId, WorthUiInteractionActivationRequest, WorthUiInteractionKind,
    WorthUiMountedInteractionGesture,
};

fn main() {
    let _forged = WorthUiInteractionActivationRequest {
        surface_id: SurfaceId::new("worth.surface.preview.primitive.proof").unwrap(),
        interaction_id: "worth.interaction.primitive.submit".to_owned(),
        kind: WorthUiInteractionKind::Submit,
        gesture: WorthUiMountedInteractionGesture::primary_click(),
    };
}
