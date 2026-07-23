//! Portable Query installation authority.
//!
//! This package owns callback-free domain package meaning, installation
//! admission, generation affinity, conflict semantics, and disposable
//! installed indexes. Execution providers and workspaces are downstream.

#![forbid(unsafe_code)]

mod admission;
mod canonical_hash_encoding;
mod domain_computation;
mod domain_operation;
mod generation;
mod installed_domain_operation;
mod installed_graph_participation;
mod installed_index;
mod installed_operation;
mod package;
mod package_requirements;

#[cfg(test)]
mod admission_profile_tests;
#[cfg(test)]
mod domain_computation_admission_tests;
#[cfg(test)]
mod domain_computation_artifact_fixture;
#[cfg(test)]
mod domain_computation_authority_tests;
#[cfg(test)]
mod domain_computation_canonical_identity_tests;
#[cfg(test)]
mod domain_computation_conflict_tests;
#[cfg(test)]
mod domain_computation_evolution_tests;
#[cfg(test)]
mod domain_computation_occurrence_tests;
#[cfg(test)]
mod domain_computation_reproducibility_tests;
#[cfg(test)]
mod domain_computation_search_tests;
#[cfg(test)]
mod domain_computation_transformation_tests;
#[cfg(test)]
mod domain_computation_validation_tests;
#[cfg(test)]
mod domain_computation_workflow_closure_tests;
#[cfg(test)]
mod domain_computation_workflow_test_support;
#[cfg(test)]
mod package_validation_tests;

pub mod facade {
    pub use crate::admission::{
        WorthQueryAdmittedPortableDomainPackage, WorthQueryArtifactVersionSupport,
        WorthQueryInstallationAdmissionDenial, WorthQueryInstallationAdmissionDenialKind,
        WorthQueryInstallationAdmissionProfile, WorthQueryInstallationSupportStatus,
    };
    pub use crate::domain_computation::*;
    pub use crate::domain_operation::*;
    pub use crate::generation::{
        WorthQueryInstallationGeneration, WorthQueryInstallationRuntimeIdentity,
    };
    pub use crate::installed_domain_operation::{
        WorthQueryConditionalDependencyLookupDenial,
        WorthQueryInstalledConditionalDependencyAuthority,
        WorthQueryInstalledDomainOperationAuthority,
    };
    pub use crate::installed_graph_participation::WorthQueryInstalledGraphParticipationAuthority;
    pub use crate::installed_index::{
        WorthQueryInstalledPackageAuthority, WorthQueryInstalledPackageIndex,
        WorthQueryInstalledPackageIndexCounters, WorthQueryInstalledPackageIndexDenial,
        WorthQueryInstalledPackageIndexDenialKind, WorthQueryInstalledPackageIndexRebuildReport,
    };
    pub use crate::installed_operation::WorthQueryInstalledOperationAuthority;
    pub use crate::package::{
        WorthQueryPortableDefinition, WorthQueryPortableDefinitionKind,
        WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
        WorthQueryPortableDomainPackageIdentity, WorthQueryPortablePackageValidationDenial,
        WorthQueryPortablePackageValidationDenialKind, WorthQueryValidatedPortableDomainPackage,
    };
    pub use crate::package_requirements::{
        WorthQueryInstallationCapabilityFamily, WorthQueryInstallationConfigSectionFamily,
        WorthQueryInstallationContributionCategory, WorthQueryInstallationOperatingRequirement,
    };
}
