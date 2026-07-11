#[cfg(any(test, feature = "certification-authority"))]
mod baseline_lsm_certification_execution;
pub mod baseline_lsm_counter_observation;
pub mod baseline_lsm_invariant_proof;
pub mod baseline_lsm_invariant_witness;
pub mod checkpoint_family;
pub mod durable_mutation_family;
pub mod replay_tail_family;
pub mod wal_record_family;
pub mod wal_segment_family;

#[cfg(any(test, feature = "certification-authority"))]
pub use baseline_lsm_certification_execution::execute_baseline_lsm_persisted_fixture;
pub use checkpoint_family::{
    AdmittedCheckpointLayoutFamily, AdmittedCheckpointLayoutRule,
    AdmittedCheckpointPublicationReceipt, CheckpointLayoutFamilyHome,
    CheckpointPublicationLayoutReport,
};
pub use durable_mutation_family::{
    AdmittedDurableMutationLayoutFamily, AdmittedWalAppendLayoutRule, AdmittedWalAppendReceipt,
    DurableMutationLayoutFamilyHome, WalAppendLayoutReport,
};
pub use replay_tail_family::{
    AdmittedReplayTailCursor, AdmittedReplayTailLayoutFamily, AdmittedWalTailLayoutRule,
    ReplayTailLayoutFamilyHome, WalReplayTailCursorReport, WalReplayTailRecordReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalLayoutAccessDenialKind {
    WrongPublicationKind,
    NonReplayTailRecord,
    ReplayTopologyDenied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalLayoutAccessDenial {
    kind: WalLayoutAccessDenialKind,
}

impl WalLayoutAccessDenial {
    pub const fn new(kind: WalLayoutAccessDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WalLayoutAccessDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalLayoutAccess;

impl WalLayoutAccess {
    pub const fn s8() -> Self {
        Self
    }

    pub fn durable_mutation(self) -> AdmittedDurableMutationLayoutFamily {
        let rule = AdmittedWalAppendLayoutRule::internal_phase21();
        let admission = DurableMutationLayoutFamilyHome::s8()
            .admit(&rule)
            .expect("owner-issued WAL append rule is admitted");
        AdmittedDurableMutationLayoutFamily::new(admission)
    }

    pub fn replay_tail(self) -> AdmittedReplayTailLayoutFamily {
        let rule = AdmittedWalTailLayoutRule::internal_phase21();
        let admission = ReplayTailLayoutFamilyHome::s8()
            .admit(&rule)
            .expect("owner-issued WAL tail rule is admitted");
        AdmittedReplayTailLayoutFamily::new(admission)
    }

    pub fn checkpoint(self) -> AdmittedCheckpointLayoutFamily {
        let rule = AdmittedCheckpointLayoutRule::internal_phase21();
        let admission = CheckpointLayoutFamilyHome::s8()
            .admit(&rule)
            .expect("owner-issued checkpoint rule is admitted");
        AdmittedCheckpointLayoutFamily::new(admission)
    }

    pub fn execute_baseline_lsm(
        self,
        session: &mut baseline_lsm_counter_observation::BaselineLsmWalIndexSession,
        intent: baseline_lsm_counter_observation::BaselineLsmExecutionIntent,
        inputs: baseline_lsm_counter_observation::BaselineLsmDurableInputs,
    ) -> Result<
        baseline_lsm_counter_observation::BaselineLsmExecutionWitness,
        baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial,
    > {
        baseline_lsm_counter_observation::BaselineLsmExecutionWitness::execute(
            baseline_lsm_counter_observation::BaselineLsmExecutionRequest::new(intent, inputs),
            session,
        )
    }

    pub fn open_baseline_lsm_index(
        self,
        durable_anchor: &AdmittedWalAppendReceipt,
    ) -> Result<
        baseline_lsm_counter_observation::BaselineLsmWalIndexSession,
        baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial,
    > {
        let root = durable_anchor.persisted_path().parent().ok_or(
            baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial::PersistedIndexIo,
        )?;
        let artifact_root = durable_anchor
            .persisted_path()
            .parent()
            .and_then(std::path::Path::parent)
            .ok_or(
                baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial::PersistedIndexIo,
            )?;
        baseline_lsm_counter_observation::BaselineLsmWalIndexSession::open(
            root,
            artifact_root,
            durable_anchor.scope().segment_id(),
            durable_anchor.scope().generation(),
        )
    }

    pub fn admit_baseline_lsm_append_durability(
        self,
        receipt: &forge_store_physical_backend::StoreDurabilityOrderingBarrierDurable<
            crate::WalFrameDurablePublicationScope,
        >,
    ) -> Result<AdmittedWalAppendReceipt, WalLayoutAccessDenial> {
        if receipt.publication()
            != forge_store_physical_backend::StoreDurabilityPublicationKind::WalFrame
        {
            return Err(WalLayoutAccessDenial::new(
                WalLayoutAccessDenialKind::WrongPublicationKind,
            ));
        }
        Ok(AdmittedWalAppendReceipt::from_receipt(receipt))
    }

    pub fn admit_baseline_lsm_manifest_durability(
        self,
        receipt: &forge_store_physical_backend::StoreDurabilityOrderingBarrierDurable<
            crate::CheckpointDurablePublicationScope,
        >,
    ) -> Result<AdmittedCheckpointPublicationReceipt, WalLayoutAccessDenial> {
        if receipt.publication()
            != forge_store_physical_backend::StoreDurabilityPublicationKind::Manifest
        {
            return Err(WalLayoutAccessDenial::new(
                WalLayoutAccessDenialKind::WrongPublicationKind,
            ));
        }
        Ok(AdmittedCheckpointPublicationReceipt::from_receipt(receipt))
    }

    pub fn persist_baseline_lsm_record(
        self,
        session: &mut baseline_lsm_counter_observation::BaselineLsmWalIndexSession,
        envelope: crate::BlobWalRecordEnvelope,
        durable: &AdmittedWalAppendReceipt,
        key: baseline_lsm_counter_observation::BaselineLsmAdmittedKey,
    ) -> Result<
        baseline_lsm_counter_observation::BaselineLsmAdmittedRecord,
        baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial,
    > {
        let record = baseline_lsm_counter_observation::BaselineLsmAdmittedRecord::admit(
            envelope, durable, key,
        )
        .ok_or(baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch)?;
        session.persist(record.clone())?;
        Ok(record)
    }

    pub fn admit_baseline_lsm_key(
        self,
        metadata: crate::WalSecurityMetadataCarrier,
        canonical_key_bytes: [u8; 8],
    ) -> Result<
        baseline_lsm_counter_observation::BaselineLsmAdmittedKey,
        baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial,
    > {
        match baseline_lsm_counter_observation::BaselineLsmAdmittedKey::admit(
            metadata,
            canonical_key_bytes,
        ) {
            Some(key) => Ok(key),
            None => Err(baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial::CanonicalKeyRequired),
        }
    }

    pub fn lower_baseline_lsm_compaction(
        self,
        session: &baseline_lsm_counter_observation::BaselineLsmWalIndexSession,
        key: baseline_lsm_counter_observation::BaselineLsmAdmittedKey,
    ) -> Result<
        baseline_lsm_counter_observation::BaselineLsmCompactionPlan,
        baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial,
    > {
        baseline_lsm_counter_observation::BaselineLsmCompactionPlan::lower_from_persisted(
            session, key,
        )
    }

    pub fn admit_baseline_lsm_persisted_source(
        self,
        plan: baseline_lsm_counter_observation::BaselineLsmCompactionPlan,
        manifest: AdmittedCheckpointPublicationReceipt,
        output_append: AdmittedWalAppendReceipt,
    ) -> Result<
        baseline_lsm_counter_observation::BaselineLsmDurableInputs,
        baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial,
    > {
        baseline_lsm_counter_observation::BaselineLsmDurableInputs::admit(
            plan,
            manifest,
            output_append,
        )
    }
}
