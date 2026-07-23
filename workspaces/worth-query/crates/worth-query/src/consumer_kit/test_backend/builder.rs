use crate::domain_capabilities::WorthQueryInvariantCatalogRegistrationArtifact;
use crate::memory_workspace::WorthQueryMemoryWorkspace;
use crate::runtime::{WorthQueryRuntimeBuilder, WorthQueryWorkspace};
use worth_relational::facade::runtime::{
    CustomInvariantRegistration, CustomInvariantRule, InvariantCatalog,
};

use super::backend::WorthQueryInMemoryTestBackend;
use super::error::{WorthQueryTestBackendError, WorthQueryTestBackendErrorKind};
use super::schema::WorthQueryTestBackendSchema;

type TestDomainInstaller = Box<
    dyn FnOnce(
        &mut crate::domain_installation::WorthQueryPendingDomainInstallations,
    ) -> Result<(), WorthQueryTestBackendError>,
>;

type TestRuntimeInstaller = Box<dyn FnOnce(WorthQueryRuntimeBuilder) -> WorthQueryRuntimeBuilder>;

#[derive(Default)]
pub struct WorthQueryInMemoryTestRuntimeBuilder {
    schema: Option<WorthQueryTestBackendSchema>,
    invariant_catalog: InvariantCatalog,
    custom_invariants: Vec<CustomInvariantRegistration>,
    domain_installers: Vec<TestDomainInstaller>,
    runtime_installers: Vec<TestRuntimeInstaller>,
    support_profile: Option<crate::runtime::WorthQueryRuntimeSupportProfile>,
    live_close_failures: usize,
    collection_entity_lookup_disabled: bool,
}

pub fn in_memory_test_runtime() -> WorthQueryInMemoryTestRuntimeBuilder {
    WorthQueryInMemoryTestRuntimeBuilder::default()
}

impl WorthQueryInMemoryTestRuntimeBuilder {
    /// Removes exact collection entity lookup to exercise honest reset paths.
    pub fn without_collection_entity_lookup(mut self) -> Self {
        self.collection_entity_lookup_disabled = true;
        self
    }

    /// Injects exact backend close failures for lifecycle ownership tests.
    pub fn fail_next_live_closes(mut self, count: usize) -> Self {
        self.live_close_failures = count;
        self
    }

    pub fn controlled_workspace(
        self,
        name: impl Into<String>,
    ) -> Result<super::WorthQueryControlledTestWorkspace, WorthQueryTestBackendError> {
        self.workspace(name)
            .map(super::WorthQueryControlledTestWorkspace::new)
    }

