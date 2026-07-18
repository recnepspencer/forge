use super::durability::{
    begin_durability_fixture, durable_record_binding, durable_record_binding_with_lsn,
};
use super::{
    admitted_store_wal_checkpoint_security_scope_for_layout_partition_test, layout_lsm_maintenance,
    lsm_membership_replacement_crash_fixture, lsm_strategy, BlobWalRecordKind,
    LsmCompactionAdmissionRequest, PreExecutionBudgetEnvelope, StoreKeyVersionPosture,
    StoreLegacySecurityPosture, StoreWalRecordIdentity, WalRecordFamily,
};
use worth_store_lsm_authority::{
    AdmittedLsmReplaySource, LsmMembershipDenial, LsmMembershipRecord, LsmReplaySourceDenial,
};
use worth_store_wal::{AdmittedWalAppendReceipt, BlobWalRecordEnvelope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmReplayHostileMatrix {
    permutation_denials: [LsmReplaySourceDenial; 5],
    duplicate_sequence_denial: LsmReplaySourceDenial,
    unsupported_kind_denials: [LsmMembershipDenial; 2],
    retired_membership_denial: LsmMembershipDenial,
}

impl LsmReplayHostileMatrix {
    pub const fn permutation_denials(&self) -> &[LsmReplaySourceDenial; 5] {
        &self.permutation_denials
    }

    pub const fn duplicate_sequence_denial(&self) -> LsmReplaySourceDenial {
        self.duplicate_sequence_denial
    }

    pub const fn unsupported_kind_denials(&self) -> &[LsmMembershipDenial; 2] {
        &self.unsupported_kind_denials
    }

    pub const fn retired_membership_denial(&self) -> LsmMembershipDenial {
        self.retired_membership_denial
    }
}

pub fn execute_lsm_replay_hostile_matrix() -> LsmReplayHostileMatrix {
    let permutations = [
        [41, 43, 42],
        [42, 41, 43],
        [42, 43, 41],
        [43, 41, 42],
        [43, 42, 41],
    ];
    let permutation_denials = permutations.map(replay_denial);
    let duplicate_sequence_denial = replay_denial([41, 41, 43]);
    let unsupported_kind_denials = [
        unsupported_kind_denial(BlobWalRecordKind::ChunkAppend),
        unsupported_kind_denial(BlobWalRecordKind::RootCandidate),
    ];

    let replaced = lsm_membership_replacement_crash_fixture();
    let reopened = super::open_lsm_index(replaced.anchor()).unwrap();
    let retired_membership_denial =
        worth_store_lsm_authority::select_lsm_compaction_membership(&reopened, replaced.key())
            .into_result()
            .unwrap_err();

    LsmReplayHostileMatrix {
        permutation_denials,
        duplicate_sequence_denial,
        unsupported_kind_denials,
        retired_membership_denial,
    }
}

pub fn execute_frontierless_lsm_replay_source_fixture() -> AdmittedLsmReplaySource {
    let last = u64::MAX;
    let (access, key) = admitted_index(last);
    let records = [
        durable_record_binding_with_lsn(key, last - 2, BlobWalRecordKind::LsmValue, 1, 1, 41),
        durable_record_binding_with_lsn(
            key,
            last - 1,
            BlobWalRecordKind::GenerationPublication,
            1,
            1,
            42,
        ),
        durable_record_binding_with_lsn(key, last, BlobWalRecordKind::LsmTombstone, 1, 1, 43),
    ];
    let mut session = super::open_lsm_index(&records[0].1).unwrap();
    for (envelope, receipt) in records {
        access
            .persist_record(&mut session, envelope, &receipt, key)
            .unwrap();
    }
    let membership = worth_store_lsm_authority::select_lsm_compaction_membership(&session, key)
        .into_result()
        .unwrap();
    AdmittedLsmReplaySource::admit_recovered_membership(membership, None, None).unwrap()
}

fn replay_denial(sequences: [u64; 3]) -> LsmReplaySourceDenial {
    let (access, key) = admitted_index(sequences[2]);
    let kinds = [
        BlobWalRecordKind::LsmValue,
        BlobWalRecordKind::GenerationPublication,
        BlobWalRecordKind::LsmTombstone,
    ];
    let records: [(BlobWalRecordEnvelope, AdmittedWalAppendReceipt); 3] =
        std::array::from_fn(|index| {
            durable_record_binding_with_lsn(
                key,
                sequences[index],
                kinds[index],
                1,
                1,
                50 + index as u64,
            )
        });
    let mut session = super::open_lsm_index(&records[0].1).unwrap();
    for (envelope, receipt) in records {
        access
            .persist_record(&mut session, envelope, &receipt, key)
            .unwrap();
    }
    let membership = worth_store_lsm_authority::select_lsm_compaction_membership(&session, key)
        .into_result()
        .unwrap();
    AdmittedLsmReplaySource::admit_recovered_membership(membership, None, None).unwrap_err()
}

fn unsupported_kind_denial(kind: BlobWalRecordKind) -> LsmMembershipDenial {
    let (access, key) = admitted_index(99);
    let (anchor_envelope, anchor) = durable_record_binding(key, 91, BlobWalRecordKind::LsmValue);
    let mut session = super::open_lsm_index(&anchor).unwrap();
    access
        .persist_record(&mut session, anchor_envelope, &anchor, key)
        .unwrap();
    let (envelope, receipt) = durable_record_binding(key, 92, kind);
    let record = LsmMembershipRecord::admit(envelope, &receipt, key).unwrap();
    worth_store_lsm_authority::persist_lsm_membership_record(&mut session, record)
        .into_result()
        .unwrap_err()
}

fn admitted_index(
    sequence: u64,
) -> (
    worth_store_layout_indexes::LsmStrategy,
    worth_store_lsm_authority::LsmMembershipKey,
) {
    begin_durability_fixture();
    let access = lsm_strategy();
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let metadata = worth_store_wal::WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let compaction = layout_lsm_maintenance()
        .admit_compaction(LsmCompactionAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(sequence),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .unwrap();
    let key = access.admit_key(metadata, compaction).unwrap();
    (access, key)
}

#[test]
fn independently_admitted_replay_sources_do_not_alias() {
    let (access, key) = admitted_index(43);
    let records = [
        durable_record_binding(key, 41, BlobWalRecordKind::LsmValue),
        durable_record_binding(key, 42, BlobWalRecordKind::GenerationPublication),
        durable_record_binding(key, 43, BlobWalRecordKind::LsmTombstone),
    ];
    let mut session = super::open_lsm_index(&records[0].1).unwrap();
    for (envelope, receipt) in records {
        access
            .persist_record(&mut session, envelope, &receipt, key)
            .unwrap();
    }
    let membership = worth_store_lsm_authority::select_lsm_compaction_membership(&session, key)
        .into_result()
        .unwrap();
    let first = AdmittedLsmReplaySource::admit_recovered_membership(membership.clone(), None, None)
        .unwrap();
    let cloned = first.clone();
    let second =
        AdmittedLsmReplaySource::admit_recovered_membership(membership, None, None).unwrap();

    assert_eq!(first.identity(), cloned.identity());
    assert_ne!(first.identity(), second.identity());
}
