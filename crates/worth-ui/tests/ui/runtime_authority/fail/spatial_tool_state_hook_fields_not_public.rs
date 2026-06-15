use worth_ui::facade::WorthUiSpatialToolStateHook;

fn main() {
    let _hook = WorthUiSpatialToolStateHook {
        hook_id: "canvas.tool_state".to_string(),
        selection_identity_digest: 1,
    };
}
