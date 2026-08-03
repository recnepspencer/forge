use std::marker::PhantomData;
use worth_foundational::facade::CanonicalDigestDerivationDenial;

use worth_query_declaration::facade::{
    application_query::{
        ApplicationQueryAuthorizationRequirement, ApplicationQueryBasisSupport,
        ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
        ApplicationQueryParameterDefinition, ErasedApplicationQueryDefinition,
    },
    application_schema::{ApplicationSchema, ApplicationSchemaBindingIdentity},
};

use crate::{
    application_operation::WorthQueryInstalledAbilityRequirement,
    application_schema::WorthQueryInstalledApplicationSchema,
    authority_cryptography::AuthoritySeal, installed_index::WorthQueryInstalledPackageAuthority,
};

use super::{
    authority_seal::{
        derive_installed_query_authority_seal, verify_installed_query_authority_seal,
    },
    canonical_basis::prepare_installed_query_basis,
    WorthQueryApplicationCanonicalArtifact, WorthQueryApplicationQueryCanonicalWorkPolicy,
    WorthQueryApplicationQueryInstallationDenial, WorthQueryApplicationQueryInstallationDenialKind,
    WorthQueryInstalledApplicationContinuationContract, WorthQueryInstalledApplicationLiveContract,
    WorthQueryInstalledApplicationQueryIdentity, WorthQueryInstalledApplicationReadFamilyBinding,
    WorthQueryInstalledGraphReadContract,
};
use crate::canonical_work::WorthQueryCanonicalWorkEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledApplicationQueryAuthorization {
    Public,
    Ability(WorthQueryInstalledAbilityRequirement),
}

pub struct WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope> {
    binding_identity: ApplicationSchemaBindingIdentity,
    canonical: WorthQueryApplicationCanonicalArtifact,
    canonical_work_policy: WorthQueryApplicationQueryCanonicalWorkPolicy,
    installation_canonical_work: WorthQueryCanonicalWorkEvidence,
    identity: WorthQueryInstalledApplicationQueryIdentity,
    authority_identity: AuthoritySeal,
    name: String,
    scope_entity: String,
    parameter_type: String,
    result_type: String,
    parameters: Vec<ApplicationQueryParameterDefinition>,
    read_family: WorthQueryInstalledApplicationReadFamilyBinding,
    continuation: Option<WorthQueryInstalledApplicationContinuationContract>,
    live: Option<WorthQueryInstalledApplicationLiveContract>,
    disclosure: ApplicationQueryDisclosureContract,
    authorization: WorthQueryInstalledApplicationQueryAuthorization,
    basis_support: ApplicationQueryBasisSupport,
    lanes: ApplicationQueryLaneEligibility,
    _marker: PhantomData<fn(Parameters) -> (Schema, Query, QueryResult, Scope)>,
}

