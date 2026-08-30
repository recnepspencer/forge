use worth_query_installation::facade::{
    WorthQueryArtifactComparisonAuthority as Comparison,
    WorthQueryArtifactDeterminismPosture as Determinism,
    WorthQueryArtifactOccurrenceContract as Occurrence,
    WorthQueryArtifactOccurrenceIdentityPolicy as IdentityPolicy,
    WorthQueryArtifactReproducibilityClass as ReproducibilityClass,
    WorthQueryArtifactReproducibilityContract as Reproducibility,
    WorthQueryArtifactSubstitutionPurpose as SubstitutionPurpose,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::decode_budget::RecordDecodeAttempt;
use crate::record::sequence::{decode_sequence, require_canonical_sequence, write_sequence};

pub(super) fn write_occurrence(
    output: &mut dyn BinaryEncodingSink,
    contract: &Occurrence,
) -> Result<(), Denial> {
    output.u16(identity_policy_tag(contract.identity_policy()))?;
    write_sequence(
        output,
        contract.permitted_substitutions(),
        |output, purpose| output.u16(substitution_tag(*purpose)),
    )
}

pub(super) fn decode_occurrence(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Occurrence, Denial> {
    let policy = identity_policy_from_tag(input.u16()?)?;
    let substitutions = decode_sequence(input, budget, 2, |input, _| {
        substitution_from_tag(input.u16()?)
    })?;
    require_canonical_sequence(&substitutions)?;
    let mut contract = match policy {
        IdentityPolicy::IndependentPerExecution => Occurrence::independent_per_execution(),
        IdentityPolicy::DomainMintedIndependent => Occurrence::domain_minted_independent(),
    };
    for purpose in substitutions {
        contract = contract.permit(purpose);
    }
    Ok(contract)
}

pub(super) fn write_reproducibility(
    output: &mut dyn BinaryEncodingSink,
    contract: &Reproducibility,
) -> Result<(), Denial> {
    output.u16(reproducibility_class_tag(contract.class()))?;
    output.u16(determinism_tag(contract.determinism()))?;
    write_comparison(output, contract.comparison())?;
    write_sequence(
        output,
        contract.environment_dependencies(),
        |output, value| output.text(value),
    )?;
    write_sequence(output, contract.entropy_dependencies(), |output, value| {
        output.text(value)
    })
}

pub(super) fn decode_reproducibility(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Reproducibility, Denial> {
    let class = reproducibility_class_from_tag(input.u16()?)?;
    let determinism = determinism_from_tag(input.u16()?)?;
    let comparison = decode_comparison(input)?;
    let environment_dependencies = decode_strings(input, budget)?;
    let entropy_dependencies = decode_strings(input, budget)?;
    Ok(Reproducibility::new(
        class,
        determinism,
        comparison,
        environment_dependencies,
        entropy_dependencies,
    ))
}

fn decode_strings(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Vec<String>, Denial> {
    let values = decode_sequence(input, budget, 4, |input, _| Ok(input.text()?.to_owned()))?;
    require_canonical_sequence(&values)?;
    Ok(values)
}

fn write_comparison(
    output: &mut dyn BinaryEncodingSink,
    comparison: &Comparison,
) -> Result<(), Denial> {
    match comparison {
        Comparison::NotDeclared => output.u16(1),
        Comparison::ExactCanonicalValue => output.u16(2),
        Comparison::CanonicalReduction { family } => tagged_text(output, 3, family),
        Comparison::RegisteredDomainComparator { family } => tagged_text(output, 4, family),
        Comparison::RegisteredErrorBoundComparator { family } => tagged_text(output, 5, family),
        Comparison::RegisteredDistributionTest { family } => tagged_text(output, 6, family),
        Comparison::NotComparable => output.u16(7),
    }
}

fn decode_comparison(input: &mut BinaryInput<'_>) -> Result<Comparison, Denial> {
    match input.u16()? {
        1 => Ok(Comparison::NotDeclared),
        2 => Ok(Comparison::ExactCanonicalValue),
        3 => Ok(Comparison::CanonicalReduction {
            family: input.text()?.to_owned(),
        }),
        4 => Ok(Comparison::RegisteredDomainComparator {
            family: input.text()?.to_owned(),
        }),
        5 => Ok(Comparison::RegisteredErrorBoundComparator {
            family: input.text()?.to_owned(),
        }),
        6 => Ok(Comparison::RegisteredDistributionTest {
            family: input.text()?.to_owned(),
        }),
        7 => Ok(Comparison::NotComparable),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn tagged_text(output: &mut dyn BinaryEncodingSink, tag: u16, value: &str) -> Result<(), Denial> {
    output.u16(tag)?;
    output.text(value)
}

const fn identity_policy_tag(value: IdentityPolicy) -> u16 {
    match value {
        IdentityPolicy::IndependentPerExecution => 1,
        IdentityPolicy::DomainMintedIndependent => 2,
    }
}

fn identity_policy_from_tag(tag: u16) -> Result<IdentityPolicy, Denial> {
    match tag {
        1 => Ok(IdentityPolicy::IndependentPerExecution),
        2 => Ok(IdentityPolicy::DomainMintedIndependent),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn substitution_tag(value: SubstitutionPurpose) -> u16 {
    match value {
        SubstitutionPurpose::ComputationalReuse => 1,
        SubstitutionPurpose::EvidentiarySubstitution => 2,
    }
}

fn substitution_from_tag(tag: u16) -> Result<SubstitutionPurpose, Denial> {
    match tag {
        1 => Ok(SubstitutionPurpose::ComputationalReuse),
        2 => Ok(SubstitutionPurpose::EvidentiarySubstitution),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn reproducibility_class_tag(value: ReproducibilityClass) -> u16 {
    match value {
        ReproducibilityClass::ExactDeterministic => 1,
        ReproducibilityClass::SeededDeterministic => 2,
        ReproducibilityClass::CanonicalReduction => 3,
        ReproducibilityClass::DomainComparator => 4,
        ReproducibilityClass::IntervalOrErrorBound => 5,
        ReproducibilityClass::Distributional => 6,
        ReproducibilityClass::NonReplayable => 7,
    }
}

fn reproducibility_class_from_tag(tag: u16) -> Result<ReproducibilityClass, Denial> {
    match tag {
        1 => Ok(ReproducibilityClass::ExactDeterministic),
        2 => Ok(ReproducibilityClass::SeededDeterministic),
        3 => Ok(ReproducibilityClass::CanonicalReduction),
        4 => Ok(ReproducibilityClass::DomainComparator),
        5 => Ok(ReproducibilityClass::IntervalOrErrorBound),
        6 => Ok(ReproducibilityClass::Distributional),
        7 => Ok(ReproducibilityClass::NonReplayable),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn determinism_tag(value: Determinism) -> u16 {
    match value {
        Determinism::Deterministic => 1,
        Determinism::SeededDeterministic => 2,
        Determinism::EnvironmentDependent => 3,
        Determinism::EntropyDependent => 4,
        Determinism::Nondeterministic => 5,
    }
}

fn determinism_from_tag(tag: u16) -> Result<Determinism, Denial> {
    match tag {
        1 => Ok(Determinism::Deterministic),
        2 => Ok(Determinism::SeededDeterministic),
        3 => Ok(Determinism::EnvironmentDependent),
        4 => Ok(Determinism::EntropyDependent),
        5 => Ok(Determinism::Nondeterministic),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}
