use crate::package::WorthQueryValidatedPortableDomainPackage;

use super::{
    WorthQueryArtifactVersionSupport, WorthQueryInstallationAdmissionProfile,
    WorthQueryInstallationSupportStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryInstallationAdmissionMeaning {
    support_identity: String,
    configuration_identity: String,
    capabilities: Vec<(String, WorthQueryInstallationSupportStatus)>,
    configuration: Vec<(String, bool)>,
    operating: Vec<(String, WorthQueryInstallationSupportStatus)>,
    artifact_versions: Vec<((String, u32, u32), WorthQueryArtifactVersionSupport)>,
    artifact_comparators: Vec<(String, WorthQueryInstallationSupportStatus)>,
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
        artifact_versions: package
            .artifact_contracts()
            .iter()
            .map(|contract| {
                let key = (
                    contract.family().as_str().to_string(),
                    contract.schema_version().get(),
                    contract.protocol_version().get(),
                );
                let status = profile.artifact_version_statuses[&key].clone();
                (key, status)
            })
            .collect(),
        artifact_comparators: package
            .artifact_contracts()
            .iter()
            .filter_map(|contract| contract.reproducibility().comparison().registered_family())
            .map(|family| {
                (
                    family.to_string(),
                    profile.artifact_comparator_statuses[family],
                )
            })
            .collect(),
    }
}