impl<Schema, Query, Parameters, QueryResult, Scope>
    WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>
{
    pub(crate) fn from_installed_schema(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        definition: &ErasedApplicationQueryDefinition,
    ) -> Result<Self, WorthQueryApplicationQueryInstallationDenial>
    where
        Schema: ApplicationSchema,
    {
        let canonical_work_policy =
            WorthQueryApplicationQueryCanonicalWorkPolicy::for_definition(definition);
        let read_graph = WorthQueryInstalledGraphReadContract::compile(
            definition,
            schema.binding_identity().schema_identity(),
            canonical_work_policy.installation(),
        )
        .map_err(|denial| canonical_work_denial(definition.name(), denial))?;
        let binding_identity = schema.binding_identity();
        let parameter_type = definition.parameter_type().to_string();
        let result_type = definition.result_type().to_string();
        let canonical = prepare_installed_query_basis(
            binding_identity.package_identity(),
            binding_identity.schema_identity(),
            definition,
            &read_graph,
            canonical_work_policy.installation(),
        )
        .map_err(|denial| canonical_work_denial(definition.name(), denial))?;
        let identity = WorthQueryInstalledApplicationQueryIdentity::from_canonical(&canonical);
        let continuation = WorthQueryInstalledApplicationContinuationContract::compile(
            definition,
            &read_graph,
            canonical_work_policy.installation(),
        )
        .map_err(|denial| canonical_work_denial(definition.name(), denial))?;
        let live = WorthQueryInstalledApplicationLiveContract::compile(
            definition,
            schema.installed_declaration(),
            &read_graph,
            continuation.as_ref(),
        )?;
        let authorization = install_authorization(schema, definition)?;
        let installation_canonical_work = schema.installation_canonical_work().combine(
            read_graph
                .canonical_basis()
                .work()
                .combine(read_graph.canonical_planning_basis().work())
                .combine(canonical.work())
                .combine(
                    continuation
                        .as_ref()
                        .map_or(WorthQueryCanonicalWorkEvidence::zero(), |contract| {
                            contract.canonical_basis().work()
                        }),
                ),
        );
        let read_family = WorthQueryInstalledApplicationReadFamilyBinding::bind(read_graph);
        let authority_identity = derive_installed_query_authority_seal(
            &schema.package_authority.authority_key,
            &binding_identity,
            &identity,
        );
        Ok(Self {
            binding_identity,
            canonical,
            canonical_work_policy,
            installation_canonical_work,
            identity,
            authority_identity,
            name: definition.name().to_string(),
            scope_entity: definition.scope_entity().to_string(),
            parameter_type,
            result_type,
            parameters: definition.parameters().to_vec(),
            read_family,
            continuation,
            live,
            disclosure: definition.disclosure().clone(),
            authorization,
            basis_support: definition.basis_support(),
            lanes: definition.lanes(),
            _marker: PhantomData,
        })
    }

    pub fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub fn identity(&self) -> &WorthQueryInstalledApplicationQueryIdentity {
        &self.identity
    }

    pub fn canonical_basis(&self) -> &WorthQueryApplicationCanonicalArtifact {
        &self.canonical
    }

    pub const fn canonical_work_policy(&self) -> WorthQueryApplicationQueryCanonicalWorkPolicy {
        self.canonical_work_policy
    }

    pub const fn installation_canonical_work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.installation_canonical_work
    }

    pub fn authority_identity(&self) -> &str {
        self.authority_identity.as_str()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn scope_entity(&self) -> &str {
        &self.scope_entity
    }

    pub fn parameter_type(&self) -> &str {
        &self.parameter_type
    }

    pub fn result_type(&self) -> &str {
        &self.result_type
    }

    pub fn parameters(&self) -> &[ApplicationQueryParameterDefinition] {
        &self.parameters
    }

    pub fn read_graph(&self) -> &WorthQueryInstalledGraphReadContract {
        self.read_family.planning_contract()
    }

    pub const fn read_family_binding(&self) -> &WorthQueryInstalledApplicationReadFamilyBinding {
        &self.read_family
    }

    pub const fn continuation(
        &self,
    ) -> Option<&WorthQueryInstalledApplicationContinuationContract> {
        self.continuation.as_ref()
    }

    pub const fn live(&self) -> Option<&WorthQueryInstalledApplicationLiveContract> {
        self.live.as_ref()
    }

    pub fn disclosure(&self) -> &ApplicationQueryDisclosureContract {
        &self.disclosure
    }

    pub const fn authorization(&self) -> &WorthQueryInstalledApplicationQueryAuthorization {
        &self.authorization
    }

    pub const fn basis_support(&self) -> ApplicationQueryBasisSupport {
        self.basis_support
    }

    pub const fn lanes(&self) -> ApplicationQueryLaneEligibility {
        self.lanes
    }

    pub(crate) fn authority_matches(&self, package: &WorthQueryInstalledPackageAuthority) -> bool {
        verify_installed_query_authority_seal(
            &self.authority_identity,
            &package.authority_key,
            &self.binding_identity,
            &self.identity,
        )
    }
}

fn canonical_work_denial(
    subject: &str,
    denial: CanonicalDigestDerivationDenial,
) -> WorthQueryApplicationQueryInstallationDenial {
    let kind = match denial {
        CanonicalDigestDerivationDenial::EntryLimitExceeded { .. } => {
            WorthQueryApplicationQueryInstallationDenialKind::CanonicalEntryBudgetExceeded
        }
        CanonicalDigestDerivationDenial::EncodedByteLimitExceeded { .. } => {
            WorthQueryApplicationQueryInstallationDenialKind::CanonicalEncodedByteBudgetExceeded
        }
        _ => WorthQueryApplicationQueryInstallationDenialKind::CanonicalDigestSlotRejected,
    };
    WorthQueryApplicationQueryInstallationDenial::new(kind, subject)
}

fn install_authorization<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    definition: &ErasedApplicationQueryDefinition,
) -> Result<
    WorthQueryInstalledApplicationQueryAuthorization,
    WorthQueryApplicationQueryInstallationDenial,
>
where
    Schema: ApplicationSchema,
{
    match definition.authorization() {
        ApplicationQueryAuthorizationRequirement::Public => {
            Ok(WorthQueryInstalledApplicationQueryAuthorization::Public)
        }
        ApplicationQueryAuthorizationRequirement::Ability {
            ability,
            scope_entity,
        } => schema
            .installed_ability_requirement(ability, scope_entity)
            .cloned()
            .map(WorthQueryInstalledApplicationQueryAuthorization::Ability)
            .ok_or_else(|| {
                WorthQueryApplicationQueryInstallationDenial::new(
                    WorthQueryApplicationQueryInstallationDenialKind::AuthorizationNotInstalled,
                    definition.name(),
                )
            }),
    }
}
