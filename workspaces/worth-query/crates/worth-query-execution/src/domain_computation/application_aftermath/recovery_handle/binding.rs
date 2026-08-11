//! Immutable recovery binding derived from the committed receipt (R8.28).

use worth_query_installation::facade::{
    InstalledAftermathRecoveryContract, PublishedAftermathPosture,
    WorthQueryInstalledAftermathContract,
};
#[cfg(test)]
use worth_relational::facade::history::CommitId;
use worth_relational::facade::history::{BranchId, CommitReference};
#[cfg(test)]
use worth_relational::facade::identity::VersionId;

use crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage;
use crate::domain_computation::application_aftermath::{
    ExternalEffectCorrelationIdentity, WorthQueryDispatchOutboxRecord,
    WorthQueryExternalDispatchPosture,
};
use crate::domain_computation::authorization::WorthQueryOperationScopeBinding;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationIdempotencyBinding,
    WorthQueryPrimaryMutationWorkEvidence, WorthQueryRetainedGovernedInput,
};

use super::denial::{WorthQueryRecoveryHandleDenial, WorthQueryRecoveryHandleDenialKind};

/// Binding axes a recovery handle retains from admitted commit truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryRecoveryHandleBinding {
    runtime_instance_id: u64,
    schema_identity: [u8; 32],
    commit: CommitReference,
    /// Generation of the installed application schema binding.
    application_binding_generation: u64,
    installed_operation: [u8; 32],
    mutation_work: Option<WorthQueryPrimaryMutationWorkEvidence>,
    retained_preimage: Option<WorthQueryRetainedPreImage>,
    retained_governed_input: Option<WorthQueryRetainedGovernedInput>,
    principal_scope: WorthQueryOperationScopeBinding,
    idempotency: WorthQueryApplicationIdempotencyBinding,
    provider_posture: Option<WorthQueryExternalDispatchPosture>,
    /// Co-committed outbox record — R8.28 correlation evidence (R8.68).
    committed_dispatch_outbox:
        Option<crate::domain_computation::primary_graph::WorthQueryCommittedDispatchOutboxBinding>,
    installed_aftermath: WorthQueryInstalledAftermathContract,
    expires_at_unix_ms: Option<u64>,
}

impl WorthQueryRecoveryHandleBinding {
    pub(super) fn from_receipt(
        receipt: &WorthQueryApplicationCommitReceipt,
        expires_at_unix_ms: Option<u64>,
    ) -> Result<Self, WorthQueryRecoveryHandleDenial> {
        let aftermath = receipt.installed_aftermath().ok_or_else(|| {
            WorthQueryRecoveryHandleDenial::new(
                WorthQueryRecoveryHandleDenialKind::RecoveryNotAdmitted,
            )
        })?;
        match aftermath.recovery() {
            InstalledAftermathRecoveryContract::NotAdmitted => {
                return Err(WorthQueryRecoveryHandleDenial::new(
                    WorthQueryRecoveryHandleDenialKind::RecoveryNotAdmitted,
                ));
            }
            InstalledAftermathRecoveryContract::Admissible { .. } => {}
        }
        let schema_identity = *receipt
            .principal_scope()
            .binding_identity()
            .schema_identity()
            .bytes();
        let commit = receipt.commit_reference().clone();
        let application_binding_generation =
            receipt.principal_scope().binding_identity().generation();
        Ok(Self {
            runtime_instance_id: receipt.provider_runtime_instance_id(),
            schema_identity,
            commit,
            application_binding_generation,
            installed_operation: *receipt.installed_operation(),
            mutation_work: receipt.mutation_work().cloned(),
            retained_preimage: receipt.retained_preimage().cloned(),
            retained_governed_input: receipt
                .authority_binding()
                .retained_governed_input_carrier()
                .cloned(),
            principal_scope: receipt.principal_scope().clone(),
            idempotency: receipt.idempotency_binding(),
            provider_posture: receipt.external_dispatch().map(|d| d.posture().clone()),
            committed_dispatch_outbox: receipt.committed_dispatch_outbox().cloned(),
            installed_aftermath: aftermath.clone(),
            expires_at_unix_ms,
        })
    }

    pub const fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    pub const fn schema_identity(&self) -> &[u8; 32] {
        &self.schema_identity
    }

    pub const fn branch(&self) -> &BranchId {
        &self.commit.branch_id
    }

    pub const fn application_binding_generation(&self) -> u64 {
        self.application_binding_generation
    }

    pub const fn installed_operation(&self) -> &[u8; 32] {
        &self.installed_operation
    }

    pub const fn attempt_commit_id(&self) -> u64 {
        self.commit.commit_id.0
    }

    /// Exact Relational commit identity retained from the ordinary commit.
    pub const fn commit_reference(&self) -> &CommitReference {
        &self.commit
    }

    pub const fn principal_scope(&self) -> &WorthQueryOperationScopeBinding {
        &self.principal_scope
    }

    pub const fn idempotency(&self) -> WorthQueryApplicationIdempotencyBinding {
        self.idempotency
    }

    pub const fn provider_posture(&self) -> Option<&WorthQueryExternalDispatchPosture> {
        self.provider_posture.as_ref()
    }

