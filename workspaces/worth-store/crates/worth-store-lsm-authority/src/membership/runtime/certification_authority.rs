use super::super::{LsmCompactionRecordIdentitySet, LsmMembershipKey};
use super::replacement::{
    PublishedLsmMembershipIdentity, PublishedLsmMembershipOutputArtifact,
    PublishedLsmMembershipReplacement,
};
use crate::{
    BlobWalRecordIdentity, BlobWalRecordKind, CheckpointDurablePublicationScope,
    StoreCheckpointRecordIdentity, WalFrameDurablePublicationScope,
};

/// Issues the smallest synthetic published-membership fact needed to falsify
/// consumers of LSM publication authority in certification-only unit worlds.
///
/// Integrated persistence and reopen claims must continue to use the real
/// replacement lifecycle; this issuer owns no durable-media evidence.
pub fn issue_published_lsm_membership_for_certification(
    key: LsmMembershipKey,
) -> PublishedLsmMembershipReplacement {
    let value = record_identity(41, BlobWalRecordKind::LsmValue);
    let generation = record_identity(42, BlobWalRecordKind::GenerationPublication);
    let tombstone = record_identity(43, BlobWalRecordKind::LsmTombstone);
    let output = record_identity(44, BlobWalRecordKind::GenerationPublication);
    let retired =
        LsmCompactionRecordIdentitySet::issued_for_certification(value, generation, tombstone);
    let output_scope = WalFrameDurablePublicationScope::new(
        1,
        1,
        tombstone.sequence(),
        output.sequence(),
        "certification-lsm-output",
        1,
    )
    .expect("certification output scope is valid");
    let activation_scope = CheckpointDurablePublicationScope::new(
        StoreCheckpointRecordIdentity::new(1),
        "certification-lsm-membership",
        value.sequence(),
        output.sequence(),
    )
    .expect("certification activation scope is valid");

    PublishedLsmMembershipReplacement::issued(
        PublishedLsmMembershipIdentity::from_activation_bytes(
            b"worth-store:certification-lsm-membership:v1",
        ),
        key,
        retired,
        output,
        output_scope,
        activation_scope,
        PublishedLsmMembershipOutputArtifact::new(
            std::path::PathBuf::from("certification-lsm-output"),
            0,
            1,
        ),
    )
}

fn record_identity(sequence: u64, kind: BlobWalRecordKind) -> BlobWalRecordIdentity {
    BlobWalRecordIdentity::new(sequence, kind)
        .expect("certification LSM record sequence is nonzero")
}
