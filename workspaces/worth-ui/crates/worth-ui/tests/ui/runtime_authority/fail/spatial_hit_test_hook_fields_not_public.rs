use worth_ui::facade::WorthUiSpatialHitTestHook;

fn main() {
    let _hook = WorthUiSpatialHitTestHook {
        hook_id: "canvas.hit_test".to_string(),
        preserved_support_digest: 1,
    };
}
