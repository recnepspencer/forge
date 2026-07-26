use crate::domain_computation::{
    WorthQueryArtifactComparisonAuthority as Authority,
    WorthQueryArtifactDeterminismPosture as Determinism,
    WorthQueryArtifactReproducibilityClass as Class, WorthQueryPortableArtifactContract,
};

use super::{
    portable_text, WorthQueryArtifactContractValidationDenial,
    WorthQueryArtifactContractValidationDenialKind as Kind,
};

pub(super) fn validate(
    contract: &WorthQueryPortableArtifactContract,
) -> Result<(), WorthQueryArtifactContractValidationDenial> {
    let reproducibility = &contract.reproducibility;
    if !authority_matches(reproducibility.class(), reproducibility.comparison())
        || !determinism_matches(reproducibility.class(), reproducibility.determinism())
        || !dependency_shape_matches(reproducibility)
    {
        return Err(denial(contract, Kind::ReproducibilityAuthorityMismatch));
    }
    if contract.lifecycle.is_reusable()
        && matches!(
            reproducibility.comparison(),
            Authority::NotDeclared | Authority::NotComparable
        )
    {
        return Err(denial(contract, Kind::MissingReusableComparator));
    }
    if reproducibility
        .comparison()
        .registered_family()
        .is_some_and(|family| !portable_text(family))
        || reproducibility
            .environment_dependencies()
            .iter()
            .chain(reproducibility.entropy_dependencies())
            .any(|dependency| !portable_text(dependency))
    {
        return Err(denial(contract, Kind::ReproducibilityAuthorityMismatch));
    }
    Ok(())
}

fn authority_matches(class: Class, authority: &Authority) -> bool {
    matches!(
        (class, authority),
        (Class::ExactDeterministic, Authority::ExactCanonicalValue)
            | (Class::SeededDeterministic, Authority::ExactCanonicalValue)
            | (
                Class::CanonicalReduction,
                Authority::CanonicalReduction { .. }
            )
            | (
                Class::DomainComparator,
                Authority::RegisteredDomainComparator { .. }
            )
            | (
                Class::IntervalOrErrorBound,
                Authority::RegisteredErrorBoundComparator { .. }
            )
            | (
                Class::Distributional,
                Authority::RegisteredDistributionTest { .. }
            )
            | (Class::NonReplayable, Authority::NotComparable)
    )
}

fn determinism_matches(class: Class, determinism: Determinism) -> bool {
    matches!(
        (class, determinism),
        (
            Class::ExactDeterministic | Class::CanonicalReduction,
            Determinism::Deterministic
        ) | (Class::SeededDeterministic, Determinism::SeededDeterministic)
            | (Class::DomainComparator | Class::IntervalOrErrorBound, _)
            | (Class::Distributional, Determinism::EntropyDependent)
            | (Class::NonReplayable, Determinism::Nondeterministic)
    )
}

fn dependency_shape_matches(
    reproducibility: &crate::domain_computation::WorthQueryArtifactReproducibilityContract,
) -> bool {
    match reproducibility.class() {
        Class::ExactDeterministic => {
            reproducibility.environment_dependencies().is_empty()
                && reproducibility.entropy_dependencies().is_empty()
        }
        Class::SeededDeterministic | Class::Distributional => {
            !reproducibility.entropy_dependencies().is_empty()
        }
        Class::CanonicalReduction | Class::DomainComparator | Class::IntervalOrErrorBound => true,
        Class::NonReplayable => {
            !reproducibility.environment_dependencies().is_empty()
                || !reproducibility.entropy_dependencies().is_empty()
        }
    }
}

fn denial(
    contract: &WorthQueryPortableArtifactContract,
    kind: Kind,
) -> WorthQueryArtifactContractValidationDenial {
    WorthQueryArtifactContractValidationDenial::new(kind, contract.family.as_str())
}
