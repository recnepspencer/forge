use worth_ui::facade::WorthUiQueryStatusSurface;

fn main() {
    let _surface = WorthUiQueryStatusSurface::from_freeform_rows(["loading"]);
}

// runtime diagnostics denials share one compiler process.
mod covered_001 { include!("raw_strings_cannot_replace_diagnostic_codes.rs"); }
mod covered_002 { include!("projection_hook_cannot_mint_runtime_truth.rs"); }
