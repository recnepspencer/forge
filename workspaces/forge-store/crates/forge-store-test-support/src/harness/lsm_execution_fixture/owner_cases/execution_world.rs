use forge_store_layout_indexes::{
    layout_lsm_maintenance, lsm_strategy, BaselineLsmCompactionAdmission,
    BaselineLsmCompactionPlan, BaselineLsmRunPublicationAdmission, LsmCompactionAdmissionRequest,
    LsmPhysicalCompactionIntent, LsmRunPublicationAdmissionRequest,
};
use forge_store_lsm_authority::{LsmMembershipArtifactDeclaration, LsmMembershipSession};
use forge_store_physical_isolation::CompactionRewritePublication;
use forge_store_wal::{
    admit_checkpoint_publication, admit_durable_append, AdmittedCheckpointPublicationReceipt,
    AdmittedWalAppendReceipt, BlobWalRecordEnvelope, BlobWalRecordKind,
    CheckpointDurablePublicationScope, StoreCheckpointRecordIdentity, StoreWalRecordIdentity,
    WalSecurityMetadataCarrier,
};

use super::super::{
    begin_durability_fixture, durable_record_binding, manifest_receipt_for_artifact,
    physical_compaction_fixture, wal_receipt, wal_scope, PreExecutionBudgetEnvelope,
    StoreKeyVersionPosture, StoreLegacySecurityPosture, WalRecordFamily,
};

pub(super) struct LsmExecutionWorld {
    pub session: LsmMembershipSession,
    pub plan: BaselineLsmCompactionPlan,
    pub publication: BaselineLsmRunPublicationAdmission,
    pub physical_intent: LsmPhysicalCompactionIntent,
    pub physical_publication: CompactionRewritePublication,
    pub output: AdmittedWalAppendReceipt,
    pub output_path: std::path::PathBuf,
}

pub(super) fn world(request_sequence: u64, record_sequences: [u64; 3]) -> LsmExecutionWorld {
    begin_durability_fixture();
    let security =
        forge_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let compaction = compaction_admission(security.witnesses(), request_sequence);
    let publication = publication_admission(security.witnesses(), request_sequence);
    let metadata = WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let key = lsm_strategy()
        .admit_key(metadata, compaction.clone())
        .unwrap();
    let kinds = [
        BlobWalRecordKind::LsmValue,
        BlobWalRecordKind::GenerationPublication,
        BlobWalRecordKind::LsmTombstone,
    ];
    let records: [(BlobWalRecordEnvelope, AdmittedWalAppendReceipt); 3] =
        std::array::from_fn(|index| {
            durable_record_binding(key, record_sequences[index], kinds[index])
        });
    let mut session =
        forge_store_lsm_authority::open_lsm_membership(&records[0].1, security.witnesses())
            .into_result()
            .unwrap();
    for (envelope, durable) in records {
        lsm_strategy()
            .persist_record(&mut session, envelope, &durable, key)
            .unwrap();
    }
    let plan = lsm_strategy()
        .lower_compaction(&session, key, compaction)
        .unwrap();
    let (physical_intent, physical_publication) = physical_compaction_fixture();
    let output_scope = wal_scope(
        record_sequences[2].checked_add(1).unwrap(),
        plan.output_frame_digest(&physical_intent),
        4096,
    );
    let output_artifact = LsmMembershipArtifactDeclaration::compaction_output(&output_scope);
    let output = admit_durable_append(&wal_receipt(output_scope, output_artifact.bytes())).unwrap();
    let output_path = output.persisted_path().to_path_buf();
    LsmExecutionWorld {
        session,
        plan,
        publication,
        physical_intent,
        physical_publication,
        output,
        output_path,
    }
}

impl LsmExecutionWorld {
    pub fn demand(&self) -> forge_store_layout_indexes::AdmittedLsmCompactionDemand {
        lsm_strategy()
            .admit_compaction_demand(
                self.plan.clone(),
                self.output.clone(),
                self.physical_intent.clone(),
            )
            .unwrap()
    }

    pub fn publication_for(&self, request_sequence: u64) -> BaselineLsmRunPublicationAdmission {
        let security =
            forge_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
        publication_admission(security.witnesses(), request_sequence)
    }

    pub fn manifest(
        &self,
        activation: &forge_store_lsm_authority::LsmMembershipActivationDeclaration,
    ) -> AdmittedCheckpointPublicationReceipt {
        let artifact = activation.artifact();
        self.manifest_with_scope(activation.scope().clone(), artifact.bytes())
    }

    pub fn manifest_with_scope(
        &self,
        scope: CheckpointDurablePublicationScope,
        bytes: &[u8],
    ) -> AdmittedCheckpointPublicationReceipt {
        admit_checkpoint_publication(&manifest_receipt_for_artifact(scope, bytes)).unwrap()
    }

    pub fn noncovering_manifest(
        &self,
        activation: &forge_store_lsm_authority::LsmMembershipActivationDeclaration,
    ) -> AdmittedCheckpointPublicationReceipt {
        let scope = CheckpointDurablePublicationScope::new(
            StoreCheckpointRecordIdentity::new(activation.scope().checkpoint().checkpoint_epoch()),
            activation.scope().manifest_digest().to_owned(),
            activation.scope().covered_lsn_start().saturating_add(1),
            activation.scope().covered_lsn_end(),
        )
        .unwrap();
        let artifact = activation.artifact();
        self.manifest_with_scope(scope, artifact.bytes())
    }
}

fn compaction_admission(
    security: &forge_store_security::StoreCurrentSecurityScopeWitnessSet,
    sequence: u64,
) -> BaselineLsmCompactionAdmission {
    layout_lsm_maintenance()
        .admit_compaction(LsmCompactionAdmissionRequest::new(
            security,
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(sequence),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .unwrap()
}

fn publication_admission(
    security: &forge_store_security::StoreCurrentSecurityScopeWitnessSet,
    sequence: u64,
) -> BaselineLsmRunPublicationAdmission {
    layout_lsm_maintenance()
        .admit_run_publication(LsmRunPublicationAdmissionRequest::new(
            security,
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(sequence),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .unwrap()
}
