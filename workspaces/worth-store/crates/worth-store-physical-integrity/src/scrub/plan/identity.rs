use crate::{PlannedScrubWindow, PlannedScrubWindowStatus, ScrubMode, ScrubOverBudgetClass};
use worth_store::physical_runtime::ScrubPhysicalAllocation;

use super::ScrubPlanPolicy;

pub(super) fn scrub_plan_identity(
    allocation: &ScrubPhysicalAllocation<'_>,
    mode: ScrubMode,
    policy: ScrubPlanPolicy,
    yield_after_windows: Option<u64>,
    windows: &[PlannedScrubWindow<'_>],
) -> u64 {
    let store = allocation.store_identity().bytes();
    let seed = [
        u64::from_le_bytes(store[..8].try_into().expect("store identity prefix")),
        u64::from_le_bytes(store[8..].try_into().expect("store identity suffix")),
        allocation.store_generation().get(),
        allocation.runtime_identity().get(),
        allocation.bytes(),
        mode_tag(mode),
        policy.streaming_window_byte_limit(),
        policy.protected_read_limit(),
        yield_after_windows.unwrap_or(u64::MAX),
        windows.len() as u64,
    ]
    .into_iter()
    .fold(0xcbf2_9ce4_8422_2325, mix);

    windows.iter().fold(seed, |acc, planned| {
        let window = planned.window();
        [
            window.ordinal().get(),
            source_tag(window.source()),
            status_tag(planned.status()),
            window.len_bytes(),
            window.checksum(),
        ]
        .into_iter()
        .fold(acc, mix)
    })
}

fn mix(acc: u64, value: u64) -> u64 {
    acc.rotate_left(5)
        .wrapping_mul(0x100_0000_01b3)
        .wrapping_add(value)
}

const fn mode_tag(mode: ScrubMode) -> u64 {
    match mode {
        ScrubMode::Online => 0x11,
        ScrubMode::Offline => 0x22,
    }
}

const fn source_tag(source: crate::ScrubWindowSource) -> u64 {
    match source {
        crate::ScrubWindowSource::OnlineProtectedRead => 0x33,
        crate::ScrubWindowSource::OfflineDeclaredVerifierInput => 0x44,
    }
}

const fn status_tag(status: PlannedScrubWindowStatus) -> u64 {
    match status {
        PlannedScrubWindowStatus::Inspect => 0x55,
        PlannedScrubWindowStatus::Skip => 0x66,
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::Allocation) => 0x77,
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::StreamingWindow) => 0x88,
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::ProtectedRead) => 0x99,
    }
}
