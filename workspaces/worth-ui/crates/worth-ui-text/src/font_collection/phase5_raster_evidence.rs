//! Governed font-backed raster evidence over admitted application sources.

pub(crate) fn prove_alpha_oracles() {
    super::application_alpha_raster_controls::public_alpha_raster_matches_exact_oracle_across_fractional_origins();
    super::application_alpha_raster_controls::public_alpha_raster_matches_exact_variable_and_last_resort_oracle();
}

pub(crate) fn prove_color_sources() -> usize {
    super::application_color_tests::every_qualified_color_table_format_crosses_public_pack_admission();
    super::application_color_tests::qualified_sbix_png_and_one_hop_dupe_rasterize_as_intrinsic_color();
    super::application_color_tests::qualified_colrv1_composite_and_gradient_enums_cross_public_admission();
    super::application_color_tests::qualified_colr_owner_produces_intrinsic_color_pixels();
    super::application_color_raster_controls::colrv0_layers_use_linear_premultiplied_order();
    super::application_color_raster_controls::bitmap_raster_selects_the_globally_nearest_qualified_strike();
    super::application_color_graph_controls::colrv1_gradient_and_composite_cross_the_font_backed_raster_boundary();
    super::application_color_graph_controls::colrv1_nonseparable_modes_match_independent_w3c_vectors_after_srgb_storage();
    super::application_color_graph_controls::cbdt_composite_crosses_admission_layout_demand_and_raster();
    super::profile_data::unicode_17_rgi_emoji().len()
}

pub(crate) fn reject_color_source_mutants() {
    super::application_color_tests::unknown_colrv1_composite_and_gradient_enums_deny_before_publication();
    super::application_color_tests::repository_color_emoji_requires_resolvable_locations_and_intact_png_chunks();
    super::application_color_tests::malformed_colr_and_sbix_tables_cannot_hide_behind_a_parseable_outline_font();
    super::application_color_raster_controls::unsupported_cbdt_pixel_formats_deny_before_application_pack_publication();
    super::application_color_graph_controls::cbdt_missing_target_cycle_and_sbix_jpg_tiff_deny_atomically();
    super::application_color_graph_controls::current_color_and_every_unbounded_colrv1_root_deny_atomically();
}
