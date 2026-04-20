use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    evidence::StoreCounterSnapshot,
};

pub(crate) fn milestone_13_counter_contract<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone13CounterContract {
    crate::Milestone13CounterContract::from_snapshot(&backend.counters().snapshot())
}

pub(crate) fn milestone_13_complexity_surface<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone13ComplexitySurface {
    complexity_surface_from_snapshot(&backend.counters().snapshot())
}

fn complexity_surface_from_snapshot(
    snapshot: &StoreCounterSnapshot,
) -> crate::Milestone13ComplexitySurface {
    let mut surface = crate::Milestone13ComplexitySurface::phase_1_default();
    let phase_3_debt = "Phase 3 execution has not yet recorded a real bounded move/recall proof path";

    surface.placement_state_reconstruction = crate::Milestone13ComplexityPathStatus::verified(
        "phase 1 and 2 expose manifest-bounded placement-state vocabulary without inventory scans",
    );
    surface.working_set_classification = if snapshot.working_set_debt_count > 0 {
        crate::Milestone13ComplexityPathStatus::debt(
            "working-set classification has recorded explicit unsupported heuristic debt",
        )
    } else {
        crate::Milestone13ComplexityPathStatus::verified(
            "working-set observation and classification are lowered through scope-typed windows",
        )
    };
    surface.tier_move_planning = if snapshot.placement_debt_count > 0 {
        crate::Milestone13ComplexityPathStatus::debt(
            "tier move planning has recorded unsupported placement ambitions or illegal move posture",
        )
    } else {
        crate::Milestone13ComplexityPathStatus::verified(
            "tier move planning lowers conservative-policy placement into typed authoritative and derived plans",
        )
    };
    surface.tier_move_cutover = if snapshot.tier_move_cutover_count > 0 {
        crate::Milestone13ComplexityPathStatus::verified(
            "tier cutover persists canonical residency through typed verified-replica witnesses",
        )
    } else {
        crate::Milestone13ComplexityPathStatus::debt(phase_3_debt)
    };
    surface.tier_move_execution = if snapshot.authoritative_tier_move_count > 0
        || snapshot.derived_tier_move_count > 0
    {
        crate::Milestone13ComplexityPathStatus::verified(
            "tier execution consumes lowered move plans and persists in-flight transfer state",
        )
    } else {
        crate::Milestone13ComplexityPathStatus::debt(phase_3_debt)
    };
    surface.cold_recall_execution = if snapshot.cold_tier_recall_count > 0 {
        crate::Milestone13ComplexityPathStatus::verified(
            "cold recall executes through explicit leases and eligibility witnesses",
        )
    } else {
        crate::Milestone13ComplexityPathStatus::debt(phase_3_debt)
    };

    surface
}
