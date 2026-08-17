use super::{exact_array_table, exact_bool_table, exact_string_table};

pub(super) fn validate_application_fonts(value: &toml::value::Table) -> Result<(), String> {
    validate_application_font_admission(value)?;
    validate_application_font_identity(value)?;
    validate_application_color(value)?;
    exact_fields(value, 16, "application-font contract")
}

fn validate_application_font_admission(value: &toml::value::Table) -> Result<(), String> {
    exact_string_table(
        value,
        "registration",
        "immutable-content-addressed-before-layout-effects",
    )?;
    exact_array_table(
        value,
        "validation",
        &[
            "container-kind",
            "table-directory",
            "face-index",
            "localized-family-style-metadata",
            "weight-width-slant",
            "variation-axes",
            "opentype-features",
            "unicode-coverage",
            "color-table-integrity",
            "license-record",
            "face-count",
            "byte-capacity",
        ],
    )?;
    exact_string_table(value, "fallback_authority", "per-span-ordered-family-stack")?;
    exact_array_table(
        value,
        "fallback_routes",
        &[
            "rgi:qualified-color-emoji->last-resort",
            "non-rgi:qualified-profile-defaults->last-resort",
        ],
    )?;
    exact_string_table(value, "generation", "font-collection-generation")?;
    exact_string_table(value, "fresh_admission", "current-generation-only")?;
    exact_string_table(
        value,
        "predecessor_layout_bytes",
        "retained-until-layout-owner-release",
    )?;
    exact_string_table(value, "ambient_installation", "excluded")?;
    exact_string_table(
        value,
        "admission_failure",
        "typed-atomic-before-layout-effects",
    )
}

fn validate_application_font_identity(value: &toml::value::Table) -> Result<(), String> {
    exact_array_table(
        value,
        "identity",
        &[
            "pack-content-digest",
            "font-bytes-digest",
            "face-index",
            "family-identity",
            "style-metadata",
            "variation-axes",
            "feature-inventory",
            "license-record",
        ],
    )
}

fn validate_application_color(value: &toml::value::Table) -> Result<(), String> {
    exact_array_table(
        value,
        "admitted_color_tables",
        &["COLR-v0-CPAL", "COLR-v1-CPAL", "CBDT-CBLC", "sbix"],
    )?;
    exact_array_table(value, "unsupported_color_tables", &["SVG"])?;
    exact_array_table(value, "sbix_graphic_types", &["png", "dupe"])?;
    exact_string_table(value, "sbix_dupe", "one-hop-to-png")?;
    exact_array_table(value, "unsupported_sbix_graphic_types", &["jpg", "tiff"])?;
    exact_string_table(
        value,
        "color_table_failure",
        "typed-atomic-before-layout-effects",
    )
}

pub(super) fn validate_run_formation(value: &toml::value::Table) -> Result<(), String> {
    exact_array_table(
        value,
        "partition_order",
        &[
            "bidi-level",
            "script",
            "language",
            "style",
            "selected-face",
            "variation-axes",
            "feature-set",
        ],
    )?;
    exact_string_table(value, "default_language", "und")?;
    exact_string_table(value, "shaping", "harfbuzz-compatible-complex")?;
    exact_array_table(
        value,
        "scripts",
        &[
            "Arabic",
            "Hebrew",
            "Indic",
            "Southeast-Asian",
            "Tibetan",
            "Hangul",
            "combining-mark",
            "ligature",
            "joining",
        ],
    )?;
    exact_string_table(value, "cluster_level", "monotone-graphemes")?;
    exact_array_table(
        value,
        "indivisible_boundaries",
        &["line", "caret", "selection", "fallback", "ellipsis"],
    )?;
    exact_string_table(
        value,
        "feature_posture",
        "profile-and-authored-span-explicit-only",
    )?;
    exact_string_table(
        value,
        "variation_posture",
        "qualified-axis-coordinates-only",
    )?;
    exact_string_table(
        value,
        "fallback_unit",
        "complete-grapheme-or-rgi-emoji-cluster",
    )?;
    exact_string_table(
        value,
        "face_admission",
        "complete-cluster-shapes-without-notdef",
    )?;
    exact_fields(value, 10, "run-formation contract")
}

