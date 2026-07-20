use std::alloc::System;

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

#[global_allocator]
static ALLOCATOR: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub(super) fn allocated_bytes_during(action: impl FnOnce()) -> usize {
    let region = Region::new(ALLOCATOR);
    action();
    region.change().bytes_allocated
}
