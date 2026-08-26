//! Complete typed logical record vocabulary for one portable package export.

use worth_query_declaration::facade::application_schema::WorthQueryPortableApplicationSchemaRecord;

use crate::application_operation::WorthQueryPortableApplicationConditionalOperationBinding;
use crate::domain_computation::WorthQueryPortableArtifactContractRecord;
use crate::package::{
    WorthQueryPortableApplicationOperationContractRecord, WorthQueryPortableDefinition,
    WorthQueryPortableDomainIdentity, WorthQueryPortableDomainOperationRecord,
    WorthQueryPortableNativeAspectContractRecord,
};
use crate::package_requirements::{
    WorthQueryInstallationCapabilityFamily, WorthQueryInstallationConfigSectionFamily,
    WorthQueryInstallationContributionCategory, WorthQueryInstallationOperatingRequirement,
};

/// Exhaustive package-export family inventory in canonical family order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum WorthQueryPortablePackageRecordFamily {
    DomainIdentity,
    CapabilityRequirement,
    ConfigurationRequirement,
    OperatingRequirement,
    Definition,
    DomainOperation,
    ArtifactContract,
    ApplicationSchema,
    ConditionalApplicationOperation,
    ContributionPolicy,
    NativeAspectContract,
    ApplicationOperationContract,
}

impl WorthQueryPortablePackageRecordFamily {
    pub const ALL: [Self; 12] = [
        Self::DomainIdentity,
        Self::CapabilityRequirement,
        Self::ConfigurationRequirement,
        Self::OperatingRequirement,
        Self::Definition,
        Self::DomainOperation,
        Self::ArtifactContract,
        Self::ApplicationSchema,
        Self::ConditionalApplicationOperation,
        Self::ContributionPolicy,
        Self::NativeAspectContract,
        Self::ApplicationOperationContract,
    ];

    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}

/// One authority-free logical record exported from a freshly validated package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPortablePackageRecord {
    DomainIdentity(WorthQueryPortableDomainIdentity),
    CapabilityRequirement(WorthQueryInstallationCapabilityFamily),
    ConfigurationRequirement(WorthQueryInstallationConfigSectionFamily),
    OperatingRequirement(WorthQueryInstallationOperatingRequirement),
    Definition(WorthQueryPortableDefinition),
    DomainOperation(WorthQueryPortableDomainOperationRecord),
    ArtifactContract(WorthQueryPortableArtifactContractRecord),
    ApplicationSchema(WorthQueryPortableApplicationSchemaRecord),
    ConditionalApplicationOperation(WorthQueryPortableApplicationConditionalOperationBinding),
    ContributionPolicy(WorthQueryInstallationContributionCategory),
    NativeAspectContract(WorthQueryPortableNativeAspectContractRecord),
    ApplicationOperationContract(WorthQueryPortableApplicationOperationContractRecord),
}

impl WorthQueryPortablePackageRecord {
    pub const fn family(&self) -> WorthQueryPortablePackageRecordFamily {
        match self {
            Self::DomainIdentity(_) => WorthQueryPortablePackageRecordFamily::DomainIdentity,
            Self::CapabilityRequirement(_) => {
                WorthQueryPortablePackageRecordFamily::CapabilityRequirement
            }
            Self::ConfigurationRequirement(_) => {
                WorthQueryPortablePackageRecordFamily::ConfigurationRequirement
            }
            Self::OperatingRequirement(_) => {
                WorthQueryPortablePackageRecordFamily::OperatingRequirement
            }
            Self::Definition(_) => WorthQueryPortablePackageRecordFamily::Definition,
            Self::DomainOperation(_) => WorthQueryPortablePackageRecordFamily::DomainOperation,
            Self::ArtifactContract(_) => WorthQueryPortablePackageRecordFamily::ArtifactContract,
            Self::ApplicationSchema(_) => WorthQueryPortablePackageRecordFamily::ApplicationSchema,
            Self::ConditionalApplicationOperation(_) => {
                WorthQueryPortablePackageRecordFamily::ConditionalApplicationOperation
            }
            Self::ContributionPolicy(_) => {
                WorthQueryPortablePackageRecordFamily::ContributionPolicy
            }
            Self::NativeAspectContract(_) => {
                WorthQueryPortablePackageRecordFamily::NativeAspectContract
            }
            Self::ApplicationOperationContract(_) => {
                WorthQueryPortablePackageRecordFamily::ApplicationOperationContract
            }
        }
    }
}
