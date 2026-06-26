use crate::{
    resident_frame_record::ResidentFrameRecord, LeaseLeakReport, LeaseScope, PageLease,
    PageLeaseId, PinLifecycleCloseoutReport, PinLifecycleCounterSnapshot, PinnedFrameView,
    ResidentFrameDenial, ResidentFrameDenialKind, ResidentFrameIdentity, ResidentFrameTable,
    ResidentFrameToken, UnpinnedPageReceipt,
};

impl ResidentFrameTable {
    pub fn lease_page(
        &mut self,
        token: ResidentFrameToken,
    ) -> Result<PageLease<'_>, ResidentFrameDenial> {
        let identity = self.validate_resident_token_for_lease(token)?;
        let lease_id = PageLeaseId::new(
            LeaseScope::new(identity),
            self.lease_epochs[token.slot().index() as usize],
        );
        Ok(PageLease::new(self, lease_id, identity))
    }

    pub fn pin_lifecycle_closeout(&mut self) -> PinLifecycleCloseoutReport {
        let leaked_pins = self.mark_unreported_leaked_pins();
        if leaked_pins > 0 {
            self.pin_counters = self.pin_counters.with_leaked_pins(leaked_pins);
            self.publish_pin_counters();
        }
        PinLifecycleCloseoutReport::new(self.pin_counters, self.counters)
    }

    pub fn leaked_pin_report(&mut self) -> Option<LeaseLeakReport> {
        let leaked_pins = self.mark_unreported_leaked_pins();
        if leaked_pins > 0 {
            self.pin_counters = self.pin_counters.with_leaked_pins(leaked_pins);
            self.publish_pin_counters();
        }
        self.first_active_pin_leak_report()
    }

    pub const fn pin_counters(&self) -> PinLifecycleCounterSnapshot {
        self.pin_counters
    }

    pub(crate) fn begin_pin(
        &mut self,
        lease_id: PageLeaseId,
        identity: ResidentFrameIdentity,
    ) -> Result<(), ResidentFrameDenial> {
        self.pin_counters = self.pin_counters.with_pin_attempt();
        self.reject_pin_budget_overflow()?;
        let record = self.record_for_current_lease_mut(lease_id, identity)?;
        if record.has_active_pin() {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::ResidentFramePinned,
            ));
        }
        record.mark_pinned();
        self.pin_counters = self.pin_counters.with_successful_pin();
        self.publish_pin_counters();
        Ok(())
    }

    pub(crate) fn pinned_frame_view(
        &self,
        identity: ResidentFrameIdentity,
    ) -> Result<PinnedFrameView<'_>, ResidentFrameDenial> {
        let record = self.record_at_slot(identity.slot())?;
        reject_stale_pin_identity(record, identity)?;
        reject_unpinned_frame(record)?;
        let bytes = record.bytes().ok_or_else(|| {
            ResidentFrameDenial::new(ResidentFrameDenialKind::ResidentBytesNotAdmitted)
        })?;
        Ok(PinnedFrameView::new(bytes.as_bytes()))
    }

    pub(crate) fn explicit_unpin(
        &mut self,
        lease_id: PageLeaseId,
        identity: ResidentFrameIdentity,
    ) -> Result<UnpinnedPageReceipt, ResidentFrameDenial> {
        let record = self.record_for_current_lease_mut(lease_id, identity)?;
        record.mark_unpinned()?;
        self.pin_counters = self.pin_counters.with_explicit_unpin();
        self.publish_pin_counters();
        Ok(UnpinnedPageReceipt::new(
            lease_id,
            identity,
            self.pin_counters,
        ))
    }

    pub(crate) fn defensive_drop_pin(&mut self, identity: ResidentFrameIdentity) {
        if let Ok(record) = self.record_at_slot_mut(identity.slot()) {
            if record.identity() == identity && record.has_active_pin() {
                record.clear_pin_after_abnormal_close();
                self.pin_counters = self.pin_counters.with_defensive_drop();
                self.publish_pin_counters();
            }
        }
    }

    pub(crate) fn record_protected_mutation_denial(&mut self) {
        self.pin_counters = self.pin_counters.with_protected_mutation_denial();
        self.publish_pin_counters();
    }

    pub(crate) fn validate_resident_token_for_lease(
        &self,
        token: ResidentFrameToken,
    ) -> Result<ResidentFrameIdentity, ResidentFrameDenial> {
        self.reject_slot_out_of_range(token.slot())?;
        let record = self.record_at_slot(token.slot())?;
        if record.identity().generation() != token.resident_generation() {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::StaleResidentGeneration,
            ));
        }
        Ok(record.identity())
    }

    fn record_for_current_lease_mut(
        &mut self,
        lease_id: PageLeaseId,
        identity: ResidentFrameIdentity,
    ) -> Result<&mut ResidentFrameRecord, ResidentFrameDenial> {
        let slot = identity.slot();
        self.reject_slot_out_of_range(slot)?;
        if self.lease_epochs[slot.index() as usize] != lease_id.epoch() {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::PageLeaseStale,
            ));
        }
        let record = self.record_at_slot_mut(slot)?;
        reject_stale_pin_identity(record, identity)?;
        Ok(record)
    }

    fn reject_pin_budget_overflow(&self) -> Result<(), ResidentFrameDenial> {
        let next_pins = self.pin_counters.active_pinned_pages() + 1;
        if next_pins > self.entry.admission().budget().pinned_pages().as_pages() as u64 {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::PinnedPageBudgetExceeded,
            ));
        }
        Ok(())
    }

    fn publish_pin_counters(&mut self) {
        self.counters = self.counters.with_pin_lifecycle(self.pin_counters);
    }

    fn mark_unreported_leaked_pins(&mut self) -> u64 {
        self.frames
            .iter_mut()
            .filter_map(Option::as_mut)
            .map(ResidentFrameRecord::mark_unreported_leak)
            .sum()
    }

    fn first_active_pin_leak_report(&self) -> Option<LeaseLeakReport> {
        self.frames
            .iter()
            .filter_map(Option::as_ref)
            .find(|record| record.has_active_pin())
            .map(|record| {
                LeaseLeakReport::new(
                    LeaseScope::new(record.identity()),
                    record.active_pin_count(),
                    self.pin_counters,
                )
            })
    }
}

fn reject_stale_pin_identity(
    record: &ResidentFrameRecord,
    identity: ResidentFrameIdentity,
) -> Result<(), ResidentFrameDenial> {
    if record.identity() != identity {
        return Err(ResidentFrameDenial::new(
            ResidentFrameDenialKind::StaleResidentGeneration,
        ));
    }
    Ok(())
}

fn reject_unpinned_frame(record: &ResidentFrameRecord) -> Result<(), ResidentFrameDenial> {
    if !record.has_active_pin() {
        return Err(ResidentFrameDenial::new(
            ResidentFrameDenialKind::PageLeaseNotPinned,
        ));
    }
    Ok(())
}
