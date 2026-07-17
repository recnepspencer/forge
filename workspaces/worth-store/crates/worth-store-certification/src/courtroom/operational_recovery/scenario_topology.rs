use super::ScenarioScaleProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct S10Phase(pub(super) u8);

impl S10Phase {
    pub const fn number(self) -> u8 {
        self.0
    }

    pub const fn all() -> [Self; 19] {
        [
            Self(1),
            Self(2),
            Self(3),
            Self(4),
            Self(5),
            Self(6),
            Self(7),
            Self(8),
            Self(9),
            Self(10),
            Self(11),
            Self(12),
            Self(13),
            Self(14),
            Self(15),
            Self(16),
            Self(17),
            Self(18),
            Self(19),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S10OperationalScenarioKind {
    BurningPrimary,
    SplitBrainPromotion,
    AuthorityRepairRollback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10OperationalScenarioProgram {
    kind: S10OperationalScenarioKind,
    profile: ScenarioScaleProfile,
}

impl S10OperationalScenarioProgram {
    pub const fn new(kind: S10OperationalScenarioKind, profile: ScenarioScaleProfile) -> Self {
        Self { kind, profile }
    }

    pub const fn kind(self) -> S10OperationalScenarioKind {
        self.kind
    }

    pub const fn profile(self) -> ScenarioScaleProfile {
        self.profile
    }
}

impl S10OperationalScenarioKind {
    pub const fn token(self) -> &'static str {
        match self {
            Self::BurningPrimary => "burning-primary",
            Self::SplitBrainPromotion => "split-brain-promotion",
            Self::AuthorityRepairRollback => "authority-repair-rollback",
        }
    }
}