    /// Correlation identity derived from the bound outbox record.
    ///
    /// Preserved for callers that only need the identity; re-dispatch requires
    /// [`Self::dispatch_outbox`] (R8.68).
    pub fn correlation(&self) -> Option<ExternalEffectCorrelationIdentity> {
        self.committed_dispatch_outbox
            .as_ref()
            .map(|binding| *binding.record().correlation())
    }

    pub fn dispatch_outbox(&self) -> Option<&WorthQueryDispatchOutboxRecord> {
        self.committed_dispatch_outbox
            .as_ref()
            .map(crate::domain_computation::primary_graph::WorthQueryCommittedDispatchOutboxBinding::record)
    }

    pub(in crate::domain_computation) fn committed_dispatch_outbox(
        &self,
    ) -> Option<&crate::domain_computation::primary_graph::WorthQueryCommittedDispatchOutboxBinding>
    {
        self.committed_dispatch_outbox.as_ref()
    }

    pub fn compatibility_generation(&self) -> u64 {
        self.installed_aftermath.compatibility_generation()
    }

    pub fn published_posture(&self) -> PublishedAftermathPosture {
        self.installed_aftermath.published_posture()
    }

    /// Identity of the exact installed aftermath retained through commit.
    pub const fn installed_aftermath_identity(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryInstalledAftermathIdentity {
        self.installed_aftermath.identity()
    }

    /// Operation slot owned by that same installed aftermath contract.
    pub fn installed_aftermath_operation_slot(&self) -> &str {
        self.installed_aftermath.operation_slot()
    }

    pub(crate) const fn installed_aftermath(&self) -> &WorthQueryInstalledAftermathContract {
        &self.installed_aftermath
    }

    pub(crate) const fn mutation_work(&self) -> Option<&WorthQueryPrimaryMutationWorkEvidence> {
        self.mutation_work.as_ref()
    }

    pub(crate) const fn retained_preimage(&self) -> Option<&WorthQueryRetainedPreImage> {
        self.retained_preimage.as_ref()
    }

    pub(crate) const fn retained_governed_input(&self) -> Option<&WorthQueryRetainedGovernedInput> {
        self.retained_governed_input.as_ref()
    }

    /// Exact original capability input carried from operation admission.
    pub fn original_input<Input: 'static>(&self) -> Option<&Input> {
        self.retained_governed_input.as_ref()?.downcast_ref()
    }

    pub const fn expires_at_unix_ms(&self) -> Option<u64> {
        self.expires_at_unix_ms
    }

    /// Named corruption fixture for per-axis drift proof (R8.28). Not production.
    #[cfg(test)]
    pub(crate) fn axis_probe(parts: WorthQueryRecoveryHandleBindingAxisProbe) -> Self {
        Self {
            runtime_instance_id: parts.runtime_instance_id,
            schema_identity: parts.schema_identity,
            commit: CommitReference {
                commit_id: CommitId(parts.attempt_commit_id),
                version_id: VersionId(parts.attempt_commit_id),
                branch_id: parts.branch,
                parents: Vec::new(),
            },
            application_binding_generation: parts.application_binding_generation,
            installed_operation: parts.installed_operation,
            mutation_work: parts.mutation_work,
            retained_preimage: parts.retained_preimage,
            retained_governed_input: parts
                .retained_governed_input_identity
                .map(WorthQueryRetainedGovernedInput::axis_probe),
            principal_scope: parts.principal_scope,
            idempotency: parts.idempotency,
            provider_posture: parts.provider_posture,
            committed_dispatch_outbox: match (parts.dispatch_outbox, parts.dispatch_outbox_record_ref) {
                (Some(record), Some(record_ref)) => Some(
                    crate::domain_computation::primary_graph::WorthQueryCommittedDispatchOutboxBinding::fixture(
                        record,
                        record_ref,
                    ),
                ),
                (None, None) => None,
                _ => panic!("dispatch outbox record and record identity must travel together"),
            },
            installed_aftermath: parts.installed_aftermath,
            expires_at_unix_ms: parts.expires_at_unix_ms,
        }
    }
}

/// Parts for [`WorthQueryRecoveryHandleBinding::axis_probe`].
#[cfg(test)]
#[derive(Clone)]
pub(crate) struct WorthQueryRecoveryHandleBindingAxisProbe {
    pub runtime_instance_id: u64,
    pub schema_identity: [u8; 32],
    pub branch: BranchId,
    pub application_binding_generation: u64,
    pub installed_operation: [u8; 32],
    pub attempt_commit_id: u64,
    pub mutation_work: Option<WorthQueryPrimaryMutationWorkEvidence>,
    pub retained_preimage: Option<WorthQueryRetainedPreImage>,
    pub retained_governed_input_identity: Option<[u8; 32]>,
    pub principal_scope: WorthQueryOperationScopeBinding,
    pub idempotency: WorthQueryApplicationIdempotencyBinding,
    pub provider_posture: Option<WorthQueryExternalDispatchPosture>,
    pub dispatch_outbox: Option<WorthQueryDispatchOutboxRecord>,
    pub dispatch_outbox_record_ref: Option<worth_relational::facade::transactions::RecordRef>,
    pub installed_aftermath: WorthQueryInstalledAftermathContract,
    pub expires_at_unix_ms: Option<u64>,
}
