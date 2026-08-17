use super::{exact_array_table, exact_bool_table, exact_string_table};

pub(super) fn validate(value: &toml::value::Table) -> Result<(), String> {
    exact_string_table(value, "alpha_format", "R8Unorm")?;
    exact_string_table(value, "color_format", "Rgba8UnormSrgb")?;
    exact_array_table(
        value,
        "source_order",
        &[
            "color-outline",
            "color-bitmap",
            "alpha-outline",
            "last-resort",
        ],
    )?;
    exact_string_table(value, "hinting", "qualified-font-instructions")?;
    exact_string_table(value, "antialiasing", "grayscale")?;
    exact_string_table(value, "fractional_origin_quantum", "1/64-pixel")?;
    exact_string_table(value, "premultiplication", "linear-rgb-before-storage")?;
    exact_bool_table(value, "emoji_foreground_substitution", false)?;
    exact_array_table(
        value,
        "admitted_color_sources",
        &["COLR-v0-CPAL", "COLR-v1-CPAL", "CBDT-CBLC", "sbix"],
    )?;
    exact_array_table(value, "unsupported_color_sources", &["SVG"])?;
    exact_array_table(value, "sbix_graphic_types", &["png", "dupe"])?;
    exact_string_table(value, "sbix_dupe", "one-hop-to-png")?;
    exact_array_table(value, "unsupported_sbix_graphic_types", &["jpg", "tiff"])?;
    exact_string_table(
        value,
        "colr_compositing",
        "ordered-premultiplied-linear-rgba-with-explicit-palette",
    )?;
    exact_string_table(
        value,
        "bitmap_strike_selection",
        "nearest-qualified-strike-then-deterministic-resample",
    )?;
    exact_string_table(value, "bitmap_resampling", "premultiplied-linear-bilinear")?;
    (value.len() == 16)
        .then_some(())
        .ok_or_else(|| "text raster contract drifted".to_owned())
}
