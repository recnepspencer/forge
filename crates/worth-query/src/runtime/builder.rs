use super::*;
use crate::domain_capabilities::WorthQueryInvariantCatalogRegistrationArtifact;
use crate::runtime::registrations_from_relational_invariant_catalog;
use worth_relational::facade::runtime::{
    CustomInvariantRegistration, CustomInvariantRule, InvariantCatalog, RelationalRuntimeBuilder,
};

mod domain_packages;
mod queued_graph_obligation_registrations;
use queued_graph_obligation_registrations::QueuedGraphObligationRegistrations;

#[derive(Default)]
struct QueuedInvariantRegistrations {
    invariant_catalog: Option<InvariantCatalog>,
    custom_invariants: Vec<CustomInvariantRegistration>,
}

impl QueuedInvariantRegistrations {
    fn is_empty(&self) -> bool {
        self.invariant_catalog.is_none() && self.custom_invariants.is_empty()
    }

    fn push_invariant_catalog(&mut self, invariant_catalog: InvariantCatalog) {
        match &mut self.invariant_catalog {
            Some(existing) => {
                existing
                    .registrations
                    .extend(invariant_catalog.registrations);
                *existing = existing.canonicalized();
            }
            None => {
                self.invariant_catalog = Some(invariant_catalog.canonicalized());
            }
        }
    }

    fn lower_into_relational_runtime(self) -> RelationalRuntime {
        let mut builder = RelationalRuntimeBuilder::new();
        if let Some(invariant_catalog) = self.invariant_catalog {
            builder = builder.invariant_catalog(invariant_catalog.canonicalized());
        }
        for custom_invariant in self.custom_invariants {
            builder = builder.custom_invariant(custom_invariant);
        }
        builder.build()
    }
}

#[derive(Default)]
pub struct WorthQueryRuntimeBuilder {
    backend: Option<Result<Box<dyn WorthQueryRuntimeBackend>, WorthQueryRuntimeError>>,
    backend_parts: WorthQueryRuntimeBackendParts,
    queued_invariant_registrations: QueuedInvariantRegistrations,
    queued_graph_obligation_registrations: QueuedGraphObligationRegistrations,
    graph_obligation_registration_catalog:
        Option<Result<WorthQueryGraphObligationRegistrationCatalog, WorthQueryRuntimeError>>,
    pending_domain_installations: crate::domain_installation::WorthQueryPendingDomainInstallations,
}

