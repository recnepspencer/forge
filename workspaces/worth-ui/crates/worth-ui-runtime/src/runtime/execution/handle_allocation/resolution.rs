use super::{WorthUiHandleArenaIdentity, WorthUiRuntimeHandle, WorthUiRuntimeHandleLocator};
use crate::runtime::WorthUiPlanNodeInputFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHandleResolutionOutcome {
    Resolved,
    TargetMissing,
    ForeignSessionArena,
    StaleSlotGeneration,
    WrongFamily,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHandleResolutionEvidence {
    active_arena_identity: WorthUiHandleArenaIdentity,
    expected_family: WorthUiPlanNodeInputFamily,
    target: WorthUiRuntimeHandleLocator,
    resolved_family: Option<WorthUiPlanNodeInputFamily>,
    outcome: WorthUiHandleResolutionOutcome,
    direct_index_lookup_count: usize,
}

impl WorthUiHandleResolutionEvidence {
    pub fn active_arena_identity(self) -> WorthUiHandleArenaIdentity {
        self.active_arena_identity
    }

    pub fn expected_family(self) -> WorthUiPlanNodeInputFamily {
        self.expected_family
    }

    pub fn target(self) -> WorthUiRuntimeHandleLocator {
        self.target
    }

    pub fn resolved_family(self) -> Option<WorthUiPlanNodeInputFamily> {
        self.resolved_family
    }

    pub fn outcome(self) -> WorthUiHandleResolutionOutcome {
        self.outcome
    }

    pub fn direct_index_lookup_count(self) -> usize {
        self.direct_index_lookup_count
    }

    pub fn registry_lookup_count(self) -> usize {
        0
    }

    pub fn string_resolution_count(self) -> usize {
        0
    }
}

pub(crate) fn resolve_handle_row<T>(
    active_arena_identity: WorthUiHandleArenaIdentity,
    expected_family: WorthUiPlanNodeInputFamily,
    target: WorthUiRuntimeHandleLocator,
    row_for_plan_index: impl FnOnce(u32) -> Option<T>,
    runtime_handle: impl FnOnce(&T) -> WorthUiRuntimeHandle,
) -> Result<(T, WorthUiHandleResolutionEvidence), WorthUiHandleResolutionEvidence> {
    if target.arena_identity() != active_arena_identity {
        return Err(evidence(
            active_arena_identity,
            expected_family,
            target,
            None,
            WorthUiHandleResolutionOutcome::ForeignSessionArena,
            0,
        ));
    }

    let Some(row) = row_for_plan_index(target.plan_index()) else {
        return Err(evidence(
            active_arena_identity,
            expected_family,
            target,
            None,
            WorthUiHandleResolutionOutcome::TargetMissing,
            1,
        ));
    };
    let resolved_handle = runtime_handle(&row);
    let resolved_family = Some(resolved_handle.family());

    if resolved_handle.arena_identity() != active_arena_identity {
        return Err(evidence(
            active_arena_identity,
            expected_family,
            target,
            resolved_family,
            WorthUiHandleResolutionOutcome::ForeignSessionArena,
            1,
        ));
    }
    if resolved_handle.slot_generation() != target.slot_generation() {
        return Err(evidence(
            active_arena_identity,
            expected_family,
            target,
            resolved_family,
            WorthUiHandleResolutionOutcome::StaleSlotGeneration,
            1,
        ));
    }
    if resolved_handle.family() != expected_family {
        return Err(evidence(
            active_arena_identity,
            expected_family,
            target,
            resolved_family,
            WorthUiHandleResolutionOutcome::WrongFamily,
            1,
        ));
    }

    Ok((
        row,
        evidence(
            active_arena_identity,
            expected_family,
            target,
            resolved_family,
            WorthUiHandleResolutionOutcome::Resolved,
            1,
        ),
    ))
}

fn evidence(
    active_arena_identity: WorthUiHandleArenaIdentity,
    expected_family: WorthUiPlanNodeInputFamily,
    target: WorthUiRuntimeHandleLocator,
    resolved_family: Option<WorthUiPlanNodeInputFamily>,
    outcome: WorthUiHandleResolutionOutcome,
    direct_index_lookup_count: usize,
) -> WorthUiHandleResolutionEvidence {
    WorthUiHandleResolutionEvidence {
        active_arena_identity,
        expected_family,
        target,
        resolved_family,
        outcome,
        direct_index_lookup_count,
    }
}
