use worth_ui_harness::facade::HarnessVisualFoundationBundle;

fn main() {
    let _foundation = HarnessVisualFoundationBundle::from_raw_colors([
        ("editor.canvas", "#1E1E1E"),
        ("focus.ring", "#3794FF"),
    ]);
}
