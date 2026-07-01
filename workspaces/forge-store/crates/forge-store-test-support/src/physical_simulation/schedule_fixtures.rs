use forge_store_physical_certification::{
    PhysicalInterleavingSchedule, PhysicalSimulationPlan, ReplaySeed, ScheduleReplayDenial,
    StateSpaceBudget,
};

pub fn developer_smoke_replay_seed() -> ReplaySeed {
    ReplaySeed::from_u64(0x5eed_45)
}

pub fn developer_smoke_state_space_budget() -> StateSpaceBudget {
    StateSpaceBudget::bounded_steps(32).expect("developer smoke budget is bounded")
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
