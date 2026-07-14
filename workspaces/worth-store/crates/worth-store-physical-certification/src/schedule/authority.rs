use super::ScheduleReplayDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScheduleOrderingAuthorityAttempt {
    DeterministicActorSteps,
    WallClock,
    UnorderedMapIteration,
    AmbientThreadOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScheduleOrderingAuthorityKind {
    DeterministicActorSteps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdmittedScheduleOrderingAuthority {
    kind: ScheduleOrderingAuthorityKind,
}

impl ScheduleOrderingAuthorityAttempt {
    pub const fn deterministic_actor_steps() -> Self {
        Self::DeterministicActorSteps
    }

    pub const fn wall_clock() -> Self {
        Self::WallClock
    }

    pub const fn unordered_map_iteration() -> Self {
        Self::UnorderedMapIteration
    }

    pub const fn ambient_thread_order() -> Self {
        Self::AmbientThreadOrder
    }

    pub fn admit(self) -> Result<AdmittedScheduleOrderingAuthority, ScheduleReplayDenial> {
        match self {
            Self::DeterministicActorSteps => Ok(AdmittedScheduleOrderingAuthority {
                kind: ScheduleOrderingAuthorityKind::DeterministicActorSteps,
            }),
            Self::WallClock => Err(ScheduleReplayDenial::WallClockScheduleDenied),
            Self::UnorderedMapIteration => Err(ScheduleReplayDenial::UnorderedMapScheduleDenied),
            Self::AmbientThreadOrder => Err(ScheduleReplayDenial::AmbientThreadScheduleDenied),
        }
    }
}

impl AdmittedScheduleOrderingAuthority {
    pub const fn kind(self) -> ScheduleOrderingAuthorityKind {
        self.kind
    }

    pub(crate) const fn canonical_token(self) -> &'static str {
        match self.kind {
            ScheduleOrderingAuthorityKind::DeterministicActorSteps => "deterministic-actor-steps",
        }
    }
}
