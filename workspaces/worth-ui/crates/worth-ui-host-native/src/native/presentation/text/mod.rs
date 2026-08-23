//! Native semantic-text presentation boundary.
//!
//! This boundary admits borrowed alpha raster output without retaining pixels or
//! performing atlas, GPU-upload, or glyph-run effects.

mod commands;

pub(super) use commands::glyph_vertices;
pub(crate) use commands::{
    clip_glyph_command, plan_glyph_commands, source_is_intrinsic_color, UiNativeGlyphCommand,
};
