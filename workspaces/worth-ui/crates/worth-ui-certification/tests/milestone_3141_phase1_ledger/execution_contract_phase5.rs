pub(super) fn main_test(requirement: &str) -> Option<&'static str> {
    Some(match requirement {
        "P5-GLYPH-RASTER-01" => {
            "milestone_3141_phase1_topology::phase_five_destination::text_owns_typed_alpha_and_color_raster_batches"
        }
        "P5-COLOR-EMOJI-01" => {
            "milestone_3141_phase1_topology::phase_five_destination::color_glyphs_preserve_intrinsic_color_and_cluster_identity"
        }
        "P5-ATLAS-01" => {
            "milestone_3141_phase1_topology::phase_five_destination::native_host_owns_separate_alpha_and_rgba_atlas_lifecycles"
        }
        "P5-ATLAS-PINNING-01" => {
            "milestone_3141_phase1_topology::phase_five_destination::live_layouts_pin_atlas_entries_without_consumer_eviction"
        }
        "P5-TEXT-DPI-01" => {
            "milestone_3141_phase1_topology::phase_five_destination::dpi_replacement_is_pure_raster_and_does_not_relayout"
        }
        "P5-TEXT-SPAN-PAINT-01" => {
            "milestone_3141_phase1_topology::phase_five_destination::paint_spans_carry_logical_foreground_without_layout_regen"
        }
        "P5-TEXT-PIXELS-01" => {
            "milestone_3141_phase1_topology::phase_five_destination::headless_and_native_pixels_report_the_same_paint_span"
        }
        "P5-TEXT-RECONSTRUCTION-01" => {
            "milestone_3141_phase1_topology::phase_five_destination::reconstruction_consumes_only_mounted_layout_authority"
        }
        "P5-TEXT-COST-01" => {
            "milestone_3141_phase1_topology::phase_five_destination::phase_five_cost_vocabulary_separates_ordinary_and_reconstructive_lanes"
        }
        _ => return None,
    })
}
