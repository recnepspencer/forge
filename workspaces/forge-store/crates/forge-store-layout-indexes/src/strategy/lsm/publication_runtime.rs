use forge_store_lsm_authority::LsmMembershipSession;
use forge_store_wal::{
    AdmittedCheckpointPublicationReceipt, BlobWalRecordEnvelope, DurablePublicationDeclaration,
    DurablePublicationScope,
};

use super::super::{
    BaselineLsmCompactionPublicationReceipt, BaselineLsmExecutionAdmissionDenial,
    BaselineLsmRunIdentity, BaselineLsmRunPublicationAdmission, InterlockedLsmCompaction,
    PublishedLsmCompaction,
};
use super::baseline_lsm_compaction_execution::{
    BaselineLsmCompactionExecutionEffects, BaselineLsmCompactionRecordKind,
};

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
        activation: forge_store_lsm_authority::LsmMembershipActivationDeclaration,
        manifest: AdmittedCheckpointPublicationReceipt,
    ) -> Result<PublishedLsmCompaction, BaselineLsmExecutionAdmissionDenial> {
        let invariants = admission
            .selected()
            .admitted_strategy()
            .expect("LSM publication selection retains admitted strategy")
            .invariant_suite()
            .require_lsm_suite()
            .map_err(BaselineLsmExecutionAdmissionDenial::StrategyInvariant)?;
        let InterlockedLsmCompaction { prepared, physical } = interlocked;
        if admission.selected().request_identity().canonical_key()
            != prepared.membership.key_ref().canonical()
        {
            return Err(BaselineLsmExecutionAdmissionDenial::SelectedOperationKeyMismatch);
        }
        let identities = prepared.membership.identities();
        let output_generation = prepared.output.identity().sequence();
        if manifest.scope().covered_lsn_start() > identities[0].sequence()
            || manifest.scope().covered_lsn_end() < output_generation
        {
            return Err(BaselineLsmExecutionAdmissionDenial::ManifestDoesNotCoverCompaction);
        }
        let manifest = forge_store_lsm_authority::admit_lsm_membership_replacement(
            &prepared.membership,
            activation,
            manifest,
        )
        .map_err(super::map_membership_denial)?;
        let membership_replacement = session
            .replace(&prepared.membership, &manifest)
            .map_err(super::map_membership_denial)?;
        let identities = *membership_replacement.retired();
        let retired_runs = identities.map(BaselineLsmRunIdentity::new_for_executor);
        let effects = BaselineLsmCompactionExecutionEffects::from_persisted_execution(retired_runs);
        let key = BaselineLsmCompactionPublicationReceipt::admitted_key(prepared.membership.key());
        let input_runs = std::array::from_fn(|index| {
            BaselineLsmCompactionPublicationReceipt::run(
                identities[index].sequence(),
                identities[index],
            )
        });
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
                identities[2],
                input_runs[2],
                BaselineLsmCompactionRecordKind::Tombstone,
            ),
            BaselineLsmCompactionPublicationReceipt::record(
                key,
                identities[0],
                input_runs[0],
                BaselineLsmCompactionRecordKind::Value,
            ),
            manifest_publication.clone(),
            identities,
            prepared
                .replay_tail
                .iter()
                .map(wal_frame_bytes)
                .sum::<u64>()
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
            memtable_records: [identities[2]],
            sorted_run_records: [identities[0], identities[1]],
            wal_publication: prepared.output.envelope().clone(),
            manifest_publication,
            replay_tail: prepared.replay_tail,
            compaction,
            physical_compaction: physical,
            membership_replacement,
        })
    }
}

fn wal_frame_bytes(record: &BlobWalRecordEnvelope) -> u64 {
    let DurablePublicationScope::WalFrame(scope) = record.durable_publication().scope() else {
        unreachable!("BlobWalRecordEnvelope admits only WAL-frame publication")
    };
    scope.expected_bytes()
}
