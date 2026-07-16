use worth_store_lsm_authority::{replace_lsm_membership, LsmMembershipSession};
use worth_store_wal::{
    AdmittedCheckpointPublicationReceipt, BlobWalRecordEnvelope, DurablePublicationDeclaration,
    DurablePublicationScope,
};

use super::super::{
    BaselineLsmExecutionAdmissionDenial, BaselineLsmExecutionAdmissionDenialKind,
    BaselineLsmRunPublicationAdmission, LsmExecutionOperation, LsmExecutionOwnerCaseDeclaration,
    LsmExecutionOwnerCaseId, LsmExecutionOwnerCaseObservation,
};
use super::evidence::BaselineLsmCompactionExecutionEffects;
use super::{
    BaselineLsmCompactionPublicationReceipt, BaselineLsmCompactionRecordKind,
    BaselineLsmRunIdentity, InterlockedLsmCompaction, PublishedLsmCompaction,
};

#[derive(Debug)]
enum CompactionPublicationCase {
    Admitted(Box<PublishedLsmCompaction>),
    Denied(BaselineLsmExecutionAdmissionDenial),
}

#[derive(Debug)]
pub struct LsmCompactionPublicationOutcome {
    case: CompactionPublicationCase,
}

#[derive(Debug)]
pub enum LsmCompactionPublicationView<'a> {
    Admitted(&'a PublishedLsmCompaction),
    Denied(&'a BaselineLsmExecutionAdmissionDenial),
}

impl LsmCompactionPublicationOutcome {
    fn issue(result: Result<PublishedLsmCompaction, BaselineLsmExecutionAdmissionDenial>) -> Self {
        Self {
            case: match result {
                Ok(value) => CompactionPublicationCase::Admitted(Box::new(value)),
                Err(denial) => CompactionPublicationCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmCompactionPublicationView<'_> {
        match &self.case {
            CompactionPublicationCase::Admitted(value) => {
                LsmCompactionPublicationView::Admitted(value)
            }
            CompactionPublicationCase::Denied(denial) => {
                LsmCompactionPublicationView::Denied(denial)
            }
        }
    }

    pub fn into_result(
        self,
    ) -> Result<PublishedLsmCompaction, BaselineLsmExecutionAdmissionDenial> {
        match self.case {
            CompactionPublicationCase::Admitted(value) => Ok(*value),
            CompactionPublicationCase::Denied(denial) => Err(denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmExecutionOwnerCaseObservation {
        LsmExecutionOwnerCaseObservation::new(match &self.case {
            CompactionPublicationCase::Admitted(_) => {
                LsmExecutionOwnerCaseId::admitted(LsmExecutionOperation::PublishCompaction)
            }
            CompactionPublicationCase::Denied(denial) => LsmExecutionOwnerCaseId::denied(
                LsmExecutionOperation::PublishCompaction,
                denial.kind(),
            ),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmPublicationRuntime;

pub const fn lsm_publication_runtime() -> LsmPublicationRuntime {
    LsmPublicationRuntime
}

impl LsmPublicationRuntime {
    pub fn publish(
        self,
        session: &mut LsmMembershipSession,
        admission: BaselineLsmRunPublicationAdmission,
        interlocked: InterlockedLsmCompaction,
        activation: worth_store_lsm_authority::LsmMembershipActivationDeclaration,
        manifest: AdmittedCheckpointPublicationReceipt,
    ) -> LsmCompactionPublicationOutcome {
        LsmCompactionPublicationOutcome::issue(self.publish_inner(
            session,
            admission,
            interlocked,
            activation,
            manifest,
        ))
    }

    fn publish_inner(
        self,
        session: &mut LsmMembershipSession,
        admission: BaselineLsmRunPublicationAdmission,
        interlocked: InterlockedLsmCompaction,
        activation: worth_store_lsm_authority::LsmMembershipActivationDeclaration,
        manifest: AdmittedCheckpointPublicationReceipt,
    ) -> Result<PublishedLsmCompaction, BaselineLsmExecutionAdmissionDenial> {
        let maintenance_mode = admission
            .selected()
            .strategy_admission()
            .request()
            .maintenance_mode();
        let invariants = admission
            .selected()
            .admitted_strategy()
            .invariant_suite()
            .require_lsm_suite()
            .map_err(BaselineLsmExecutionAdmissionDenial::StrategyInvariant)?;
        let InterlockedLsmCompaction { prepared, physical } = interlocked;
        if admission.selected().request_identity().canonical_key()
            != prepared.membership.key_ref().canonical()
        {
            return Err(BaselineLsmExecutionAdmissionDenial::SelectedOperationKeyMismatch);
        }
        let selected = prepared.membership.identity_set();
        let output_generation = prepared.output.identity().sequence();
        if manifest.scope().covered_lsn_start() > selected.value().sequence()
            || manifest.scope().covered_lsn_end() < output_generation
        {
            return Err(BaselineLsmExecutionAdmissionDenial::ManifestDoesNotCoverCompaction);
        }
        let manifest = worth_store_lsm_authority::admit_lsm_membership_replacement(
            &prepared.membership,
            activation,
            manifest,
        )
        .map_err(super::super::execution::map_membership_denial)?;
        let membership_replacement =
            replace_lsm_membership(session, &prepared.membership, &manifest)
                .into_result()
                .map_err(super::super::execution::map_membership_denial)?;
        let retired = membership_replacement.retired_records();
        let identities = retired.in_replay_order();
        let retired_runs = identities.map(BaselineLsmRunIdentity::new_for_executor);
        let effects = BaselineLsmCompactionExecutionEffects::from_persisted_execution(retired_runs);
        let key = BaselineLsmCompactionPublicationReceipt::admitted_key(prepared.membership.key());
        let value_run = BaselineLsmCompactionPublicationReceipt::run(
            retired.value().sequence(),
            retired.value(),
        );
        let generation_run = BaselineLsmCompactionPublicationReceipt::run(
            retired.generation().sequence(),
            retired.generation(),
        );
        let tombstone_run = BaselineLsmCompactionPublicationReceipt::run(
            retired.tombstone().sequence(),
            retired.tombstone(),
        );
        let input_runs = [value_run, generation_run, tombstone_run];
        let manifest_publication =
            DurablePublicationDeclaration::manifest(manifest.scope().clone());
        let compaction = BaselineLsmCompactionPublicationReceipt::new(
            key,
            input_runs,
            BaselineLsmCompactionPublicationReceipt::run(
                output_generation,
                membership_replacement.output(),
            ),
            prepared.output.envelope().clone(),
            BaselineLsmCompactionPublicationReceipt::record(
                key,
                retired.tombstone(),
                tombstone_run,
                BaselineLsmCompactionRecordKind::Tombstone,
            ),
            BaselineLsmCompactionPublicationReceipt::record(
                key,
                retired.value(),
                value_run,
                BaselineLsmCompactionRecordKind::Value,
            ),
            manifest_publication.clone(),
            identities,
            wal_frame_bytes(prepared.replay_tail.value())
                + wal_frame_bytes(prepared.replay_tail.generation())
                + wal_frame_bytes(prepared.replay_tail.tombstone())
                + prepared
                    .membership
                    .base()
                    .map_or(0, |base| base.persisted_bytes()),
            wal_frame_bytes(prepared.output.envelope()),
            effects,
        );
        invariants
            .verify_owner_mutation_and_compaction(&compaction)
            .map_err(BaselineLsmExecutionAdmissionDenial::StrategyInvariant)?;
        Ok(PublishedLsmCompaction {
            maintenance_mode,
            memtable_records: [retired.tombstone()],
            sorted_run_records: [retired.value(), retired.generation()],
            wal_publication: prepared.output.envelope().clone(),
            manifest_publication,
            replay_tail: prepared.replay_tail,
            compaction,
            physical_compaction: physical,
            membership_replacement,
        })
    }
}

pub(super) fn owner_cases() -> impl Iterator<Item = LsmExecutionOwnerCaseDeclaration> {
    use BaselineLsmExecutionAdmissionDenialKind as Denial;
    const DENIALS: [Denial; 5] = [
        Denial::SelectedOperationKeyMismatch,
        Denial::ManifestDoesNotCoverCompaction,
        Denial::ManifestMembershipMismatch,
        Denial::OutputPublicationMismatch,
        Denial::PersistedMembershipStale,
    ];
    std::iter::once(LsmExecutionOwnerCaseDeclaration::new(
        LsmExecutionOwnerCaseId::admitted(LsmExecutionOperation::PublishCompaction),
    ))
    .chain(DENIALS.into_iter().map(|denial| {
        LsmExecutionOwnerCaseDeclaration::new(LsmExecutionOwnerCaseId::denied(
            LsmExecutionOperation::PublishCompaction,
            denial,
        ))
    }))
}

fn wal_frame_bytes(record: &BlobWalRecordEnvelope) -> u64 {
    let DurablePublicationScope::WalFrame(scope) = record.durable_publication().scope() else {
        unreachable!("BlobWalRecordEnvelope admits only WAL-frame publication")
    };
    scope.expected_bytes()
}
