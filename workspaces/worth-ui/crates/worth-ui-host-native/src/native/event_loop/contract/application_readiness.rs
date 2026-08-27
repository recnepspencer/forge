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
}

impl UiNativeApplicationReadinessOwnerCount {
    pub const MAXIMUM: u8 = 5;

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
    pub(in crate::native::event_loop) const fn issued(owner_ordinal: u8, generation: u64) -> Self {
        Self {
            owner_ordinal,
            generation,
        }
    }

    pub const fn owner_ordinal(&self) -> u8 {
        self.owner_ordinal
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UiNativeApplicationReadinessOwnerCount, UiNativeApplicationReadinessOwnerCountDenial,
    };

    #[test]
    fn application_owner_count_preserves_the_five_slots_after_mechanics_registration() {
        assert_eq!(UiNativeApplicationReadinessOwnerCount::none().get(), 0);
        assert_eq!(
            UiNativeApplicationReadinessOwnerCount::new(5)
                .expect("five application slots remain")
                .get(),
            5
        );
        assert_eq!(
            UiNativeApplicationReadinessOwnerCount::new(6),
            Err(UiNativeApplicationReadinessOwnerCountDenial::CapacityExceeded)
        );
    }
}
