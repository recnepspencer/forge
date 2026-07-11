use forge_store_wal::{
    AdmittedCheckpointPublicationReceipt, AdmittedWalAppendReceipt, BlobWalRecordEnvelope,
    WalSecurityMetadataCarrier,
};

use super::{
    execution::BaselineLsmExecutionRequest, BaselineLsmAdmittedKey, BaselineLsmAdmittedRecord,
    BaselineLsmCompactionPlan, BaselineLsmDurableInputs, BaselineLsmExecutionAdmissionDenial,
    BaselineLsmExecutionIntent, BaselineLsmExecutionWitness, BaselineLsmWalIndexSession,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmStrategy;

pub const fn lsm_strategy() -> LsmStrategy {
    LsmStrategy
}

impl LsmStrategy {
    pub fn open_index(
        self,
        durable_anchor: &AdmittedWalAppendReceipt,
    ) -> Result<BaselineLsmWalIndexSession, BaselineLsmExecutionAdmissionDenial> {
        let root = durable_anchor
            .persisted_path()
            .parent()
            .ok_or(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
        let artifact_root = root
            .parent()
            .ok_or(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
        BaselineLsmWalIndexSession::open(
            root,
            artifact_root,
            durable_anchor.scope().segment_id(),
            durable_anchor.scope().generation(),
        )
    }

    pub fn admit_key(
        self,
        metadata: WalSecurityMetadataCarrier,
        canonical_key_bytes: [u8; 8],
    ) -> Result<BaselineLsmAdmittedKey, BaselineLsmExecutionAdmissionDenial> {
        BaselineLsmAdmittedKey::admit(metadata, canonical_key_bytes)
            .ok_or(BaselineLsmExecutionAdmissionDenial::CanonicalKeyRequired)
    }

    pub fn persist_record(
        self,
        session: &mut BaselineLsmWalIndexSession,
        envelope: BlobWalRecordEnvelope,
        durable: &AdmittedWalAppendReceipt,
        key: BaselineLsmAdmittedKey,
    ) -> Result<BaselineLsmAdmittedRecord, BaselineLsmExecutionAdmissionDenial> {
        let record = BaselineLsmAdmittedRecord::admit(envelope, durable, key)
            .ok_or(BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch)?;
        session.persist(record.clone())?;
        Ok(record)
    }

    pub fn lower_compaction(
        self,
        session: &BaselineLsmWalIndexSession,
        key: BaselineLsmAdmittedKey,
    ) -> Result<BaselineLsmCompactionPlan, BaselineLsmExecutionAdmissionDenial> {
        BaselineLsmCompactionPlan::lower_from_persisted(session, key)
    }

    pub fn admit_durable_inputs(
        self,
        plan: BaselineLsmCompactionPlan,
        manifest: AdmittedCheckpointPublicationReceipt,
        output_append: AdmittedWalAppendReceipt,
    ) -> Result<BaselineLsmDurableInputs, BaselineLsmExecutionAdmissionDenial> {
        BaselineLsmDurableInputs::admit(plan, manifest, output_append)
    }

    pub fn execute(
        self,
        session: &mut BaselineLsmWalIndexSession,
        intent: BaselineLsmExecutionIntent,
        inputs: BaselineLsmDurableInputs,
    ) -> Result<BaselineLsmExecutionWitness, BaselineLsmExecutionAdmissionDenial> {
        BaselineLsmExecutionWitness::execute(
            BaselineLsmExecutionRequest::new(intent, inputs),
            session,
        )
    }
}
