use worth_ui::facade::diagnostics::WorthUiBindingObservationSurface;

fn main() {
    let _surface = WorthUiBindingObservationSurface::from_freeform_rows(["loading"]);
}

// runtime diagnostics denials share one compiler process.
mod covered_001 { include!("raw_strings_cannot_replace_diagnostic_codes.rs"); }
mod covered_002 { include!("projection_hook_cannot_mint_runtime_truth.rs"); }
mod covered_003 { include!("query_operating_world_gateway_is_not_a_facade.rs"); }
mod covered_004 { include!("local_query_support_mint_is_not_a_facade.rs"); }
mod covered_005 { include!("query_progression_authority_is_not_a_product_facade.rs"); }
