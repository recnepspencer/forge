use forge_store_security::{StoreKeyScope, StoreTenantScope};

use crate::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, DurablePublicationDeclaration,
    WalFrameDurablePublicationScope,
};

use super::{
    BaselineLsmAdmittedRecord, BaselineLsmDurableInputs, BaselineLsmExecutionIntent,
    BaselineLsmPhysicalPublicationBinding, BaselineLsmWalIndexSession,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BaselineLsmExecutionRequest {
    intent: BaselineLsmExecutionIntent,
    inputs: BaselineLsmDurableInputs,
}

impl BaselineLsmExecutionRequest {
    pub(crate) const fn new(
        intent: BaselineLsmExecutionIntent,
        inputs: BaselineLsmDurableInputs,
    ) -> Self {
        Self { intent, inputs }
    }

    pub(crate) const fn tenant_scope(&self) -> StoreTenantScope {
        self.inputs.key().tenant_scope()
    }

    pub(crate) const fn key_scope(&self) -> StoreKeyScope {
        self.inputs.key().key_scope()
    }

    pub(crate) const fn canonical_key_bytes(&self) -> [u8; 8] {
        self.inputs.key().canonical_key_bytes()
    }

    pub(crate) const fn physical_publication(&self) -> BaselineLsmPhysicalPublicationBinding {
        self.intent.physical_publication
    }

    pub(crate) fn replay_tail(&self) -> [&BlobWalRecordEnvelope; 3] {
        [
            &self.inputs.value.envelope,
            &self.inputs.generation.envelope,
            &self.inputs.tombstone.envelope,
        ]
    }

    pub(crate) fn identities(&self) -> [BlobWalRecordIdentity; 3] {
        self.replay_tail().map(|record| record.identity())
    }

    pub(crate) const fn records(&self) -> [&BaselineLsmAdmittedRecord; 3] {
        [
            &self.inputs.value,
            &self.inputs.generation,
            &self.inputs.tombstone,
        ]
    }

    pub(crate) fn manifest_publication(&self) -> DurablePublicationDeclaration {
        DurablePublicationDeclaration::manifest(self.inputs.manifest.scope().clone())
    }

    pub(crate) fn output_scope(&self) -> &WalFrameDurablePublicationScope {
        self.inputs.output_append.scope()
    }

    pub(crate) fn retire_persisted_inputs(
        &self,
        session: &mut BaselineLsmWalIndexSession,
    ) -> Result<
        (
            [super::super::BaselineLsmRunIdentity; 3],
            super::super::BaselineLsmCounterObservation,
        ),
        super::super::BaselineLsmExecutionAdmissionDenial,
    > {
        let expected = self.identities();
        session.retire(
            self.inputs.key(),
            self.inputs.key_version,
            expected,
            &self.inputs.manifest,
        )?;
        let mut retired = [super::super::BaselineLsmRunIdentity::new_for_executor(expected[0]); 3];
        let mut counters = super::super::BaselineLsmCounterObservation::default();
        for (slot, expected_identity) in expected.into_iter().enumerate() {
            retired[slot] =
                super::super::BaselineLsmRunIdentity::new_for_executor(expected_identity);
            counters.record_maintenance_read();
        }
        counters.record_publication();
        Ok((retired, counters))
    }
}