    pub fn conditional_runtime(
        mut self,
        bridge: worth_runtime_bridge::facade::RuntimeBridge,
        graph: worth_signal::facade::SignalGraph,
    ) -> Self {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.conditional_runtime_for_test(bridge, graph)
        }));
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn conditional_node<D, O, F, G, P>(
        mut self,
        domain: D,
        operation: O,
        family: F,
        graph: G,
        location: crate::domain_installation::WorthQueryConditionalNodeLocation,
        dependencies: Vec<crate::domain_installation::WorthQueryConditionalDependencyInstallation>,
        providers: worth_runtime_bridge::facade::BridgeConditionalProviderSet,
        compute: P,
    ) -> Self
    where
        D: 'static,
        O: 'static,
        F: 'static,
        G: 'static,
        P: crate::domain_installation::WorthQueryConditionalNodeComputeProvider<D, O, F>,
    {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.conditional_node(
                domain,
                operation,
                family,
                graph,
                location,
                dependencies,
                providers,
                compute,
            )
        }));
        self
    }

    pub fn with_schema(mut self, schema: WorthQueryTestBackendSchema) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn invariant_catalog(mut self, invariant_catalog: InvariantCatalog) -> Self {
        self.merge_invariant_catalog(invariant_catalog);
        self
    }

    pub fn invariant_registration_artifact(
        mut self,
        artifact: WorthQueryInvariantCatalogRegistrationArtifact,
    ) -> Self {
        self.merge_invariant_catalog(artifact.invariant_catalog().clone());
        self
    }

    pub fn custom_invariant(mut self, custom_invariant: CustomInvariantRegistration) -> Self {
        self.custom_invariants.push(custom_invariant);
        self
    }

    pub fn support_profile(
        mut self,
        profile: crate::runtime::WorthQueryRuntimeSupportProfile,
    ) -> Self {
        self.support_profile = Some(profile);
        self
    }

    pub fn consumer_support_posture(
        mut self,
        dimension: crate::domain_installation::WorthQueryConsumerSupportDimension,
        posture: crate::domain_installation::WorthQueryConsumerSupportPosture,
    ) -> Self {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.consumer_support_posture(dimension, posture)
        }));
        self
    }

    pub fn domain_package<D: crate::application::WorthQueryDomainEntryMarker + 'static>(
        mut self,
        package: crate::domain_installation::WorthQueryDomainPackage<D>,
    ) -> Self {
        self.domain_installers.push(Box::new(move |installations| {
            let validated = package.validate().map_err(|error| {
                WorthQueryTestBackendError::new(
                    WorthQueryTestBackendErrorKind::DomainInstallationFailed,
                    format!("failed to validate in-memory test domain: {error}"),
                )
            })?;
            let admitted =
                crate::domain_installation::admit_domain_package(validated).map_err(|error| {
                    WorthQueryTestBackendError::new(
                        WorthQueryTestBackendErrorKind::DomainInstallationFailed,
                        format!("failed to admit in-memory test domain: {error}"),
                    )
                })?;
            installations.install(admitted).map_err(|error| {
                WorthQueryTestBackendError::new(
                    WorthQueryTestBackendErrorKind::DomainInstallationFailed,
                    format!("failed to compile in-memory test domain: {error}"),
                )
            })
        }));
        self
    }

    pub fn graph_participation<G: 'static>(
        mut self,
        definition: crate::domain_installation::WorthQueryGraphParticipationDefinition<G>,
    ) -> Self {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.graph_participation(definition)
        }));
        self
    }

    pub fn domain_operation_executor<D: 'static, O, F: 'static, E>(
        mut self,
        domain: D,
        operation: O,
        family: F,
        executor: E,
    ) -> Self
    where
        O: crate::domain_installation::WorthQueryExecutableDomainOperation<
            D,
            F,
            Execution = crate::domain_installation::WorthQueryDirectOperation,
        >,
        E: crate::domain_installation::WorthQueryDomainOperationExecutor<D, O, F>,
    {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.domain_operation_executor(domain, operation, family, executor)
        }));
        self
    }

    pub fn workflow_stage_executor<D: 'static, O, F: 'static, E>(
        mut self,
        domain: D,
        operation: O,
        family: F,
        executor: E,
    ) -> Self
    where
        O: 'static
            + crate::domain_installation::WorthQueryExecutableDomainOperation<
                D,
                F,
                Execution = crate::domain_installation::WorthQueryWorkflowOperation,
            >,
        E: crate::domain_installation::WorthQueryDomainWorkflowStageExecutor<D, O, F>,
    {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.workflow_stage_executor(domain, operation, family, executor)
        }));
        self
    }

    pub fn replayable_workflow_stage_executor<D: 'static, O, F: 'static, E>(
        mut self,
        domain: D,
        operation: O,
        family: F,
        executor: E,
    ) -> Self
    where
        O: 'static
            + crate::domain_installation::WorthQueryExecutableDomainOperation<
                D,
                F,
                Execution = crate::domain_installation::WorthQueryWorkflowOperation,
            >,
        E: crate::domain_installation::WorthQueryDomainWorkflowStageExecutor<D, O, F>
            + crate::domain_installation::WorthQueryDomainReplaySemanticComparator<D, O, F>,
    {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.replayable_workflow_stage_executor(domain, operation, family, executor)
        }));
        self
    }

    pub fn workflow_parallel_admission_provider<D: 'static, O: 'static, F: 'static, P>(
        mut self,
        domain: D,
        operation: O,
        family: F,
        provider: P,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryWorkflowParallelAdmissionProvider<D, O, F>,
    {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.workflow_parallel_admission_provider(domain, operation, family, provider)
        }));
        self
    }

    pub fn graph_participation_provider<
        G: 'static,
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>,
    >(
        mut self,
        marker: G,
        provider: P,
    ) -> Self {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.graph_participation_provider(marker, provider)
        }));
        self
    }

    pub fn atomic_graph_participation_provider<G: 'static, C: 'static, P>(
        mut self,
        marker: G,
        provider: P,
        commit: C,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>,
    {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.atomic_graph_participation_provider(marker, provider, commit)
        }));
        self
    }

    pub fn graph_commit_provider<C: 'static, P>(mut self, commit: C, provider: P) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphCommitProvider<C>,
    {
        self.runtime_installers.push(Box::new(move |builder| {
            builder.graph_commit_provider(commit, provider)
        }));
        self
    }

    pub fn register_invariant<R>(mut self, rule: R) -> Result<Self, WorthQueryTestBackendError>
    where
        R: CustomInvariantRule + std::panic::UnwindSafe + 'static,
    {
        let registration = CustomInvariantRegistration::new(rule).map_err(|error| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::InvariantRegistrationFailed,
                format!("failed to register in-memory test backend invariant: {error:?}"),
            )
        })?;
        self.custom_invariants.push(registration);
        Ok(self)
    }

    pub fn workspace(
        mut self,
        name: impl Into<String>,
    ) -> Result<WorthQueryWorkspace, WorthQueryTestBackendError> {
        let schema = self.schema.ok_or_else(|| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::MissingSchema,
                "in-memory test runtime requires a schema before workspace creation",
            )
        })?;
        let mut domain_installations =
            crate::domain_installation::WorthQueryPendingDomainInstallations::default();
        for install in self.domain_installers {
            install(&mut domain_installations)?;
        }
        let compiled = domain_installations.take_compiled_substrates();
        self.custom_invariants.extend(compiled.custom_invariants);
        let memory_workspace = WorthQueryMemoryWorkspace::collection_with_native_contracts(
            schema.collection(),
            schema.memory_aspects()?,
            schema.contracts().cloned(),
            self.invariant_catalog,
            self.custom_invariants,
        )
        .map_err(|error| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::WorkspaceBuildFailed,
                format!("failed to build in-memory test backend workspace: {error}"),
            )
        })?;
        let backend = WorthQueryInMemoryTestBackend::with_close_failures(
            memory_workspace,
            self.support_profile,
            self.live_close_failures,
            !self.collection_entity_lookup_disabled,
        );
        let mut runtime_builder = WorthQueryRuntimeBuilder::new()
            .backend(backend)
            .with_precompiled_domain_installations(domain_installations);
        for install in self.runtime_installers {
            runtime_builder = install(runtime_builder);
        }
        runtime_builder = runtime_builder
            .aspect_contracts(schema.contracts().cloned())
            .map_err(|error| {
                WorthQueryTestBackendError::new(
                    WorthQueryTestBackendErrorKind::WorkspaceBuildFailed,
                    format!("failed to install in-memory test schema aspect contracts: {error}"),
                )
            })?;
        for obligation in compiled.graph_obligations {
            runtime_builder = runtime_builder.graph_obligation(obligation);
        }
        let runtime = runtime_builder.build().map_err(|error| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::WorkspaceBuildFailed,
                format!("failed to build in-memory test runtime: {error}"),
            )
        })?;
        WorthQueryWorkspace::new(name, runtime).map_err(|error| {
            WorthQueryTestBackendError::new(
                WorthQueryTestBackendErrorKind::WorkspaceBuildFailed,
                format!("failed to build in-memory test workspace facade: {error}"),
            )
        })
    }

    fn merge_invariant_catalog(&mut self, invariant_catalog: InvariantCatalog) {
        self.invariant_catalog
            .registrations
            .extend(invariant_catalog.registrations);
        self.invariant_catalog = self.invariant_catalog.clone().canonicalized();
    }
}
