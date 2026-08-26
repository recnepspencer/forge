//! Exact source ordering checks before ordinary validation may normalize roots.

use crate::package::{
    WorthQueryPortableDomainPackage, WorthQueryPortablePackageRecordFamily as Family,
};

use super::{
    WorthQueryPortablePackageReconstructionDenial as Denial,
    WorthQueryReconstructedPortablePackageCandidate,
};

pub(super) fn validate_candidate_order(
    candidate: &WorthQueryReconstructedPortablePackageCandidate,
) -> Result<(), Denial> {
    let package = &candidate.package;
    validate_root_order(package)?;
    require_strict_by(
        &candidate.expected_native_aspects,
        Family::NativeAspectContract,
        |left, right| {
            (left.schema(), left.entity(), left.aspect())
                < (right.schema(), right.entity(), right.aspect())
        },
    )?;
    require_strict_by(
        &candidate.expected_application_operations,
        Family::ApplicationOperationContract,
        |left, right| {
            (left.schema(), left.operation(), left.input_type())
                < (right.schema(), right.operation(), right.input_type())
        },
    )
}

fn validate_root_order(package: &WorthQueryPortableDomainPackage) -> Result<(), Denial> {
    require_strict(&package.capabilities, Family::CapabilityRequirement)?;
    require_strict(&package.configuration, Family::ConfigurationRequirement)?;
    require_strict(&package.operating, Family::OperatingRequirement)?;
    require_strict(&package.definitions, Family::Definition)?;
    require_strict_by(
        &package.domain_operations,
        Family::DomainOperation,
        |left, right| left.identity() < right.identity(),
    )?;
    require_strict_by(
        &package.artifact_contracts,
        Family::ArtifactContract,
        |left, right| {
            (
                left.family(),
                left.schema_version(),
                left.protocol_version(),
            ) < (
                right.family(),
                right.schema_version(),
                right.protocol_version(),
            )
        },
    )?;
    require_strict_by(
        &package.application_schemas,
        Family::ApplicationSchema,
        |left, right| (left.name(), left.identity()) < (right.name(), right.identity()),
    )?;
    require_strict(
        &package.conditional_application_operations,
        Family::ConditionalApplicationOperation,
    )?;
    require_strict(&package.contributions, Family::ContributionPolicy)
}

fn require_strict<T: Ord>(values: &[T], family: Family) -> Result<(), Denial> {
    if values.windows(2).all(|pair| pair[0] < pair[1]) {
        Ok(())
    } else {
        Err(Denial::IllegalRecordOrdering { family })
    }
}

fn require_strict_by<T>(
    values: &[T],
    family: Family,
    precedes: impl Fn(&T, &T) -> bool,
) -> Result<(), Denial> {
    if values.windows(2).all(|pair| precedes(&pair[0], &pair[1])) {
        Ok(())
    } else {
        Err(Denial::IllegalRecordOrdering { family })
    }
}
