use super::{
    S8LayoutStrategyFamily, S8StrategyDeclaration, S8StrategyDenial, S8StrategyInvariantSuite,
};
use crate::artifact_family::ArtifactFamilyLifecycleAdmission;
use crate::key_domain::{PhysicalKeyDomain, PhysicalKeyDomainWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AdmittedLayoutStrategy {
    declaration: S8StrategyDeclaration,
    invariants: S8StrategyInvariantSuite,
}

impl S8AdmittedLayoutStrategy {
    pub(crate) const fn new(
        declaration: S8StrategyDeclaration,
        invariants: S8StrategyInvariantSuite,
    ) -> Self {
        Self {
            declaration,
            invariants,
        }
    }

    pub const fn family(&self) -> S8LayoutStrategyFamily {
        self.declaration.family()
    }

    pub const fn invariant_suite(&self) -> S8StrategyInvariantSuite {
        self.invariants
    }

    pub const fn key_domain(&self) -> PhysicalKeyDomainWitness {
        self.declaration.key_domain()
    }

    pub const fn lifecycle(&self) -> ArtifactFamilyLifecycleAdmission {
        self.declaration.lifecycle()
    }
}

pub(crate) fn declare_strategy(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    family: S8LayoutStrategyFamily,
) -> Result<S8StrategyDeclaration, S8StrategyDenial> {
    if lifecycle.family_id() != key_domain.family_id() {
        return Err(S8StrategyDenial::FamilyDoesNotMatchKeyDomain);
    }

    if !family.is_baseline_family() {
        return Err(S8StrategyDenial::UnsupportedFamily);
    }

    let domain = key_domain.domain();
    let supported = match family {
        S8LayoutStrategyFamily::BTree => matches!(
            domain,
            PhysicalKeyDomain::PageAddressKey
                | PhysicalKeyDomain::SegmentAddressKey
                | PhysicalKeyDomain::ExtentAddressKey
                | PhysicalKeyDomain::PhysicalReferenceKey
        ),
        S8LayoutStrategyFamily::Lsm => matches!(
            domain,
            PhysicalKeyDomain::WalRecordKey | PhysicalKeyDomain::BlobIdentityKey
        ),
        S8LayoutStrategyFamily::ChunkTree | S8LayoutStrategyFamily::ExactScan => false,
    };

    if !supported {
        return Err(match family {
            S8LayoutStrategyFamily::BTree => {
                S8StrategyDenial::PhysicalKeyDomainDoesNotSupportBaselineBTree
            }
            S8LayoutStrategyFamily::Lsm => {
                S8StrategyDenial::PhysicalKeyDomainDoesNotSupportBaselineLsm
            }
            S8LayoutStrategyFamily::ChunkTree | S8LayoutStrategyFamily::ExactScan => {
                S8StrategyDenial::UnsupportedFamily
            }
        });
    }

    Ok(S8StrategyDeclaration::new(lifecycle, key_domain, family))
}

pub(crate) fn admit_baseline_strategy(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    family: S8LayoutStrategyFamily,
) -> Result<S8AdmittedLayoutStrategy, S8StrategyDenial> {
    let declaration = declare_strategy(lifecycle, key_domain, family)?;
    let invariants = S8StrategyInvariantSuite::declare(declaration)?;
    Ok(S8AdmittedLayoutStrategy::new(declaration, invariants))
}
