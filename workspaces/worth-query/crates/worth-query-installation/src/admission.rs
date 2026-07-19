use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use crate::canonical_hash_encoding::hash_text_field;
use crate::package::WorthQueryValidatedPortableDomainPackage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstallationSupportStatus {
    Admitted,
    Deferred,
    Unsupported,
}

impl WorthQueryInstallationSupportStatus {
    fn canonical_part(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorthQueryInstallationAdmissionProfile {
    support_identity: String,
    configuration_identity: String,
    capability_statuses: BTreeMap<String, WorthQueryInstallationSupportStatus>,
    configuration_statuses: BTreeMap<String, bool>,
    operating_statuses: BTreeMap<String, WorthQueryInstallationSupportStatus>,
    conflicting_rows: BTreeSet<String>,
}

impl WorthQueryInstallationAdmissionProfile {
    pub fn new(
        support_identity: impl Into<String>,
        configuration_identity: impl Into<String>,
    ) -> Self {
        Self {
            support_identity: support_identity.into(),
            configuration_identity: configuration_identity.into(),
            capability_statuses: BTreeMap::new(),
            configuration_statuses: BTreeMap::new(),
            operating_statuses: BTreeMap::new(),
            conflicting_rows: BTreeSet::new(),
        }
    }

    pub fn capability(
        mut self,
        family: impl Into<String>,
        status: WorthQueryInstallationSupportStatus,
    ) -> Self {
        let family = family.into();
        retain_profile_row(
            &mut self.capability_statuses,
            &mut self.conflicting_rows,
            "capability",
            family,
            status,
        );
        self
    }

    pub fn configuration(mut self, section: impl Into<String>, enabled: bool) -> Self {
        let section = section.into();
        retain_profile_row(
            &mut self.configuration_statuses,
            &mut self.conflicting_rows,
            "configuration",
            section,
            enabled,
        );
        self
    }

    pub fn operating_requirement(
        mut self,
        requirement: impl Into<String>,
        status: WorthQueryInstallationSupportStatus,
    ) -> Self {
        let requirement = requirement.into();
        retain_profile_row(
            &mut self.operating_statuses,
            &mut self.conflicting_rows,
            "operating",
            requirement,
            status,
        );
        self
    }

    pub fn admit(
        &self,
        package: WorthQueryValidatedPortableDomainPackage,
    ) -> Result<WorthQueryAdmittedPortableDomainPackage, WorthQueryInstallationAdmissionDenial>
    {
        if self.support_identity.trim().is_empty() {
            return Err(WorthQueryInstallationAdmissionDenial {
                kind: WorthQueryInstallationAdmissionDenialKind::InvalidSupportProfileIdentity,
                subject: self.support_identity.clone(),
            });
        }
        if self.configuration_identity.trim().is_empty() {
            return Err(WorthQueryInstallationAdmissionDenial {
                kind:
                    WorthQueryInstallationAdmissionDenialKind::InvalidConfigurationProfileIdentity,
                subject: self.configuration_identity.clone(),
            });
        }
        if let Some(conflict) = self.conflicting_rows.first() {
            return Err(WorthQueryInstallationAdmissionDenial {
                kind: WorthQueryInstallationAdmissionDenialKind::ConflictingProfileRow,
                subject: conflict.clone(),
            });
        }
        for capability in package.capabilities() {
            match self
                .capability_statuses
                .get(capability.as_str())
                .copied()
                .unwrap_or(WorthQueryInstallationSupportStatus::Unsupported)
            {
                WorthQueryInstallationSupportStatus::Admitted => {}
                WorthQueryInstallationSupportStatus::Deferred => {
                    return Err(WorthQueryInstallationAdmissionDenial {
                        kind: WorthQueryInstallationAdmissionDenialKind::DeferredCapability,
                        subject: capability.as_str().to_string(),
                    });
                }
                WorthQueryInstallationSupportStatus::Unsupported => {
                    return Err(WorthQueryInstallationAdmissionDenial {
                        kind: WorthQueryInstallationAdmissionDenialKind::UnsupportedCapability,
                        subject: capability.as_str().to_string(),
                    });
                }
            }
        }

        for section in package.configuration() {
            if !self
                .configuration_statuses
                .get(section.as_str())
                .copied()
                .unwrap_or(false)
            {
                return Err(WorthQueryInstallationAdmissionDenial {
                    kind: WorthQueryInstallationAdmissionDenialKind::DisabledConfiguration,
                    subject: section.as_str().to_string(),
                });
            }
        }

        for requirement in package.operating_requirements() {
            match self
                .operating_statuses
                .get(requirement.as_str())
                .copied()
                .unwrap_or(WorthQueryInstallationSupportStatus::Unsupported)
            {
                WorthQueryInstallationSupportStatus::Admitted => {}
                WorthQueryInstallationSupportStatus::Deferred => {
                    return Err(WorthQueryInstallationAdmissionDenial {
                        kind:
                            WorthQueryInstallationAdmissionDenialKind::DeferredOperatingRequirement,
                        subject: requirement.as_str().to_string(),
                    });
                }
                WorthQueryInstallationSupportStatus::Unsupported => {
                    return Err(WorthQueryInstallationAdmissionDenial {
                        kind: WorthQueryInstallationAdmissionDenialKind::UnsupportedOperatingRequirement,
                        subject: requirement.as_str().to_string(),
                    });
                }
            }
        }

        let admission_identity = admission_identity(&package, self);
        Ok(WorthQueryAdmittedPortableDomainPackage {
            package,
            admission_identity,
        })
    }
}

#[derive(Clone, Debug)]
pub struct WorthQueryAdmittedPortableDomainPackage {
    package: WorthQueryValidatedPortableDomainPackage,
    admission_identity: String,
}

impl WorthQueryAdmittedPortableDomainPackage {
    pub fn package(&self) -> &WorthQueryValidatedPortableDomainPackage {
        &self.package
    }

    pub fn admission_identity(&self) -> &str {
        &self.admission_identity
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstallationAdmissionDenialKind {
    InvalidSupportProfileIdentity,
    InvalidConfigurationProfileIdentity,
    ConflictingProfileRow,
    UnsupportedCapability,
    DeferredCapability,
    DisabledConfiguration,
    DeferredOperatingRequirement,
    UnsupportedOperatingRequirement,
}

fn retain_profile_row<T: Copy + Eq>(
    rows: &mut BTreeMap<String, T>,
    conflicts: &mut BTreeSet<String>,
    dimension: &str,
    subject: String,
    value: T,
) {
    if rows
        .insert(subject.clone(), value)
        .is_some_and(|prior| prior != value)
    {
        conflicts.insert(format!("{dimension}:{subject}"));
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstallationAdmissionDenial {
    kind: WorthQueryInstallationAdmissionDenialKind,
    subject: String,
}

impl WorthQueryInstallationAdmissionDenial {
    pub fn kind(&self) -> WorthQueryInstallationAdmissionDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

fn admission_identity(
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
        let status = profile
            .capability_statuses
            .get(family.as_str())
            .expect("admitted package capability has a retained profile row");
        hash_profile_row(
            &mut hasher,
            "capability",
            family.as_str(),
            status.canonical_part(),
        );
    }
    for section in package.configuration() {
        let enabled = profile
            .configuration_statuses
            .get(section.as_str())
            .expect("admitted package configuration has a retained profile row");
        hash_profile_row(
            &mut hasher,
            "configuration",
            section.as_str(),
            if *enabled { "enabled" } else { "disabled" },
        );
    }
    for requirement in package.operating_requirements() {
        let status = profile
            .operating_statuses
            .get(requirement.as_str())
            .expect("admitted operating requirement has a retained profile row");
        hash_profile_row(
            &mut hasher,
            "operating",
            requirement.as_str(),
            status.canonical_part(),
        );
    }
    format!("{:x}", hasher.finalize())
}

fn hash_profile_row(hasher: &mut Sha256, dimension: &str, subject: &str, status: &str) {
    hash_text_field(hasher, "profile-dimension", dimension);
    hash_text_field(hasher, "profile-subject", subject);
    hash_text_field(hasher, "profile-status", status);
}
