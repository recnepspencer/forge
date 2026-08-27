use crate::durability::data::{DurabilityError, DurableCheckpoint, DurableStore};
use crate::history::data::PositionedCanonicalCommit;
use crate::runtime::RelationalRuntime;

pub(crate) trait DurabilityRead {
    fn durable_checkpoints(&self) -> &[DurableCheckpoint];
    fn durable_store(&self) -> Option<&DurableStore>;
    fn durable_log(&self) -> &[PositionedCanonicalCommit];
}

impl DurabilityRead for RelationalRuntime {
    fn durable_checkpoints(&self) -> &[DurableCheckpoint] {
        &self.durability.checkpoints
    }

    fn durable_store(&self) -> Option<&DurableStore> {
        self.durability.store.as_ref()
    }

    fn durable_log(&self) -> &[PositionedCanonicalCommit] {
        &self.durability.log
    }
}

pub(crate) trait DurabilityWrite {
    fn append_durable_envelope(
        &mut self,
        authority: crate::durability::authority::DurableAppendAuthority,
        commit: &PositionedCanonicalCommit,
    ) -> Result<(), DurabilityError>;
}

impl DurabilityWrite for RelationalRuntime {
    fn append_durable_envelope(
        &mut self,
        authority: crate::durability::authority::DurableAppendAuthority,
        commit: &PositionedCanonicalCommit,
    ) -> Result<(), DurabilityError> {
        self.durability_authority().append_commit(authority, commit)
    }
}
