#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFreeSpaceSearchPolicy {
    candidate_class_bound: u32,
    foreground_candidate_bound: u32,
}

impl PhysicalFreeSpaceSearchPolicy {
    pub const fn foreground_bounded(
        candidate_class_bound: u32,
        foreground_candidate_bound: u32,
    ) -> Self {
        Self {
            candidate_class_bound,
            foreground_candidate_bound,
        }
    }

    pub const fn candidate_class_bound(self) -> u32 {
        self.candidate_class_bound
    }

    pub const fn foreground_candidate_bound(self) -> u32 {
        self.foreground_candidate_bound
    }

    pub const fn evaluate(
        self,
        candidate_classes: u32,
        fragmented_candidates: u32,
    ) -> PhysicalForegroundBoundednessReport {
        let pressure = PhysicalFragmentationPressureReport::new(
            candidate_classes,
            fragmented_candidates,
            self,
        );
        let outcome = if candidate_classes > self.candidate_class_bound
            || fragmented_candidates > self.foreground_candidate_bound
        {
            PhysicalForegroundBoundednessOutcome::DeferredForMaintenance
        } else {
            PhysicalForegroundBoundednessOutcome::Bounded
        };
        PhysicalForegroundBoundednessReport::new(self, pressure, outcome)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFragmentationPressureReport {
    candidate_classes: u32,
    fragmented_candidates: u32,
    class_bound: u32,
    foreground_bound: u32,
}

impl PhysicalFragmentationPressureReport {
    const fn new(
        candidate_classes: u32,
        fragmented_candidates: u32,
        policy: PhysicalFreeSpaceSearchPolicy,
    ) -> Self {
        Self {
            candidate_classes,
            fragmented_candidates,
            class_bound: policy.candidate_class_bound(),
            foreground_bound: policy.foreground_candidate_bound(),
        }
    }

    pub const fn candidate_classes(self) -> u32 {
        self.candidate_classes
    }

    pub const fn fragmented_candidates(self) -> u32 {
        self.fragmented_candidates
    }

    pub const fn exceeds_policy(self) -> bool {
        self.candidate_classes > self.class_bound
            || self.fragmented_candidates > self.foreground_bound
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalForegroundBoundednessReport {
    policy: PhysicalFreeSpaceSearchPolicy,
    pressure: PhysicalFragmentationPressureReport,
    outcome: PhysicalForegroundBoundednessOutcome,
}

impl PhysicalForegroundBoundednessReport {
    const fn new(
        policy: PhysicalFreeSpaceSearchPolicy,
        pressure: PhysicalFragmentationPressureReport,
        outcome: PhysicalForegroundBoundednessOutcome,
    ) -> Self {
        Self {
            policy,
            pressure,
            outcome,
        }
    }

    pub const fn policy(self) -> PhysicalFreeSpaceSearchPolicy {
        self.policy
    }

    pub const fn pressure(self) -> PhysicalFragmentationPressureReport {
        self.pressure
    }

    pub const fn outcome(self) -> PhysicalForegroundBoundednessOutcome {
        self.outcome
    }

    pub const fn is_admitted(self) -> bool {
        matches!(self.outcome, PhysicalForegroundBoundednessOutcome::Bounded)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalForegroundBoundednessOutcome {
    Bounded,
    DeferredForMaintenance,
}
