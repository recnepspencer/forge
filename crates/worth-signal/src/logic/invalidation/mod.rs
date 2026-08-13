pub(crate) mod causality;
mod cycles;
mod routing;
mod subscription;

#[cfg(any(test, doctest))]
pub use routing::mark_dirty;
pub use routing::mark_dirty_batch;
#[cfg(any(test, doctest))]
pub use routing::mark_dirty_with_regions;
