use std::sync::Arc;

use worth_foundational::facade::{CanonicalDigestDerivationDenial, CanonicalDigestId};
use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaBindingIdentity, ApplicationSchemaDeclaration,
    ApplicationSchemaMemberProvenance,
};

use super::native_contract::WorthQueryInstalledApplicationSchemaContractCatalog;
use crate::application_capability::{
    compile_capability_registry, ApplicationCapabilityRegistry,
    WorthQueryApplicationCapabilityInstallationDenial,
};
use crate::application_operation::{
    compile_authorization_policy_registry, ApplicationAuthorizationPolicyRegistry,
};
use crate::canonical_work::WorthQueryCanonicalWorkEvidence;
use crate::installed_index::WorthQueryInstalledPackageAuthority;
use crate::package::{
    WorthQueryPortableApplicationOperationContractRecord,
    WorthQueryPortableNativeAspectContractRecord,
};

pub(crate) enum ApplicationSchemaCompilationDenial {
    Capability(WorthQueryApplicationCapabilityInstallationDenial),
    Canonical(CanonicalDigestDerivationDenial),
}

pub(crate) struct ApplicationSchemaCompilationInput<'a, Schema> {
    pub package_authority: WorthQueryInstalledPackageAuthority,
    pub declaration: &'a ApplicationSchemaDeclaration<Schema>,
    pub schema_identity: CanonicalDigestId,
    pub native_contract_catalog: Arc<WorthQueryInstalledApplicationSchemaContractCatalog>,
    pub portable_native_contracts: Arc<Vec<WorthQueryPortableNativeAspectContractRecord>>,
    pub portable_operation_contracts:
        Arc<Vec<WorthQueryPortableApplicationOperationContractRecord>>,
    pub upstream_installation_work: WorthQueryCanonicalWorkEvidence,
}

pub(crate) struct CompiledApplicationSchema<Schema> {
    pub package_authority: WorthQueryInstalledPackageAuthority,
    pub schema_name: String,
    pub schema_identity: CanonicalDigestId,
    pub schema:
        worth_query_declaration::facade::application_schema::ErasedApplicationSchemaDeclaration,
    pub member_provenance: ApplicationSchemaMemberProvenance,
    pub capability_registry: ApplicationCapabilityRegistry,
    pub authorization_policy_registry: ApplicationAuthorizationPolicyRegistry,
    pub native_contract_catalog: Arc<WorthQueryInstalledApplicationSchemaContractCatalog>,
    pub portable_native_contracts: Arc<Vec<WorthQueryPortableNativeAspectContractRecord>>,
    pub portable_operation_contracts:
        Arc<Vec<WorthQueryPortableApplicationOperationContractRecord>>,
    pub installation_canonical_work: WorthQueryCanonicalWorkEvidence,
    pub marker: std::marker::PhantomData<fn() -> Schema>,
}

pub(crate) fn compile_application_schema<Schema>(
    input: ApplicationSchemaCompilationInput<'_, Schema>,
) -> Result<CompiledApplicationSchema<Schema>, ApplicationSchemaCompilationDenial>
where
    Schema: ApplicationSchema,
{
    let binding_identity = ApplicationSchemaBindingIdentity::from_installed_parts(
        input.package_authority.runtime_ordinal,
        input.package_authority.generation.ordinal(),
        *input.package_authority.package_identity().digest(),
        input.schema_identity,
    );
    let members = input.declaration.erased().members();
    let capability_registry =
        compile_capability_registry(&input.package_authority, &binding_identity, members)
            .map_err(ApplicationSchemaCompilationDenial::Capability)?;
    let authorization_policy_registry = compile_authorization_policy_registry(members)
        .map_err(ApplicationSchemaCompilationDenial::Canonical)?;
    let capability_work = capability_registry.values().fold(
        WorthQueryCanonicalWorkEvidence::zero(),
        |work, capability| work.combine(capability.canonical().work()),
    );
    let policy_work = authorization_policy_registry
        .values()
        .flat_map(|scopes| scopes.values())
        .fold(
            WorthQueryCanonicalWorkEvidence::zero(),
            |work, requirement| work.combine(requirement.canonical_work()),
        );
    Ok(CompiledApplicationSchema {
        schema_name: input.declaration.erased().name().to_string(),
        schema_identity: input.schema_identity,
        schema: input.declaration.erased().clone(),
        member_provenance: input.declaration.member_provenance().clone(),
        package_authority: input.package_authority,
        capability_registry,
        authorization_policy_registry,
        native_contract_catalog: input.native_contract_catalog,
        portable_native_contracts: input.portable_native_contracts,
        portable_operation_contracts: input.portable_operation_contracts,
        installation_canonical_work: input
            .upstream_installation_work
            .combine(capability_work)
            .combine(policy_work),
        marker: std::marker::PhantomData,
    })
}
