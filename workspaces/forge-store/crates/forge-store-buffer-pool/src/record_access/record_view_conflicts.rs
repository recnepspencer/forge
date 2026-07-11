use crate::{
    ResidentFrameDenial, ResidentFrameDenialKind, ResidentFrameIdentity, ResidentFrameTable,
};

impl ResidentFrameTable {
    pub(crate) fn reject_dirty_mutation_behind_record_view(
        &mut self,
        identity: ResidentFrameIdentity,
    ) -> Result<(), ResidentFrameDenial> {
        if self.record_at_slot(identity.slot())?.has_active_pin() {
            self.record_view_counters = self
                .record_view_counters
                .with_dirty_mutation_conflict_denial();
            self.record_protected_mutation_denial();
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::DirtyMutationBehindLiveRecordView,
            ));
        }
        Ok(())
    }

    pub(crate) fn record_publication_conflict_behind_record_view(&mut self) {
        self.record_view_counters = self.record_view_counters.with_publication_conflict_denial();
    }
}
