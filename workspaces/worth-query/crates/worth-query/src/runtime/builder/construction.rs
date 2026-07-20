use super::*;

impl WorthQueryRuntimeBuilder {
    pub fn build(mut self) -> Result<WorthQueryRuntime, WorthQueryRuntimeError> {
        self.queue_installed_domain_substrates();
        let conditional_runtime_bridge = self.conditional_runtime_bridge.take();
        let conditional_signal_graph = self.conditional_signal_graph.take();
        let pending_conditional_installations =
            std::mem::take(&mut self.pending_conditional_installations);
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
        self.assemble_graph_obligation_registration_catalog()?;
        let backend = self
            .backend
            .ok_or(WorthQueryRuntimeError::MissingBackend)??;
        let consumer_support_profile =
            crate::domain_installation::WorthQueryConsumerSupportProfile::from_runtime(
                &backend.support_profile(),
            )
            .with_runtime_overrides(self.consumer_support_postures);
        let graph_obligation_registration_catalog = match self.graph_obligation_registration_catalog
        {
            Some(result) => result?,
            None => WorthQueryGraphObligationRegistrationCatalog::empty(),
        };
        let graph_obligation_index =
            WorthQueryGraphObligationIndex::from_catalog(&graph_obligation_registration_catalog);
        let authority_identity = super::super::WorthQueryRuntimeAuthorityIdentity::mint();
        let installation_runtime =
            worth_query_installation::facade::WorthQueryInstallationRuntimeIdentity::fresh();
        let graph_participation_registry = self
            .pending_graph_participations
            .install(authority_identity, &installation_runtime)
            .map_err(|denial| WorthQueryRuntimeError::InvariantRegistration {
                stage: "graph_participation_installation",
                message: format!("{:?}: {}", denial.kind(), denial.detail()),
            })?;
        let domain_installation_registry =
            crate::domain_installation::WorthQueryDomainInstallationRegistry::from_artifacts(
                self.pending_domain_installations.into_artifacts(),
                authority_identity,
                installation_runtime,
            );
        let (conditional_signal_runtime, conditional_execution_registry) =
            WorthQueryRuntimeBuilder::install_conditional_execution(
                conditional_runtime_bridge,
                conditional_signal_graph,
                pending_conditional_installations,
                &domain_installation_registry,
                &graph_participation_registry,
            )?;
        let domain_operation_executor_registry = self
            .pending_domain_operation_executors
            .install(
                &domain_installation_registry
                    .execution_index()
                    .domain_operation_execution_descriptors(),
            )
            .map_err(|message| WorthQueryRuntimeError::InvariantRegistration {
                stage: "domain_operation_executor_installation",
                message: message.to_string(),
            })?;
        let workflow_descriptors = domain_installation_registry
            .execution_index()
            .workflow_operation_execution_descriptors();
        let workflow_stage_executor_registry = self
            .pending_workflow_stage_executors
            .install(&workflow_descriptors)
            .map_err(|message| WorthQueryRuntimeError::InvariantRegistration {
                stage: "workflow_stage_executor_installation",
                message: message.to_string(),
            })?;
        let workflow_parallel_admission_provider_registry = self
            .pending_workflow_parallel_admission_providers
            .install(&workflow_descriptors)
            .map_err(|message| WorthQueryRuntimeError::InvariantRegistration {
                stage: "workflow_parallel_admission_provider_installation",
                message: message.to_string(),
            })?;
        let native_aspect_contracts = self.native_aspect_contracts;
        Ok(WorthQueryRuntime {
            backend,
            evidence_authority: WorthQueryRuntimeEvidenceAuthority::new(),
            authority_identity,
            domain_installation_registry,
            domain_operation_executor_registry,
            workflow_stage_executor_registry,
            workflow_parallel_admission_provider_registry,
            graph_participation_registry,
            conditional_signal_runtime,
            conditional_execution_registry,
            consumer_support_profile,
            native_aspect_contracts,
            preview_session_labels: BTreeSet::new(),
            branch_session_labels: BTreeSet::new(),
            active_subscriptions: ActiveSubscriptionRuntime::new(),
            live_subscriptions: BTreeMap::new(),
            materialized_read_views: BTreeMap::new(),
            live_subscription_index: Vec::new(),
            installed_programs: BTreeMap::new(),
            run_traces: BTreeMap::new(),
            derived_views: BTreeMap::new(),
            shared_read_pins:
                super::super::shared_read_pins::WorthQuerySharedReadPinRegistry::default(),
            published_artifacts:
                super::super::published_artifacts::WorthQueryPublishedArtifactRegistry::default(),
            journal_replay: super::super::journal_replay::WorthQueryJournalReplayRegistry::default(
            ),
            derived_dependency_index: WorthQueryComputedDependencyIndex::default(),
            effects: BTreeMap::new(),
            effect_index: WorthQueryEffectIndex::default(),
            graph_obligation_registration_catalog,
            graph_obligation_index,
            managed_live_resource_capability: WorthQueryManagedLiveWorkspaceCapability::shared(),
            next_run_id: 0,
        })
    }
}
