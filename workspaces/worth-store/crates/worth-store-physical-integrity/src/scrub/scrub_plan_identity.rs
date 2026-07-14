use crate::{
    PlannedScrubWindow, PlannedScrubWindowStatus, ScrubMode, ScrubOverBudgetClass, ScrubPlanBudget,
    ScrubWindowSource,
};

pub(crate) fn scrub_plan_identity(
    mode: ScrubMode,
    budget: ScrubPlanBudget,
    yield_after_windows: Option<u64>,
    windows: &[PlannedScrubWindow<'_>],
) -> u64 {
    let seed = mix(0xcbf2_9ce4_8422_2325, mode_tag(mode));
    let with_budget = [
        budget.resident_byte_limit(),
        budget.pin_page_limit() as u64,
        budget.allocation_byte_limit(),
    ]
    .into_iter()
    .chain([
        budget.streaming_window_byte_limit(),
        budget.protected_read_limit(),
        yield_after_windows.unwrap_or(u64::MAX),
        windows.len() as u64,
    ])
    .fold(seed, mix);

    windows.iter().fold(with_budget, |acc, planned| {
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

const fn source_tag(source: ScrubWindowSource) -> u64 {
    match source {
        ScrubWindowSource::OnlineProtectedRead => 0x33,
        ScrubWindowSource::OfflineDeclaredVerifierInput => 0x44,
    }
}

const fn status_tag(status: PlannedScrubWindowStatus) -> u64 {
    match status {
        PlannedScrubWindowStatus::Inspect => 0x55,
        PlannedScrubWindowStatus::Skip => 0x66,
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::ResidentMemory) => 0x77,
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::PinPage) => 0x88,
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::Allocation) => 0x99,
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::StreamingWindow) => 0xaa,
        PlannedScrubWindowStatus::DeferOverBudget(ScrubOverBudgetClass::ProtectedRead) => 0xbb,
    }
}
