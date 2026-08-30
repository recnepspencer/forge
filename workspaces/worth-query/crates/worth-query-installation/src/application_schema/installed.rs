use std::marker::PhantomData;
use std::sync::Arc;

use super::compilation::CompiledApplicationSchema;
use super::native_contract::WorthQueryInstalledApplicationSchemaContractCatalog;
use super::principal_binding_match::{principal_binding_matches, principal_binding_name};
use crate::application_ability::{
    WorthQueryAbilityInstallationDenial, WorthQueryAbilityInstallationDenialKind,
    WorthQueryInstalledAbility,
};
use crate::application_capability::{
    ApplicationCapabilityRegistry, WorthQueryInstalledApplicationCapability,
};
use crate::application_operation::{
    ApplicationAuthorizationPolicyRegistry, WorthQueryApplicationOperationInstallationDenial,
    WorthQueryInstalledAbilityRequirement, WorthQueryInstalledApplicationOperation,
};
use crate::application_principal_binding::{
    WorthQueryInstalledPrincipalBinding, WorthQueryPrincipalBindingInstallationDenial,
    WorthQueryPrincipalBindingInstallationDenialKind,
};
use crate::application_query::{
    WorthQueryApplicationQueryInstallationDenial, WorthQueryInstalledApplicationQuery,
};
use crate::canonical_work::WorthQueryCanonicalWorkEvidence;
use crate::installed_index::WorthQueryInstalledPackageAuthority;
use crate::package::WorthQueryPortableDomainPackageIdentity;
use crate::package::{
    WorthQueryPortableApplicationOperationContractRecord,
    WorthQueryPortableNativeAspectContractRecord,
};
use worth_foundational::facade::CanonicalDigestId;
use worth_query_declaration::facade::application_schema::{
    ApplicationAbilityRef, ApplicationEntityRef, ApplicationOperationRef,
    ApplicationPrincipalBindingRef, ApplicationSchema, ApplicationSchemaAuthoringContext,
    ApplicationSchemaBindingIdentity, ApplicationSchemaMember, ApplicationSchemaMemberProvenance,
    ErasedApplicationSchemaDeclaration, TypedApplicationValue, TypedEffectIntentBuilder,
    TypedOperationBuilder, TypedReadDeclarationBuilder,
};
use worth_query_declaration::facade::portable_identity::WorthQueryPortableType;

/// Opaque proof that one typed schema declaration belongs to an exact
/// installed package, runtime, and generation.
pub struct WorthQueryInstalledApplicationSchema<Schema> {
    pub(crate) package_authority: WorthQueryInstalledPackageAuthority,
    pub(crate) schema_name: String,
    pub(crate) schema_identity: CanonicalDigestId,
    pub(crate) schema: ErasedApplicationSchemaDeclaration,
    pub(crate) member_provenance: ApplicationSchemaMemberProvenance,
    pub(crate) capability_registry: ApplicationCapabilityRegistry,
    authorization_policy_registry: ApplicationAuthorizationPolicyRegistry,
    native_contract_catalog: Arc<WorthQueryInstalledApplicationSchemaContractCatalog>,
    portable_native_contracts: Arc<Vec<WorthQueryPortableNativeAspectContractRecord>>,
    portable_operation_contracts: Arc<Vec<WorthQueryPortableApplicationOperationContractRecord>>,
    installation_canonical_work: WorthQueryCanonicalWorkEvidence,
    _schema: PhantomData<fn() -> Schema>,
}

impl<Schema> std::fmt::Debug for WorthQueryInstalledApplicationSchema<Schema> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryInstalledApplicationSchema")
            .field("owner", &self.package_authority.owner())
            .field("schema_name", &self.schema_name)
            .field("schema_identity", &self.schema_identity)
            .finish_non_exhaustive()
    }
}

