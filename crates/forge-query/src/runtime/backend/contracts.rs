use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;
use serde_json::Value;

use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{
    ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle, ForgeQueryMutationReceipt,
    ForgeQueryWorkspaceError,
};
use crate::schema_view::QuerySchemaView;
use crate::subscription::SubscriptionActivationInput;

use crate::runtime::{
    ForgeQueryEffectPolicy, ForgeQueryExistingTruthAssertionDenial,
    ForgeQueryExistingTruthBindingDenial, ForgeQueryExistingTruthProbe,
    ForgeQueryExistingTruthProbeDenial, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution,
    ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeError, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeSupportProfile,
    ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
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

    fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError>;

    fn admit_existing_truth_binding(
        &self,
        _binding: &ForgeQueryExistingTruthTargetBinding,
    ) -> Result<(), ForgeQueryExistingTruthBindingDenial> {
        Ok(())
    }

    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        _aspects: &[crate::runtime::ForgeQueryAspectValue],
    ) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial>
    {
        Err(ForgeQueryExistingTruthAssertionDenial::new(
            binding,
            crate::runtime::ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported,
            None,
            None,
            None,
            "this runtime backend does not admit backend-verified existing-truth assertions yet",
        ))
    }

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<ForgeQueryExistingTruthProbe, ForgeQueryExistingTruthProbeDenial> {
        Err(ForgeQueryExistingTruthProbeDenial::new(
            request.binding(),
            crate::runtime::ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported,
            None,
            "this runtime backend does not admit backend-verified existing-truth probes yet",
        ))
    }

    fn execute_intent(
        &mut self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryRuntimeError>;

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

pub trait ForgeQueryRuntimeExistingTruthVerificationAdapter {
    fn verify_existing_truth_assertion(
        &self,
        binding: &ForgeQueryExistingTruthTargetBinding,
        aspects: &[crate::runtime::ForgeQueryAspectValue],
    ) -> Result<(), ForgeQueryExistingTruthAssertionDenial>;

    fn probe_existing_truth(
        &self,
        request: &ForgeQueryExistingTruthProbeRequest,
    ) -> Result<Vec<(String, Value)>, ForgeQueryExistingTruthProbeDenial>;
}

pub trait ForgeQueryRuntimeWriteAuthorityAdapter {
    fn write(
        &mut self,
        bridge: &RuntimeBridge,
        relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError>;

    fn write_batch(
        &mut self,
        bridge: &RuntimeBridge,
        mut relational_runtime: Option<&mut RelationalRuntime>,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        let mut receipts = Vec::with_capacity(commands.len());
        for command in commands {
            receipts.push(self.write(bridge, relational_runtime.as_deref_mut(), command)?);
        }
        Ok(receipts)
    }
}

pub trait ForgeQueryRuntimeSignalSinkAdapter {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<(), ForgeQueryWorkspaceError>;

    fn route_write_batch(
        &mut self,
        receipts: &[ForgeQueryMutationReceipt],
    ) -> Result<(), ForgeQueryWorkspaceError> {
        for receipt in receipts {
            self.route_write_receipt(receipt)?;
        }
        Ok(())
    }
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
