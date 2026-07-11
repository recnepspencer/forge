use crate::{
    record_view::{reject_mismatched_framed_record, reject_unadmitted_view_profile},
    RecordCopyCounterSnapshot, RecordViewAdmission, RecordViewDenial, RecordViewDenialKind,
    RecordViewMaterializationProfile, ResidentFrameIdentity, ResidentFrameTable,
};
use forge_store_physical_format::FramedRecordView;

#[derive(Debug, Clone, Copy)]
struct ResidentRecordViewBasis {
    request: crate::ResidentFrameLoadRequest,
    resident_len: usize,
}

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
        let counters = self.begin_zero_copy_record_view_attempt();
        let resident_basis = self.load_record_view_basis(identity)?;
        self.verify_record_view_profile(profile, counters)?;
        self.verify_framed_record_matches_resident(resident_basis, framed_record, counters)?;
        Ok(self.complete_zero_copy_record_view_admission(resident_basis.request, framed_record))
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

    fn begin_zero_copy_record_view_attempt(&mut self) -> RecordCopyCounterSnapshot {
        self.record_view_counters = self.record_view_counters.with_zero_copy_attempt();
        self.record_view_counters
    }

    fn load_record_view_basis(
        &mut self,
        identity: ResidentFrameIdentity,
    ) -> Result<ResidentRecordViewBasis, RecordViewDenial> {
        let record = match self.record_at_slot(identity.slot()) {
            Ok(record) => record,
            Err(denial) => return Err(self.deny_record_view_from_resident(denial)),
        };
        if record.identity() != identity {
            return Err(self.deny_record_view(RecordViewDenialKind::ResidentLeaseDenied));
        }
        Ok(ResidentRecordViewBasis {
            request: record.request(),
            resident_len: record.bytes().map_or(0, |bytes| bytes.as_bytes().len()),
        })
    }

    fn verify_record_view_profile(
        &mut self,
        profile: RecordViewMaterializationProfile,
        counters: RecordCopyCounterSnapshot,
    ) -> Result<(), RecordViewDenial> {
        reject_unadmitted_view_profile(profile, counters)
            .map_err(|_| self.deny_record_view(RecordViewDenialKind::ProfileForbidsMaterialization))
    }

    fn verify_framed_record_matches_resident(
        &mut self,
        basis: ResidentRecordViewBasis,
        framed_record: FramedRecordView<'_>,
        counters: RecordCopyCounterSnapshot,
    ) -> Result<(), RecordViewDenial> {
        reject_mismatched_framed_record(basis.request, framed_record, basis.resident_len, counters)
            .map_err(|denial| self.deny_record_view(denial.kind()))
    }

    fn complete_zero_copy_record_view_admission(
        &mut self,
        request: crate::ResidentFrameLoadRequest,
        framed_record: FramedRecordView<'_>,
    ) -> RecordViewAdmission {
        self.record_view_counters = self.record_view_counters.with_zero_copy_admission();
        RecordViewAdmission::new(
            request,
            framed_record.placement(),
            self.record_view_counters,
        )
    }

    fn deny_record_view(&mut self, kind: RecordViewDenialKind) -> RecordViewDenial {
        self.record_view_counters = self
            .record_view_counters
            .with_denied_before_view_construction();
        RecordViewDenial::new(kind, self.record_view_counters)
    }

    fn deny_record_view_from_resident(
        &mut self,
        denial: crate::ResidentFrameDenial,
    ) -> RecordViewDenial {
        self.record_view_counters = self
            .record_view_counters
            .with_denied_before_view_construction();
        RecordViewDenial::from_resident(denial, self.record_view_counters)
    }
}
