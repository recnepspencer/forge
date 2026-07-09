use worth_store_physical_certification::{
    PhysicalInterleavingSchedule, PhysicalSimulationPlan, ReplaySeed, ScheduleReplayDenial,
    StateSpaceBudget,
};

pub fn developer_smoke_replay_seed() -> ReplaySeed {
    ReplaySeed::from_u64(0x5eed_45)
}

pub fn ci_certification_replay_seed() -> ReplaySeed {
    ReplaySeed::from_u64(0xc1_ce_57)
}

pub fn developer_smoke_state_space_budget() -> StateSpaceBudget {
    StateSpaceBudget::bounded_steps(32).expect("developer smoke budget is bounded")
}

pub fn ci_certification_state_space_budget() -> StateSpaceBudget {
    StateSpaceBudget::bounded_steps(128).expect("CI certification budget is bounded")
}

pub fn deterministic_developer_smoke_schedule(
    plan: &PhysicalSimulationPlan,
) -> Result<PhysicalInterleavingSchedule, ScheduleReplayDenial> {
    PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        developer_smoke_replay_seed(),
        developer_smoke_state_space_budget(),
    )
}

pub fn deterministic_ci_certification_schedule(
    plan: &PhysicalSimulationPlan,
) -> Result<PhysicalInterleavingSchedule, ScheduleReplayDenial> {
    PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        ci_certification_replay_seed(),
        ci_certification_state_space_budget(),
    )
}
