#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InterferenceReplayScope {
    policy_decisions: bool,
    counter_topology: bool,
    proof_progression: bool,
    wall_clock_timing: bool,
    os_completion_order: bool,
}

impl InterferenceReplayScope {
    pub const fn deterministic_policy_counter_and_proof_scope() -> Self {
        Self {
            policy_decisions: true,
            counter_topology: true,
            proof_progression: true,
            wall_clock_timing: false,
            os_completion_order: false,
        }
    }

    pub const fn includes_policy_decisions(self) -> bool {
        self.policy_decisions
    }

    pub const fn includes_counter_topology(self) -> bool {
        self.counter_topology
    }

    pub const fn includes_proof_progression(self) -> bool {
        self.proof_progression
    }

    pub const fn excludes_wall_clock_timing(self) -> bool {
        !self.wall_clock_timing
    }

    pub const fn excludes_os_completion_order(self) -> bool {
        !self.os_completion_order
    }
}
