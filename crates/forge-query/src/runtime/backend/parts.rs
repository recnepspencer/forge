use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use super::{
    ForgeQueryRuntimeInspectorEvidenceAdapter, ForgeQueryRuntimePreviewBasisAdapter,
    ForgeQueryRuntimeSchemaAdapter, ForgeQueryRuntimeSignalSinkAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeWriteAuthorityAdapter,
};
use crate::runtime::{ForgeQueryIntentAuthorityAdapter, ForgeQueryRuntimeSupportProfile};

#[derive(Default)]
pub struct ForgeQueryRuntimeBackendParts {
    pub(super) relational_runtime: Option<RelationalRuntime>,
    pub(super) runtime_bridge: Option<RuntimeBridge>,
    pub(super) schema_adapter: Option<Box<dyn ForgeQueryRuntimeSchemaAdapter>>,
    pub(super) source_adapter: Option<Box<dyn ForgeQueryRuntimeSourceAdapter>>,
    pub(super) write_authority: Option<Box<dyn ForgeQueryRuntimeWriteAuthorityAdapter>>,
    pub(super) signal_sink: Option<Box<dyn ForgeQueryRuntimeSignalSinkAdapter>>,
    pub(super) subscription_activation:
        Option<Box<dyn ForgeQueryRuntimeSubscriptionActivationAdapter>>,
    pub(super) preview_basis: Option<Box<dyn ForgeQueryRuntimePreviewBasisAdapter>>,
    pub(super) inspector_evidence: Option<Box<dyn ForgeQueryRuntimeInspectorEvidenceAdapter>>,
    pub(super) intent_authority: Option<Box<dyn ForgeQueryIntentAuthorityAdapter>>,
    pub(super) support_profile: Option<ForgeQueryRuntimeSupportProfile>,
}

impl ForgeQueryRuntimeBackendParts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn relational_runtime(mut self, runtime: RelationalRuntime) -> Self {
        self.relational_runtime = Some(runtime);
        self
    }

    pub fn runtime_bridge(mut self, bridge: RuntimeBridge) -> Self {
        self.runtime_bridge = Some(bridge);
        self
    }

    pub fn schema_adapter(
        mut self,
        adapter: impl ForgeQueryRuntimeSchemaAdapter + 'static,
    ) -> Self {
        self.schema_adapter = Some(Box::new(adapter));
        self
    }

    pub fn source_adapter(
        mut self,
        adapter: impl ForgeQueryRuntimeSourceAdapter + 'static,
    ) -> Self {
        self.source_adapter = Some(Box::new(adapter));
        self
    }

    pub fn write_authority(
        mut self,
        authority: impl ForgeQueryRuntimeWriteAuthorityAdapter + 'static,
    ) -> Self {
        self.write_authority = Some(Box::new(authority));
        self
    }

    pub fn signal_sink(mut self, sink: impl ForgeQueryRuntimeSignalSinkAdapter + 'static) -> Self {
        self.signal_sink = Some(Box::new(sink));
        self
    }

    pub fn subscription_activation(
        mut self,
        adapter: impl ForgeQueryRuntimeSubscriptionActivationAdapter + 'static,
    ) -> Self {
        self.subscription_activation = Some(Box::new(adapter));
        self
    }

    pub fn preview_basis(
        mut self,
        adapter: impl ForgeQueryRuntimePreviewBasisAdapter + 'static,
    ) -> Self {
        self.preview_basis = Some(Box::new(adapter));
        self
    }

    pub fn inspector_evidence(
        mut self,
        adapter: impl ForgeQueryRuntimeInspectorEvidenceAdapter + 'static,
    ) -> Self {
        self.inspector_evidence = Some(Box::new(adapter));
        self
    }

    pub fn intent_authority(
        mut self,
        adapter: impl ForgeQueryIntentAuthorityAdapter + 'static,
    ) -> Self {
        self.intent_authority = Some(Box::new(adapter));
        self
    }

    pub fn support_profile(mut self, profile: ForgeQueryRuntimeSupportProfile) -> Self {
        self.support_profile = Some(profile);
        self
    }
}
