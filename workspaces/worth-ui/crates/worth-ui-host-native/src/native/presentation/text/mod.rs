//! Native semantic-text presentation boundary.
//!
//! This boundary admits borrowed alpha raster output without retaining pixels or
//! performing atlas, GPU-upload, or glyph-run effects.

mod validation;

pub(crate) use validation::semantic_text_before_effects_denial;
