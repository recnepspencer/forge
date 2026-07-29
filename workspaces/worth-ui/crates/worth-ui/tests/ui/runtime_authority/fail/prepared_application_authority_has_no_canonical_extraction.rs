use worth_ui::facade::app::WorthUiPreparedApplicationAuthority;
use worth_ui::facade::rebind::{UiAuthoredChangedFact, UiQueryChangedFact};
use worth_ui_runtime::facade::mounted::{
    UiMountedFramePublicationReceipt, UiMountedNodeReceipt, UiMountedProjectionView,
    UiProjectedMountedFrameCandidate,
};

fn extract(authority: WorthUiPreparedApplicationAuthority) {
    let _ = authority.into_canonical_artifact();
}

fn extract_forbidden_mounted_authority(view: &UiMountedProjectionView) {
    let _ = view.query_key();
    let _ = view.query_artifact();
    let _ = view.query_settlement();
    let _ = view.query_rows();
    let _ = view.query_patches();
    let _ = view.query_operational_identity();
    let _ = view.native_widget_handle();
    let _ = view.native_texture_handle();
    let _ = view.native_resource_handle();
}

fn forge_node_receipt() -> UiMountedNodeReceipt {
    UiMountedNodeReceipt {}
}

fn forge_projected_candidate() -> UiProjectedMountedFrameCandidate {
    UiProjectedMountedFrameCandidate {}
}

fn forge_current_mounted_frame() -> UiMountedFramePublicationReceipt {
    UiMountedFramePublicationReceipt {}
}

fn forge_produced_fact() {
    let _forged = UiAuthoredChangedFact {};
}

fn forge_query_reset_without_owner_consequence() {
    let _forged = UiQueryChangedFact::from_owner_consequence(());
}

fn main() {}
