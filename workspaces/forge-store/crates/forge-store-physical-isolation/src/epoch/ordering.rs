#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalOrderingSite {
    RootSwap,
    HazardPublication,
    ReaderEpochPublication,
    GenerationAdvancement,
    AllocatorPublication,
    Validation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalOrderingStrength {
    AcquireRelease,
    SequentiallyConsistent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalOrderingContract {
    site: PhysicalOrderingSite,
    strength: PhysicalOrderingStrength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalOrderingContractDenial {
    RelaxedOrdering,
    AmbientOrdering,
    WrongOrderingSite {
        expected: PhysicalOrderingSite,
        actual: PhysicalOrderingSite,
    },
}

impl PhysicalOrderingContract {
    pub const fn acquire_release_for(site: PhysicalOrderingSite) -> Self {
        Self {
            site,
            strength: PhysicalOrderingStrength::AcquireRelease,
        }
    }

    pub const fn sequentially_consistent_for(site: PhysicalOrderingSite) -> Self {
        Self {
            site,
            strength: PhysicalOrderingStrength::SequentiallyConsistent,
        }
    }

    pub const fn root_swap_acquire_release() -> Self {
        Self::acquire_release_for(PhysicalOrderingSite::RootSwap)
    }

    pub const fn site(self) -> PhysicalOrderingSite {
        self.site
    }

    pub const fn strength(self) -> PhysicalOrderingStrength {
        self.strength
    }

    pub fn require_site(
        self,
        expected: PhysicalOrderingSite,
    ) -> Result<Self, PhysicalOrderingContractDenial> {
        if self.site == expected {
            Ok(self)
        } else {
            Err(PhysicalOrderingContractDenial::WrongOrderingSite {
                expected,
                actual: self.site,
            })
        }
    }

    pub const fn reject_relaxed() -> Result<Self, PhysicalOrderingContractDenial> {
        Err(PhysicalOrderingContractDenial::RelaxedOrdering)
    }

    pub const fn reject_ambient() -> Result<Self, PhysicalOrderingContractDenial> {
        Err(PhysicalOrderingContractDenial::AmbientOrdering)
    }
}

pub const fn required_physical_isolation_ordering_contracts() -> [PhysicalOrderingContract; 6] {
    [
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::RootSwap),
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::HazardPublication),
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::ReaderEpochPublication),
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::GenerationAdvancement),
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::AllocatorPublication),
        PhysicalOrderingContract::acquire_release_for(PhysicalOrderingSite::Validation),
    ]
}
