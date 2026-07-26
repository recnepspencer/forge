use sha2::{Digest, Sha256};

use crate::canonical_hash_encoding::hash_text_field;
use crate::package::WorthQueryValidatedPortableDomainPackage;

use super::WorthQueryInstallationAdmissionProfile;

pub(super) fn admission_identity(
    package: &WorthQueryValidatedPortableDomainPackage,
    profile: &WorthQueryInstallationAdmissionProfile,
) -> String {
    let mut hasher = Sha256::new();
    hash_text_field(&mut hasher, "package", package.identity().as_str());
    hash_text_field(&mut hasher, "support", &profile.support_identity);
    hash_text_field(
        &mut hasher,
        "configuration-profile",
        &profile.configuration_identity,
    );
    hash_required_support(&mut hasher, package, profile);
    hash_artifact_support(&mut hasher, package, profile);
    format!("{:x}", hasher.finalize())
}

fn hash_required_support(
    hasher: &mut Sha256,
    package: &WorthQueryValidatedPortableDomainPackage,
    profile: &WorthQueryInstallationAdmissionProfile,
) {
    for family in package.capabilities() {
        let status = profile.capability_statuses[family.as_str()];
        hash_profile_row(
            hasher,
            "capability",
            family.as_str(),
            status.canonical_part(),
        );
    }
    for section in package.configuration() {
        hash_profile_row(
            hasher,
            "configuration",
            section.as_str(),
            if profile.configuration_statuses[section.as_str()] {
                "enabled"
            } else {
                "disabled"
            },
        );
    }
    for requirement in package.operating_requirements() {
        let status = profile.operating_statuses[requirement.as_str()];
        hash_profile_row(
            hasher,
            "operating",
            requirement.as_str(),
            status.canonical_part(),
        );
    }
}

fn hash_artifact_support(
    hasher: &mut Sha256,
    package: &WorthQueryValidatedPortableDomainPackage,
    profile: &WorthQueryInstallationAdmissionProfile,
) {
    for contract in package.artifact_contracts() {
        let key = (
            contract.family().as_str().to_string(),
            contract.schema_version().get(),
            contract.protocol_version().get(),
        );
        let status = &profile.artifact_version_statuses[&key];
        hash_profile_row(
            hasher,
            "artifact-version",
            &format!("{}:{}:{}", key.0, key.1, key.2),
            &status.canonical_part(),
        );
        if let Some(comparator) = contract.reproducibility().comparison().registered_family() {
            hash_profile_row(
                hasher,
                "artifact-comparator",
                comparator,
                profile.artifact_comparator_statuses[comparator].canonical_part(),
            );
        }
    }
}

fn hash_profile_row(hasher: &mut Sha256, dimension: &str, subject: &str, status: &str) {
    hash_text_field(hasher, "profile-dimension", dimension);
    hash_text_field(hasher, "profile-subject", subject);
    hash_text_field(hasher, "profile-status", status);
}
