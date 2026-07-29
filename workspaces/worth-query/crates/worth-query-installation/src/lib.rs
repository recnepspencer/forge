//! Portable Query installation authority.
//!
//! This package owns callback-free domain package meaning, installation
//! admission, generation affinity, conflict semantics, and disposable
//! installed indexes. Execution providers and workspaces are downstream.

#![forbid(unsafe_code)]

mod admission;
mod application_ability;
mod application_operation;
mod application_principal_binding;
mod application_schema;
mod canonical_hash_encoding;
mod domain_computation;
mod domain_operation;
mod generation;
mod installed_domain_operation;
mod installed_graph_participation;
mod installed_handle_denial;
mod installed_index;
mod installed_operation;
mod package;
mod package_requirements;

#[cfg(test)]
mod admission_profile_tests;
#[cfg(test)]
mod application_principal_binding_tests;
#[cfg(test)]
mod application_schema_tests;
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
mod domain_computation_evidence_conflict_tests;
#[cfg(test)]
mod domain_computation_evidence_schema_tests;
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
    pub use worth_query_declaration::facade::application_schema::{
        ApplicationAuthorizationPath, ApplicationAuthorizationPathEffect,
        ApplicationAuthorizationPredicate, ApplicationAuthorizationTraversal,
        ApplicationAuthorizationTraversalDirection, ApplicationEntityRef, ApplicationFieldCurrency,
        ApplicationFieldRef, ApplicationOperationDecisionReadTarget,
        ApplicationOperationProgramTarget, ApplicationRelationRef, ApplicationSchema,
        ApplicationSchemaBindingIdentity, ApplicationSchemaMember, EqualityPosture,
        EqualityPredicate, ErasedApplicationSchemaDeclaration, OperationCreates, OperationDeletes,
        OperationLinks, OperationReads, OperationUnlinks, OperationWrites,
        TypedApplicationIdentityValue, TypedApplicationReadableValue,
        TypedApplicationSignedAggregateValue, TypedApplicationValue, WritableCapability,
        WritePosture,
    };
    pub use worth_query_declaration::facade::authentication::{
        WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
    };

    pub use crate::admission::{
        WorthQueryAdmittedPortableDomainPackage, WorthQueryArtifactVersionSupport,
        WorthQueryInstallationAdmissionDenial, WorthQueryInstallationAdmissionDenialKind,
        WorthQueryInstallationAdmissionProfile, WorthQueryInstallationSupportStatus,
    };
    pub use crate::application_ability::{
        WorthQueryAbilityInstallationDenial, WorthQueryAbilityInstallationDenialKind,
        WorthQueryInstalledAbility,
    };
    pub use crate::application_operation::{
        WorthQueryApplicationOperationInstallationDenial,
        WorthQueryApplicationOperationInstallationDenialKind,
        WorthQueryCompiledApplicationOperationContracts, WorthQueryInstalledAbilityRequirement,
        WorthQueryInstalledApplicationOperation, APPLICATION_AUTHORIZATION_FACT_FAMILY,
        APPLICATION_DECISION_FACT_FAMILY, APPLICATION_EXECUTION_ACCESS_PRODUCT_FAMILY,
        APPLICATION_EXECUTION_ALLOCATOR_FAMILY, APPLICATION_EXECUTION_PROVIDER_FAMILY,
        APPLICATION_EXECUTION_SAFE_POINT_FAMILY, APPLICATION_INVARIANT_SLOT,
    };
    pub use crate::application_principal_binding::{
        WorthQueryInstalledPrincipalBinding, WorthQueryPrincipalBindingInstallationDenial,
        WorthQueryPrincipalBindingInstallationDenialKind,
    };
    pub use crate::application_schema::{
        WorthQueryInstalledApplicationSchema, WorthQueryInstalledApplicationSchemaDenial,
        WorthQueryInstalledApplicationSchemaDenialKind,
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
    pub use crate::installed_handle_denial::{
        WorthQueryDomainHandleDenial, WorthQueryDomainHandleDenialKind,
    };
    pub use crate::installed_index::{
        WorthQueryInstalledPackageAuthority, WorthQueryInstalledPackageIndex,
        WorthQueryInstalledPackageIndexCounters, WorthQueryInstalledPackageIndexDenial,
        WorthQueryInstalledPackageIndexDenialKind, WorthQueryInstalledPackageIndexRebuildReport,
        WorthQueryInstalledPackageIndexRelation,
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
