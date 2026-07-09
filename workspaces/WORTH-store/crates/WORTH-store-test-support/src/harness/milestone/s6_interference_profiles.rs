use worth_store_budgets::CounterEvidenceStrength;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S6InterferenceTestProfile {
    profile_scope: &'static str,
    queue_depth_strength: CounterEvidenceStrength,
}

pub const fn deterministic_s6_interference_profile() -> S6InterferenceTestProfile {
    S6InterferenceTestProfile {
        profile_scope: "s6.deterministic-miniature.interference",
        queue_depth_strength: CounterEvidenceStrength::Sampled,
    }
}

impl S6InterferenceTestProfile {
    pub const fn profile_scope(self) -> &'static str {
        self.profile_scope
    }

    pub const fn queue_depth_strength(self) -> CounterEvidenceStrength {
        self.queue_depth_strength
    }
}
