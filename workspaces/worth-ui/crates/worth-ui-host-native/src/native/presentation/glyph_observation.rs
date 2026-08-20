//! Owner-issued glyph-command attribution retained with native presentation evidence.

use super::{text, UiNativeRetainedDrawList};

pub(super) fn intrinsic(
    retained: &UiNativeRetainedDrawList,
    atlas: &crate::native::text_atlas::UiNativeTextAtlas,
    extent: [u32; 2],
) -> Box<[crate::native::UiNativeGlyphObservation]> {
    collect(retained, atlas, extent, text::source_is_intrinsic_color)
}

pub(super) fn alpha(
    retained: &UiNativeRetainedDrawList,
    atlas: &crate::native::text_atlas::UiNativeTextAtlas,
    extent: [u32; 2],
) -> Box<[crate::native::UiNativeGlyphObservation]> {
    collect(retained, atlas, extent, |command| {
        !text::source_is_intrinsic_color(command)
    })
}

fn collect(
    retained: &UiNativeRetainedDrawList,
    atlas: &crate::native::text_atlas::UiNativeTextAtlas,
    extent: [u32; 2],
    select: impl Fn(text::UiNativeGlyphCommand) -> bool,
) -> Box<[crate::native::UiNativeGlyphObservation]> {
    let runs = retained.all_glyph_runs();
    let mut observations = text::plan_glyph_commands(&runs, atlas, extent)
        .expect("retained qualified glyph runs have atlas entries")
        .iter()
        .copied()
        .filter(|command| select(*command))
        .map(crate::native::UiNativeGlyphObservation::from_native_command)
        .collect::<Vec<_>>();
    observations.sort_by_key(|observation| {
        (
            observation.original_range(),
            observation.glyph_id(),
            observation.target_bounds(),
        )
    });
    observations.into_boxed_slice()
}
