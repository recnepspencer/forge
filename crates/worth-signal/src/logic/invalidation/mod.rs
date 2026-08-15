pub(crate) mod causality;
mod routing;
pub(crate) mod scheduling;

#[cfg(any(test, doctest))]
pub use routing::mark_dirty;
pub use routing::mark_dirty_batch;
#[cfg(any(test, doctest))]
pub use routing::mark_dirty_with_regions;
