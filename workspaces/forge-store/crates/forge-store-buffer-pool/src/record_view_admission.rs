use crate::{
    record_view::{reject_mismatched_framed_record, reject_unadmitted_view_profile},
    RecordCopyCounterSnapshot, RecordViewAdmission, RecordViewDenial, RecordViewDenialKind,
    RecordViewMaterializationProfile, ResidentFrameIdentity, ResidentFrameTable,
};
use forge_store_physical_format::FramedRecordView;

impl ResidentFrameTable {
    pub const fn record_view_counters(&self) -> RecordCopyCounterSnapshot {
        self.record_view_counters
    }

    pub(crate) fn admit_record_view_basis(
        &mut self,
        identity: ResidentFrameIdentity,
        framed_record: FramedRecordView<'_>,
        profile: RecordViewMaterializationProfile,
    ) -> Result<RecordViewAdmission, RecordViewDenial> {
        self.record_view_counters = self.record_view_counters.with_zero_copy_attempt();
        let counters = self.record_view_counters;
        let record = match self.record_at_slot(identity.slot()) {
            Ok(record) => record,
            Err(denial) => {
                self.record_view_counters = self
                    .record_view_counters
                    .with_denied_before_view_construction();
                return Err(RecordViewDenial::from_resident(
                    denial,
                    self.record_view_counters,
                ));
            }
        };
        if record.identity() != identity {
            self.record_view_counters = self
                .record_view_counters
                .with_denied_before_view_construction();
            return Err(RecordViewDenial::new(
                RecordViewDenialKind::ResidentLeaseDenied,
                self.record_view_counters,
            ));
        }
        let request = record.request();
        let resident_len = record.bytes().map_or(0, |bytes| bytes.as_bytes().len());
        let placement = framed_record.placement();
        if reject_unadmitted_view_profile(profile, counters).is_err() {
            self.record_view_counters = self
                .record_view_counters
                .with_denied_before_view_construction();
            return Err(RecordViewDenial::new(
                RecordViewDenialKind::ProfileForbidsMaterialization,
                self.record_view_counters,
            ));
        }
        if let Err(denial) =
            reject_mismatched_framed_record(request, framed_record, resident_len, counters)
        {
            self.record_view_counters = self
                .record_view_counters
                .with_denied_before_view_construction();
            return Err(RecordViewDenial::new(
                denial.kind(),
                self.record_view_counters,
            ));
        }
        self.record_view_counters = self.record_view_counters.with_zero_copy_admission();
        Ok(RecordViewAdmission::new(
            request,
            placement,
            self.record_view_counters,
        ))
    }

    pub(crate) fn deny_mutable_record_view_without_exclusive_lease(&mut self) -> RecordViewDenial {
        self.record_view_counters = self
            .record_view_counters
            .with_zero_copy_attempt()
            .with_denied_before_view_construction();
        RecordViewDenial::new(
            RecordViewDenialKind::MutableViewRequiresExclusiveLease,
            self.record_view_counters,
        )
    }
}
