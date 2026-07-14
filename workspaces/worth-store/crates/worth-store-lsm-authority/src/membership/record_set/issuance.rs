use crate::BlobWalRecordKind;

use super::LsmCompactionRecordSet;
use crate::membership::{LsmMembershipDenial, LsmMembershipKey, LsmMembershipRecord};

impl LsmCompactionRecordSet {
    pub(in crate::membership) fn issue(
        key: LsmMembershipKey,
        value: LsmMembershipRecord,
        generation: LsmMembershipRecord,
        tombstone: LsmMembershipRecord,
    ) -> Result<Self, LsmMembershipDenial> {
        require_role(&value, key, BlobWalRecordKind::LsmValue)
            .map_err(|()| LsmMembershipDenial::ValueRecordRequired)?;
        require_role(&generation, key, BlobWalRecordKind::GenerationPublication)
            .map_err(|()| LsmMembershipDenial::GenerationRecordRequired)?;
        require_role(&tombstone, key, BlobWalRecordKind::LsmTombstone)
            .map_err(|()| LsmMembershipDenial::TombstoneRecordRequired)?;
        Ok(Self {
            value,
            generation,
            tombstone,
        })
    }
}

fn require_role(
    record: &LsmMembershipRecord,
    key: LsmMembershipKey,
    kind: BlobWalRecordKind,
) -> Result<(), ()> {
    (record.key() == key && record.kind() == kind)
        .then_some(())
        .ok_or(())
}
