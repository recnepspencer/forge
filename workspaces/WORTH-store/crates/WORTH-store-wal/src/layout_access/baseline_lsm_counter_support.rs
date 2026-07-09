use crate::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind,
    CheckpointDurablePublicationScope, DurablePublicationDeclaration,
    StoreCheckpointRecordIdentity, WalFrameDurablePublicationScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SeededLsmCompactionState {
    pub older_generation: u64,
    pub middle_generation: u64,
    pub newer_generation: u64,
    pub output_generation: u64,
    pub stale_runs_retired: bool,
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub rewritten_runs: u16,
}

pub(super) fn seeded_memtable_records() -> [BlobWalRecordIdentity; 1] {
    [
        BlobWalRecordIdentity::new(43, BlobWalRecordKind::ChunkAppend)
            .expect("non-zero memtable identity"),
    ]
}

pub(super) fn seeded_sorted_run_records() -> [BlobWalRecordIdentity; 2] {
    [
        BlobWalRecordIdentity::new(41, BlobWalRecordKind::GenerationPublication)
            .expect("non-zero sorted run identity"),
        BlobWalRecordIdentity::new(42, BlobWalRecordKind::GenerationPublication)
            .expect("non-zero sorted run identity"),
    ]
}

pub(super) fn seeded_wal_publication() -> BlobWalRecordEnvelope {
    BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(3, BlobWalRecordKind::RootCandidate)
            .expect("non-zero manifest candidate identity"),
        DurablePublicationDeclaration::wal_frame(wal_scope()),
        "sha256:payload",
    )
    .expect("wal record publication")
}

pub(super) fn seeded_manifest_publication() -> DurablePublicationDeclaration {
    DurablePublicationDeclaration::manifest(
        CheckpointDurablePublicationScope::new(
            StoreCheckpointRecordIdentity::new(1),
            "sha256:manifest",
            10,
            20,
        )
        .expect("valid checkpoint scope"),
    )
}

pub(super) fn seeded_replay_tail() -> [BlobWalRecordEnvelope; 3] {
    [
        wal_record(41, BlobWalRecordKind::ChunkAppend, "sha256:chunk-append"),
        wal_record(
            42,
            BlobWalRecordKind::GenerationPublication,
            "sha256:generation-publication",
        ),
        wal_record(
            43,
            BlobWalRecordKind::RootCandidate,
            "sha256:root-candidate",
        ),
    ]
}

pub(super) const fn seeded_compaction_state() -> SeededLsmCompactionState {
    SeededLsmCompactionState {
        older_generation: 41,
        middle_generation: 42,
        newer_generation: 43,
        output_generation: 44,
        stale_runs_retired: true,
        bytes_in: 1536,
        bytes_out: 2048,
        rewritten_runs: 3,
    }
}

pub(super) fn wal_record(
    sequence: u64,
    kind: BlobWalRecordKind,
    payload_digest: &str,
) -> BlobWalRecordEnvelope {
    BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(sequence, kind).expect("non-zero wal identity"),
        DurablePublicationDeclaration::wal_frame(wal_scope()),
        payload_digest,
    )
    .expect("wal record publication")
}

fn wal_scope() -> WalFrameDurablePublicationScope {
    WalFrameDurablePublicationScope::new(1, 1, 10, 20, "sha256:wal-frame", 4096)
        .expect("valid wal scope")
}