impl<Schema> WorthQueryInstalledApplicationSchema<Schema>
where
    Schema: ApplicationSchema,
{
    pub(crate) fn from_compilation(compiled: CompiledApplicationSchema<Schema>) -> Self {
        Self {
            package_authority: compiled.package_authority,
            schema_name: compiled.schema_name,
            schema_identity: compiled.schema_identity,
            schema: compiled.schema,
            member_provenance: compiled.member_provenance,
            capability_registry: compiled.capability_registry,
            authorization_policy_registry: compiled.authorization_policy_registry,
            native_contract_catalog: compiled.native_contract_catalog,
            portable_native_contracts: compiled.portable_native_contracts,
            portable_operation_contracts: compiled.portable_operation_contracts,
            installation_canonical_work: compiled.installation_canonical_work,
            _schema: compiled.marker,
        }
    }

    fn authoring_context(&self) -> ApplicationSchemaAuthoringContext {
        ApplicationSchemaAuthoringContext::from_installed_declaration(
            self.binding_identity(),
            &self.schema,
            &self.member_provenance,
        )
    }

    pub fn owner(&self) -> &str {
        self.package_authority.owner()
    }

    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    pub fn package_identity(&self) -> &WorthQueryPortableDomainPackageIdentity {
        self.package_authority.package_identity()
    }

    pub fn binding_identity(&self) -> ApplicationSchemaBindingIdentity {
        ApplicationSchemaBindingIdentity::from_installed_parts(
            self.package_authority.runtime_ordinal,
            self.package_authority.generation.ordinal(),
            *self.package_authority.package_identity.digest(),
            self.schema_identity,
        )
    }

    /// Returns the descriptive schema meaning retained by this installed proof.
    ///
    /// The declaration itself carries no installation authority. Callers must
    /// retain this handle when an operation requires proof of the installed
    /// runtime and generation.
    pub fn installed_declaration(&self) -> &ErasedApplicationSchemaDeclaration {
        &self.schema
    }

    pub fn native_contracts(&self) -> &WorthQueryInstalledApplicationSchemaContractCatalog {
        self.native_contract_catalog.as_ref()
    }

    pub(crate) fn retain_native_contracts(
        &self,
    ) -> Arc<WorthQueryInstalledApplicationSchemaContractCatalog> {
        Arc::clone(&self.native_contract_catalog)
    }

    pub(crate) fn portable_native_contracts(
        &self,
    ) -> &[WorthQueryPortableNativeAspectContractRecord] {
        self.portable_native_contracts.as_ref()
    }

    pub(crate) fn portable_operation_contracts(
        &self,
    ) -> &[WorthQueryPortableApplicationOperationContractRecord] {
        self.portable_operation_contracts.as_ref()
    }

    pub fn installed_ability_requirement(
        &self,
        ability: &str,
        scope_entity: &str,
    ) -> Option<&WorthQueryInstalledAbilityRequirement> {
        self.authorization_policy_registry
            .get(ability)
            .and_then(|policies| policies.get(scope_entity))
    }

    pub const fn installation_canonical_work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.installation_canonical_work
    }

    pub fn query<Entity>(
        &self,
        entity: ApplicationEntityRef<Schema, Entity>,
    ) -> TypedReadDeclarationBuilder<Schema, Entity> {
        TypedReadDeclarationBuilder::new(entity).with_installed_context(self.authoring_context())
    }

    pub fn operation<Operation: 'static, Input>(
        &self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> TypedOperationBuilder<Schema, Operation, Input>
    where
        Input: WorthQueryPortableType + 'static,
    {
        TypedOperationBuilder::new(operation).with_installed_context(self.authoring_context())
    }

    pub fn effects<Operation: 'static, Input>(
        &self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> TypedEffectIntentBuilder<Schema, Operation, Input>
    where
        Input: WorthQueryPortableType + 'static,
    {
        TypedEffectIntentBuilder::new(operation).with_installed_context(self.authoring_context())
    }

    pub fn principal_binding<Binding, Mapping, Principal, PrincipalIdentity>(
        &self,
        binding: ApplicationPrincipalBindingRef<
            Schema,
            Binding,
            Mapping,
            Principal,
            PrincipalIdentity,
        >,
    ) -> Result<
        WorthQueryInstalledPrincipalBinding<Schema, Binding, Mapping, Principal, PrincipalIdentity>,
        WorthQueryPrincipalBindingInstallationDenial,
    >
    where
        PrincipalIdentity: TypedApplicationValue,
    {
        let installed = self
            .schema
            .members()
            .iter()
            .find(|member| principal_binding_name(member) == Some(binding.name()))
            .ok_or_else(|| {
                WorthQueryPrincipalBindingInstallationDenial::new(
                    WorthQueryPrincipalBindingInstallationDenialKind::BindingNotInstalled,
                    binding.name(),
                )
            })?;
        if !principal_binding_matches(installed, binding) {
            return Err(WorthQueryPrincipalBindingInstallationDenial::new(
                WorthQueryPrincipalBindingInstallationDenialKind::BindingMeaningChanged,
                binding.name(),
            ));
        }
        Ok(WorthQueryInstalledPrincipalBinding::from_installed_schema(
            self,
            binding.name(),
            binding.mapping_entity(),
            binding.identity_aspect(),
            binding.identity_field(),
            binding.status_aspect(),
            binding.status_field(),
            binding.target_relation(),
            binding.principal_entity(),
            binding.principal_identity_aspect(),
            binding.principal_identity_field(),
            binding.principal_identity_scalar_family(),
            binding.principal_identity_value_type(),
        ))
    }

    pub fn ability<Ability, Scope>(
        &self,
        ability: ApplicationAbilityRef<Schema, Ability, Scope>,
    ) -> Result<
        WorthQueryInstalledAbility<Schema, Ability, Scope>,
        WorthQueryAbilityInstallationDenial,
    > {
        let installed = self
            .schema
            .members()
            .iter()
            .find(|member| {
                matches!(
                    member,
                    ApplicationSchemaMember::Ability {
                        ability: installed,
                        ..
                    } if installed == ability.name()
                )
            })
            .ok_or_else(|| {
                WorthQueryAbilityInstallationDenial::new(
                    WorthQueryAbilityInstallationDenialKind::AbilityNotInstalled,
                    ability.name(),
                )
            })?;
        let ApplicationSchemaMember::Ability {
            ability: installed_name,
            scope_entity,
        } = installed
        else {
            unreachable!("ability lookup returned a non-ability member")
        };
        if installed_name != ability.name() || scope_entity != ability.scope() {
            return Err(WorthQueryAbilityInstallationDenial::new(
                WorthQueryAbilityInstallationDenialKind::AbilityMeaningChanged,
                ability.name(),
            ));
        }
        Ok(WorthQueryInstalledAbility::from_installed_schema(
            self,
            installed_name,
            scope_entity,
        ))
    }

    pub fn installed_operation<Operation: 'static, Input>(
        &self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> Result<
        WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
        WorthQueryApplicationOperationInstallationDenial,
    >
    where
        Input: WorthQueryPortableType + 'static,
    {
        WorthQueryInstalledApplicationOperation::from_installed_schema(self, operation.name())
    }

    #[doc(hidden)]
    pub fn installed_operation_for_capability<Capability, Operation: 'static, Input>(
        &self,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    ) -> Result<
        crate::application_operation::WorthQueryInstalledApplicationOperationGraphAuthority<
            Schema,
            Operation,
            Input,
        >,
        WorthQueryApplicationOperationInstallationDenial,
    >
    where
        Input: WorthQueryPortableType + 'static,
    {
        WorthQueryInstalledApplicationOperation::graph_authority_from_installed_schema(
            self, capability,
        )
    }

    pub fn validate_installed_query<Query, Parameters, QueryResult, Scope>(
        &self,
        query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    ) -> Result<(), WorthQueryApplicationQueryInstallationDenial> {
        let expected = self.binding_identity();
        let actual = query.binding_identity();
        let kind = if actual.runtime_ordinal() != expected.runtime_ordinal() {
            Some(
                crate::application_query::WorthQueryApplicationQueryInstallationDenialKind::ForeignRuntime,
            )
        } else if actual.generation() != expected.generation() {
            Some(
                crate::application_query::WorthQueryApplicationQueryInstallationDenialKind::StaleGeneration,
            )
        } else if actual.package_identity() != expected.package_identity() {
            Some(
                crate::application_query::WorthQueryApplicationQueryInstallationDenialKind::PackageIdentityChanged,
            )
        } else if actual.schema_identity() != expected.schema_identity() {
            Some(
                crate::application_query::WorthQueryApplicationQueryInstallationDenialKind::SchemaMeaningChanged,
            )
        } else {
            None
        };
        if let Some(kind) = kind {
            return Err(WorthQueryApplicationQueryInstallationDenial::new(
                kind,
                query.name(),
            ));
        }
        if !query.authority_matches(&self.package_authority) {
            return Err(WorthQueryApplicationQueryInstallationDenial::new(
                crate::application_query::WorthQueryApplicationQueryInstallationDenialKind::AuthorityMismatch,
                query.name(),
            ));
        }
        Ok(())
    }
}
