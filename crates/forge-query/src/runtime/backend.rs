use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{
    ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle, ForgeQueryMutationReceipt,
    ForgeQueryWorkspaceError,
};
use crate::schema_view::QuerySchemaView;
use crate::subscription::SubscriptionActivationInput;

use super::{
    ForgeQueryEffectPolicy, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeError,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeSupportProfile, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
};

pub trait ForgeQueryRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile;

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError>;

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError>;

    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError>;

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity>;

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch>;

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String>;

    fn snapshot_token(&self) -> String;

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError>;

    fn admit_preview_basis(
        &self,
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError>;

    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError>;

    fn grouped_baseline_members(
        &self,
        _request: &DeclarativeLiveQueryRequest,
    ) -> Result<Option<Vec<(String, String)>>, ForgeQueryWorkspaceError> {
        Ok(None)
    }
}

pub trait ForgeQueryRuntimeSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimeSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError>;

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity>;

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch>;

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String>;

    fn snapshot_token(&self) -> String;
}

pub trait ForgeQueryRuntimeWriteAuthorityAdapter {
    fn write(
        &mut self,
        bridge: &RuntimeBridge,
        relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimeSignalSinkAdapter {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<(), ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimeSubscriptionActivationAdapter {
    fn support_evidence(&self) -> String;

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimePreviewBasisAdapter {
    fn admit_preview_basis(
        &self,
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimeInspectorEvidenceAdapter {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError>;
}

#[derive(Default)]
pub struct ForgeQueryRuntimeBackendParts {
    relational_runtime: Option<RelationalRuntime>,
    runtime_bridge: Option<RuntimeBridge>,
    schema_adapter: Option<Box<dyn ForgeQueryRuntimeSchemaAdapter>>,
    source_adapter: Option<Box<dyn ForgeQueryRuntimeSourceAdapter>>,
    write_authority: Option<Box<dyn ForgeQueryRuntimeWriteAuthorityAdapter>>,
    signal_sink: Option<Box<dyn ForgeQueryRuntimeSignalSinkAdapter>>,
    subscription_activation: Option<Box<dyn ForgeQueryRuntimeSubscriptionActivationAdapter>>,
    preview_basis: Option<Box<dyn ForgeQueryRuntimePreviewBasisAdapter>>,
    inspector_evidence: Option<Box<dyn ForgeQueryRuntimeInspectorEvidenceAdapter>>,
    support_profile: Option<ForgeQueryRuntimeSupportProfile>,
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

    pub fn support_profile(mut self, profile: ForgeQueryRuntimeSupportProfile) -> Self {
        self.support_profile = Some(profile);
        self
    }
}

pub struct ForgeQueryBridgeBackedRuntimeBackend {
    relational_runtime: Option<RelationalRuntime>,
    runtime_bridge: RuntimeBridge,
    schema_adapter: Box<dyn ForgeQueryRuntimeSchemaAdapter>,
    source_adapter: Box<dyn ForgeQueryRuntimeSourceAdapter>,
    write_authority: Box<dyn ForgeQueryRuntimeWriteAuthorityAdapter>,
    signal_sink: Box<dyn ForgeQueryRuntimeSignalSinkAdapter>,
    subscription_activation: Box<dyn ForgeQueryRuntimeSubscriptionActivationAdapter>,
    preview_basis: Box<dyn ForgeQueryRuntimePreviewBasisAdapter>,
    inspector_evidence: Box<dyn ForgeQueryRuntimeInspectorEvidenceAdapter>,
    support_profile: ForgeQueryRuntimeSupportProfile,
}

impl ForgeQueryBridgeBackedRuntimeBackend {
    pub fn from_parts(
        parts: ForgeQueryRuntimeBackendParts,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        let relational_runtime = parts.relational_runtime;
        let runtime_bridge = parts
            .runtime_bridge
            .ok_or(ForgeQueryRuntimeError::MissingRuntimeBridge)?;
        let schema_adapter = parts
            .schema_adapter
            .ok_or(ForgeQueryRuntimeError::MissingSchemaAdapter)?;
        let source_adapter = parts
            .source_adapter
            .ok_or(ForgeQueryRuntimeError::MissingSourceAdapter)?;
        let write_authority = parts
            .write_authority
            .ok_or(ForgeQueryRuntimeError::MissingWriteAuthority)?;
        let signal_sink = parts
            .signal_sink
            .ok_or(ForgeQueryRuntimeError::MissingSignalSink)?;
        let subscription_activation = parts
            .subscription_activation
            .ok_or(ForgeQueryRuntimeError::MissingSubscriptionActivation)?;
        let preview_basis = parts
            .preview_basis
            .ok_or(ForgeQueryRuntimeError::MissingPreviewBasis)?;
        let inspector_evidence = parts
            .inspector_evidence
            .ok_or(ForgeQueryRuntimeError::MissingInspectorEvidence)?;
        let support_profile = parts.support_profile.unwrap_or_else(|| {
            ForgeQueryRuntimeSupportProfile::bridge_backed(
                subscription_activation.support_evidence(),
                "preview-basis-admission",
                "inspector-evidence-adapter",
            )
        });

        support_profile
            .validate_batch_one_backend_claims()
            .map_err(ForgeQueryRuntimeError::UnsupportedFacadeFamily)?;

        Ok(Self {
            relational_runtime,
            runtime_bridge,
            schema_adapter,
            source_adapter,
            write_authority,
            signal_sink,
            subscription_activation,
            preview_basis,
            inspector_evidence,
            support_profile,
        })
    }
}

impl ForgeQueryRuntimeBackend for ForgeQueryBridgeBackedRuntimeBackend {
    fn support_profile(&self) -> ForgeQueryRuntimeSupportProfile {
        self.support_profile.clone()
    }

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        self.source_adapter
            .declare_live_view(name, request, schema_view)
    }

    fn admit_live_view_declaration(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        self.schema_adapter
            .admit_live_view(name, request, schema_view)
    }

    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.write_authority.write(
            &self.runtime_bridge,
            self.relational_runtime.as_mut(),
            command,
        )?;
        self.signal_sink.route_write_receipt(&receipt)?;
        Ok(receipt)
    }

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity> {
        self.source_adapter.live_entities(view_name)
    }

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch> {
        self.source_adapter.drain_live_patches(view_name)
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        self.source_adapter.affected_live_view_ids(receipt)
    }

    fn snapshot_token(&self) -> String {
        self.source_adapter.snapshot_token()
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        self.subscription_activation
            .admit_activation(view_name, activation)
    }

    fn admit_preview_basis(
        &self,
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        self.preview_basis
            .admit_preview_basis(label, effect_policy, authority)
    }

    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        self.inspector_evidence
            .inspect_write_receipt(receipt, authority)
    }
}
