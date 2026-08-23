pub(crate) const PHYSICAL_SIGNAL_ASPECT_COUNT: usize = 6;

use worth_signal::facade::Aspect;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePhysicalSignalAspect {
    HostLineage,
    WorkIdentity,
    Demand,
    Target,
    Submission,
    Recovery,
}

impl UiNativePhysicalSignalAspect {
    pub(crate) const fn from_index(index: usize) -> Self {
        match index {
            0 => Self::HostLineage,
            1 => Self::WorkIdentity,
            2 => Self::Demand,
            3 => Self::Target,
            4 => Self::Submission,
            5 => Self::Recovery,
            _ => panic!("physical Signal aspect index is bounded"),
        }
    }

    pub(crate) const fn index(self) -> usize {
        match self {
            Self::HostLineage => 0,
            Self::WorkIdentity => 1,
            Self::Demand => 2,
            Self::Target => 3,
            Self::Submission => 4,
            Self::Recovery => 5,
        }
    }

    pub(crate) const fn signal_aspect(self) -> Aspect {
        Aspect::new(self.index() as u8)
    }

    pub(crate) const fn partition(self) -> &'static str {
        match self {
            Self::HostLineage => "host-lineage",
            Self::WorkIdentity => "work-identity",
            Self::Demand => "demand-raster-key-set",
            Self::Target => "target-binding",
            Self::Submission => "submission",
            Self::Recovery => "recovery",
        }
    }

    pub(crate) const fn typed() -> [Self; PHYSICAL_SIGNAL_ASPECT_COUNT] {
        [
            Self::HostLineage,
            Self::WorkIdentity,
            Self::Demand,
            Self::Target,
            Self::Submission,
            Self::Recovery,
        ]
    }

    pub(crate) const fn all() -> [Aspect; PHYSICAL_SIGNAL_ASPECT_COUNT] {
        [
            Self::HostLineage.signal_aspect(),
            Self::WorkIdentity.signal_aspect(),
            Self::Demand.signal_aspect(),
            Self::Target.signal_aspect(),
            Self::Submission.signal_aspect(),
            Self::Recovery.signal_aspect(),
        ]
    }
}
