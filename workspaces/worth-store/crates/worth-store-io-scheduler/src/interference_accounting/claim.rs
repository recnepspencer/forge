use worth_store_budgets::CounterEvidenceStrength;

use crate::{QueueExecutionReplayIdentity, QueueWorkClass};

use super::InterferenceCounterName;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InterferenceCounterRequirement {
    name: InterferenceCounterName,
    required_strength: CounterEvidenceStrength,
    attribution_required: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LatencyEnvelopeClaim {
    profile_scope: &'static str,
    lane: QueueWorkClass,
    replay_identity: QueueExecutionReplayIdentity,
    max_interference_events: Option<u64>,
    requires_attribution: bool,
    requirements: Vec<InterferenceCounterRequirement>,
}

pub type InterferenceCounterClaim = LatencyEnvelopeClaim;

impl InterferenceCounterRequirement {
    pub const fn new(
        name: InterferenceCounterName,
        required_strength: CounterEvidenceStrength,
    ) -> Self {
        Self {
            name,
            required_strength,
            attribution_required: false,
        }
    }

    pub const fn foreground_wait() -> Self {
        Self::new(
            InterferenceCounterName::QueueForegroundWaitEvents,
            CounterEvidenceStrength::Exact,
        )
    }

    pub const fn queue_depth() -> Self {
        Self::new(
            InterferenceCounterName::QueuePeakDepth,
            CounterEvidenceStrength::Sampled,
        )
    }

    pub const fn violation_events() -> Self {
        Self::new(
            InterferenceCounterName::QueueViolationEvents,
            CounterEvidenceStrength::Exact,
        )
    }

    pub const fn with_strength(mut self, required_strength: CounterEvidenceStrength) -> Self {
        self.required_strength = required_strength;
        self
    }

    pub const fn requiring_attribution(mut self) -> Self {
        self.attribution_required = true;
        self
    }

    pub const fn name(self) -> InterferenceCounterName {
        self.name
    }

    pub const fn required_strength(self) -> CounterEvidenceStrength {
        self.required_strength
    }

    pub const fn attribution_required(self) -> bool {
        self.attribution_required
    }
}

impl LatencyEnvelopeClaim {
    pub const fn for_queue_execution(
        replay_identity: QueueExecutionReplayIdentity,
        profile_scope: &'static str,
        lane: QueueWorkClass,
    ) -> Self {
        Self {
            profile_scope,
            lane,
            replay_identity,
            max_interference_events: None,
            requires_attribution: false,
            requirements: Vec::new(),
        }
    }

    pub fn require_counter(mut self, requirement: InterferenceCounterRequirement) -> Self {
        self.requirements.push(requirement);
        self
    }

    pub const fn with_max_interference_events(mut self, max_interference_events: u64) -> Self {
        self.max_interference_events = Some(max_interference_events);
        self
    }

    pub const fn require_attribution(mut self) -> Self {
        self.requires_attribution = true;
        self
    }

    pub const fn profile_scope(&self) -> &'static str {
        self.profile_scope
    }

    pub const fn lane(&self) -> QueueWorkClass {
        self.lane
    }

    pub fn replay_identity(&self) -> QueueExecutionReplayIdentity {
        self.replay_identity.clone()
    }

    pub const fn max_interference_events(&self) -> Option<u64> {
        self.max_interference_events
    }

    pub const fn requires_attribution(&self) -> bool {
        self.requires_attribution
    }

    pub fn requirements(&self) -> &[InterferenceCounterRequirement] {
        &self.requirements
    }
}
