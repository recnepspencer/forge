use super::baseline_lsm_compaction_execution::{
    BaselineLsmCompactionExecutionEffects, BaselineLsmCompactionPublicationReceipt,
    BaselineLsmCompactionRecordKind,
};
use super::{
    BaselineLsmCounterObservation, BaselineLsmExecutionAdmissionDenial,
    BaselineLsmExecutionRequest, BaselineLsmExecutionWitness, BaselineLsmLookupDisposition,
    BaselineLsmLookupExecution, BaselineLsmManifestPublicationExecution,
    BaselineLsmReplayExecution,
};
use forge_store_wal::{
    record_kind_admits_recovery_replay, BlobWalRecordEnvelope, BlobWalRecordIdentity,
    BlobWalRecordKind, DurablePublicationDeclaration, DurablePublicationScope,
};

impl BaselineLsmExecutionWitness {
    pub(crate) fn execute(
        request: BaselineLsmExecutionRequest,
        session: &mut super::BaselineLsmWalIndexSession,
    ) -> Result<Self, BaselineLsmExecutionAdmissionDenial> {
        let identities = request.identities();
        let memtable_records = [identities[2]];
        let sorted_run_records = [identities[0], identities[1]];
        let replay_tail = std::array::from_fn(|index| request.replay_tail()[index].clone());
        if request.records().iter().any(|record| {
            record.tenant_scope() != request.tenant_scope()
                || record.key_scope() != request.key_scope()
                || record.canonical_key_bytes() != request.canonical_key_bytes()
        }) {
            return Err(BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch);
        }
        if sorted_run_records[0].sequence() >= sorted_run_records[1].sequence() {
            return Err(BaselineLsmExecutionAdmissionDenial::SortedRunsNotCanonical);
        }
        if memtable_records[0].sequence() <= sorted_run_records[1].sequence() {
            return Err(BaselineLsmExecutionAdmissionDenial::MemtableDoesNotFollowSortedRuns);
        }
        if !(replay_tail[0].identity().sequence() < replay_tail[1].identity().sequence()
            && replay_tail[1].identity().sequence() < replay_tail[2].identity().sequence())
        {
            return Err(BaselineLsmExecutionAdmissionDenial::ReplayTailNotCanonical);
        }
        if replay_tail[0].identity() != sorted_run_records[0]
            || replay_tail[1].identity() != sorted_run_records[1]
            || replay_tail[2].identity() != memtable_records[0]
        {
            return Err(BaselineLsmExecutionAdmissionDenial::ReplayBindingMismatch);
        }
        if replay_tail[0].identity().kind() != BlobWalRecordKind::LsmValue {
            return Err(BaselineLsmExecutionAdmissionDenial::ValueRecordRequired);
        }
        if replay_tail[2].identity().kind() != BlobWalRecordKind::LsmTombstone {
            return Err(BaselineLsmExecutionAdmissionDenial::TombstoneRecordRequired);
        }
        if request.canonical_key_bytes() == [0; 8] {
            return Err(BaselineLsmExecutionAdmissionDenial::CanonicalKeyRequired);
        }
        if !matches!(
            request.manifest_publication().scope(),
            DurablePublicationScope::Manifest(_)
        ) {
            return Err(BaselineLsmExecutionAdmissionDenial::ManifestPublicationRequired);
        }
        let expected_output_generation = replay_tail[2]
            .identity()
            .sequence()
            .checked_add(1)
            .ok_or(BaselineLsmExecutionAdmissionDenial::OutputGenerationOverflow)?;
        require_manifest_covers_execution(&request, expected_output_generation)?;
        let manifest_publication = request.manifest_publication();
        let DurablePublicationScope::Manifest(_manifest_scope) = manifest_publication.scope()
        else {
            return Err(BaselineLsmExecutionAdmissionDenial::ManifestPublicationRequired);
        };
        let output_publication = BlobWalRecordEnvelope::new(
            BlobWalRecordIdentity::new(
                expected_output_generation,
                BlobWalRecordKind::GenerationPublication,
            )
            .expect("checked nonzero generation"),
            DurablePublicationDeclaration::wal_frame(request.output_scope().clone()),
            request.output_scope().frame_digest().to_owned(),
        )
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::OutputPublicationMismatch)?;
        let compaction = execute_compaction(
            &request,
            session,
            expected_output_generation,
            output_publication.clone(),
            manifest_publication.clone(),
        )?;
        Ok(Self {
            memtable_records,
            sorted_run_records,
            wal_publication: output_publication,
            manifest_publication,
            replay_tail,
            compaction,
        })
    }

    pub fn execute_lookup_latest_visible_record(
        &self,
        probe_sequence: u64,
    ) -> BaselineLsmLookupExecution {
        let memtable_record = self.memtable_records[0];
        let sorted_run_record = self.sorted_run_records[0];
        let disposition = self.lookup_disposition_for(probe_sequence);
        let tombstone_blocks_older = disposition == BaselineLsmLookupDisposition::NotFound
            && probe_sequence == sorted_run_record.sequence();
        BaselineLsmLookupExecution::new(
            probe_sequence,
            disposition,
            memtable_record,
            sorted_run_record,
            disposition == BaselineLsmLookupDisposition::Memtable,
            disposition == BaselineLsmLookupDisposition::SortedRun,
            tombstone_blocks_older,
            BaselineLsmCounterObservation::new(1, 1, 0, 0, 0),
        )
    }

    pub fn execute_lookup_older_visible_record(&self) -> BaselineLsmLookupExecution {
        let older_visible = self.sorted_run_records[1].sequence();
        let sorted_run_record = self.sorted_run_records[1];
        BaselineLsmLookupExecution::new(
            older_visible,
            BaselineLsmLookupDisposition::SortedRun,
            self.memtable_records[0],
            sorted_run_record,
            false,
            true,
            false,
            BaselineLsmCounterObservation::new(1, 1, 0, 0, 0),
        )
    }

    pub fn execute_lookup_tombstone_blocked_record(&self) -> BaselineLsmLookupExecution {
        let blocked_older = self.sorted_run_records[0].sequence();
        BaselineLsmLookupExecution::new(
            blocked_older,
            BaselineLsmLookupDisposition::NotFound,
            self.memtable_records[0],
            self.sorted_run_records[0],
            false,
            true,
            true,
            BaselineLsmCounterObservation::new(1, 1, 0, 0, 0),
        )
    }

    pub fn execute_manifest_publication(&self) -> BaselineLsmManifestPublicationExecution {
        assert!(matches!(
            self.wal_publication.durable_publication().scope(),
            DurablePublicationScope::WalFrame(_)
        ));
        assert!(matches!(
            self.manifest_publication.scope(),
            DurablePublicationScope::Manifest(_)
        ));
        BaselineLsmManifestPublicationExecution::new(
            self.wal_publication.clone(),
            self.manifest_publication.clone(),
            self.sorted_run_records.len() as u16,
            self.sorted_run_records.len() > 1,
            false,
            BaselineLsmCounterObservation::new(0, 0, 0, 2, self.sorted_run_records.len() as u16),
        )
    }

    pub fn execute_replay_wal_tail(&self) -> BaselineLsmReplayExecution {
        let replay_tail = self
            .replay_tail
            .clone()
            .map(|record| record.identity().kind());
        let replayable = self
            .replay_tail
            .iter()
            .filter(|record| record_kind_admits_recovery_replay(record.identity().kind()))
            .count() as u16;
        BaselineLsmReplayExecution::new(
            replay_tail,
            replayable,
            (self.sorted_run_records.len() - 1) as u16,
            1,
            self.sorted_run_records.len() as u16,
            BaselineLsmCounterObservation::new(0, 0, replayable, 0, 1),
        )
    }

    pub const fn compaction_publication_receipt(&self) -> &BaselineLsmCompactionPublicationReceipt {
        &self.compaction
    }

    pub fn lookup_disposition_for(&self, probe_sequence: u64) -> BaselineLsmLookupDisposition {
        if self
            .memtable_records
            .iter()
            .any(|record| record.sequence() == probe_sequence)
        {
            return BaselineLsmLookupDisposition::Memtable;
        }
        if probe_sequence == self.sorted_run_records[1].sequence() {
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

    pub const fn replay_tail(&self) -> &[BlobWalRecordEnvelope; 3] {
        &self.replay_tail
    }
}

fn require_manifest_covers_execution(
    request: &BaselineLsmExecutionRequest,
    output_generation: u64,
) -> Result<(), BaselineLsmExecutionAdmissionDenial> {
    let manifest = request.manifest_publication();
    let DurablePublicationScope::Manifest(scope) = manifest.scope() else {
        return Err(BaselineLsmExecutionAdmissionDenial::ManifestPublicationRequired);
    };
    let oldest = request.replay_tail()[0].identity().sequence();
    if scope.covered_lsn_start() > oldest || scope.covered_lsn_end() < output_generation {
        return Err(BaselineLsmExecutionAdmissionDenial::ManifestDoesNotCoverCompaction);
    }
    let output_scope = request.output_scope();
    if output_scope.lsn_start() > output_generation
        || output_scope.lsn_end() < output_generation
        || output_scope.expected_bytes() == 0
    {
        return Err(BaselineLsmExecutionAdmissionDenial::OutputPublicationMismatch);
    }
    Ok(())
}

fn execute_compaction(
    request: &BaselineLsmExecutionRequest,
    session: &mut super::BaselineLsmWalIndexSession,
    output_generation: u64,
    output_publication: BlobWalRecordEnvelope,
    manifest_publication: DurablePublicationDeclaration,
) -> Result<BaselineLsmCompactionPublicationReceipt, BaselineLsmExecutionAdmissionDenial> {
    let replay = request.replay_tail();
    let key = BaselineLsmCompactionPublicationReceipt::admitted_key(
        request.tenant_scope(),
        request.key_scope(),
        request.canonical_key_bytes(),
    );
    let input_runs = std::array::from_fn(|index| {
        BaselineLsmCompactionPublicationReceipt::run(
            replay[index].identity().sequence(),
            replay[index].identity(),
        )
    });
    let (retired_runs, counters) = request.retire_persisted_inputs(session)?;
    let effects =
        BaselineLsmCompactionExecutionEffects::from_persisted_execution(retired_runs, counters);
    let bytes_in = replay.iter().map(|record| wal_frame_bytes(record)).sum();
    let bytes_out = wal_frame_bytes(&output_publication);
    Ok(BaselineLsmCompactionPublicationReceipt::new(
        key,
        input_runs,
        BaselineLsmCompactionPublicationReceipt::run(
            output_generation,
            output_publication.identity(),
        ),
        output_publication,
        BaselineLsmCompactionPublicationReceipt::record(
            key,
            replay[2].identity(),
            input_runs[2],
            BaselineLsmCompactionRecordKind::Tombstone,
        ),
        BaselineLsmCompactionPublicationReceipt::record(
            key,
            replay[0].identity(),
            input_runs[0],
            BaselineLsmCompactionRecordKind::Value,
        ),
        manifest_publication,
        std::array::from_fn(|index| replay[index].identity()),
        bytes_in,
        bytes_out,
        effects,
        request.physical_publication(),
    ))
}

fn wal_frame_bytes(record: &BlobWalRecordEnvelope) -> u64 {
    let DurablePublicationScope::WalFrame(scope) = record.durable_publication().scope() else {
        unreachable!("BlobWalRecordEnvelope admits only WAL-frame publication")
    };
    scope.expected_bytes()
}
