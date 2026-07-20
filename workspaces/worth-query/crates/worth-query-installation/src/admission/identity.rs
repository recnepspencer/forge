use sha2::{Digest, Sha256};

use crate::canonical_hash_encoding::hash_text_field;
use crate::package::WorthQueryValidatedPortableDomainPackage;

use super::{WorthQueryInstallationAdmissionProfile, WorthQueryInstallationSupportStatus};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryInstallationAdmissionMeaning {
    support_identity: String,
    configuration_identity: String,
    capabilities: Vec<(String, WorthQueryInstallationSupportStatus)>,
    configuration: Vec<(String, bool)>,
    operating: Vec<(String, WorthQueryInstallationSupportStatus)>,
}

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
    for family in package.capabilities() {
        let status = profile.capability_statuses[family.as_str()];
        hash_profile_row(
            &mut hasher,
            "capability",
            family.as_str(),
            status.canonical_part(),
        );
    }
    for section in package.configuration() {
        hash_profile_row(
            &mut hasher,
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
            &mut hasher,
            "operating",
            requirement.as_str(),
            status.canonical_part(),
        );
    }
    format!("{:x}", hasher.finalize())
}

pub(super) fn admission_meaning(
    package: &WorthQueryValidatedPortableDomainPackage,
    profile: &WorthQueryInstallationAdmissionProfile,
) -> WorthQueryInstallationAdmissionMeaning {
    WorthQueryInstallationAdmissionMeaning {
        support_identity: profile.support_identity.clone(),
        configuration_identity: profile.configuration_identity.clone(),
        capabilities: package
            .capabilities()
            .iter()
            .map(|family| {
                (
                    family.as_str().to_string(),
                    profile.capability_statuses[family.as_str()],
                )
            })
            .collect(),
        configuration: package
            .configuration()
            .iter()
            .map(|section| {
                (
                    section.as_str().to_string(),
                    profile.configuration_statuses[section.as_str()],
                )
            })
            .collect(),
        operating: package
            .operating_requirements()
            .iter()
            .map(|requirement| {
                (
                    requirement.as_str().to_string(),
                    profile.operating_statuses[requirement.as_str()],
                )
            })
            .collect(),
    }
}

fn hash_profile_row(hasher: &mut Sha256, dimension: &str, subject: &str, status: &str) {
    hash_text_field(hasher, "profile-dimension", dimension);
    hash_text_field(hasher, "profile-subject", subject);
    hash_text_field(hasher, "profile-status", status);
}
