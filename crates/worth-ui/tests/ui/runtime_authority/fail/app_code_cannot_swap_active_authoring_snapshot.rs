use worth_ui::facade::{
    WorthUiCandidateRuntimeAuthoringSnapshot, WorthUiRuntimeHost,
};

fn main() {}

fn swap_authoring_snapshot(
    runtime: &mut WorthUiRuntimeHost,
    candidate: WorthUiCandidateRuntimeAuthoringSnapshot,
) {
    runtime.promote_authoring_snapshot_after_activation(Some(candidate));
}
