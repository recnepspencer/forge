use std::any::TypeId;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use super::{
    WorthQueryGraphCommitCall, WorthQueryGraphCommitPosture, WorthQueryGraphCommitProvider,
    WorthQueryGraphParticipationDefinition, WorthQueryGraphParticipationInstallationDenial,
    WorthQueryGraphParticipationInstallationDenialKind, WorthQueryGraphParticipationLookupDenial,
    WorthQueryGraphParticipationLookupDenialKind, WorthQueryGraphParticipationProvider,
    WorthQueryGraphProviderCall, WorthQueryGraphProviderCallKind, WorthQueryGraphProviderFailure,
    WorthQueryGraphProviderReceipt,
};

pub(crate) trait ErasedGraphParticipationProvider: Send + Sync {
    fn call(
        &self,
        kind: WorthQueryGraphProviderCallKind,
        call: &WorthQueryGraphProviderCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure>;
}

struct TypedGraphParticipationProvider<G, P> {
    provider: P,
    _marker: PhantomData<fn() -> G>,
}

impl<G: 'static, P: WorthQueryGraphParticipationProvider<G>> ErasedGraphParticipationProvider
    for TypedGraphParticipationProvider<G, P>
{
    fn call(
        &self,
        kind: WorthQueryGraphProviderCallKind,
        call: &WorthQueryGraphProviderCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure> {
        match kind {
            WorthQueryGraphProviderCallKind::Observe => self.provider.observe(call),
            WorthQueryGraphProviderCallKind::Project => self.provider.project(call),
            WorthQueryGraphProviderCallKind::TouchEffect => self.provider.touch_effect(call),
            WorthQueryGraphProviderCallKind::CommitAdmission => {
                unreachable!("commit admission has one separately installed group provider")
            }
        }
    }
}

pub(crate) trait ErasedGraphCommitProvider: Send + Sync {
    fn admit_commit(
        &self,
        call: &WorthQueryGraphCommitCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure>;
}

struct TypedGraphCommitProvider<C, P> {
    provider: P,
    _marker: PhantomData<fn() -> C>,
}

impl<C: 'static, P: WorthQueryGraphCommitProvider<C>> ErasedGraphCommitProvider
    for TypedGraphCommitProvider<C, P>
{
    fn admit_commit(
        &self,
        call: &WorthQueryGraphCommitCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure> {
        self.provider.admit_commit(call)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ErasedGraphParticipationDefinition {
    pub role: String,
    pub contract: super::WorthQueryGraphParticipationContract,
    pub marker_name: &'static str,
}

pub(crate) struct GraphParticipationProviderRegistration {
    pub provider: Arc<dyn ErasedGraphParticipationProvider>,
    pub provider_identity: &'static str,
    pub commit_marker: Option<(TypeId, &'static str)>,
    pub resource_support: crate::domain_installation::WorthQueryExecutionResourceSupport,
}

struct GraphCommitProviderRegistration {
    provider: Arc<dyn ErasedGraphCommitProvider>,
    resource_support: crate::domain_installation::WorthQueryExecutionResourceSupport,
}

#[derive(Default)]
pub(crate) struct WorthQueryPendingGraphParticipations {
    definitions: HashMap<TypeId, ErasedGraphParticipationDefinition>,
    providers: HashMap<TypeId, GraphParticipationProviderRegistration>,
    commit_providers: HashMap<TypeId, GraphCommitProviderRegistration>,
    denial: Option<WorthQueryGraphParticipationInstallationDenial>,
}

impl WorthQueryPendingGraphParticipations {
    pub(crate) fn definition<G: 'static>(
        mut self,
        definition: WorthQueryGraphParticipationDefinition<G>,
    ) -> Self {
        if self.denial.is_some() {
            return self;
        }
        if let Err(detail) = definition.validate() {
            self.denial = Some(WorthQueryGraphParticipationInstallationDenial::new(
                WorthQueryGraphParticipationInstallationDenialKind::InvalidDefinition,
                detail,
            ));
            return self;
        }
        let marker = TypeId::of::<G>();
        let erased = ErasedGraphParticipationDefinition {
            role: definition.role().to_string(),
            contract: definition.contract().clone(),
            marker_name: std::any::type_name::<G>(),
        };
        if let Some(existing) = self.definitions.get(&marker) {
            if existing != &erased {
                self.denial = Some(WorthQueryGraphParticipationInstallationDenial::new(
                    WorthQueryGraphParticipationInstallationDenialKind::ConflictingDefinition,
                    definition.role(),
                ));
            }
        } else {
            self.definitions.insert(marker, erased);
        }
        self
    }

    pub(crate) fn provider<G: 'static, P: WorthQueryGraphParticipationProvider<G>>(
        mut self,
        provider: P,
        commit_marker: Option<(TypeId, &'static str)>,
    ) -> Self {
        let marker = TypeId::of::<G>();
        let resource_support = provider.execution_resource_support();
        let registration = GraphParticipationProviderRegistration {
            provider: Arc::new(TypedGraphParticipationProvider::<G, P> {
                provider,
                _marker: PhantomData,
            }),
            provider_identity: std::any::type_name::<P>(),
            commit_marker,
            resource_support,
        };
        if self.providers.insert(marker, registration).is_some() && self.denial.is_none() {
            self.denial = Some(WorthQueryGraphParticipationInstallationDenial::new(
                WorthQueryGraphParticipationInstallationDenialKind::DuplicateProvider,
                "one graph marker received multiple providers",
            ));
        }
        self
    }

    pub(crate) fn commit_provider<C: 'static, P: WorthQueryGraphCommitProvider<C>>(
        mut self,
        provider: P,
    ) -> Self {
        let marker = TypeId::of::<C>();
        let resource_support = provider.execution_resource_support();
        let registration = GraphCommitProviderRegistration {
            provider: Arc::new(TypedGraphCommitProvider::<C, P> {
                provider,
                _marker: PhantomData,
            }),
            resource_support,
        };
        if self.commit_providers.insert(marker, registration).is_some() && self.denial.is_none() {
            self.denial = Some(WorthQueryGraphParticipationInstallationDenial::new(
                WorthQueryGraphParticipationInstallationDenialKind::DuplicateProvider,
                "one commit marker received multiple providers",
            ));
        }
        self
    }

    pub(crate) fn install(
        self,
        runtime_authority: crate::runtime::WorthQueryRuntimeAuthorityIdentity,
        installation_runtime: &worth_query_installation::facade::WorthQueryInstallationRuntimeIdentity,
    ) -> Result<
        WorthQueryInstalledGraphParticipationRegistry,
        WorthQueryGraphParticipationInstallationDenial,
    > {
        if let Some(denial) = self.denial {
            return Err(denial);
        }
        for marker in self.definitions.keys() {
            if !self.providers.contains_key(marker) {
                return Err(WorthQueryGraphParticipationInstallationDenial::new(
                    WorthQueryGraphParticipationInstallationDenialKind::MissingProvider,
                    "graph participation definition has no exact typed provider",
                ));
            }
        }
        if self
            .providers
            .keys()
            .any(|marker| !self.definitions.contains_key(marker))
        {
            return Err(WorthQueryGraphParticipationInstallationDenial::new(
                WorthQueryGraphParticipationInstallationDenialKind::ExtraProvider,
                "graph participation provider has no exact typed definition",
            ));
        }
        let referenced_commit_markers = self
            .providers
            .values()
            .filter_map(|registration| registration.commit_marker.map(|(marker, _)| marker))
            .collect::<std::collections::HashSet<_>>();
        if referenced_commit_markers
            .iter()
            .any(|marker| !self.commit_providers.contains_key(marker))
        {
            return Err(WorthQueryGraphParticipationInstallationDenial::new(
                WorthQueryGraphParticipationInstallationDenialKind::CommitAuthorityRequired,
                "atomic graph participation has no exact commit provider",
            ));
        }
        if self
            .commit_providers
            .keys()
            .any(|marker| !referenced_commit_markers.contains(marker))
        {
            return Err(WorthQueryGraphParticipationInstallationDenial::new(
                WorthQueryGraphParticipationInstallationDenialKind::UnexpectedCommitAuthority,
                "commit provider is not referenced by an atomic graph participation",
            ));
        }
        let commit_authorities = referenced_commit_markers
            .into_iter()
            .map(|marker| {
                let registration = self
                    .commit_providers
                    .get(&marker)
                    .expect("commit provider set was closed");
                (
                    marker,
                    Arc::new(WorthQueryInstalledGraphCommitAuthority {
                        runtime_authority,
                        marker,
                        provider: Arc::clone(&registration.provider),
                        resource_support: registration.resource_support.clone(),
                    }),
                )
            })
            .collect::<HashMap<_, _>>();
        let mut records = HashMap::with_capacity(self.definitions.len());
        for (marker, definition) in self.definitions {
            let registration = self
                .providers
                .get(&marker)
                .expect("provider set was closed");
            let requires_commit =
                definition.contract.commit == WorthQueryGraphCommitPosture::AtomicAuthorityRequired;
            if requires_commit && registration.commit_marker.is_none() {
                return Err(WorthQueryGraphParticipationInstallationDenial::new(
                    WorthQueryGraphParticipationInstallationDenialKind::CommitAuthorityRequired,
                    &definition.role,
                ));
            }
            if !requires_commit && registration.commit_marker.is_some() {
                return Err(WorthQueryGraphParticipationInstallationDenial::new(
                    WorthQueryGraphParticipationInstallationDenialKind::UnexpectedCommitAuthority,
                    &definition.role,
                ));
            }
            let provider_anchor = Arc::new(InstalledGraphProviderAnchor {
                provider: Arc::clone(&registration.provider),
            });
            let installation_authority = Arc::new(
                worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority::install(
                    installation_runtime,
                    definition.role.clone(),
                    registration.provider_identity,
                    requires_commit,
                    registration.commit_marker.map(|(_, identity)| identity),
                    provider_anchor,
                )
                .expect("validated Query graph participation must mint installation authority"),
            );
            records.insert(
                marker,
                Arc::new(WorthQueryInstalledGraphParticipationRecord {
                    authority_identity: installation_authority.authority_identity().to_string(),
                    installation_authority,
                    definition,
                    runtime_authority: runtime_authority.as_u64(),
                    provider: Arc::clone(&registration.provider),
                    resource_support: registration.resource_support.clone(),
                    commit_authority: registration
                        .commit_marker
                        .and_then(|(marker, _)| commit_authorities.get(&marker).cloned()),
                }),
            );
        }
        Ok(WorthQueryInstalledGraphParticipationRegistry { records })
    }
}

pub(crate) struct WorthQueryInstalledGraphParticipationRecord {
    pub authority_identity: String,
    pub installation_authority:
        Arc<worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority>,
    pub definition: ErasedGraphParticipationDefinition,
    pub runtime_authority: u64,
    pub provider: Arc<dyn ErasedGraphParticipationProvider>,
    pub resource_support: crate::domain_installation::WorthQueryExecutionResourceSupport,
    pub commit_authority: Option<Arc<WorthQueryInstalledGraphCommitAuthority>>,
}

struct InstalledGraphProviderAnchor {
    #[allow(dead_code)]
    provider: Arc<dyn ErasedGraphParticipationProvider>,
}

pub(crate) struct WorthQueryInstalledGraphCommitAuthority {
    runtime_authority: crate::runtime::WorthQueryRuntimeAuthorityIdentity,
    marker: TypeId,
    pub provider: Arc<dyn ErasedGraphCommitProvider>,
    pub resource_support: crate::domain_installation::WorthQueryExecutionResourceSupport,
}

impl WorthQueryInstalledGraphCommitAuthority {
    pub(crate) fn identity(&self) -> (u64, TypeId) {
        (self.runtime_authority.as_u64(), self.marker)
    }
}

pub(crate) struct WorthQueryInstalledGraphParticipationRegistry {
    records: HashMap<TypeId, Arc<WorthQueryInstalledGraphParticipationRecord>>,
}

impl WorthQueryInstalledGraphParticipationRegistry {
    pub(crate) fn get_by_marker(
        &self,
        marker: TypeId,
    ) -> Result<
        Arc<WorthQueryInstalledGraphParticipationRecord>,
        WorthQueryGraphParticipationLookupDenial,
    > {
        self.records.get(&marker).map(Arc::clone).ok_or_else(|| {
            WorthQueryGraphParticipationLookupDenial::new(
                WorthQueryGraphParticipationLookupDenialKind::NotInstalled,
            )
        })
    }

    pub(crate) fn get<G: 'static>(
        &self,
    ) -> Result<
        Arc<WorthQueryInstalledGraphParticipationRecord>,
        WorthQueryGraphParticipationLookupDenial,
    > {
        self.get_by_marker(TypeId::of::<G>())
    }
}
