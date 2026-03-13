mod cycles;
mod routing;
mod subscription;

pub use routing::mark_dirty_batch;
#[cfg(any(test, doctest))]
pub use routing::{mark_dirty, mark_dirty_with_regions};
