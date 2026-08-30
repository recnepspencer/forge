use std::marker::PhantomData;
use worth_foundational::facade::CanonicalDigestDerivationDenial;

use worth_query_declaration::facade::{
    application_query::{
        ApplicationQueryAuthorizationRequirement, ApplicationQueryBasisSupport,
        ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
        ApplicationQueryParameterDefinition, ErasedApplicationQueryDefinition,
    },
    application_schema::{ApplicationSchema, ApplicationSchemaBindingIdentity},
    portable_identity::WorthQueryPortableTypeIdentity,
};

use crate::{
    application_operation::WorthQueryInstalledAbilityRequirement,
    application_schema::WorthQueryInstalledApplicationSchema,
    authority_cryptography::AuthoritySeal,
    graph_obligation::{
        bind_query_obligations, WorthQueryGraphObligationInstallationDenial,
        WorthQueryInstalledGraphCapabilityRequirement,
        WorthQueryInstalledGraphObligationInspection, WorthQueryInstalledGraphObligationSet,
    },
    installed_index::WorthQueryInstalledPackageAuthority,
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
    parameter_type: WorthQueryPortableTypeIdentity,
    result_type: WorthQueryPortableTypeIdentity,
    parameters: Vec<ApplicationQueryParameterDefinition>,
    read_family: WorthQueryInstalledApplicationReadFamilyBinding,
    continuation: Option<WorthQueryInstalledApplicationContinuationContract>,
    live: Option<WorthQueryInstalledApplicationLiveContract>,
    disclosure: ApplicationQueryDisclosureContract,
    authorization: WorthQueryInstalledApplicationQueryAuthorization,
    obligations: WorthQueryInstalledGraphObligationSet,
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
        let parameter_type = definition.parameter_identity();
        let result_type = definition.result_identity();
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
        let disclosure_capabilities =
            install_disclosure_capabilities(schema, definition.disclosure());
        let obligations = bind_query_obligations(
            &binding_identity,
            definition.name(),
            &identity,
            &read_graph,
            &authorization,
            &disclosure_capabilities,
        )
        .map_err(|denial| graph_obligation_denial(definition.name(), denial))?;
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
                )
                .combine(obligations.installation_evidence().canonical_work()),
        );
        let read_family = WorthQueryInstalledApplicationReadFamilyBinding::bind(read_graph);
        let authority_identity = derive_installed_query_authority_seal(
            &schema.package_authority.authority_key,
            &binding_identity,
            &identity,
            obligations.identity(),
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
            obligations,
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
        self.parameter_type.as_str()
    }

    pub fn result_type(&self) -> &str {
        self.result_type.as_str()
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

    pub const fn graph_obligations(&self) -> WorthQueryInstalledGraphObligationInspection<'_> {
        self.obligations.inspect()
    }

    #[doc(hidden)]
    pub fn retain_graph_obligations_for_admission(&self) -> WorthQueryInstalledGraphObligationSet {
        self.obligations.clone()
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
            self.obligations.identity(),
        )
    }
}

fn install_disclosure_capabilities<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    disclosure: &ApplicationQueryDisclosureContract,
) -> Vec<WorthQueryInstalledGraphCapabilityRequirement>
where
    Schema: ApplicationSchema,
{
    let (Some(name), Some(capability_type)) =
        (disclosure.capability_name(), disclosure.capability_type())
    else {
        return Vec::new();
    };
    schema
        .capability_registry
        .values()
        .filter(|compiled| {
            compiled.contract().name() == name
                && compiled.contract().capability_type() == capability_type
        })
        .map(|compiled| {
            WorthQueryInstalledGraphCapabilityRequirement::new(
                compiled.identity().clone(),
                compiled.contract().clone(),
            )
        })
        .collect()
}

fn graph_obligation_denial(
    subject: &str,
    denial: WorthQueryGraphObligationInstallationDenial,
) -> WorthQueryApplicationQueryInstallationDenial {
    let kind = match denial {
        WorthQueryGraphObligationInstallationDenial::InvalidContract => {
            WorthQueryApplicationQueryInstallationDenialKind::InvalidGraphObligationContract
        }
        WorthQueryGraphObligationInstallationDenial::Canonical(denial) => {
            return canonical_work_denial(subject, denial)
        }
    };
    WorthQueryApplicationQueryInstallationDenial::new(kind, subject)
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
