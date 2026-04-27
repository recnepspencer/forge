use super::*;

#[derive(Default)]
pub struct ForgeQueryRuntimeBuilder {
    backend: Option<Result<Box<dyn ForgeQueryRuntimeBackend>, ForgeQueryRuntimeError>>,
    backend_parts: ForgeQueryRuntimeBackendParts,
}

impl ForgeQueryRuntimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_memory_collections(
        self,
        collections: impl IntoIterator<Item = ForgeQueryCollection>,
    ) -> Self {
        self.compatibility_in_memory_collections(collections)
    }

    pub fn compatibility_in_memory_collections(
        mut self,
        collections: impl IntoIterator<Item = ForgeQueryCollection>,
    ) -> Self {
        self.backend = Some(
            ForgeQueryMemoryApp::compatibility_backend(collections)
                .map(|backend| Box::new(backend) as Box<dyn ForgeQueryRuntimeBackend>)
                .map_err(ForgeQueryRuntimeError::Workspace),
        );
        self
    }

    pub fn backend(mut self, backend: impl ForgeQueryRuntimeBackend + 'static) -> Self {
        self.backend = Some(Ok(Box::new(backend)));
        self
    }

    pub fn relational_runtime(mut self, runtime: RelationalRuntime) -> Self {
        self.backend_parts = self.backend_parts.relational_runtime(runtime);
        self
    }

    pub fn runtime_bridge(mut self, bridge: RuntimeBridge) -> Self {
        self.backend_parts = self.backend_parts.runtime_bridge(bridge);
        self
    }

    pub fn schema_adapter(
        mut self,
        adapter: impl ForgeQueryRuntimeSchemaAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.schema_adapter(adapter);
        self
    }

    pub fn source_adapter(
        mut self,
        adapter: impl ForgeQueryRuntimeSourceAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.source_adapter(adapter);
        self
    }

    pub fn write_authority(
        mut self,
        authority: impl ForgeQueryRuntimeWriteAuthorityAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.write_authority(authority);
        self
    }

    pub fn signal_sink(mut self, sink: impl ForgeQueryRuntimeSignalSinkAdapter + 'static) -> Self {
        self.backend_parts = self.backend_parts.signal_sink(sink);
        self
    }

    pub fn subscription_activation(
        mut self,
        adapter: impl ForgeQueryRuntimeSubscriptionActivationAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.subscription_activation(adapter);
        self
    }

    pub fn preview_basis(
        mut self,
        adapter: impl ForgeQueryRuntimePreviewBasisAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.preview_basis(adapter);
        self
    }

    pub fn inspector_evidence(
        mut self,
        adapter: impl ForgeQueryRuntimeInspectorEvidenceAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.inspector_evidence(adapter);
        self
    }

    pub fn intent_authority(
        mut self,
        adapter: impl ForgeQueryIntentAuthorityAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.intent_authority(adapter);
        self
    }

    pub fn support_profile(mut self, profile: ForgeQueryRuntimeSupportProfile) -> Self {
        self.backend_parts = self.backend_parts.support_profile(profile);
        self
    }

    pub fn build_backend_from_parts(mut self) -> Self {
        self.backend = Some(
            ForgeQueryBridgeBackedRuntimeBackend::from_parts(self.backend_parts)
                .map(|backend| Box::new(backend) as Box<dyn ForgeQueryRuntimeBackend>),
        );
        self.backend_parts = ForgeQueryRuntimeBackendParts::new();
        self
    }

    pub fn build(self) -> Result<ForgeQueryRuntime, ForgeQueryRuntimeError> {
        let backend = self
            .backend
            .ok_or(ForgeQueryRuntimeError::MissingBackend)??;
        Ok(ForgeQueryRuntime {
            backend,
            evidence_authority: ForgeQueryRuntimeEvidenceAuthority::new(),
            active_subscriptions: ActiveSubscriptionRuntime::new(),
            live_subscriptions: BTreeMap::new(),
            live_subscription_index: BTreeMap::new(),
            installed_programs: BTreeMap::new(),
            run_traces: BTreeMap::new(),
            derived_views: BTreeMap::new(),
            derived_dependency_index: ForgeQueryComputedDependencyIndex::default(),
            effects: BTreeMap::new(),
            effect_index: ForgeQueryEffectIndex::default(),
            next_run_id: 0,
        })
    }
}
