use std::alloc::System;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use stats_alloc::{Region, Stats, StatsAlloc, INSTRUMENTED_SYSTEM};

#[global_allocator]
static ALLOCATOR: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub(super) fn allocations_during<T>(operation: impl FnOnce() -> T) -> (T, Stats) {
    let region = Region::new(ALLOCATOR);
    let result = operation();
    (result, region.change())
}

pub(super) fn peak_live_bytes_during<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    let baseline = live_bytes(ALLOCATOR.stats());
    let running = Arc::new(AtomicBool::new(true));
    let peak = Arc::new(AtomicUsize::new(baseline));
    let sampler_running = Arc::clone(&running);
    let sampler_peak = Arc::clone(&peak);
    let sampler = std::thread::spawn(move || {
        while sampler_running.load(Ordering::Acquire) {
            sampler_peak.fetch_max(live_bytes(ALLOCATOR.stats()), Ordering::AcqRel);
            std::thread::yield_now();
        }
        sampler_peak.fetch_max(live_bytes(ALLOCATOR.stats()), Ordering::AcqRel);
    });
    let result = operation();
    peak.fetch_max(live_bytes(ALLOCATOR.stats()), Ordering::AcqRel);
    running.store(false, Ordering::Release);
    sampler.join().unwrap();
    (
        result,
        peak.load(Ordering::Acquire).saturating_sub(baseline),
    )
}

fn live_bytes(stats: Stats) -> usize {
    let net = stats.bytes_allocated as i128 - stats.bytes_deallocated as i128
        + stats.bytes_reallocated as i128;
    net.max(0) as usize
}
