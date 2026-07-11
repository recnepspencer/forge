use super::admitted_strategy::S8AdmittedLayoutStrategy;
use super::counter_planning::{
    declared_strategy_counter_envelope, family_requires_shape_specific_lookup_envelope,
};
use super::key_law_validation::admit_strategy_key_laws;
use super::{
    S8LayoutStrategyFamily, S8StrategyDeclaration, S8StrategyDenial, S8StrategyInvariantSuite,
};
use crate::catalog::ArtifactFamilyLifecycleAdmission;
use crate::keyspace::{PhysicalKeyDomain, PhysicalKeyDomainWitness};

pub(crate) fn declare_strategy(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    family: S8LayoutStrategyFamily,
) -> Result<S8StrategyDeclaration, S8StrategyDenial> {
    require_matching_family(lifecycle, key_domain)?;
    require_supported_key_domain(family, key_domain.domain())?;
    let key_laws = admit_strategy_key_laws(family, key_domain)?;
    let declaration = match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => {
            S8StrategyDeclaration::baseline_btree_range(lifecycle, key_domain, key_laws)
        }
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => {
            S8StrategyDeclaration::baseline_lsm_write_optimized(lifecycle, key_domain, key_laws)
        }
        _ => return Err(S8StrategyDenial::UnsupportedFamily),
    };
    require_complete_declaration(declaration)?;
    Ok(declaration)
}

pub(crate) fn admit_strategy(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    family: S8LayoutStrategyFamily,
) -> Result<S8AdmittedLayoutStrategy, S8StrategyDenial> {
    let declaration = declare_strategy(lifecycle, key_domain, family)?;
    let invariants = S8StrategyInvariantSuite::declare(declaration).into_admitted()?;
    Ok(S8AdmittedLayoutStrategy::new(declaration, invariants))
}

fn require_matching_family(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
) -> Result<(), S8StrategyDenial> {
    if lifecycle.family_id() != key_domain.family_id() {
        return Err(S8StrategyDenial::FamilyDoesNotMatchKeyDomain);
    }
    Ok(())
}

fn require_supported_key_domain(
    family: S8LayoutStrategyFamily,
    domain: PhysicalKeyDomain,
) -> Result<(), S8StrategyDenial> {
    let supported = match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => matches!(
            domain,
            PhysicalKeyDomain::PageAddressKey
                | PhysicalKeyDomain::SegmentAddressKey
                | PhysicalKeyDomain::ExtentAddressKey
                | PhysicalKeyDomain::PhysicalReferenceKey
        ),
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => matches!(
            domain,
            PhysicalKeyDomain::WalRecordKey | PhysicalKeyDomain::BlobIdentityKey
        ),
        _ => return Err(S8StrategyDenial::UnsupportedFamily),
    };
    if supported {
        Ok(())
    } else {
        Err(match family {
            S8LayoutStrategyFamily::BaselineBTreeRange => {
                S8StrategyDenial::PhysicalKeyDomainDoesNotSupportBaselineBTree
            }
            S8LayoutStrategyFamily::BaselineLsmWriteOptimized => {
                S8StrategyDenial::PhysicalKeyDomainDoesNotSupportBaselineLsm
            }
            _ => S8StrategyDenial::UnsupportedFamily,
        })
    }
}

fn require_complete_declaration(
    declaration: S8StrategyDeclaration,
) -> Result<(), S8StrategyDenial> {
    if !declaration
        .capability()
        .allows_lane(declaration.access_lane())
    {
        return Err(S8StrategyDenial::StrategyDoesNotSupportDeclaredAccessLane);
    }
    if !family_requires_shape_specific_lookup_envelope(declaration.family())
        && declared_strategy_counter_envelope(declaration.family()).is_none()
    {
        return Err(S8StrategyDenial::StrategyDoesNotDeclarePlannedCounterEnvelope);
    }
    Ok(())
}
