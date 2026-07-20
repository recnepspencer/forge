use std::collections::{BTreeMap, BTreeSet};

use worth_proof::{
    Admitted, AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness,
    CurrentValidity, FreshnessScopedBasis, Recipe, Unresolved,
};

use crate::package::WorthQueryValidatedPortableDomainPackage;

mod identity;

use identity::{admission_identity, admission_meaning, WorthQueryInstallationAdmissionMeaning};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstallationSupportStatus {
    Admitted,
    Deferred,
    Unsupported,
}

impl WorthQueryInstallationSupportStatus {
    pub(super) fn canonical_part(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
        let admission_meaning = admission_meaning(&package, self);
        let basis = WorthQueryInstallationAdmissionBasis {
            package_identity: package.identity().as_str().to_string(),
            support_identity: self.support_identity.clone(),
            configuration_identity: self.configuration_identity.clone(),
            admission_identity: admission_identity.clone(),
        };
        let resolved = Recipe::<Unresolved, _>::new(package).resolve_with_authority(
            basis,
            AuthorityWitness::from_authority_marker(InstallationProfileResolutionAuthority {
                _private: (),
            }),
        );
        let lowered = resolved.lower_with_capability(CapabilityWitness::from_capability_marker(
            InstallationSupportMatchedCapability { _private: () },
        ));
        let recipe = lowered.admit_with_authority(AuthorityWitness::from_authority_marker(
            InstallationAdmissionAuthority { _private: () },
        ));
        Ok(WorthQueryAdmittedPortableDomainPackage {
            recipe,
            admission_identity,
            admission_meaning,
        })
    }
}

#[derive(Clone, Debug)]
pub struct WorthQueryAdmittedPortableDomainPackage {
    recipe: AdmittedPortablePackageRecipe,
    admission_identity: String,
    admission_meaning: WorthQueryInstallationAdmissionMeaning,
}

impl WorthQueryAdmittedPortableDomainPackage {
    pub fn package(&self) -> &WorthQueryValidatedPortableDomainPackage {
        self.recipe.payload()
    }

    pub fn admission_identity(&self) -> &str {
        &self.admission_identity
    }

    pub(crate) fn has_same_admission_authority(&self, other: &Self) -> bool {
        self.admission_meaning == other.admission_meaning
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorthQueryInstallationAdmissionBasis {
    package_identity: String,
    support_identity: String,
    configuration_identity: String,
    admission_identity: String,
}

struct InstallationProfileResolutionAuthority {
    _private: (),
}
impl AuthorityMarker for InstallationProfileResolutionAuthority {}

struct InstallationSupportMatchedCapability {
    _private: (),
}
impl CapabilityMarker for InstallationSupportMatchedCapability {}

struct InstallationAdmissionAuthority {
    _private: (),
}
impl AuthorityMarker for InstallationAdmissionAuthority {}

type AdmittedPortablePackageRecipe = Recipe<
    Admitted,
    WorthQueryValidatedPortableDomainPackage,
    FreshnessScopedBasis<
        CurrentValidity,
        worth_proof::AssumptionBasis<WorthQueryInstallationAdmissionBasis>,
    >,
>;

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
