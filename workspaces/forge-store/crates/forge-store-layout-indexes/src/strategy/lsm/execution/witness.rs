use super::{
    AdmittedLsmCompactionDemand, BaselineLsmExecutionAdmissionDenial, BaselineLsmLookupDisposition,
    BaselineLsmLookupSource, BaselineLsmManifestPublicationExecution, PreparedLsmCompaction,
    PublishedLsmCompaction,
};
use forge_store_wal::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind, DurablePublicationDeclaration,
    DurablePublicationScope,
};

impl PreparedLsmCompaction {
    pub(crate) fn execute(
        demand: AdmittedLsmCompactionDemand,
    ) -> Result<Self, BaselineLsmExecutionAdmissionDenial> {
        let key = demand.key().canonical();
        if demand
            .compaction_admission()
            .selected()
            .request_identity()
            .canonical_key()
            != key
        {
            return Err(BaselineLsmExecutionAdmissionDenial::SelectedOperationKeyMismatch);
        }
        let identities = demand.identities();
        if demand.records().iter().any(|record| {
            record.key().tenant_scope() != demand.key().tenant_scope()
                || record.key().key_scope() != demand.key().key_scope()
                || record.key().canonical() != key
        }) {
            return Err(BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch);
        }
        if identities[0].sequence() >= identities[1].sequence() {
            return Err(BaselineLsmExecutionAdmissionDenial::SortedRunsNotCanonical);
        }
        if identities[2].sequence() <= identities[1].sequence() {
            return Err(BaselineLsmExecutionAdmissionDenial::MemtableDoesNotFollowSortedRuns);
        }
        if identities[0].kind() != BlobWalRecordKind::LsmValue {
            return Err(BaselineLsmExecutionAdmissionDenial::ValueRecordRequired);
        }
        if identities[2].kind() != BlobWalRecordKind::LsmTombstone {
            return Err(BaselineLsmExecutionAdmissionDenial::TombstoneRecordRequired);
        }
        if key == [0; 8] {
            return Err(BaselineLsmExecutionAdmissionDenial::CanonicalKeyRequired);
        }
        let output_generation = identities[2]
            .sequence()
            .checked_add(1)
            .ok_or(BaselineLsmExecutionAdmissionDenial::OutputGenerationOverflow)?;
        let output = demand.output();
        if output.identity().sequence() != output_generation
            || output.identity().kind() != BlobWalRecordKind::GenerationPublication
        {
            return Err(BaselineLsmExecutionAdmissionDenial::OutputPublicationMismatch);
        }
        Ok(Self {
            membership: demand.membership().clone(),
            replay_tail: std::array::from_fn(|index| demand.replay_tail()[index].clone()),
            output: output.clone(),
            physical_intent: demand.physical_intent().clone(),
        })
    }

    pub fn input_identities(&self) -> [BlobWalRecordIdentity; 3] {
        self.membership.identities()
    }

    pub const fn output_publication(&self) -> &BlobWalRecordEnvelope {
        self.output.envelope()
    }
}

impl PublishedLsmCompaction {
    pub fn membership_replacement(
        &self,
    ) -> &forge_store_lsm_authority::PublishedLsmMembershipReplacement {
        &self.membership_replacement
    }

    pub fn observe_reader_cutover(
        &self,
        recovery: forge_store_recovery_physics::CompactionCutoverRecoveryPosture,
        pre_cutover_read: forge_store_physical_isolation::StablePhysicalReadReceipt,
        post_cutover_read: forge_store_physical_isolation::StablePhysicalReadReceipt,
    ) -> Result<
        forge_store_physical_isolation::ReadDuringCompactionVerdict,
        forge_store_physical_isolation::CompactionReadInterlockDenial,
    > {
        forge_store_physical_isolation::execute_read_during_compaction_cutover(
            self.physical_compaction.clone(),
            recovery,
            pre_cutover_read,
            post_cutover_read,
        )
    }

    pub fn admit_lookup_source(&self) -> BaselineLsmLookupSource {
        BaselineLsmLookupSource::from_published_replacement(&self.membership_replacement)
    }

    pub fn publication_execution(&self) -> BaselineLsmManifestPublicationExecution {
        debug_assert!(matches!(
            self.wal_publication.durable_publication().scope(),
            DurablePublicationScope::WalFrame(_)
        ));
        debug_assert!(matches!(
            self.manifest_publication.scope(),
            DurablePublicationScope::Manifest(_)
        ));
        BaselineLsmManifestPublicationExecution::new(
            self.membership_replacement.clone(),
            self.wal_publication.clone(),
            self.manifest_publication.clone(),
            self.sorted_run_records.len() as u16,
            self.sorted_run_records.len() > 1,
            false,
        )
    }

    pub const fn compaction_publication_receipt(
        &self,
    ) -> &super::BaselineLsmCompactionPublicationReceipt {
        &self.compaction
    }

    pub fn lookup_disposition_for(&self, probe_sequence: u64) -> BaselineLsmLookupDisposition {
        if self.memtable_records[0].sequence() == probe_sequence {
            return BaselineLsmLookupDisposition::Memtable;
        }
        if self.sorted_run_records[1].sequence() == probe_sequence {
            return BaselineLsmLookupDisposition::SortedRun;
        }
        BaselineLsmLookupDisposition::NotFound
    }

    pub const fn memtable_records(&self) -> &[BlobWalRecordIdentity; 1] {
        &self.memtable_records
    }

    pub const fn sorted_run_records(&self) -> &[BlobWalRecordIdentity; 2] {
        &self.sorted_run_records
    }

    pub const fn wal_publication(&self) -> &BlobWalRecordEnvelope {
        &self.wal_publication
    }

    pub const fn manifest_publication(&self) -> &DurablePublicationDeclaration {
        &self.manifest_publication
    }

    pub const fn physical_compaction(
        &self,
    ) -> &forge_store_physical_isolation::CompactionRewritePublication {
        &self.physical_compaction
    }
}