impl WorthQueryRuntimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn backend(mut self, backend: impl WorthQueryRuntimeBackend + 'static) -> Self {
        self.backend = Some(Ok(Box::new(backend)));
        self
    }

    pub fn relational_runtime(mut self, runtime: RelationalRuntime) -> Self {
        self.backend_parts = self.backend_parts.relational_runtime(runtime);
        self
    }

    pub fn invariant_catalog(mut self, invariant_catalog: InvariantCatalog) -> Self {
        self.queue_relational_schema_contract_obligations(&invariant_catalog);
        self.queued_invariant_registrations
            .push_invariant_catalog(invariant_catalog);
        self
    }

    pub fn invariant_registration_artifact(
        mut self,
        artifact: WorthQueryInvariantCatalogRegistrationArtifact,
    ) -> Self {
        self.queue_relational_schema_contract_obligations(artifact.invariant_catalog());
        self.queued_invariant_registrations
            .push_invariant_catalog(artifact.invariant_catalog().clone());
        self
    }

    pub fn graph_obligation(mut self, registration: WorthQueryGraphObligationRegistration) -> Self {
        self.queued_graph_obligation_registrations
            .push(registration);
        self
    }

    pub fn graph_scoped_custom_invariant(
        mut self,
        registration: WorthQueryGraphScopedCustomInvariantRegistration,
    ) -> Self {
        let (custom_invariant, graph_obligation) = registration.into_parts();
        self.queued_invariant_registrations
            .custom_invariants
            .push(custom_invariant);
        self.queued_graph_obligation_registrations
            .push(graph_obligation);
        self
    }

    pub fn custom_invariant(mut self, custom_invariant: CustomInvariantRegistration) -> Self {
        self.queued_invariant_registrations
            .custom_invariants
            .push(custom_invariant);
        self
    }

    pub fn register_invariant<R>(mut self, rule: R) -> Result<Self, WorthQueryRuntimeError>
    where
        R: CustomInvariantRule + std::panic::UnwindSafe + 'static,
    {
        let registration = CustomInvariantRegistration::new(rule).map_err(|error| {
            WorthQueryRuntimeError::InvariantRegistration {
                stage: "query_builder_custom_invariant_registration",
                message: format!("{error:?}"),
            }
        })?;
        self.queued_invariant_registrations
            .custom_invariants
            .push(registration);
        Ok(self)
    }

    pub fn runtime_bridge(mut self, bridge: RuntimeBridge) -> Self {
        self.backend_parts = self.backend_parts.runtime_bridge(bridge);
        self
    }

    pub fn schema_adapter(
        mut self,
        adapter: impl WorthQueryRuntimeSchemaAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.schema_adapter(adapter);
        self
    }

    pub fn source_adapter(
        mut self,
        adapter: impl WorthQueryRuntimeSourceAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.source_adapter(adapter);
        self
    }

    pub fn snapshot_identity(
        mut self,
        adapter: impl WorthQueryRuntimeSnapshotIdentityAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.snapshot_identity(adapter);
        self
    }

    pub fn existing_truth_verification(
        mut self,
        adapter: impl WorthQueryRuntimeExistingTruthVerificationAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.existing_truth_verification(adapter);
        self
    }

    pub fn write_authority(
        mut self,
        authority: impl WorthQueryRuntimeWriteAuthorityAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.write_authority(authority);
        self
    }

    pub fn signal_sink(mut self, sink: impl WorthQueryRuntimeSignalSinkAdapter + 'static) -> Self {
        self.backend_parts = self.backend_parts.signal_sink(sink);
        self
    }

    pub fn subscription_activation(
        mut self,
        adapter: impl WorthQueryRuntimeSubscriptionActivationAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.subscription_activation(adapter);
        self
    }

    pub fn preview_basis(
        mut self,
        adapter: impl WorthQueryRuntimePreviewBasisAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.preview_basis(adapter);
        self
    }

    pub fn inspector_evidence(
        mut self,
        adapter: impl WorthQueryRuntimeInspectorEvidenceAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.inspector_evidence(adapter);
        self
    }

    pub fn declaration_initialization(
        mut self,
        adapter: impl WorthQueryRuntimeDeclarationInitializationAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.declaration_initialization(adapter);
        self
    }

    pub fn intent_authority(
        mut self,
        adapter: impl WorthQueryIntentAuthorityAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.intent_authority(adapter);
        self
    }

    pub fn support_profile(mut self, profile: WorthQueryRuntimeSupportProfile) -> Self {
        self.backend_parts = self.backend_parts.support_profile(profile);
        self
    }

    pub fn build_backend_from_parts(mut self) -> Self {
        if self.backend.is_some() {
            self.backend = Some(Err(WorthQueryRuntimeError::InvariantRegistration {
                stage: "runtime_backend_authority_selection",
                message: "build_backend_from_parts() cannot replace an explicit runtime backend selected through backend(...); choose one backend authority path".to_string(),
            }));
            self.backend_parts = WorthQueryRuntimeBackendParts::new();
            self.queued_invariant_registrations = QueuedInvariantRegistrations::default();
            self.queued_graph_obligation_registrations =
                QueuedGraphObligationRegistrations::default();
            self.graph_obligation_registration_catalog = None;
            return self;
        }
        if let Err(error) = self.assemble_graph_obligation_registration_catalog() {
            self.backend = Some(Err(error));
            self.backend_parts = WorthQueryRuntimeBackendParts::new();
            self.queued_invariant_registrations = QueuedInvariantRegistrations::default();
            self.queued_graph_obligation_registrations =
                QueuedGraphObligationRegistrations::default();
            return self;
        }
        if let Err(error) = self.lower_queued_invariant_registrations_into_backend_parts() {
            self.backend = Some(Err(error));
            self.backend_parts = WorthQueryRuntimeBackendParts::new();
            self.queued_invariant_registrations = QueuedInvariantRegistrations::default();
            self.queued_graph_obligation_registrations =
                QueuedGraphObligationRegistrations::default();
            return self;
        }
        self.backend = Some(self.lower_bridge_backed_backend_from_parts());
        self.backend_parts = WorthQueryRuntimeBackendParts::new();
        self.queued_invariant_registrations = QueuedInvariantRegistrations::default();
        self.queued_graph_obligation_registrations = QueuedGraphObligationRegistrations::default();
        self
    }

    pub fn build(self) -> Result<WorthQueryRuntime, WorthQueryRuntimeError> {
        if self.backend.is_some() && !self.backend_parts.is_empty() {
            return Err(WorthQueryRuntimeError::InvariantRegistration {
                stage: "runtime_backend_authority_selection",
                message: "explicit runtime backends cannot be combined with backend parts such as runtime_bridge(...), schema_adapter(...), or write_authority(...); choose one backend authority path".to_string(),
            });
        }
        if self.backend.is_some() && !self.queued_invariant_registrations.is_empty() {
            return Err(WorthQueryRuntimeError::InvariantRegistration {
                stage: "runtime_backend_selection",
                message: "queued Query-owned invariant registrations cannot be applied after selecting an explicit runtime backend; lower them through WorthQueryRuntimeBuilder before backend(...) or relational_runtime(...)".to_string(),
            });
        }
        if self.backend.is_some() && !self.queued_graph_obligation_registrations.is_empty() {
            return Err(WorthQueryRuntimeError::InvariantRegistration {
                stage: "runtime_backend_selection",
                message: "queued Query-owned graph obligation registrations cannot be applied after selecting an explicit runtime backend; lower them through WorthQueryRuntimeBuilder before backend(...) or build_backend_from_parts()".to_string(),
            });
        }
        let backend = self
            .backend
            .ok_or(WorthQueryRuntimeError::MissingBackend)??;
        let graph_obligation_registration_catalog = match self.graph_obligation_registration_catalog
        {
            Some(result) => result?,
            None => WorthQueryGraphObligationRegistrationCatalog::empty(),
        };
        let graph_obligation_index =
            WorthQueryGraphObligationIndex::from_catalog(&graph_obligation_registration_catalog);
        let authority_identity = super::WorthQueryRuntimeAuthorityIdentity::mint();
        let domain_installation_registry =
            crate::domain_installation::WorthQueryDomainInstallationRegistry::from_artifacts(
                self.pending_domain_installations.into_artifacts(),
                authority_identity,
            );
        Ok(WorthQueryRuntime {
            backend,
            evidence_authority: WorthQueryRuntimeEvidenceAuthority::new(),
            authority_identity,
            domain_installation_registry,
            preview_session_labels: BTreeSet::new(),
            branch_session_labels: BTreeSet::new(),
            active_subscriptions: ActiveSubscriptionRuntime::new(),
            live_subscriptions: BTreeMap::new(),
            materialized_read_views: BTreeMap::new(),
            live_subscription_index: Vec::new(),
            installed_programs: BTreeMap::new(),
            run_traces: BTreeMap::new(),
            derived_views: BTreeMap::new(),
            shared_read_pins: super::shared_read_pins::WorthQuerySharedReadPinRegistry::default(),
            published_artifacts:
                super::published_artifacts::WorthQueryPublishedArtifactRegistry::default(),
            journal_replay: super::journal_replay::WorthQueryJournalReplayRegistry::default(),
            derived_dependency_index: WorthQueryComputedDependencyIndex::default(),
            effects: BTreeMap::new(),
            effect_index: WorthQueryEffectIndex::default(),
            graph_obligation_registration_catalog,
            graph_obligation_index,
            managed_live_resource_capability: WorthQueryManagedLiveWorkspaceCapability::shared(),
            next_run_id: 0,
        })
    }

    fn queue_relational_schema_contract_obligations(&mut self, catalog: &InvariantCatalog) {
        match registrations_from_relational_invariant_catalog(catalog) {
            Ok(registrations) => {
                self.queued_graph_obligation_registrations
                    .extend(registrations);
            }
            Err(error) => {
                self.graph_obligation_registration_catalog =
                    Some(Err(graph_obligation_registration_error(
                        "relational_schema_contract_obligation_lowering",
                        error,
                    )));
            }
        }
    }

    fn assemble_graph_obligation_registration_catalog(
        &mut self,
    ) -> Result<(), WorthQueryRuntimeError> {
        if self.graph_obligation_registration_catalog.is_some() {
            return Ok(());
        }
        let queued = std::mem::take(&mut self.queued_graph_obligation_registrations);
        let catalog = WorthQueryGraphObligationRegistrationCatalog::from_registrations(
            queued.into_explicit_registrations(),
        )
        .map_err(|error| {
            graph_obligation_registration_error(
                "graph_obligation_registration_catalog_assembly",
                error,
            )
        })?;
        self.graph_obligation_registration_catalog = Some(Ok(catalog));
        Ok(())
    }

    fn lower_queued_invariant_registrations_into_backend_parts(
        &mut self,
    ) -> Result<(), WorthQueryRuntimeError> {
        if self.queued_invariant_registrations.is_empty() {
            return Ok(());
        }
        if self.backend_parts.has_relational_runtime() {
            return Err(WorthQueryRuntimeError::InvariantRegistration {
                stage: "relational_runtime_authority_selection",
                message: "queued Query-owned invariant registrations conflict with an explicitly supplied relational runtime; choose one authority path".to_string(),
            });
        }
        let queued = std::mem::take(&mut self.queued_invariant_registrations);
        self.backend_parts = std::mem::take(&mut self.backend_parts)
            .relational_runtime(queued.lower_into_relational_runtime());
        Ok(())
    }

    fn lower_bridge_backed_backend_from_parts(
        &mut self,
    ) -> Result<Box<dyn WorthQueryRuntimeBackend>, WorthQueryRuntimeError> {
        let bootstrap = std::mem::take(&mut self.backend_parts).lower_bridge_backed_bootstrap()?;
        Ok(Box::new(
            WorthQueryBridgeBackedRuntimeBackend::from_validated_bootstrap(bootstrap),
        ))
    }
}

fn graph_obligation_registration_error(
    stage: &'static str,
    error: WorthQueryGraphObligationRegistrationDenial,
) -> WorthQueryRuntimeError {
    WorthQueryRuntimeError::InvariantRegistration {
        stage,
        message: format!("{error}"),
    }
}
