use std::marker::PhantomData;

mod canonical_identity;
mod capability;
mod compilation;
mod denial;
mod principal_binding_match;

pub use denial::{
    WorthQueryInstalledApplicationSchemaDenial, WorthQueryInstalledApplicationSchemaDenialKind,
};

use crate::application_ability::{
    WorthQueryAbilityInstallationDenial, WorthQueryAbilityInstallationDenialKind,
    WorthQueryInstalledAbility,
};
use crate::application_capability::{
    compile_capability_registry, ApplicationCapabilityRegistry,
    WorthQueryInstalledApplicationCapability,
};
use crate::application_operation::{
    compile_authorization_policy_registry, ApplicationAuthorizationPolicyRegistry,
    WorthQueryApplicationOperationInstallationDenial, WorthQueryInstalledAbilityRequirement,
    WorthQueryInstalledApplicationOperation,
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
use canonical_identity::derive_installed_schema_identity;
#[cfg(test)]
pub(crate) use canonical_identity::derive_installed_schema_identity_with_budget;
pub(crate) use compilation::ApplicationSchemaCompilationDenial;
use principal_binding_match::{principal_binding_matches, principal_binding_name};
use worth_foundational::facade::CanonicalDigestId;
use worth_query_declaration::facade::application_schema::{
    ApplicationAbilityRef, ApplicationEntityRef, ApplicationOperationRef,
    ApplicationPrincipalBindingRef, ApplicationSchema, ApplicationSchemaAuthoringContext,
    ApplicationSchemaBindingIdentity, ApplicationSchemaDeclaration, ApplicationSchemaMember,
    ErasedApplicationSchemaDeclaration, TypedApplicationValue, TypedEffectIntentBuilder,
    TypedOperationBuilder, TypedReadDeclarationBuilder,
};

/// Opaque proof that one typed schema declaration belongs to an exact
/// installed package, runtime, and generation.
pub struct WorthQueryInstalledApplicationSchema<Schema> {
    pub(crate) package_authority: WorthQueryInstalledPackageAuthority,
    pub(crate) schema_name: String,
    pub(crate) schema_identity: CanonicalDigestId,
    pub(crate) schema: ErasedApplicationSchemaDeclaration,
    pub(crate) capability_registry: ApplicationCapabilityRegistry,
    authorization_policy_registry: ApplicationAuthorizationPolicyRegistry,
    installation_canonical_work: WorthQueryCanonicalWorkEvidence,
    _schema: PhantomData<fn() -> Schema>,
}

impl<Schema> WorthQueryInstalledApplicationSchema<Schema>
where
    Schema: ApplicationSchema,
{
    pub(crate) fn new(
        package_authority: WorthQueryInstalledPackageAuthority,
        declaration: &ApplicationSchemaDeclaration<Schema>,
        upstream_installation_work: WorthQueryCanonicalWorkEvidence,
    ) -> Result<Self, ApplicationSchemaCompilationDenial> {
        let (schema_identity, schema_work) =
            derive_installed_schema_identity(declaration.identity())
                .map_err(ApplicationSchemaCompilationDenial::Canonical)?;
        let binding_identity = ApplicationSchemaBindingIdentity::from_installed_parts(
            package_authority.runtime_ordinal,
            package_authority.generation.ordinal(),
            *package_authority.package_identity.digest(),
            schema_identity,
        );
        let capability_registry = compile_capability_registry(
            &package_authority,
            &binding_identity,
            declaration.erased().members(),
        )
        .map_err(ApplicationSchemaCompilationDenial::Capability)?;
        let authorization_policy_registry =
            compile_authorization_policy_registry(declaration.erased().members())
                .map_err(ApplicationSchemaCompilationDenial::Canonical)?;
        let installation_canonical_work = upstream_installation_work.combine(
            capability_registry
                .values()
                .fold(schema_work, |work, capability| {
                    work.combine(capability.canonical().work())
                })
                .combine(
                    authorization_policy_registry
                        .values()
                        .flat_map(|scopes| scopes.values())
                        .fold(
                            WorthQueryCanonicalWorkEvidence::zero(),
                            |work, requirement| work.combine(requirement.canonical_work()),
                        ),
                ),
        );
        Ok(Self {
            schema_name: declaration.erased().name().to_string(),
            schema_identity,
            schema: declaration.erased().clone(),
            package_authority,
            capability_registry,
            authorization_policy_registry,
            installation_canonical_work,
            _schema: PhantomData,
        })
    }

    fn authoring_context(&self) -> ApplicationSchemaAuthoringContext {
        ApplicationSchemaAuthoringContext::from_installed_declaration(
            self.binding_identity(),
            &self.schema,
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

    pub fn installed_ability_requirement(
        &self,
        ability: &str,
        scope_entity: &str,
    ) -> Option<&WorthQueryInstalledAbilityRequirement> {
        self.authorization_policy_registry
            .get(ability)
            .and_then(|policies| policies.get(scope_entity))
    }

    pub(crate) fn installed_capability_count_for_operation(
        &self,
        operation: &str,
        input_type: &str,
    ) -> usize {
        self.capability_registry
            .values()
            .filter(|capability| {
                let contract = capability.contract();
                contract.operation() == operation && contract.input_type() == input_type
            })
            .count()
    }

    pub(crate) fn lifecycle_request_support_fact_count(
        &self,
        operation: &str,
        input_type: &str,
    ) -> usize {
        usize::from(self.capability_registry.values().any(|capability| {
            capability
                .contract()
                .elevation()
                .definition()
                .is_some_and(|definition| {
                    let request = definition.lifecycle().request().operation();
                    request.operation() == operation && request.input_type() == input_type
                })
        }))
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

    pub fn operation<Operation, Input>(
        &self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> TypedOperationBuilder<Schema, Operation, Input> {
        TypedOperationBuilder::new(operation).with_installed_context(self.authoring_context())
    }

    pub fn effects<Operation, Input>(
        &self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> TypedEffectIntentBuilder<Schema, Operation, Input> {
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

    pub fn installed_operation<Operation, Input>(
        &self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> Result<
        WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
        WorthQueryApplicationOperationInstallationDenial,
    > {
        WorthQueryInstalledApplicationOperation::from_installed_schema(self, operation.name())
    }

    #[doc(hidden)]
    pub fn installed_operation_for_capability<Capability, Operation, Input>(
        &self,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    ) -> Result<
        crate::application_operation::WorthQueryInstalledApplicationOperationGraphAuthority<
            Schema,
            Operation,
            Input,
        >,
        WorthQueryApplicationOperationInstallationDenial,
    > {
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
