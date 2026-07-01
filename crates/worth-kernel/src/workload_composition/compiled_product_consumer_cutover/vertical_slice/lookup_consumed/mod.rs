mod admitted_slice;
mod displaced_surface;
mod execution_posture;
mod reuse_resolution;

#[cfg(test)]
mod tests;

pub(crate) use admitted_slice::LookupConsumedVerticalSlice;
pub(crate) use displaced_surface::{
    current_lookup_consumed_vertical_slice_displaced_surfaces,
    LookupConsumedVerticalSliceDisplacedSurfaceDisposition,
};
pub(crate) use reuse_resolution::resolve_lookup_reuse_for_handoff;
