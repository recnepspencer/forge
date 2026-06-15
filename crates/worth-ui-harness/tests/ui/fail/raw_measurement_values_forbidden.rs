use worth_ui_harness::facade::HarnessDensity;

fn main() {
    let _density = HarnessDensity::from_raw_pixels([
        ("sidebar.width", 320),
        ("toolbar.height", 40),
    ]);
}
