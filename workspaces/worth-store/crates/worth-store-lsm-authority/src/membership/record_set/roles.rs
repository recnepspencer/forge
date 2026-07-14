use crate::BlobWalRecordIdentity;

use super::super::LsmMembershipRecord;

/// The three semantically distinct durable records required by compaction.
///
/// Private fields prevent callers from substituting positional records for
/// owner-admitted roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmCompactionRecordSet {
    pub(super) value: LsmMembershipRecord,
    pub(super) generation: LsmMembershipRecord,
    pub(super) tombstone: LsmMembershipRecord,
}

impl LsmCompactionRecordSet {
    pub const fn value(&self) -> &LsmMembershipRecord {
        &self.value
    }

    pub const fn generation(&self) -> &LsmMembershipRecord {
        &self.generation
    }

    pub const fn tombstone(&self) -> &LsmMembershipRecord {
        &self.tombstone
    }

    pub fn identities(&self) -> [BlobWalRecordIdentity; 3] {
        self.identity_set().in_replay_order()
    }

    pub fn identity_set(&self) -> super::LsmCompactionRecordIdentitySet {
        super::LsmCompactionRecordIdentitySet::from_records(self)
    }

    pub(in crate::membership) fn iter(&self) -> impl Iterator<Item = &LsmMembershipRecord> {
        [&self.value, &self.generation, &self.tombstone].into_iter()
    }
}
