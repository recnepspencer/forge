mod completion;
#[cfg(test)]
mod completion_tests;
mod seed;

#[cfg(test)]
pub(super) use completion::complete_static_filled_rects;
pub(super) use completion::{complete_static_filled_rect, rebind_filled_rects};
pub(super) use seed::{lower_static_paint_seed, parse_rgba, UiMountedStaticPaintSeed};
