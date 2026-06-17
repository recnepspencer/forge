use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::runtime::ForgeQueryRuntimeError;

use super::{
    bootstrap::BridgeBackedRuntimeBootstrap, ForgeQueryIntentAuthorityAdapter,
    ForgeQueryRuntimeDeclarationInitializationAdapter,
    ForgeQueryRuntimeExistingTruthVerificationAdapter, ForgeQueryRuntimeInspectorEvidenceAdapter,
    ForgeQueryRuntimePreviewBasisAdapter, ForgeQueryRuntimeSchemaAdapter,
    ForgeQueryRuntimeSignalSinkAdapter, ForgeQueryRuntimeSnapshotIdentityAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeWriteAuthorityAdapter,
};
use crate::runtime::ForgeQueryRuntimeSupportProfile;

#[derive(Default)]
pub struct ForgeQueryRuntimeBackendParts {
    pub(super) relational_runtime: Option<RelationalRuntime>,
    pub(super) runtime_bridge: Option<RuntimeBridge>,
    pub(super) schema_adapter: Option<Box<dyn ForgeQueryRuntimeSchemaAdapter>>,
    pub(super) source_adapter: Option<Box<dyn ForgeQueryRuntimeSourceAdapter>>,
    pub(super) snapshot_identity: Option<Box<dyn ForgeQueryRuntimeSnapshotIdentityAdapter>>,
    pub(super) existing_truth_verification:
        Option<Box<dyn ForgeQueryRuntimeExistingTruthVerificationAdapter>>,
    pub(super) write_authority: Option<Box<dyn ForgeQueryRuntimeWriteAuthorityAdapter>>,
    pub(super) signal_sink: Option<Box<dyn ForgeQueryRuntimeSignalSinkAdapter>>,
    pub(super) subscription_activation:
        Option<Box<dyn ForgeQueryRuntimeSubscriptionActivationAdapter>>,
    pub(super) preview_basis: Option<Box<dyn ForgeQueryRuntimePreviewBasisAdapter>>,
    pub(super) inspector_evidence: Option<Box<dyn ForgeQueryRuntimeInspectorEvidenceAdapter>>,
    pub(super) declaration_initialization:
        Option<Box<dyn ForgeQueryRuntimeDeclarationInitializationAdapter>>,
    pub(super) intent_authority: Option<Box<dyn ForgeQueryIntentAuthorityAdapter>>,
    pub(super) support_profile: Option<ForgeQueryRuntimeSupportProfile>,
}

impl ForgeQueryRuntimeBackendParts {
    pub fn new() -> Self {
        Self::default()
    }

    pub(in crate::runtime) fn is_empty(&self) -> bool {
        self.relational_runtime.is_none()
            && self.runtime_bridge.is_none()
            && self.schema_adapter.is_none()
            && self.source_adapter.is_none()
            && self.snapshot_identity.is_none()
            && self.existing_truth_verification.is_none()
            && self.write_authority.is_none()
            && self.signal_sink.is_none()
            && self.subscription_activation.is_none()
            && self.preview_basis.is_none()
            && self.inspector_evidence.is_none()
            && self.declaration_initialization.is_none()
            && self.intent_authority.is_none()
            && self.support_profile.is_none()
    }

    pub(in crate::runtime) fn has_relational_runtime(&self) -> bool {
        self.relational_runtime.is_some()
    }

    pub(in crate::runtime) fn lower_bridge_backed_bootstrap(
        self,
    ) -> Result<BridgeBackedRuntimeBootstrap, ForgeQueryRuntimeError> {
        BridgeBackedRuntimeBootstrap::lower_from_parts(self)
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

    pub fn snapshot_identity(
        mut self,
        adapter: impl ForgeQueryRuntimeSnapshotIdentityAdapter + 'static,
    ) -> Self {
        self.snapshot_identity = Some(Box::new(adapter));
        self
    }

    pub fn existing_truth_verification(
        mut self,
        adapter: impl ForgeQueryRuntimeExistingTruthVerificationAdapter + 'static,
    ) -> Self {
        self.existing_truth_verification = Some(Box::new(adapter));
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

    pub fn declaration_initialization(
        mut self,
        adapter: impl ForgeQueryRuntimeDeclarationInitializationAdapter + 'static,
    ) -> Self {
        self.declaration_initialization = Some(Box::new(adapter));
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