pub(super) fn validate_layout_identity(value: &toml::value::Table) -> Result<(), String> {
    exact_string_table(
        value,
        "artifact",
        "immutable-reconstructible-qualified-text-layout",
    )?;
    exact_string_table(value, "original_ranges", "exact-source-utf8")?;
    exact_array_table(
        value,
        "consumers",
        &[
            "measurement",
            "baseline",
            "hit-testing",
            "selection",
            "rasterization",
            "native-rendering",
            "accessibility-geometry",
        ],
    )?;
    exact_bool_table(value, "independent_reshape", false)?;
    exact_array_table(
        value,
        "affinity",
        &[
            "profile",
            "font-collection-content",
            "locale",
            "direction",
            "width",
            "text-scale",
        ],
    )?;
    validate_layout_records(value)?;
    exact_string_table(value, "authority", "non-authoritative-derived")?;
    exact_array_table(
        value,
        "reconstruction_source",
        &[
            "mounted-text",
            "exact-layout-request",
            "profile-generation",
            "font-collection-generation",
            "font-collection-content",
            "pinned-face-bytes",
            "text-scale-generation",
        ],
    )?;
    exact_fields(value, 8, "layout identity contract")
}

fn validate_layout_records(value: &toml::value::Table) -> Result<(), String> {
    exact_array_table(
        value,
        "records",
        &[
            "line-order",
            "visual-run-order",
            "selected-face",
            "feature-set",
            "variation-coordinates",
            "glyph-clusters",
            "glyph-positions",
            "logical-metrics",
            "ink-metrics",
            "baselines",
            "break-decisions",
            "overflow-decisions",
            "original-range-caret-stops",
            "bidi-caret-affinities",
            "point-hit-records",
            "selection-rectangles-per-visual-run",
            "coverage-disposition",
            "cost",
        ],
    )
}

pub(super) fn validate_scale_generation(value: &toml::value::Table) -> Result<(), String> {
    exact_array_table(
        value,
        "logical_layout_keys",
        &["logical-constraints", "text-scale-generation"],
    )?;
    exact_string_table(
        value,
        "dpi_change",
        "replace-raster-and-atlas-generations-only",
    )?;
    exact_string_table(
        value,
        "text_scale_change",
        "replace-layout-before-presentation",
    )?;
    exact_string_table(value, "width_change", "replace-layout-before-presentation")?;
    exact_string_table(
        value,
        "font_collection_change",
        "replace-layout-before-presentation",
    )?;
    exact_string_table(
        value,
        "profile_change",
        "replace-layout-before-presentation",
    )?;
    exact_fields(value, 6, "scale-generation contract")
}

pub(super) fn validate_saturation(value: &toml::value::Table) -> Result<(), String> {
    exact_string_table(
        value,
        "retained_layouts",
        "pin-exact-alpha-and-color-glyphs",
    )?;
    exact_string_table(
        value,
        "candidate_admission",
        "complete-before-raster-or-upload",
    )?;
    exact_string_table(value, "eviction", "deterministic-unpinned-only")?;
    for field in [
        "live_eviction",
        "unbounded_growth",
        "silent_quality_reduction",
        "color_to_monochrome_fallback",
    ] {
        exact_bool_table(value, field, false)?;
    }
    exact_array_table(
        value,
        "atlas_key",
        &[
            "font-collection-generation",
            "profile-generation",
            "face",
            "glyph",
            "variation-coordinates",
            "palette",
            "size",
            "raster-source",
            "dpi-scale",
            "fractional-origin",
        ],
    )?;
    exact_bool_table(value, "separate_alpha_color_atlases", true)?;
    exact_string_table(value, "reconstructive_saturation", "named-posture-only")?;
    exact_fields(value, 10, "saturation contract")
}

pub(super) fn exact_fields(
    value: &toml::value::Table,
    expected: usize,
    contract: &str,
) -> Result<(), String> {
    (value.len() == expected)
        .then_some(())
        .ok_or_else(|| format!("{contract} drifted"))
}
