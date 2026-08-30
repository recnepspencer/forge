#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeApplicationReadinessOwnerCount {
    count: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeApplicationReadinessOwnerCountDenial {
    CapacityExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeApplicationReadinessGrant {
    owner_ordinal: u8,
    generation: u64,
    physical_tick: u64,
    reduced_motion: UiNativeReducedMotionPosture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeReducedMotionPosture {
    NoPreference,
    Reduce,
    Unavailable,
}

impl UiNativeApplicationReadinessOwnerCount {
    pub const MAXIMUM: u8 = 6;

    pub const fn new(count: u8) -> Result<Self, UiNativeApplicationReadinessOwnerCountDenial> {
        if count <= Self::MAXIMUM {
            Ok(Self { count })
        } else {
            Err(UiNativeApplicationReadinessOwnerCountDenial::CapacityExceeded)
        }
    }

    pub const fn none() -> Self {
        Self { count: 0 }
    }

    pub const fn get(self) -> u8 {
        self.count
    }
}

impl UiNativeApplicationReadinessGrant {
    pub(in crate::native::event_loop) const fn issued(
        owner_ordinal: u8,
        generation: u64,
        physical_tick: u64,
        reduced_motion: UiNativeReducedMotionPosture,
    ) -> Self {
        Self {
            owner_ordinal,
            generation,
            physical_tick,
            reduced_motion,
        }
    }

    pub const fn owner_ordinal(&self) -> u8 {
        self.owner_ordinal
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub const fn physical_tick(&self) -> u64 {
        self.physical_tick
    }

    pub const fn reduced_motion(&self) -> UiNativeReducedMotionPosture {
        self.reduced_motion
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UiNativeApplicationReadinessGrant, UiNativeApplicationReadinessOwnerCount,
        UiNativeApplicationReadinessOwnerCountDenial, UiNativeReducedMotionPosture,
    };

    #[test]
    fn application_owner_count_reserves_six_total_slots_for_runtime_internal_readiness() {
        assert_eq!(UiNativeApplicationReadinessOwnerCount::none().get(), 0);
        assert_eq!(
            UiNativeApplicationReadinessOwnerCount::new(5)
                .expect("five application slots remain")
                .get(),
            5
        );
        assert_eq!(
            UiNativeApplicationReadinessOwnerCount::new(6),
            Ok(UiNativeApplicationReadinessOwnerCount { count: 6 })
        );
        assert_eq!(
            UiNativeApplicationReadinessOwnerCount::new(7),
            Err(UiNativeApplicationReadinessOwnerCountDenial::CapacityExceeded)
        );
    }

    #[test]
    fn readiness_grant_carries_host_clock_and_system_animation_posture() {
        let grant = UiNativeApplicationReadinessGrant::issued(
            5,
            19,
            3_141,
            UiNativeReducedMotionPosture::Reduce,
        );
        assert_eq!(grant.owner_ordinal(), 5);
        assert_eq!(grant.generation(), 19);
        assert_eq!(grant.physical_tick(), 3_141);
        assert_eq!(grant.reduced_motion(), UiNativeReducedMotionPosture::Reduce);
    }
}
