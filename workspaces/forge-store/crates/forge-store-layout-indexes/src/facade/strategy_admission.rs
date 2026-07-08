use crate::artifact_family::ArtifactFamilyLifecycleAdmission;
use crate::key_domain::PhysicalKeyDomainWitness;
use crate::strategy::admit_baseline_strategy;
pub use crate::strategy::{S8AdmittedLayoutStrategy, S8LayoutStrategyFamily, S8StrategyDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrategyAdmissionFacade;

impl StrategyAdmissionFacade {
    pub fn admit_baseline_strategy(
        &self,
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        family: S8LayoutStrategyFamily,
    ) -> Result<S8AdmittedLayoutStrategy, S8StrategyDenial> {
        admit_baseline_strategy(lifecycle, key_domain, family)
    }
}

pub const fn strategy_admission() -> StrategyAdmissionFacade {
    StrategyAdmissionFacade
}
