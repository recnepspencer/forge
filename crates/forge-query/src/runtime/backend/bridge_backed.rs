use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{
    ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle, ForgeQueryMutationReceipt,
    ForgeQueryWorkspaceError,
};
use crate::schema_view::QuerySchemaView;
use crate::subscription::SubscriptionActivationInput;

use super::ForgeQueryRuntimeBackendParts;
use crate::runtime::{
    ForgeQueryEffectPolicy, ForgeQueryExistingTruthBindingDenial,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryIntentAuthorityAdapter,
    ForgeQueryIntentDeclaration, ForgeQueryIntentDenialEvidence, ForgeQueryIntentExecution,
    ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeBackend, ForgeQueryRuntimeError,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeSupportProfile, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
};

pub struct ForgeQueryBridgeBackedRuntimeBackend {
    relational_runtime: Option<RelationalRuntime>,
    runtime_bridge: RuntimeBridge,
    schema_adapter: Box<dyn super::ForgeQueryRuntimeSchemaAdapter>,
    source_adapter: Box<dyn super::ForgeQueryRuntimeSourceAdapter>,
    write_authority: Box<dyn super::ForgeQueryRuntimeWriteAuthorityAdapter>,
    signal_sink: Box<dyn super::ForgeQueryRuntimeSignalSinkAdapter>,
    subscription_activation: Box<dyn super::ForgeQueryRuntimeSubscriptionActivationAdapter>,
    preview_basis: Box<dyn super::ForgeQueryRuntimePreviewBasisAdapter>,
    inspector_evidence: Box<dyn super::ForgeQueryRuntimeInspectorEvidenceAdapter>,
    intent_authority: Option<Box<dyn ForgeQueryIntentAuthorityAdapter>>,
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
        let intent_authority = parts.intent_authority;
        let support_profile = parts.support_profile.unwrap_or_else(|| {
            ForgeQueryRuntimeSupportProfile::bridge_backed(
                subscription_activation.support_evidence(),
                "preview-basis-admission",
                "inspector-evidence-adapter",
            )
        });

        support_profile
            .validate_backend_claims(intent_authority.is_some())
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
            intent_authority,
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

    fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        let receipts = self.write_authority.write_batch(
            &self.runtime_bridge,
            self.relational_runtime.as_mut(),
            commands,
        )?;
        self.signal_sink.route_write_batch(&receipts)?;
        Ok(receipts)
    }

    fn admit_existing_truth_binding(
        &self,
        _binding: &ForgeQueryExistingTruthTargetBinding,
    ) -> Result<(), ForgeQueryExistingTruthBindingDenial> {
        Ok(())
    }

    fn execute_intent(
        &mut self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError> {
        let Some(intent_authority) = self.intent_authority.as_mut() else {
            return Err(ForgeQueryRuntimeError::MissingIntentAuthority);
        };
        let execution = intent_authority
            .execute_intent(
                &self.runtime_bridge,
                self.relational_runtime.as_mut(),
                declaration,
            )
            .map_err(ForgeQueryRuntimeError::Workspace)?;
        super::super::intent::admit_authoritative_intent_execution(declaration, &execution)
            .map_err(|denial| {
                let evidence =
                    ForgeQueryIntentDenialEvidence::new(declaration, &denial, Some(&execution));
                ForgeQueryRuntimeError::IntentCommitDenied {
                    intent_name: declaration.name().to_string(),
                    stage: denial.stage(),
                    message: denial.message().to_string(),
                    evidence,
                }
            })?;
        if execution.should_route_mutation_receipt() {
            self.signal_sink
                .route_write_receipt(execution.mutation_receipt())
                .map_err(ForgeQueryRuntimeError::Workspace)?;
        }
        Ok(execution)
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
