use worth_ui::facade::WorthUiCanvasDrawHook;

fn main() {
    let _hook = WorthUiCanvasDrawHook {
        hook_id: "canvas.draw".to_string(),
        preserved_support_digest: 1,
    };
}
